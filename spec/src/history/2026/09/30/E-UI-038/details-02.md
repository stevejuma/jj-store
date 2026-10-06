---
schema: 1
id: E-UI-038
---

# E-UI-038 details 2

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

Before: 1 failed, 1 passed. New bottom-time geometry assertion reproduces requested layout change. Log .scratch/commit-picker-columns-before.log.
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

After: both Chromium interaction/accessibility stories pass. Checks trigger/avatar/chevron centering, oval indicator height, font relative to actual copy control, Git/JJ left edges across varied titles/missing change ID, bottom-right time and selection update. Log .scratch/commit-picker-columns-after.log.
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

6 directly affected picker and mutation-form interaction/accessibility stories pass. Shared fixed targets and searchable choices retain request/selection flows. Log .scratch/commit-picker-actions.log.
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

Zero errors/warnings.
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

Zero errors/warnings; 75-module catalog boundary audit passes.
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

289-module production/catalog/script boundary and size audit passes.
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

Fresh built UI passes with existing nonfatal Design Core CSS minification and bundle-size warnings. No Rust rebuild. Log .scratch/commit-picker-build.log.
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

Owned development demo ready at 127.0.0.1:7460 with existing explicit binaries and preserved data; PID 36675. No reset/reseed or unrelated daemon restart.
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

Temporary hidden tab 7 checks 12 real loaded commit options: labels compute to 10px; every Git chip starts at 834.40625px and JJ at 912px; time is below the primary row with zero right-edge gap. Create-change trigger shares the same row. Screenshot .scratch/commit-picker-columns.png saved and tab closed; user tabs preserved. No mutation preview/apply.
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
pnpm exec prettier --write web/src/lib/ui/store-mutations/StoreMutationCommitTarget.svelte spec/src/ui.md spec/src/api.md
```

**Notes**

Production row and canonical chapters formatted.
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
pnpm exec prettier --write web/src/lib/ui/store-mutations/StoreMutationCommitTarget.svelte catalog/fixtures/StoreMutationCommitPickerFixture.svelte catalog/components/store-mutations/StoreMutationCommitPicker.stories.svelte spec/src/api.md spec/src/ui.md spec/src/verification.md spec/src/milestones.md
```

**Notes**

All seven changed files formatted.
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

Same-change canonical gate passes for 3 protected files and 3 chapters.
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

78 tasks pass same-change evidence bookkeeping audit.
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
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --check web/src/lib/ui/store-mutations/StoreMutationCommitTarget.svelte catalog/fixtures/StoreMutationCommitPickerFixture.svelte catalog/components/store-mutations/StoreMutationCommitPicker.stories.svelte spec/src/api.md spec/src/ui.md spec/src/verification.md spec/src/milestones.md
```

**Notes**

All seven changed files pass formatting.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

[]
<!-- /evidence-field -->

<!-- evidence-field:deferred list -->
## Deferred

- Changed stories remain visual-pending; baseline approval is separate.
- Root aggregate pnpm spec:check, pnpm check, pnpm test and pnpm build;
  unrelated UI suites; Rust, native contracts and real-RustFS qualification are
  out of scope under targeted UI-only policy.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

jj 0.45.1; Node 24.10.0; pnpm 10.32.1; Svelte 5.57.1; Storybook 10.5.7; Vitest 4.1.11; Vite 8.3.1. Existing released dependency resolution unchanged.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Presentation only. Noncopyable GIT/JJ labels use existing Community secondary-font token at 0.625rem (10px), one pixel above the existing 9px split-copy control. Fixed Git/JJ columns reserve missing identities and align options independently of title length. Secondary bookmark/workspace metadata keeps relative time at bottom right. Full bound native IDs, search, copy controls, typed mutations, HTTP/native object/manifest/WAL/receipt formats unchanged.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Retain focused picker and mutation-form acceptance; human visual baseline approval remains separate.
<!-- /evidence-field -->

