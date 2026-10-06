---
schema: 1
id: E-UI-041
---

# E-UI-041 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreRepositoryNavigation.stories.svelte catalog/components/store-mutations/StoreConflictEditor.stories.svelte
```

**Notes**

Before: 3 failed, 9 passed. Log .scratch/evolution-chrome-before.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreRepositoryNavigation.stories.svelte catalog/components/store-mutations/StoreConflictEditor.stories.svelte
```

**Notes**

After: all 12 interaction/accessibility stories pass, including independent evolution column open/close, collapsed/expanded zero margin and 1px conflict header border. Log .scratch/evolution-chrome-after.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreRepositoryNavigation.stories.svelte catalog/components/store-mutations/StoreConflictEditor.stories.svelte catalog/components/store-application/StoreRepositoryBranchContext.stories.svelte catalog/components/store-mutations/StoreRepositoryActions.stories.svelte
```

**Notes**

17 directly affected navigation, branch context, action and conflict interaction/accessibility stories pass. Log .scratch/evolution-stories.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-store-evolution.test.ts web/tests/vite/store-route-history.test.ts web/tests/vite/store-workspaces.test.ts
```

**Notes**

19 tests pass: full pinned native identity, predecessor rather than commit-parent edges, opaque exact-snapshot paging, deduplication, incomplete history, mismatch/repeated-cursor rejection, late-response guards and 1000-node display bound, plus route-history and workspace regressions. Log .scratch/evolution-unit.log.
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
pnpm exec tsx --test web/tests/store-route-codec.test.ts web/tests/production-store-loaders.test.ts web/tests/production-store-search.test.ts
```

**Notes**

22 direct-consumer route codec, loaders and search tests pass, including evolution route validation/roundtrip. Log .scratch/evolution-node.log.
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
pnpm web:check
```

**Notes**

Zero errors/warnings. Log .scratch/evolution-web-check.log.
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
pnpm catalog:check
```

**Notes**

Story uses unsupported Testing Library exact option. Corrected to anchored Op Log regex; production unaffected. Log .scratch/evolution-catalog-check.log.
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
pnpm catalog:check
```

**Notes**

Zero diagnostics; 76-module boundary audit passes after correction. Log .scratch/evolution-catalog-check-after.log.
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
pnpm check:architecture
```

**Notes**

Repository log component has 303 counted lines against 300 maximum. Extracted its existing leaf CSS unchanged into repository-log.css.
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
pnpm check:architecture
```

**Notes**

295-module boundary and source-size audit passes after style extraction.
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
pnpm web:build
```

**Notes**

Fresh UI build passes with existing nonfatal CSS minification and bundle-size warnings; no Rust build. Log .scratch/evolution-build.log.
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
pnpm exec playwright test --config .scratch/workspace-browser.config.mts web/tests/browser/evolution.spec.ts web/tests/browser/context-actions.spec.ts
```

**Notes**

First cold run: 1 failed, 4 passed. Test incorrectly clicked nonexistent Load more history button; graph uses automatic virtual-row pagination. Corrected to verify predecessor row and cursor request. Log .scratch/evolution-browser.log.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec playwright test --config .scratch/workspace-browser.config.mts web/tests/browser/evolution.spec.ts web/tests/browser/context-actions.spec.ts
```

**Notes**

Second cold run: 1 failed, 4 passed. Paging, selected-anchor browsing, reload and back pass; final test toggle closes already-open rail. Corrected to ensure rail visible before navigation. Log .scratch/evolution-browser-after.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreEvolutionColumn.stories.svelte
```

**Notes**

4 interaction/accessibility state stories pass: loading, empty, failed, incomplete predecessor history. Stories remain visual-pending. Log .scratch/evolution-states.log.
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
pnpm exec prettier --write web/tests/browser/evolution.spec.ts web/tests/browser/context-actions.spec.ts
```

**Notes**

Final browser navigation correction and enabled read-only Evo action with empty mutation capabilities formatted.
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
pnpm exec playwright test --config .scratch/workspace-browser.config.mts web/tests/browser/evolution.spec.ts web/tests/browser/context-actions.spec.ts
```

**Notes**

Final cold fresh-preview run: all 5 browser tests pass. Native evolution requests use full IDs and pinned operation; automatic paging, independent anchor selection without mutation retargeting, reload, close, back, navigation clearing and read-only action without mutation capabilities pass. Existing selected operation/change rail actions still pass. Log .scratch/evolution-browser-final.log.
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

Final zero errors/warnings. Log .scratch/evolution-web-check-final.log.
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
pnpm catalog:check
```

**Notes**

Final zero diagnostics; 78-module production/catalog boundary audit passes. Log .scratch/evolution-catalog-check-final.log.
<!-- /run -->
<!-- /evidence-field -->

