mod format;

use std::fs;
use std::io::Write;
use std::path::PathBuf;
use clap::{Parser, ValueEnum};
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

use format::{format_timestamp_srt, format_timestamp_vtt, output_prefix_for};

#[derive(Debug, Clone, ValueEnum)]
enum Format {
    Srt,
    Txt,
    Vtt,
    Json,
}

#[derive(Parser, Debug)]
#[command(
    name = "whisper-transcribe",
    about = "Transcribe audio using whisper.cpp via Rust bindings"
)]
struct Args {
    /// Input WAV file (16 kHz mono PCM, produced by ffmpeg)
    input: PathBuf,

    /// Output format
    #[arg(long, value_enum, default_value = "srt")]
    format: Format,

    /// Model name — ggml-<name>.bin in the models directory
    #[arg(long, default_value = "large-v3-turbo")]
    model: String,

    /// Output file prefix; default is next to the input file
    #[arg(long)]
    output_prefix: Option<PathBuf>,

    /// Language code (e.g. en, ja); omit for auto-detect
    #[arg(long)]
    language: Option<String>,

    /// Force GPU/Metal acceleration
    #[arg(long, conflicts_with = "no_gpu")]
    gpu: bool,

    /// Force CPU mode
    #[arg(long, conflicts_with = "gpu")]
    no_gpu: bool,
}

fn models_dir() -> PathBuf {
    let home = std::env::var("HOME").expect("HOME not set");
    PathBuf::from(home).join(".local/share/whisper-transcribe/models")
}

fn load_samples(path: &std::path::Path) -> Result<Vec<f32>, Box<dyn std::error::Error>> {
    let mut reader = hound::WavReader::open(path)?;
    let spec = reader.spec();
    let samples: Vec<f32> = match spec.sample_format {
        hound::SampleFormat::Int => {
            let max = (1i64 << (spec.bits_per_sample - 1)) as f32;
            reader
                .samples::<i32>()
                .map(|s| s.map(|v| v as f32 / max))
                .collect::<Result<_, _>>()?
        }
        hound::SampleFormat::Float => reader.samples::<f32>().collect::<Result<_, _>>()?,
    };
    Ok(samples)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let model_path = models_dir().join(format!("ggml-{}.bin", args.model));
    if !model_path.exists() {
        eprintln!("error: model not found at {}", model_path.display());
        eprintln!(
            "hint: run /connect0459-agent-skills:whisper-transcribe-setup to download the default model"
        );
        std::process::exit(1);
    }

    if !args.input.exists() {
        eprintln!("error: input not found: {}", args.input.display());
        std::process::exit(1);
    }

    let use_gpu = if args.no_gpu {
        false
    } else if args.gpu {
        true
    } else {
        std::env::var("SANDBOX_RUNTIME").as_deref() != Ok("1")
            && std::env::var("CODEX_SANDBOX").as_deref() != Ok("seatbelt")
    };

    let samples = load_samples(&args.input)?;

    let mut ctx_params = WhisperContextParameters::default();
    ctx_params.use_gpu(use_gpu);
    let ctx = WhisperContext::new_with_params(model_path.to_str().unwrap(), ctx_params)?;

    let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
    params.set_print_progress(false);
    params.set_print_realtime(false);
    params.set_print_timestamps(false);
    params.set_print_special(false);
    params.set_n_max_text_ctx(0);

    if let Some(ref lang) = args.language {
        params.set_language(Some(lang.as_str()));
    }

    let mut state = ctx.create_state()?;
    state.full(params, &samples)?;

    let n_segments = state.full_n_segments()?;
    let prefix = output_prefix_for(&args.input, args.output_prefix.as_deref());

    match args.format {
        Format::Txt => {
            let stdout = std::io::stdout();
            let mut out = stdout.lock();
            for i in 0..n_segments {
                let text = state.full_get_segment_text(i)?;
                writeln!(out, "{}", text.trim())?;
            }
        }
        Format::Srt => {
            let out_path = prefix.with_extension("srt");
            let mut out = fs::File::create(&out_path)?;
            for i in 0..n_segments {
                let text = state.full_get_segment_text(i)?;
                let t0 = state.full_get_segment_t0(i)?;
                let t1 = state.full_get_segment_t1(i)?;
                writeln!(out, "{}", i + 1)?;
                writeln!(
                    out,
                    "{} --> {}",
                    format_timestamp_srt(t0),
                    format_timestamp_srt(t1)
                )?;
                writeln!(out, "{}", text.trim())?;
                writeln!(out)?;
            }
            println!("wrote {}", out_path.display());
        }
        Format::Vtt => {
            let out_path = prefix.with_extension("vtt");
            let mut out = fs::File::create(&out_path)?;
            writeln!(out, "WEBVTT")?;
            writeln!(out)?;
            for i in 0..n_segments {
                let text = state.full_get_segment_text(i)?;
                let t0 = state.full_get_segment_t0(i)?;
                let t1 = state.full_get_segment_t1(i)?;
                writeln!(
                    out,
                    "{} --> {}",
                    format_timestamp_vtt(t0),
                    format_timestamp_vtt(t1)
                )?;
                writeln!(out, "{}", text.trim())?;
                writeln!(out)?;
            }
            println!("wrote {}", out_path.display());
        }
        Format::Json => {
            let out_path = prefix.with_extension("json");
            let mut segments = Vec::new();
            for i in 0..n_segments {
                let text = state.full_get_segment_text(i)?;
                let t0 = state.full_get_segment_t0(i)?;
                let t1 = state.full_get_segment_t1(i)?;
                segments.push(serde_json::json!({
                    "id": i,
                    "t0_cs": t0,
                    "t1_cs": t1,
                    "text": text.trim()
                }));
            }
            let output = serde_json::json!({ "segments": segments });
            fs::write(&out_path, serde_json::to_string_pretty(&output)?)?;
            println!("wrote {}", out_path.display());
        }
    }

    Ok(())
}
