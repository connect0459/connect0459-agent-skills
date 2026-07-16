use std::ffi::OsString;
use std::path::{Path, PathBuf};

fn centisecs_to_components(centisecs: i64) -> (i64, i64, i64, i64) {
    let cs = centisecs.max(0);
    let total_ms = cs * 10;
    let ms = total_ms % 1000;
    let total_s = total_ms / 1000;
    let s = total_s % 60;
    let total_m = total_s / 60;
    let m = total_m % 60;
    let h = total_m / 60;
    (h, m, s, ms)
}

pub fn format_timestamp_srt(centisecs: i64) -> String {
    let (h, m, s, ms) = centisecs_to_components(centisecs);
    format!("{:02}:{:02}:{:02},{:03}", h, m, s, ms)
}

pub fn format_timestamp_vtt(centisecs: i64) -> String {
    let (h, m, s, ms) = centisecs_to_components(centisecs);
    format!("{:02}:{:02}:{:02}.{:03}", h, m, s, ms)
}

pub fn output_prefix_for(input: &Path, given: Option<&Path>) -> PathBuf {
    given.map(|p| p.to_path_buf()).unwrap_or_else(|| {
        let parent = input.parent().unwrap_or(Path::new("."));
        let stem = input.file_stem().unwrap_or_default();
        parent.join(stem)
    })
}

/// Append `.ext` to `prefix` by string concatenation, avoiding
/// `PathBuf::with_extension` which strips any existing dot-segment
/// (e.g. `"React 18.2 Explained [id]"` → `"React 18.srt"` instead of
/// `"React 18.2 Explained [id].srt"`).
pub fn append_extension(prefix: &Path, ext: &str) -> PathBuf {
    let mut s: OsString = prefix.as_os_str().to_owned();
    s.push(".");
    s.push(ext);
    PathBuf::from(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn srt_zero() {
        assert_eq!(format_timestamp_srt(0), "00:00:00,000");
    }

    #[test]
    fn srt_one_second() {
        assert_eq!(format_timestamp_srt(100), "00:00:01,000");
    }

    #[test]
    fn srt_one_minute() {
        assert_eq!(format_timestamp_srt(6000), "00:01:00,000");
    }

    #[test]
    fn srt_one_hour() {
        assert_eq!(format_timestamp_srt(360000), "01:00:00,000");
    }

    #[test]
    fn srt_complex() {
        // 1h 2m 3.45s = 372345 centisecs
        assert_eq!(format_timestamp_srt(372345), "01:02:03,450");
    }

    #[test]
    fn srt_negative_clamped_to_zero() {
        // whisper-rs may return negative t0 for silent leading audio
        assert_eq!(format_timestamp_srt(-50), "00:00:00,000");
    }

    #[test]
    fn vtt_separator_is_dot() {
        assert_eq!(format_timestamp_vtt(0), "00:00:00.000");
        assert_eq!(format_timestamp_vtt(372345), "01:02:03.450");
    }

    #[test]
    fn vtt_negative_clamped_to_zero() {
        assert_eq!(format_timestamp_vtt(-1), "00:00:00.000");
    }

    #[test]
    fn output_prefix_from_input_no_given() {
        let prefix = output_prefix_for(Path::new("/tmp/audio.wav"), None);
        assert_eq!(prefix, PathBuf::from("/tmp/audio"));
    }

    #[test]
    fn output_prefix_given_overrides() {
        let prefix = output_prefix_for(Path::new("/tmp/audio.wav"), Some(Path::new("/out/result")));
        assert_eq!(prefix, PathBuf::from("/out/result"));
    }

    #[test]
    fn append_extension_simple() {
        assert_eq!(
            append_extension(Path::new("/tmp/audio"), "srt"),
            PathBuf::from("/tmp/audio.srt")
        );
    }

    #[test]
    fn append_extension_dotted_title_not_broken() {
        // PathBuf::with_extension on "React 18.2 Explained [id]" would treat
        // "2 Explained [id]" as the existing extension and produce "React 18.srt".
        // append_extension appends literally, preserving the full stem.
        assert_eq!(
            append_extension(Path::new("/dir/React 18.2 Explained [id]"), "srt"),
            PathBuf::from("/dir/React 18.2 Explained [id].srt")
        );
    }
}
