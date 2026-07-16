<!-- # PULL_REQUEST_TEMPLATE -->

<!-- Remove unnecessary sections to keep the review focused -->

## Related Links

- Issues
  - <!-- <https://github.com/connect0459/connect0459-agent-skills/issues/xxx> -->
- PRs
  - <!-- <https://github.com/connect0459/connect0459-agent-skills/pull/xxx> -->

## [Required] Overview

- Describe the problem being solved, its background, and what changes when this PR is merged.
- Links to specs, design documents, or other references are welcome.

```txt
It is difficult to review without knowing the specifications and background.
```

## Scope of Change

- [ ] `skills/xxx` skill
- [ ] `src/whisper-transcribe` (Rust binary)
- [ ] Tooling / CI
- [ ] Documentation

## Breaking Changes

- [ ] No breaking changes
- [ ] Breaking changes (describe below)

<!--
If this changes a skill's invocation name or frontmatter, describe what
breaks for existing users and why the breakage is justified.
-->

## Deferred Items and TODOs

- Items intentionally deferred and the reasons why.

```txt
If you deferred something due to time constraints, document it here.
Reviewers cannot tell whether something was intentionally skipped or overlooked
without this information.
```

## Test Items

- Describe how the skill was exercised (e.g. `claude --plugin-dir .` + `/reload-plugins`, then invoked the skill).
- For `src/whisper-transcribe`, note whether `cargo test` / `cargo clippy` were run.

## [Required] Quality Checklist

**Please check all items before merging.**

- [ ] **Skill Invocation Tested**: the skill was loaded locally and invoked at least once
- [ ] **Frontmatter**: `name` and `description` are accurate and in English
- [ ] **Docs in Sync**: `README.md` skill table is updated for any added/renamed/removed skill

> **Important**: This checklist ensures quality. Please verify all items before requesting review.
