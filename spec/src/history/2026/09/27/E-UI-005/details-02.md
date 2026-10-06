---
schema: 1
id: E-UI-005
---

# E-UI-005 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-application/RepositoryReview.stories.svelte
```

**Notes**

Before implementation: redundant sidebar header and missing directory table reproduced in two review stories.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-application/RepositoryReview.stories.svelte
```

**Notes**

First rerun: two story failures and two unhandled errors from newly imported folder icon triggering optimizer reload. Added the icon to startup prebundling.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-application/RepositoryReview.stories.svelte
```

**Notes**

Header breadcrumb label assertion and duplicate breadcrumb navigation axe landmark failure. Public List now composes within the existing application navigation landmark.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-application/RepositoryReview.stories.svelte
```

**Notes**

Two review stories passed before extending the assertions to right rail and branch dividers.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-application/RepositoryReview.stories.svelte
```

**Notes**

Rail border assertion expected zero width; Community uses a transparent border with retained width. Copied the source rail styles and asserted visible color.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-application/RepositoryReview.stories.svelte
```

**Notes**

Every branch card matched last-child inside its own command wrapper, removing all dividers. Reproduced in the live full-app DOM.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-application/RepositoryReview.stories.svelte catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.stories.svelte
```

**Notes**

Expanded regression: both card/divider stories failed and the requested body-title removal failed; two other stories passed.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-application/RepositoryReview.stories.svelte catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.stories.svelte
```

**Notes**

All five stories pass after targeting the final populated wrapper and removing redundant desktop titles. Mobile keeps navigation controls without duplicate title.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:test
```

**Notes**

20 catalog unit tests passed, including controller_restores_directory_history.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-application catalog/components/community-repository-branch-command
```

**Notes**

18 affected stories pass, including graph alignment, sticky bookmark heading, branch/revision/diff/operation-details secondary headers, directory drill-down, transparent rail, filtered card separators and redundant-title removal.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check
```

**Notes**

Four TypeScript diagnostics in new review assertions used Playwright-only exact options with Testing Library getByRole. Removed invalid options; exact name matching is the Testing Library default.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check
```

**Notes**

Cold catalog passed; catalog type lane reported the same four getByRole option diagnostics, then stopped. No runtime or accessibility failures.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:check
```

**Notes**

Focused rerun after assertion fix: zero errors/warnings; 73 catalog modules pass the boundary audit.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check
```

**Notes**

62 requirements, 37-task bookkeeping, boundaries, contracts, web/catalog types, rustfmt and all-target Clippy pass.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check
```

**Notes**

All 18 lanes pass: 20 catalog unit tests, 37 Storybook interactions with blocking axe, cold static catalog, 8 existing browser acceptance cases, 369 Rust tests (6 external-service/credential cases intentionally ignored), contracts, governance, rustfmt, mdBook and mapped spec-first.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm format:check
```

**Notes**

Rustfmt and Prettier pass before final evidence-only updates.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test
```

**Notes**

Aggregate passes: governance, storage harness, contracts, browser unit tests, 20 catalog unit tests, 37 Storybook interaction/axe cases, cold catalog and 369 Rust tests. Six external-service/credential cases remain intentionally ignored.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build
```

**Notes**

Static Storybook, existing browser bundle/manifest and locked Rust workspace build pass.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

