---
schema: 1
id: E-UI-040
---

# E-UI-040 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-mutations/StoreMutationCommitPicker.stories.svelte
```

**Notes**

Before: 1 failed, 1 passed; requested time-under-title alignment fails.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-mutations/StoreMutationCommitPicker.stories.svelte
```

**Notes**

After: both Chromium interaction/accessibility stories pass; checks time below title at same left edge, one Git/JJ column with Git above JJ, missing-JJ row reservation, cross-row alignment, 10px oval chips, avatar/chevron centering and selection. Log .scratch/commit-picker-stacked-after.log.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-mutations/StoreMutationCommitPicker.stories.svelte catalog/components/store-mutations/StoreRepositoryActions.stories.svelte
```

**Notes**

6 directly affected picker and mutation-form interaction/accessibility stories pass. Log .scratch/commit-picker-stacked-actions.log.
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
pnpm web:check
```

**Notes**

Zero errors/warnings. Log .scratch/commit-picker-stacked-web-check.log.
<!-- /run -->

<!-- run -->
### Run 5

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

Zero errors/warnings; 75-module catalog boundary audit passes. Log .scratch/commit-picker-stacked-catalog-check.log.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:architecture
```

**Notes**

291-module production boundary and size audit passes.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:build
```

**Notes**

Fresh UI build passes with existing nonfatal Design Core CSS minification and bundle-size warnings. No Rust build. Log .scratch/commit-picker-stacked-build.log.
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
node .scratch/restart-workspace-ui.mjs
```

**Notes**

Owned demo ready at 127.0.0.1:7460; PID 47252. Existing explicit binaries and repository data preserved.
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
CUA live built-UI verification
```

**Notes**

Hidden temporary tab 9 shows 12 loaded commit options. Every time is below its title with zero left-edge gap; Git/JJ both start at 904px with JJ below Git; labels remain 10px. Selected trigger shares the layout. Screenshot .scratch/commit-picker-stacked.png saved; temporary tab closed and user tabs preserved. No mutation preview/apply.
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
pnpm exec prettier --write web/src/lib/ui/store-mutations/StoreMutationCommitTarget.svelte catalog/components/store-mutations/StoreMutationCommitPicker.stories.svelte spec/src/ui.md spec/src/api.md
```

**Notes**

Production row, regression story and canonical chapters formatted.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --write spec/src/verification.md spec/src/milestones.md
```

**Notes**

Evidence matrix and ledger formatted.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:validate
```

**Notes**

4 validators, 18 chapters and 107 requirements pass.
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
pnpm spec:first
```

**Notes**

2 protected files and 3 canonical chapters pass same-change gate.
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
pnpm check:progress
```

**Notes**

80 tasks pass evidence bookkeeping audit.
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
pnpm spec:build
```

**Notes**

Canonical mdBook HTML builds.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-mutations/StoreMutationCommitPicker.stories.svelte
```

**Notes**

Refinement before: 1 failed, 1 passed; bookmark/time centers differ by 25.390625px. Log .scratch/commit-picker-time-chips-before.log.
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
pnpm exec prettier --write web/src/lib/ui/store-mutations/StoreMutationCommitTarget.svelte catalog/components/store-mutations/StoreMutationCommitPicker.stories.svelte spec/src/ui.md spec/src/api.md
```

**Notes**

Refined presentation, story and canonical chapters formatted.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-mutations/StoreMutationCommitPicker.stories.svelte
```

**Notes**

Refinement after: both interaction/accessibility stories pass. Chips share the time row under title; Git/JJ stacked-column, empty/missing-JJ and font/shape alignment still pass. Log .scratch/commit-picker-time-chips-after.log.
<!-- /run -->
<!-- /evidence-field -->

