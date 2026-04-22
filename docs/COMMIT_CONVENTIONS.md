# Commit Conventions

## Format

```text
<type>(<scope>): <subject>

<body>

<footer>
```

## Types

| Type | Description |
| :--- | :--- |
| `feat` | New skill or plugin feature |
| `fix` | Bug fix or correction in a skill |
| `docs` | Documentation only |
| `refactor` | Restructure skill content without changing behavior |
| `tidy` | Small, safe cleanup (< 2 min; no behavior change) |
| `chore` | Plugin manifest, tooling, or config changes |

## Scopes

Scope is **required** when the change targets a specific area; omit it only for project-wide changes (e.g., `docs: update README`).

| Scope | When to use |
| :--- | :--- |
| `skill` | Changes to any file under `skills/` |
| `plugin` | Changes to `.connect0459-agent-skills/plugin.json` |

## Rules

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

## Examples

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
