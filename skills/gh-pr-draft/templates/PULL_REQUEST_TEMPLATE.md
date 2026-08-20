<!-- # PULL_REQUEST_TEMPLATE -->

<!-- Remove unnecessary sections to keep the review focused -->

## Related Links

- Issues
  - <!-- <https://github.com/<organization>/<repository>/issues/xxx> -->
- PRs
  - <!-- <https://github.com/<organization>/<repository>/pull/xxx> -->

## [Required] Overview

- Describe the problem being solved, its background, and what changes when this PR is merged.
- Links to specs, design documents, or other references are welcome.

```txt
It is difficult to review without knowing the specifications and background.
```

## Scope of Change

- [ ] `<package/module>`
- [ ] Tooling / CI
- [ ] Documentation

## Breaking Changes

- [ ] No breaking changes
- [ ] Breaking changes (describe below)

<!--
If this changes a public API or on-disk output format, describe what breaks and why the breakage is justified.
-->

## Deferred Items and TODOs

- Items intentionally deferred and the reasons why.

```txt
If you deferred something due to time constraints, document it here.
Reviewers cannot tell whether something was intentionally skipped or overlooked
without this information.
```

## Test Items

- Describe the tests added, following Red/Green TDD (which test was written first, and what it confirmed failed before the implementation existed).
- Note coverage changes if they changed meaningfully.
- Confirm the project's build, lint, and test commands all pass with no regressions.

## [Required] Quality Checklist

**Please check all items before merging.**

- [ ] **CI Workflow Execution**: All checks passed on CI for this PR.
- [ ] **Code Comments**: Limited to doc-comments and non-obvious WHY/WHY-NOT explanations, per this project's comment policy.
- [ ] **Reference Docs**: Relevant documentation updated to reflect this change.

> **Important**: This checklist ensures quality. Please verify all items before requesting review.
