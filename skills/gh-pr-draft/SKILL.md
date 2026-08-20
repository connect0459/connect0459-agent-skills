---
name: gh-pr-draft
description: >
  Generates a draft PR body for the current branch and writes it to `.connect0459/gh-pr-draft.md`,
  then optionally creates a GitHub Draft PR from it.
  Trigger whenever the user asks to create a PR draft, write a PR description, prepare PR body text, or submit a PR —
  including phrases like "PR bodyの下書きを作って", "PRの説明文を書いて", "draft PRを書いて", "PRを出して",
  "create PR draft", "write PR body", "submit a draft PR", "PR下書き", or any variation meaning
  "prepare or submit the pull request". Also trigger when the user finishes implementation work and asks
  what to do next in the PR workflow.
---

# PR Draft Generator

Generate a draft PR body for the current branch, write it to `.connect0459/gh-pr-draft.md`, then ask the user whether to create a GitHub Draft PR from it.

## Multiple repositories

When this skill runs across more than one repository in the same request:

1. **Ask up front, before drafting anything.** Use AskUserQuestion to ask whether the user wants:
   - **One repo at a time** — draft, confirm, and create the Draft PR for a single repository before moving to the next.
   - **Batch** — write every repository's draft first, then create all the Draft PRs together once the drafts are approved.

   Carry out the rest of this process in whichever mode the user picks.

2. **Always `cd` into the target repository's directory explicitly** immediately before any git/gh command tied to a specific repository — this includes Step 1.3 (branch changes), Step 6's branch/upstream check, `git push`, and `gh pr create`. Never assume the shell's working directory carried over from a previous command: parallel Bash tool calls in the same turn share one persistent shell, so a `cd` issued for one repository can leak into a sibling call meant for another, silently running branch/push/PR operations against the wrong repo. Prefix each command with `cd <repo-path> &&`, and when repo-scoped commands must run back-to-back, issue them sequentially rather than in parallel.

## Process

### Step 1: Gather context

Run these in parallel:

1. **Recent PR style** — fetch the last 5 closed/merged PRs and read their bodies:

   ```bash
   gh pr list --state merged --limit 5 --json number,title,body
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

```text
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

**Important**: If `.connect0459/gh-pr-draft.md` already exists, you **must** read it with the Read tool before writing. Skipping the read will cause the Write tool to fail.

Write the following to `.connect0459/gh-pr-draft.md`, overwriting any previous content:

```markdown
<!-- # <suggested title here> -->

<PR body here>
```

The title line is a HTML comment so it doesn't appear in the rendered PR body — it's just a convenient suggestion for the author to copy when they open the PR.

### Step 5: Ask the user

Tell the user:

- The file has been written to `.connect0459/gh-pr-draft.md`
- The suggested title (in plain text so they can copy it easily)
- One sentence on what template or style source was used

Then ask in the user's language. Examples:

**English:**
> Choose your next step:
>
> 1. Create a Draft PR directly from the draft file as-is
> 2. I've edited the draft file — reload it and create a Draft PR
> 3. Something else

**Japanese:**
> 次のステップを選んでください:
>
> 1. このままの下書きファイルからDraft PRを作成する
> 2. 下書きファイルを編集したので、再読み込みしてからDraft PRを作成する
> 3. その他

Wait for the user's response before proceeding.

### Step 6: Create the Draft PR (if the user chose option 1 or 2)

**If multiple repositories are in play, `cd` into this repository's directory explicitly before running any command below** (see "Multiple repositories" above).

**First, check that the branch is pushed to the remote:**

```bash
git rev-parse --abbrev-ref --symbolic-full-name @{u} 2>/dev/null
```

- If this succeeds, the branch has a remote tracking branch — proceed.
- If this fails (exit code non-zero or empty output), the branch has not been pushed. Push it now:

  ```bash
  git push -u origin HEAD
  ```

  Inform the user that the branch was pushed before creating the PR.

**If the user chose option 2**, re-read `.connect0459/gh-pr-draft.md` with the Read tool before proceeding.

**Then run:**

```bash
gh pr create --draft --title "<suggested title>" --body-file .connect0459/gh-pr-draft.md
```

After the command succeeds, report the PR URL to the user.

If the command fails (e.g. no remote, already has a PR), explain the error and suggest a fix rather than retrying silently.
