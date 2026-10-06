---
schema: 1
id: E-UI-034
---

# E-UI-034 details 3

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

Final complete version: zero errors and warnings; catalog ownership validates 73 modules.
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

Initial and label-fixed passes: zero errors/warnings. Catalog validates 73 modules.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:check
```

**Notes**

Owned-grid intermediate build failed only the $props.id() declaration placement; corrected.
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
pnpm catalog:check
```

**Notes**

Final complete version: zero errors and warnings; catalog ownership validates 73 modules.
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
pnpm check:architecture
```

**Notes**

Initial and final no-growth/source-ownership checks pass across 285 production modules; new component remains below the 300-line budget.
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
pnpm web:build
```

**Notes**

Initial, label, avatar and card/grid iteration builds passed. Existing nonfatal Design Core minification and large-chunk warnings remain.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:build
```

**Notes**

Owned-grid intermediate failed compilation for nested $props.id() use; fixed with a direct initializer.
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
pnpm web:build
```

**Notes**

Corrected final and 4px-gap builds pass; fresh built assets serve the user UI.
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
node .scratch/restart-workspace-ui.mjs
```

**Notes**

UI-only restarts use existing development binaries and persistent demo data. No reset/reseed or Rust build; final readiness confirmed on 127.0.0.1:7460.
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
pnpm exec prettier --write web/src/lib/ui/store-application/StoreComparisonParentPicker.svelte web/src/lib/ui/store-application/StoreApplication.svelte web/src/lib/ui/store-application/StoreRepositoryNotices.svelte web/src/lib/ui/store-application/store-application.css catalog/components/store-application/StoreComparisonParentPicker.stories.svelte web/tests/browser/comparison-parents.spec.ts spec/src/ui.md spec/src/api.md
```

**Notes**

Changed-file formatting applied in focused subsets through the implementation.
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
pnpm exec prettier --check web/src/lib/ui/store-application/StoreComparisonParentPicker.svelte web/src/lib/ui/store-application/StoreApplication.svelte web/src/lib/ui/store-application/StoreRepositoryNotices.svelte web/src/lib/ui/store-application/store-application.css catalog/components/store-application/StoreComparisonParentPicker.stories.svelte web/tests/browser/comparison-parents.spec.ts spec/src/api.md spec/src/ui.md
```

**Notes**

Changed product, regression and canonical chapter files formatted before final ledger update.
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

Four validators, 18 chapters and 107 requirements pass.
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

Six protected files and three canonical chapters pass the same-change gate.
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
pnpm spec:build
```

**Notes**

Canonical mdBook HTML build passes.
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
pnpm check:progress
```

**Notes**

74 tasks pass evidence bookkeeping; this does not substitute for product acceptance.
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
CUA live browser verification
```

**Notes**

Live header has Compare parent immediately before workspace, no explanatory box; menu uses native Op Log card components and computed 4px row gap. Saved ignored .scratch/comparison-parent-picker.png; comparison remains undecided.
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
pnpm exec prettier --write spec/src/api.md spec/src/ui.md spec/src/milestones.md spec/src/verification.md
```

**Notes**

Canonical chapter, evidence ledger and verification rows formatted.
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

<!-- evidence-field:versions markdown -->
## Versions

jj 0.45.1; pnpm 10.32.1; Node 24.10.0; Svelte 5.57.1; Vitest 4.1.11; Storybook 10.5.7; Vite 8.3.1; unchanged released Design Core dependency. Installed Design Core 0.1.2 public exports; dependency resolution unchanged.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Read-only UI slice: native operation parents remain exact, distinct from commit/change IDs; missing graph metadata never gains invented dates or badges. Workspace header shows Compare parent immediately before the workspace selector with no explanatory notice. Rich rows reuse existing Op Log author/message/time/badge and full operation ID copy components, with 4px gaps and accessible grid cells. Clipboard failure remains visible; copying never selects a parent. No controller, HTTP, generated DTO, native object, manifest, WAL, receipt, credential or persistence format changes. Stories remain visual-pending.
<!-- /evidence-field -->

