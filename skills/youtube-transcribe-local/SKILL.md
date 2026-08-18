---
name: youtube-transcribe-local
argument-hint: "<audio|video> [--format srt|txt|vtt|json] [--language CODE] [--gpu|--no-gpu]"
description: >-
  Transcribe a local audio or video file using ffmpeg + whisper-transcribe.
  Accepts any format ffmpeg can decode (mp3, m4a, wav, mp4, mkv, mov, webm, opus, flac, …) —
  transcodes to 16 kHz mono PCM WAV automatically before passing to the Whisper backend.
  Default output is an SRT file written next to the input.
  Use whenever the user wants to transcribe a local media file, generate subtitles, get a
  transcript from audio/video, or convert speech to text — including phrases like
  "transcribe this file", "get the transcript", "subtitle this video", "speech to text",
  "run whisper on", or any time the user references a local audio/video file path and asks
  for text output. Trigger even when the user doesn't say "whisper" — file extension plus
  transcription intent is enough.
  Requires /connect0459-agent-skills:whisper-transcribe-setup to have been run first.
context: fork
---

# youtube-transcribe-local

Transcribe a local audio or video file by composing ffmpeg normalization and the `whisper-transcribe` Rust binary.

## Script

```bash
<skill-path>/scripts/local-transcribe <input-file> [options]
```

Options:

```text
--format srt|txt|vtt|json    Output format. Default: srt (file next to input).
--model NAME                 Model name. Default: large-v3-turbo.
--output-prefix PATH         Write output to <PATH>.<ext>. Default: next to input.
--language CODE              Language code (e.g. en, ja). Default: auto-detect.
--gpu                        Force GPU/Metal acceleration.
--no-gpu                     Force CPU mode.
-h, --help                   Show usage.
```

## Workflow

1. Verify `~/.local/bin/whisper-transcribe` and the model exist; exit early with a setup hint if not.
2. ffmpeg: transcode input → 16 kHz mono PCM WAV (temp file, deleted on exit).
3. Run `whisper-transcribe` on the WAV.
4. For `--format srt|vtt|json`: write `<output-prefix>.<ext>` and print `wrote <path>`. For `--format txt`: stream plain transcript to stdout.

## When to use this skill

- The user has a local file and wants a transcript or subtitles.
- Prefer this over invoking `whisper-transcribe` directly: this skill handles the ffmpeg transcoding step and sets the correct output prefix relative to the original input.

## Long content

Long transcripts can be tens of KB. This skill runs with `context: fork` so the transcript stays out of the calling session's prompt cache. Return the artifact path and a short summary to the parent session.

## Examples

```bash
# SRT subtitles (default)
./scripts/local-transcribe ~/Downloads/lecture.m4a
# wrote /Users/you/Downloads/lecture.srt

# Plain text to stdout
./scripts/local-transcribe ~/Downloads/lecture.m4a --format txt > transcript.txt

# Japanese audio, JSON output
./scripts/local-transcribe interview.mp4 --language ja --format json
# wrote /path/to/interview.json
```
