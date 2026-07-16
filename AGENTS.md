# AGENTS.md / CLAUDE.md

## Primary Directive

- Think in English. For user interaction language, follow the setting in the user's global `AGENTS.md` , `CLAUDE.md` or `CLAUDE.local.md`.

## Language Convention

This project is intended for public release. All of the following must be written in **English**:

- Commit messages
- Documentation (including `AGENTS.md`, `README.md`, `CONTRIBUTING.md`, etc.)
- Skill descriptions and frontmatter

## Project Overview

`connect0459-agent-skills` is a [Claude Code Plugin](https://code.claude.com/docs/en/plugins.md) that packages reusable agent skills for `connect0459`.

See [`README.md`](./README.md) for the installed skill list and repository layout.

## Plugin Development

### Adding a skill

1. Create `skills/<skill-name>/SKILL.md`
2. Add frontmatter with `name` and `description`
3. Write the skill body (instructions for Claude)
4. Add an entry to the skill table in `README.md`

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

- Conventional Commits in English — see [`CONTRIBUTING.md`](./CONTRIBUTING.md) for types, scopes, and examples
- Branch naming: `feature/xxx`, `fix/xxx`, `docs/xxx`
