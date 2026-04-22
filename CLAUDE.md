# CLAUDE.md

## Primary Directive

- Think in English. For user interaction language, follow the setting in the user's global `CLAUDE.md` or `CLAUDE.local.md`.

## Language Convention

This project is intended for public release. All of the following must be written in **English**:

- Commit messages
- Documentation (including `docs/`, `CLAUDE.md`, `README.md`, etc.)
- Skill descriptions and frontmatter

## Project Overview

`connect0459-agent-skills` is a [Claude Code Plugin](https://code.claude.com/docs/en/plugins.md) that packages reusable agent skills for `connect0459`.

### Repository Layout

```text
connect0459-agent-skills/
├── .connect0459-agent-skills/
│   └── plugin.json          # Plugin manifest (name, version, author)
├── skills/                  # Agent skills invoked via /connect0459-agent-skills:<skill-name>
│   └── swift-ios-review/
│       └── SKILL.md
└── docs/
    └── COMMIT_CONVENTIONS.md
```

### Installed Skills

| Skill | Invocation | Description |
| :--- | :--- | :--- |
| `swift-ios-review` | `/connect0459-agent-skills:swift-ios-review` | Swift code review for iOS apps |

## Plugin Development

### Adding a skill

1. Create `skills/<skill-name>/SKILL.md`
2. Add frontmatter with `name` and `description`
3. Write the skill body (instructions for Claude)
4. Add an entry to the table above

### Testing locally

```bash
claude --plugin-dir /path/to/connect0459-agent-skills
```

Reload without restarting:

```text
/reload-plugins
```

## Conventions

### Git

- Conventional Commits in English — see `docs/COMMIT_CONVENTIONS.md` for types, scopes, and examples
- Branch naming: `feature/xxx`, `fix/xxx`, `docs/xxx`
