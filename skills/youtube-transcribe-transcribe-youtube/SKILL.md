---
name: youtube-transcribe-transcribe-youtube
argument-hint: "<youtube-url> [--language CODE] [--model NAME]"
description: >-
  End-to-end YouTube → SRT + Markdown workflow. Downloads audio via yt-dlp, transcribes with
  whisper-transcribe via ffmpeg, and converts the SRT to a structured Markdown article.
  Use whenever the user wants to transcribe a YouTube video, get a transcript of a YouTube talk,
  convert YouTube speech to text, or produce a readable article from a YouTube video —
  including phrases like "transcribe this YouTube video", "get the transcript of this talk",
  "YouTubeの文字起こし", "YouTubeの動画を書き起こして", "make an article from this video",
  or any time the user shares a YouTube URL and asks for text, transcript, or summary.
  Requires /connect0459-agent-skills:whisper-transcribe-setup to have been run first.
context: fork
---

# youtube-transcribe-transcribe-youtube

Full YouTube → SRT → Markdown pipeline, running in a forked context so large transcript artifacts stay out of the calling session's prompt cache.

## Script

```bash
<skill-path>/scripts/transcribe-youtube <youtube-url> [--language CODE] [--model NAME]
```

The script handles audio download and SRT generation. You (Claude) handle the SRT → Markdown conversion and offer translation.

## Workflow

### Steps 1–2: run the script (shell)

```bash
<skill-path>/scripts/transcribe-youtube <url> [--language CODE] [--model NAME]
```

The script:

1. Downloads audio from the YouTube URL via `yt-dlp --extract-audio` to `~/Downloads`.
2. Runs `local-transcribe` (ffmpeg + whisper-transcribe) on the audio → `.srt` file.
3. Prints `audio: <path>` and `srt: <path>` so you know the artifact locations.

### Step 3: SRT → Markdown (you do this, in this forked context)

Read the `.srt` file with the Read tool. Convert it to a structured Markdown article:

- **Title** (H1): infer from the video title embedded in the filename `Title [id].ext`, or ask.
- **Introduction**: 1–2 sentences summarising what the talk is about.
- **Body**: convert each SRT segment to readable prose, grouped into logical sections with H2 headings.
  - Remove filler words ("um", "uh", etc.).
  - Fix obvious transcription errors where the context makes the correction clear.
  - Preserve technical terms, names, and code verbatim.
  - Do not add information the speaker did not say.
- **Conclusion**: 2–3 sentences on the key takeaways.

Output file: same stem as the `.srt`, with `.md` extension — e.g.:
`~/Downloads/Talk Title [abc123].srt` → `~/Downloads/Talk Title [abc123].md`

Write the Markdown with the Write tool. Then print `wrote <md-path>`.

### Step 4: offer translation (do not auto-run)

After writing the Markdown, offer:

> Translation available: run `/connect0459-agent-skills:youtube-transcribe-translate <md-path> --to ja`
> for a Japanese version, or `--to en` for English.

Do not start the translation automatically — the user decides.

## Output

Both files are durable artifacts on disk:

- `.srt` — canonical subtitle artifact, timestamped, downgradeable to plain text
- `.md` — readable article form

## Running inside the Claude Code sandbox

`whisper-transcribe` auto-detects the sandbox via `SANDBOX_RUNTIME=1` and falls back to CPU mode.
CPU transcription is 5–10× slower than Metal; a 30-minute talk takes ~5–10 minutes. The fork handles this blocking wait — the parent session is unblocked while the fork runs.
