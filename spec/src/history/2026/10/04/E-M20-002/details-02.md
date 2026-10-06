---
schema: 1
id: E-M20-002
---

# E-M20-002 details 2

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
node scripts/web-architecture.mjs
```

**Notes**

Output: .local/stack-header-drops-evidence/all-architecture-final.log.  New header story family exceeded the 400 counted-line budget; split presentation refinements from header drag scenarios.
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
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace catalog/components/community-projects/CommunityProjectsRevisionGraph.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLogFiles.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLogDiff.stories.svelte catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.stories.svelte catalog/components/store-application/StoreRepositoryPicker.stories.svelte catalog/components/store-application/StoreRepositoryNavigation.stories.svelte catalog/components/store-application/StoreRepositoryBranchContext.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte --reporter=verbose
```

**Notes**

Output: .local/stack-header-drops-evidence/all-consumer-stories-final.log. Test Files  1 failed | 18 passed (19) | Tests  1 failed | 102 passed (103) 102/103 passed; existing BookmarkCards CSS assertion failed with inherited native pointer position. Exact standalone and paired scenarios passed; new pointer stories now park at the neutral corner. Assertion unchanged.
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
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceHeaderDrops.stories.svelte --reporter=verbose
```

**Notes**

Output: .local/stack-header-drops-evidence/all-header-refinements-final.log. Test Files  1 passed (1) | Tests  11 passed (11)
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
node scripts/web-architecture.mjs
```

**Notes**

Output: .local/stack-header-drops-evidence/architecture-check.log. web architecture ok (1012 modules)
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
node scripts/web-architecture.mjs
```

**Notes**

Output: .local/stack-header-drops-evidence/architecture-final.log. web architecture ok (1012 modules)
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
node scripts/web-architecture.mjs
```

**Notes**

Output: .local/stack-header-drops-evidence/architecture-qualified.log. web architecture ok (1013 modules)
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
node scripts/web-architecture.mjs
```

**Notes**

Output: .local/stack-header-drops-evidence/body-architecture-check.log. web architecture ok (1012 modules)
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceHeaderDrops.stories.svelte -t 'InlineFilesAndSelectedRowsUseInsetSpacingAndLeftMarkers|CanvasStartsWithoutTopPadding|BranchActionsDifferFromAdjacentCommitHover|SelectedBranchChipsShareAWrappingRow' --reporter=verbose
```

**Notes**

Output: .local/stack-header-drops-evidence/body-refinements-after.log. Test Files  1 failed (1) | Tests  2 failed | 2 passed | 5 skipped (9) Two interim failures: synthetic hover did not activate CSS, and chips of different heights require center alignment. Corrected native pointer and center measurement without relaxing geometric tolerance.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceHeaderDrops.stories.svelte -t 'InlineFilesAndSelectedRowsUseInsetSpacingAndLeftMarkers|CanvasStartsWithoutTopPadding|BranchActionsDifferFromAdjacentCommitHover' --reporter=verbose
```

**Notes**

Output: .local/stack-header-drops-evidence/body-refinements-before.log. Test Files  1 failed (1) | Tests  3 failed | 5 skipped (8) Three requested layout regressions reproduced. The initial background comparison used synthetic hover; the native follow-up is authoritative for CSS hover.
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
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.stories.svelte -t BookmarkCards --reporter=verbose
```

**Notes**

Output: .local/stack-header-drops-evidence/branch-hover-exact.log. Test Files  1 passed (1) | Tests  1 passed | 2 skipped (3)
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
svelte-check --tsconfig catalog/tsconfig.json --fail-on-warnings && node scripts/check-catalog.mjs
```

**Notes**

Output: .local/stack-header-drops-evidence/catalog-check.log. svelte-check found 0 errors and 0 warnings | catalog boundaries ok (210 modules)
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
svelte-check --tsconfig catalog/tsconfig.json --fail-on-warnings && node scripts/check-catalog.mjs
```

**Notes**

Output: .local/stack-header-drops-evidence/catalog-final-check.log. svelte-check found 0 errors and 0 warnings | catalog boundaries ok (210 modules)
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
svelte-check --tsconfig catalog/tsconfig.json --fail-on-warnings && node scripts/check-catalog.mjs
```

**Notes**

Output: .local/stack-header-drops-evidence/catalog-qualified.log. svelte-check found 0 errors and 0 warnings | catalog boundaries ok (211 modules)
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
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceRefinements.stories.svelte -t BranchAndCommitChipsRemainDistinctOnHover --reporter=verbose
```

**Notes**

Output: .local/stack-header-drops-evidence/chip-contrast-before.log. Test Files  1 failed (1) | Tests  1 failed | 6 skipped (7)
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceHeaderDrops.stories.svelte -t SelectedBranchChipsShareAWrappingRow --reporter=verbose
```

**Notes**

Output: .local/stack-header-drops-evidence/chips-before.log. Test Files  1 failed (1) | Tests  1 failed | 8 skipped (9)
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
node scripts/web-architecture.mjs
```

**Notes**

Output: .local/stack-header-drops-evidence/final-architecture-check.log. web architecture ok (1013 modules)
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
svelte-check --tsconfig catalog/tsconfig.json --fail-on-warnings && node scripts/check-catalog.mjs
```

**Notes**

Output: .local/stack-header-drops-evidence/final-catalog-check.log. svelte-check found 2 errors and 0 warnings in 1 file Two unsupported Testing Library exact options removed; exact tab name uses an anchored expression, branch body uses its public column identity.
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
svelte-check --tsconfig catalog/tsconfig.json --fail-on-warnings && node scripts/check-catalog.mjs
```

**Notes**

Output: .local/stack-header-drops-evidence/final-catalog-repaired.log. svelte-check found 0 errors and 0 warnings | catalog boundaries ok (211 modules)
<!-- /run -->
<!-- /evidence-field -->

