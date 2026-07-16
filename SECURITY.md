# Security Policy

## Supported Versions

Only the latest release on the `main` branch is actively maintained.
Older versions do not receive security fixes.

| Version  | Supported |
| :------- | :-------- |
| latest   | ✓         |
| < latest | ✗         |

## Reporting a Vulnerability

**Please do not open a public GitHub issue for security vulnerabilities.**

Use GitHub's [private vulnerability reporting][private-report] feature to
disclose issues confidentially. You will receive an acknowledgment within
**5 business days** and a resolution timeline once the report has been
triaged.

[private-report]: https://github.com/connect0459/connect0459-agent-skills/security/advisories/new

## Scope

The following vulnerability classes are in scope for this project:

- **Prompt/instruction injection** — skill content (`SKILL.md` files) or
  their generated output causing Claude Code to execute unintended commands,
  exfiltrate secrets, or bypass the user's tool-approval flow when a skill is
  invoked.
- **Unsafe command execution** — skill instructions that pass untrusted input
  (filenames, URLs, transcript content) unsanitized to a shell command such as
  `ffmpeg`, `yt-dlp`, or the bundled `whisper-transcribe` binary.
- **Supply-chain issues in the bundled binary** — the `whisper-transcribe`
  Rust build (`src/whisper-transcribe/`) or the model download step in the
  `whisper-transcribe-setup` skill fetching or executing untrusted code.
- **Secret exposure** — CI workflows or skill instructions that could leak
  tokens, API keys, or credentials (e.g. `GITHUB_TOKEN`, `gh` auth state).

The following are **out of scope**:

- Issues in third-party dependencies (report those upstream: `ffmpeg`,
  `yt-dlp`, `whisper.cpp`, etc.).
- Risks that require the user to already run Claude Code in an
  unsupervised/auto-approve mode against untrusted skill sources.
- Theoretical issues without a reproducible proof-of-concept.

## Disclosure Policy

Once a fix is ready and released, a GitHub Security Advisory will be
published with full details. The typical timeline from report to public
disclosure is **30 days**, though this may be extended by mutual agreement
when a fix requires significant changes.
