---
schema: 1
id: E-UI-035
---

# E-UI-035 details 2

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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreRepositoryNavigation.stories.svelte -t 'Selected (change|operation) actions'
```

**Notes**

No acceptance evidence: all four tests were skipped because the generated Storybook names did not match this filter. Reran the exact story file without a name filter.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreRepositoryNavigation.stories.svelte
```

**Notes**

Before implementation: two new selection stories failed on the missing Selected repository actions group; two existing navigation stories passed. Ignored .scratch/context-actions-before.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreRepositoryNavigation.stories.svelte catalog/components/store-application/StoreRepositoryBranchContext.stories.svelte
```

**Notes**

Initial and corrected story-query runs both passed 5 interaction/accessibility tests. Includes typed Edit description and Reconcile operations dialogs, deselection through navigation and bookmark Move command target. Stories remain visual-pending.
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

Zero errors and warnings.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:check
```

**Notes**

New story queries incorrectly used Playwright-only exact properties with Testing Library. Removed those properties without weakening the name assertions.
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
pnpm catalog:check
```

**Notes**

Corrected version: zero errors/warnings; catalog ownership passes across 73 modules.
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
pnpm check:architecture
```

**Notes**

286 production modules pass ownership/no-growth checks; integration modules remain below 300 lines.
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

Built assets pass. Existing nonfatal Design Core minification and chunk warnings remain. No Rust builds.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec playwright test --config .scratch/workspace-browser.config.mts context-actions.spec.ts repository-navigation.spec.ts
```

**Notes**

Six of seven focused cold built-UI checks passed. New reload fixture expected the closed details rail to remain visible; corrected to reopen it. No product persistence contract changed.
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
pnpm exec playwright test --config .scratch/workspace-browser.config.mts context-actions.spec.ts -g 'selected JJ Log change'
```

**Notes**

Exact failed regression now passes: selected commit restored after reload, Edit description opens for its full native commit, and changing view clears the actions. Combined with the prior six passes, all seven focused cases have passing evidence.
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
node .scratch/restart-workspace-ui.mjs
```

**Notes**

Owned development UI restarted using existing explicit binaries and persistent demo data; readiness confirmed at 127.0.0.1:7460, PID 9124. No reset/reseed, installation or unrelated daemon restart.
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
CUA live browser verification
```

**Notes**

Selected live Op Log operation shows Actions beneath Details; one repository head correctly omits Reconcile operations. Saved ignored .scratch/oplog-context-actions.png. Preserved the user-opened Create change dialog when concurrent navigation was observed; no mutation applied.
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
pnpm exec prettier --write web/src/lib/ui/store-application/StoreApplication.svelte web/src/lib/ui/store-application/StoreRepositoryContextActions.svelte catalog/components/store-application/StoreRepositoryNavigation.stories.svelte catalog/components/store-application/StoreRepositoryBranchContext.stories.svelte web/tests/browser/context-actions.spec.ts spec/src/api.md spec/src/ui.md
```

**Notes**

Changed source, stories, browser test and canonical chapter formatting applied in focused subsets.
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
pnpm exec prettier --check web/src/lib/ui/store-application/StoreApplication.svelte web/src/lib/ui/store-application/StoreRepositoryContextActions.svelte catalog/components/store-application/StoreRepositoryNavigation.stories.svelte catalog/components/store-application/StoreRepositoryBranchContext.stories.svelte web/tests/browser/context-actions.spec.ts spec/src/api.md spec/src/ui.md
```

**Notes**

Story formatting needed refresh after removing unsupported exact query properties; corrected.
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
pnpm spec:validate
```

**Notes**

Four validators, 18 chapters and 107 requirements pass.
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
pnpm spec:first
```

**Notes**

Five protected files and three canonical chapters pass the same-change gate.
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
pnpm spec:build
```

**Notes**

Canonical mdBook HTML build passes.
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
pnpm check:progress
```

**Notes**

75 tasks pass evidence bookkeeping; does not substitute for product acceptance.
<!-- /run -->
<!-- /evidence-field -->

