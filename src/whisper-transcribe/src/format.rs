use std::path::{Path, PathBuf};

pub fn format_timestamp_srt(centisecs: i64) -> String {
    let total_ms = centisecs * 10;
    let ms = total_ms % 1000;
    let total_s = total_ms / 1000;
    let s = total_s % 60;
    let total_m = total_s / 60;
    let m = total_m % 60;
    let h = total_m / 60;
    format!("{:02}:{:02}:{:02},{:03}", h, m, s, ms)
}

pub fn format_timestamp_vtt(centisecs: i64) -> String {
    let total_ms = centisecs * 10;
    let ms = total_ms % 1000;
    let total_s = total_ms / 1000;
    let s = total_s % 60;
    let total_m = total_s / 60;
    let m = total_m % 60;
    let h = total_m / 60;
    format!("{:02}:{:02}:{:02}.{:03}", h, m, s, ms)
}

pub fn output_prefix_for(input: &Path, given: Option<&Path>) -> PathBuf {
    given.map(|p| p.to_path_buf()).unwrap_or_else(|| {
        let parent = input.parent().unwrap_or(Path::new("."));
        let stem = input.file_stem().unwrap_or_default();
        parent.join(stem)
    })
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
    fn vtt_separator_is_dot() {
        assert_eq!(format_timestamp_vtt(0), "00:00:00.000");
        assert_eq!(format_timestamp_vtt(372345), "01:02:03.450");
    }

    #[test]
    fn output_prefix_from_input_no_given() {
        let prefix = output_prefix_for(Path::new("/tmp/audio.wav"), None);
        assert_eq!(prefix, PathBuf::from("/tmp/audio"));
    }

    #[test]
    fn output_prefix_given_overrides() {
        let prefix = output_prefix_for(
            Path::new("/tmp/audio.wav"),
            Some(Path::new("/out/result")),
        );
        assert_eq!(prefix, PathBuf::from("/out/result"));
    }
}
