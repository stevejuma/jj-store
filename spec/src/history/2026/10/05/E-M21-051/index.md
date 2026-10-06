---
schema: 1
id: E-M21-051
---

# E-M21-051

<!-- evidence-field:id markdown -->
## Id

E-M21-051
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Bounded native dependent segments and preserved per-column diff ownership
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Existing toolchain; native JJ 0.45.1 graph read profile
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Internal segment boundaries nullable until proven; TOML and HTTP contracts unchanged
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-stack-workspace/StoreStackWorkspaceBranches.stories.svelte
- catalog/components/store-stack-workspace/StoreStackWorkspaceProduction.stories.svelte
- catalog/fixtures/StackWorkspaceProductionFixture.svelte
- catalog/fixtures/stack-workspace/production-http-data.ts
- catalog/fixtures/stack-workspace/production-http.ts
- spec/src/api.md
- spec/src/local-jj.md
- spec/src/milestones.md
- spec/src/spec-governance.md
- spec/src/ui.md
- web/src/lib/controller/stack-workspace/controller.svelte.ts
- web/src/lib/controller/stack-workspace/diff-selection.ts
- web/src/lib/controller/stack-workspace/files.ts
- web/src/lib/controller/stack-workspace/issued-history.ts
- web/src/lib/controller/stack-workspace/loaded-ranges.ts
- web/src/lib/controller/stack-workspace/retained-diffs.ts
- web/src/lib/controller/stack-workspace/source-range.ts
- web/src/lib/controller/stack-workspace/stack-history.svelte.ts
- web/src/lib/controller/stack-workspace/stack-segments.ts
- web/src/lib/controller/stack-workspace/types.ts
- web/src/lib/ui/store-stack-workspace/StoreStackBookmarkCard.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackCommitActions.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackLane.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceCanvas.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceDiff.svelte
- web/tests/vite/production-stack-adapter.test.ts
- web/tests/vite/stack-workspace-segments.test.ts
- web/tests/vite/stack-workspace.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/vite/stack-workspace-segments.test.ts#dependent segments use native
  parent links and compare each head with its lower boundary
- web/tests/vite/stack-workspace-segments.test.ts#partial segment boundaries
  remain unavailable until bounded continuation proves them
- web/tests/vite/production-stack-adapter.test.ts#dependent native ranges retain
  top cursors and issue per-segment tree comparisons
<!-- /evidence-field -->

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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/stack-workspace-segments.test.ts (before)
```

**Notes**

Segment projection module absent.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/stack-workspace-segments.test.ts web/tests/vite/production-stack-adapter.test.ts web/tests/vite/stack-workspace-lazy-controller.test.ts web/tests/vite/stack-workspace.test.ts
```

**Notes**

70 units pass, including native segment comparisons and independent diff ownership for shared commits.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceProduction.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceBranches.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceLazy.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceMenus.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceCommands.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceHeaderDrops.stories.svelte (initial)
```

**Notes**

Branch body story still expected Delete local to be disabled after qualified native bookmark deletion. Updated to verify availability.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceProduction.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceBranches.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceLazy.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceMenus.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceCommands.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceHeaderDrops.stories.svelte
```

**Notes**

33 interaction/accessibility scenarios pass; native dependent stack story included.
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
pnpm catalog:check (initial)
```

**Notes**

Story used unsupported Testing Library exact option; removed it while retaining exact name string.
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
pnpm web:check; pnpm catalog:check; pnpm check:ui-ownership
```

**Notes**

Initial focused types/ownership passed before fixture extraction; source budget failure and subsequent extraction checks are recorded below.
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
pnpm check:architecture (initial)
```

**Notes**

production-http.ts had 304 counted lines against the existing 300-line budget. Extracted fixture data; no budget increase.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 10
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:check (fixture extraction initial)
```

**Notes**

Extracted fixture declarations were missing exports. Restored the intended exports.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceProduction.stories.svelte (fixture extraction initial)
```

**Notes**

One suite could not import missing base export; fixed with the fixture exports.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:check (fixture extraction second)
```

**Notes**

Type-only api/client import violated the catalog transport boundary. Replaced it with a local plain fixture shape; no allowlist changes.
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
pnpm catalog:check (fixture extraction final)
```

**Notes**

Zero diagnostics; catalog boundaries pass for 233 modules. segments-catalog-final-accepted.log.
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
pnpm check:architecture (fixture extraction final)
```

**Notes**

1110 modules pass existing source budgets. segments-architecture-accepted.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceProduction.stories.svelte (fixture extraction final)
```

**Notes**

3 production interaction/accessibility stories pass after extraction. segments-leaf-stories-after.log.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec spec-validator first (initial)
```

**Notes**

Missing mapped API/GOV chapter changes. Updated both canonical chapters.
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
pnpm exec spec-validator first (final); pnpm check:progress
```

**Notes**

25 protected files and 5 canonical chapters pass; 142 task progress bookkeeping passes.
<!-- /run -->
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M21-051 details 2](details-02.md)
