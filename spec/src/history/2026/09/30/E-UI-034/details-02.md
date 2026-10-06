---
schema: 1
id: E-UI-034
---

# E-UI-034 details 2

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
pnpm exec playwright test --config .scratch/workspace-browser.config.mts comparison-parents.spec.ts
```

**Notes**

Before: combobox absent. The initial fixture incorrectly required query strings for operation detail and diff; it did not reliably expose the intended merge notice. .scratch/comparison-parent-before.log.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec playwright test --config .scratch/workspace-browser.config.mts comparison-parents.spec.ts
```

**Notes**

First integration attempt: same fixture route mismatch; detail and diff routes were corrected to cover queryless requests. .scratch/comparison-parent-after.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreComparisonParentPicker.stories.svelte
```

**Notes**

Initial story failed locating the mounted/reopened search field; switched to awaited queries and focus restoration. .scratch/comparison-parent-stories.log.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec playwright test --config .scratch/workspace-browser.config.mts comparison-parents.spec.ts
```

**Notes**

Corrected fixture exposes two parents, but search accessibility name came from Command root label rather than input aria-label. .scratch/comparison-parent-browser-fixed.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreComparisonParentPicker.stories.svelte
```

**Notes**

Awaited queries still failed because of the Command root/input accessibility-name mismatch. .scratch/comparison-parent-story-fixed.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreComparisonParentPicker.stories.svelte
```

**Notes**

Corrected search label and autofocus: one interaction/a11y story passes. .scratch/comparison-parent-story-label.log.
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
pnpm exec playwright test --config .scratch/workspace-browser.config.mts comparison-parents.spec.ts
```

**Notes**

Selection, exact parent request and reload succeeded; final unscoped content-fixture locator matched both graph and selected-operation buttons. Scoped to Selected operation without weakening the assertion. .scratch/comparison-parent-browser-label.log.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreComparisonParentPicker.stories.svelte
```

**Notes**

After author/copy addition: one passes, one accessibility failure. Sibling copy buttons violate listbox required-child semantics. .scratch/comparison-parent-story-avatar.log.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreComparisonParentPicker.stories.svelte
```

**Notes**

Embedding copy controls in option cards triggered nested-interactive accessibility failures in both stories. .scratch/comparison-parent-story-cards.log.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreComparisonParentPicker.stories.svelte
```

**Notes**

Attempted row/grid role props did not override native Command primitive roles. Role-specific queries failed; no accessibility rule was suppressed. .scratch/comparison-parent-story-grid.log.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreComparisonParentPicker.stories.svelte
```

**Notes**

Replacing listbox with an owned grid exposed a compile error: $props.id() must be a direct declaration initializer. No tests imported; fixed declaration placement. .scratch/comparison-parent-story-rich-grid.log.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreComparisonParentPicker.stories.svelte
```

**Notes**

Owned rich grid: independent copy story passes, identity fallback click hit the copy area of the row. Targeted the explicit selection button. .scratch/comparison-parent-story-rich-grid-fixed.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreComparisonParentPicker.stories.svelte catalog/components/store-application/StoreRepositoryNavigation.stories.svelte catalog/components/store-application/RepositoryReview.stories.svelte catalog/components/store-application/StoreRepositoryBranchContext.stories.svelte
```

**Notes**

All eight directly affected interaction/a11y checks pass, including the open rich grid, exact-ID fallback, keyboard selection, copied/failed feedback and copy remaining independent of selection. .scratch/comparison-parent-story-complete.log.
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
pnpm exec playwright test --config .scratch/workspace-browser.config.mts comparison-parents.spec.ts repository-navigation.spec.ts
```

**Notes**

Three navigation/error-notice/browser tests pass. Comparison copy locator matched both graph and dropdown instances; scoped it to Recorded parents. .scratch/comparison-parent-browser-complete.log.
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
pnpm exec playwright test --config .scratch/workspace-browser.config.mts comparison-parents.spec.ts
```

**Notes**

Exact regression passes on fresh built UI preview: desktop and 390px header order/bounds, no explanation box, pointer/keyboard full-ID copy without selection, open-grid axe audit, description search, native parent request, pinned workspace/snapshot/hash and reload restoration. .scratch/comparison-parent-browser-scoped.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreComparisonParentPicker.stories.svelte
```

**Notes**

Final 4px card gap: both focused interaction/a11y stories pass in a fresh runner. .scratch/comparison-parent-story-gap.log.
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
pnpm web:check
```

**Notes**

Initial and label-fixed passes: zero errors/warnings. Catalog validates 73 modules.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check
```

**Notes**

Owned-grid intermediate build failed only the $props.id() declaration placement; corrected.
<!-- /run -->
<!-- /evidence-field -->

