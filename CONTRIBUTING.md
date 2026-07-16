# Contributing

## Prerequisites

- [Claude Code](https://code.claude.com/docs/en/plugins.md) — to install and test the plugin
- [pre-commit](https://pre-commit.com/) — formatting and lint hooks
- [Rust toolchain](https://www.rust-lang.org/tools/install) — only needed when working on `src/whisper-transcribe/` (see `rust-toolchain.toml`)

## Setup

```sh
git clone https://github.com/connect0459/connect0459-agent-skills
cd connect0459-agent-skills
```

### pre-commit hooks

```sh
pip install pre-commit   # or: brew install pre-commit
pre-commit install
```

To run all hooks manually:

```sh
pre-commit run --all-files
```

## Development workflow

| Command | Purpose |
| :--- | :--- |
| `claude --plugin-dir .` | Load this plugin locally |
| `/reload-plugins` | Reload skill changes without restarting Claude Code |
| `pre-commit run --all-files` | Run formatting and lint checks |

To add a new skill, see the "Adding a skill" workflow in [`AGENTS.md`](./AGENTS.md).

For `src/whisper-transcribe/` (Rust), see the CI checks in
[`.github/workflows/ci-whisper-transcribe.yml`](./.github/workflows/ci-whisper-transcribe.yml):

```sh
cd src/whisper-transcribe
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

## Commit format

```text
<type>(<scope>): <subject>

<body>

<footer>
```

### Types

| Type | Description |
| :--- | :--- |
| `feat` | New skill or plugin feature |
| `fix` | Bug fix or correction in a skill |
| `docs` | Documentation only |
| `refactor` | Restructure skill content without changing behavior |
| `tidy` | Small, safe cleanup (< 2 min; no behavior change) |
| `chore` | Plugin manifest, tooling, or config changes |
| `ci` | CI/CD pipeline changes (GitHub Actions, workflows) |

### Scopes

Scope is **required** when the change targets a specific area; omit it only
for project-wide changes (e.g., `docs: update README`).

| Scope | When to use |
| :--- | :--- |
| `skill` | Changes to any file under `skills/` |
| `plugin` | Changes to `.claude-plugin/plugin.json` |

### Subject line

- Use the imperative mood: "add", "fix", "remove" — not "added" or "adds"
- 72 characters max
- No trailing period

### Body (optional)

- Wrap at 72 characters
- Explain **why**, not what — the diff already shows what changed
- Leave one blank line between subject and body

### Footer (optional)

- `BREAKING CHANGE: <description>` for breaking changes
- `Closes #123` or `Fixes #456` to link issues

### Examples

```text
feat(skill): add swift-ios-review skill for iOS code review
```

```text
fix(skill): guard isViewLoaded check in notification handler example
```

```text
refactor(skill): reorganize swift-ios-review sections by severity
```

```text
chore(plugin): bump version to 1.1.0
```

```text
docs: add commit conventions guide
```

### Branch naming

`feature/xxx`, `fix/xxx`, `docs/xxx`

## Pull request process

1. Fork the repository and create a branch: `feature/xxx`, `fix/xxx`, `docs/xxx`.
2. Add or update the skill under `skills/<skill-name>/SKILL.md`.
3. Run `pre-commit run --all-files` and commit any resulting diffs.
4. Update the skill table in [`README.md`](./README.md) if you added, renamed, or removed a skill.
5. Open a pull request using the provided template.

## Code style

- No code comments unless the **why** is genuinely non-obvious.
- Skill descriptions and frontmatter must be in **English** (see [`AGENTS.md`](./AGENTS.md#language-convention)).
- Keep each `SKILL.md` focused on a single skill's invocation and behavior.
