# connect0459-agent-skills

A [Claude Code Plugin](https://code.claude.com/docs/en/plugins.md) that packages reusable agent skills for `connect0459`.

## Installation

```bash
claude --plugin-dir /path/to/connect0459-agent-skills
```

Reload without restarting Claude Code:

```text
/reload-plugins
```

## Skills

| Skill | Invocation | Description |
| :--- | :--- | :--- |
| `swift-ios-review` | `/connect0459-agent-skills:swift-ios-review` | Swift code review for iOS apps |
| `kotlin-android-review` | `/connect0459-agent-skills:kotlin-android-review` | Kotlin code review for Android apps |
| `ts-solidjs-review` | `/connect0459-agent-skills:ts-solidjs-review` | SolidJS code review covering reactivity and design |
| `md-tech-blog-review` | `/connect0459-agent-skills:md-tech-blog-review` | Personal tech blog review for Zenn and similar platforms |
| `md-tech-report-review` | `/connect0459-agent-skills:md-tech-report-review` | Technical report review for work assessments and project evaluations |
| `gh-pr-draft` | `/connect0459-agent-skills:gh-pr-draft` | Generate a draft PR body from recent PR style and branch changes, then optionally create a GitHub Draft PR |
| `whisper-transcribe-setup` | `/connect0459-agent-skills:whisper-transcribe-setup` | Build the whisper-transcribe Rust binary and download the default Whisper model |
| `whisper-transcribe-transcribe` | internal only | Direct transcription backend wrapping the installed binary |
| `youtube-transcribe-local` | `/connect0459-agent-skills:youtube-transcribe-local` | Transcribe a local audio/video file via ffmpeg + whisper-transcribe |
| `youtube-transcribe-transcribe-youtube` | `/connect0459-agent-skills:youtube-transcribe-transcribe-youtube` | Full YouTube URL → SRT + Markdown pipeline |
| `youtube-transcribe-translate` | `/connect0459-agent-skills:youtube-transcribe-translate` | Translate a transcript or article between English and Japanese |

## Repository Layout

```text
connect0459-agent-skills/
├── .claude-plugin/
│   └── plugin.json          # Plugin manifest (name, version, author)
├── skills/                  # Agent skills invoked via /connect0459-agent-skills:<skill-name>
│   └── swift-ios-review/
│       └── SKILL.md
├── AGENTS.md                # Agent workflow conventions (CLAUDE.md is a symlink to this)
└── CONTRIBUTING.md          # Human contributor workflow and commit/branch conventions
```

## Development

See [`AGENTS.md`](./AGENTS.md) for agent workflow conventions (adding a skill, local testing) and
[`CONTRIBUTING.md`](./CONTRIBUTING.md) for the human contributor workflow, including commit and
branch conventions.

## Contributing

Contributions are welcome. Please read [`CONTRIBUTING.md`](./CONTRIBUTING.md) and follow the
[`CODE_OF_CONDUCT.md`](./CODE_OF_CONDUCT.md).

## Security

See [`SECURITY.md`](./SECURITY.md) for the vulnerability reporting process.

## License

[MIT](./LICENSE)
