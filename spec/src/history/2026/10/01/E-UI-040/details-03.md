---
schema: 1
id: E-UI-040
---

# E-UI-040 details 3

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

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

Refined layout: zero errors/warnings. Log .scratch/commit-picker-time-chips-web-check.log.
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
pnpm catalog:check
```

**Notes**

Refined layout: zero errors/warnings; 75-module boundary audit passes. Log .scratch/commit-picker-time-chips-catalog-check.log.
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
pnpm check:architecture
```

**Notes**

Refined layout: 291-module architecture audit passes.
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
pnpm web:build
```

**Notes**

Refined UI build passes with existing nonfatal minification/bundle-size warnings. Log .scratch/commit-picker-time-chips-build.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-mutations/StoreMutationCommitPicker.stories.svelte catalog/components/store-mutations/StoreRepositoryActions.stories.svelte
```

**Notes**

Final refined layout: 6 directly affected interaction/accessibility stories pass. Log .scratch/commit-picker-time-chips-actions.log.
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
node .scratch/restart-workspace-ui.mjs
```

**Notes**

Final built demo ready at 127.0.0.1:7460; PID 48341. Existing explicit binaries and data preserved.
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
CUA live built-UI verification
```

**Notes**

Final hidden tab 10 checks 12 loaded options; all five bookmark/workspace chips have zero vertical center gap from time. Git/JJ remain stacked in their aligned column, with time and chips under title in both picker and trigger. Screenshot .scratch/commit-picker-time-chips.png saved. Temporary tab closed; user tabs preserved. No mutation preview/apply.
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
pnpm exec prettier --write spec/src/milestones.md
```

**Notes**

Evidence ledger formatted.
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
pnpm exec prettier --check web/src/lib/ui/store-mutations/StoreMutationCommitTarget.svelte catalog/components/store-mutations/StoreMutationCommitPicker.stories.svelte spec/src/ui.md spec/src/api.md spec/src/verification.md spec/src/milestones.md
```

**Notes**

All six changed files pass formatting after inline-chip refinement.
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
pnpm spec:validate
```

**Notes**

Inline-chip refinement: 4 validators, 18 chapters and 107 requirements pass.
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
pnpm spec:first
```

**Notes**

Inline-chip refinement: 2 protected files and 3 canonical chapters pass.
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
pnpm check:progress
```

**Notes**

Inline-chip refinement: 80 tasks pass evidence bookkeeping audit.
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
pnpm spec:build
```

**Notes**

Inline-chip refinement: canonical mdBook HTML builds.
<!-- /run -->

<!-- run -->
### Run 14

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

Badge-width refinement before: 1 failed, 1 passed; width mismatch 5.578125px. Log .scratch/commit-picker-badge-width-before.log.
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
pnpm exec prettier --write web/src/lib/ui/store-mutations/StoreMutationCommitTarget.svelte catalog/components/store-mutations/StoreMutationCommitPicker.stories.svelte spec/src/ui.md spec/src/api.md
```

**Notes**

Equal badge width refinement formatted.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-mutations/StoreMutationCommitPicker.stories.svelte
```

**Notes**

Badge-width refinement after: both interaction/accessibility stories pass. Badge widths and code left edges match; time/chips share one row beneath title and stacked identifiers remain aligned. Log .scratch/commit-picker-badge-width-after.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-mutations/StoreMutationCommitPicker.stories.svelte catalog/components/store-mutations/StoreRepositoryActions.stories.svelte
```

**Notes**

Final badge-width refinement: 6 directly affected interaction/accessibility stories pass. Log .scratch/commit-picker-badge-width-actions.log.
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
pnpm web:check
```

**Notes**

Final badge-width refinement: zero errors/warnings. Log .scratch/commit-picker-badge-width-web-check.log.
<!-- /run -->
<!-- /evidence-field -->

