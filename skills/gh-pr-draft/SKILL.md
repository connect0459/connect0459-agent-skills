---
name: gh-pr-draft
description: >
  Generates a draft PR body for the current branch and writes it to `.connect0459/gh-pr-draft.md`.
  Trigger whenever the user asks to create a PR draft, write a PR description, or prepare PR body text —
  including phrases like "PR bodyの下書きを作って", "PRの説明文を書いて", "draft PRを書いて",
  "create PR draft", "write PR body", "PR下書き", or any variation meaning "prepare the pull request description".
  Also trigger when the user finishes implementation work and asks what to do next in the PR workflow.
---

# PR Draft Generator

Generate a draft PR body for the current branch, write it to `.connect0459/gh-pr-draft.md`.

## Process

### Step 1: Gather context

Run these in parallel:

1. **Recent PR style** — fetch the last 10 closed/merged PRs and read their bodies:
   ```bash
   gh pr list --state merged --limit 10 --json number,title,body
   ```
   Study the tone, section headers, level of detail, and what the user consistently includes or omits. This is the primary style guide.

2. **PR template** — check these paths in order, use the first one found:
   - `.github/PULL_REQUEST_TEMPLATE.md`
   - `.github/pull_request_template.md`
   - `docs/PULL_REQUEST_TEMPLATE.md`
   - `PULL_REQUEST_TEMPLATE.md`
   - `.github/PULL_REQUEST_TEMPLATE/` (directory — use the first `.md` file inside)

3. **Branch changes** — understand what was done:
   ```bash
   git log origin/main..HEAD --oneline
   git diff origin/main..HEAD --stat
   ```
   Read the commit messages to understand intent. Glance at a few key changed files if needed to understand the "why", but do not list file names or paths in the draft.

4. **Current draft** — read `.connect0459/gh-pr-draft.md` if it exists (it may contain a prior draft or a template hint).

### Step 2: Infer a suggested title

From the commit messages and branch name, infer a Conventional Commits title:
```
<type>(<scope>): <short description>
```
Types: `feat`, `fix`, `refactor`, `docs`, `chore`, `test`, `ci`, `perf`

### Step 3: Write the draft

**If a PR template exists**: fill it in. Follow its structure exactly. The template defines what the author expects reviewers to read — respect that.

**If no template**: mirror the structure and length of the user's recent PRs. If their PRs typically have 3 bullet points, write 3 bullet points. If they write a short paragraph, write a short paragraph. Don't over-engineer what is usually concise.

**Evergreen writing rule**: write at the level of *intent and behavior*, not implementation detail.
- Describe *what the system now does* or *what problem this solves*, not *which files were changed* or *which method was added*.
- File names, method names, and class names drift as code evolves; behavior descriptions don't.

**Bad** (implementation-detail-heavy):
> `UserRepository.swift` was updated to call `fetchProfile(userId:)` and the result is mapped in `ProfileMapper`.

**Good** (intent-focused):
> Profile data is now fetched and displayed correctly when the user opens the settings screen.

### Step 4: Write the output

Write the following to `.connect0459/gh-pr-draft.md`, overwriting any previous content:

```markdown
<!-- # <suggested title here> -->

<PR body here>
```

The title line is a HTML comment so it doesn't appear in the rendered PR body — it's just a convenient suggestion for the author to copy when they open the PR.

### Step 5: Confirm

Tell the user:
- The file has been written to `.connect0459/gh-pr-draft.md`
- The suggested title (repeat it in plain text so they can copy it easily)
- One sentence on what template or style source was used
