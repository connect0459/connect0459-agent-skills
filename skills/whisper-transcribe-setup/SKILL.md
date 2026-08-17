---
name: whisper-transcribe-setup
argument-hint: "[--model NAME] [--no-smoke-test]"
description: >-
  Build the whisper-transcribe Rust binary from source and download the default Whisper model
  (~809 MB, ggml-large-v3-turbo). Installs the binary to ~/.local/bin/whisper-transcribe and
  the model to ~/.local/share/whisper-transcribe/models/. Runs English and Japanese smoke tests
  to verify the install works end-to-end.
  Trigger whenever the user wants to set up whisper transcription, install the whisper backend,
  build the Rust binary, download the Whisper model, or prepare for local or YouTube transcription —
  including phrases like "set up whisper", "install whisper", "build whisper-transcribe",
  "I need to transcribe audio", "prepare for transcription", or any time the user is missing
  the binary or model required by youtube-transcribe-local or youtube-transcribe-transcribe-youtube.
  Idempotent: safe to re-run. Build skips when the binary is current; model download skips when
  the file already exists; smoke tests always run.
---

# whisper-transcribe-setup

Build the `whisper-transcribe` Rust binary from `src/whisper-transcribe/` in this repository and download the default Whisper model. The binary and model are the shared backend for `/connect0459-agent-skills:youtube-transcribe-local` and `/connect0459-agent-skills:youtube-transcribe-transcribe-youtube`.

## Script

```bash
<skill-path>/scripts/setup [--model NAME] [--no-smoke-test]
```

Defaults:

- Binary install path: `~/.local/bin/whisper-transcribe`
- Model directory: `~/.local/share/whisper-transcribe/models/`
- Model: `large-v3-turbo` (~809 MB quantised, multilingual, fast on Apple Silicon Metal)

## Workflow

The script runs these steps in order. Each step prints a `==> step name` header.

1. **Prerequisite check.** Verifies `cargo`, `cmake`, `clang`, `ffmpeg`, and `yt-dlp` are on PATH.
   Exits with install hints if any are missing.
2. **Build.** Runs `cargo build --release` in `src/whisper-transcribe/`.
   Skipped when the installed binary is newer than all source files (idempotent).
   Copies `target/release/whisper-transcribe` to `~/.local/bin/`.
3. **Model download.** Downloads `ggml-<model>.bin` from HuggingFace via `curl`.
   Skipped when the model file already exists.
4. **Smoke tests.** Always run — verifies the install produces correct output, not just that it built.
   - **English**: fetches `samples/jfk.wav` from the whisper.cpp upstream repository;
     checks transcript contains `ask` and `country`.
   - **Japanese**: generates a clip via `say -v Kyoko` and converts with `ffmpeg`;
     checks transcript contains `天気` and `明日`.

## Prerequisites

The script checks and reports all missing tools before doing any work:

- `cargo` — Rust toolchain (install via `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)
- `cmake` — required by `whisper-sys` build script (`brew install cmake` on macOS)
- `clang` — C++ compiler for whisper.cpp FFI (`xcode-select --install` on macOS)
- `ffmpeg` — audio transcoding for local transcription (`brew install ffmpeg`)
- `yt-dlp` — YouTube download for the transcribe-youtube skill (`brew install yt-dlp`)

`cmake` and `clang` are toolchain dependencies — the user authors only Rust code; the C++ compilation happens inside `cargo build` via `whisper-sys`.

## Disk

- Build artifacts under `src/whisper-transcribe/target/`: ~400 MB (in `.gitignore`)
- Default model `ggml-large-v3-turbo.bin`: ~809 MB
- Plan for ~1.2 GB total on first setup; subsequent runs are near instant.

## After setup

- Binary: `~/.local/bin/whisper-transcribe`
- Model: `~/.local/share/whisper-transcribe/models/ggml-large-v3-turbo.bin`

Use `/connect0459-agent-skills:youtube-transcribe-local` for local file transcription, or `/connect0459-agent-skills:youtube-transcribe-transcribe-youtube` for the full YouTube workflow.
