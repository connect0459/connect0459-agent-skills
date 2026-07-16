---
name: whisper-transcribe-transcribe
argument-hint: "<input.wav> [--format srt|txt|vtt|json] [--model NAME] [--output-prefix PATH] [--language CODE] [--gpu|--no-gpu]"
description: >-
  Internal transcription backend wrapping the `~/.local/bin/whisper-transcribe` binary.
  Not intended for direct user invocation — called by
  `/connect0459-agent-skills:youtube-transcribe-local` to transcribe a 16 kHz mono PCM WAV
  file into SRT, TXT, VTT, or JSON.
---

# whisper-transcribe-transcribe

Internal transcription backend. Wraps the `~/.local/bin/whisper-transcribe` binary built by
`/connect0459-agent-skills:whisper-transcribe-setup`. Not intended for direct user invocation —
called by `/connect0459-agent-skills:youtube-transcribe-local`.

## CLI

```text
whisper-transcribe <input.wav>
    [--format srt|txt|vtt|json]   default: srt
    [--model NAME]                default: large-v3-turbo
    [--output-prefix PATH]        default: next to input (stem only, no extension)
    [--language CODE]             default: auto-detect
    [--gpu|--no-gpu]              default: auto (GPU outside sandbox, CPU inside)
```

## Behaviour

- Input must be a 16 kHz mono PCM WAV — produced by ffmpeg in `youtube-transcribe-local`.
- Model must exist at `~/.local/share/whisper-transcribe/models/ggml-<model>.bin`.
- For `--format srt|vtt|json`: writes `<prefix>.<ext>` and prints `wrote <path>`.
- For `--format txt`: writes plain transcript to stdout.
- GPU auto-detection: CPU inside Claude Code sandbox (`SANDBOX_RUNTIME=1` or
  `CODEX_SANDBOX=seatbelt`), GPU otherwise. Override with `--gpu` or `--no-gpu`.

## Missing install

If the binary or model is missing, the binary exits with a hint to run
`/connect0459-agent-skills:whisper-transcribe-setup`.
