---
schema: 1
id: E-M20-002
---

# E-M20-002 details 3

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
spec-validator validate --only ui-ownership
```

**Notes**

Output: .local/stack-header-drops-evidence/final-ownership-check.log. Specification validated: 1 validators, 23 chapters, 173 requirements.
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
svelte-check --tsconfig web/tsconfig.json
```

**Notes**

Output: .local/stack-header-drops-evidence/final-web-check.log. svelte-check found 0 errors and 0 warnings
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
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceRefinements.stories.svelte -t 'ColumnScrollbarsKeepAnOuterGutter|BranchAndCommitChipsRemainDistinctOnHover' --reporter=verbose
```

**Notes**

Output: .local/stack-header-drops-evidence/gutter-chip-corrected-final.log. Test Files  1 passed (1) | Tests  2 passed | 5 skipped (7)
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
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceRefinements.stories.svelte -t 'ColumnScrollbarsKeepAnOuterGutter|BranchAndCommitChipsRemainDistinctOnHover' --reporter=verbose
```

**Notes**

Output: .local/stack-header-drops-evidence/gutter-chip-exact-final.log. Test Files  1 failed (1) | Tests  1 failed | 1 passed | 5 skipped (7) Gutter and chip paint assertions passed, but the branch page lookup used a nonexistent region name. Corrected to the rendered column identity; measurements retained.
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
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceRefinements.stories.svelte catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.stories.svelte --reporter=verbose
```

**Notes**

Output: .local/stack-header-drops-evidence/hover-consumers-exact-final.log. Test Files  2 passed (2) | Tests  10 passed (10)
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
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceHeaderDrops.stories.svelte -t LeftMarkersReplaceSelectedChips --reporter=verbose
```

**Notes**

Output: .local/stack-header-drops-evidence/marker-only-before.log. Test Files  1 failed (1) | Tests  1 failed | 10 skipped (11)
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
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceHeaderDrops.stories.svelte -t BranchActionsDifferFromAdjacentCommitHover --reporter=verbose
```

**Notes**

Output: .local/stack-header-drops-evidence/native-hover-before.log. Test Files  1 passed (1) | Tests  1 passed | 8 skipped (9) Native hover passed for the original default action-bar surface; the subsequently requested softer 4% action background was still implemented and qualified.
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
spec-validator validate --only ui-ownership
```

**Notes**

Output: .local/stack-header-drops-evidence/ownership-check.log. Specification validated: 1 validators, 23 chapters, 173 requirements.
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
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceHeaderDrops.stories.svelte -t ColumnScrollbarsKeepAnOuterGutter --reporter=verbose
```

**Notes**

Output: .local/stack-header-drops-evidence/scroll-gutter-before.log. Test Files  1 failed (1) | Tests  1 failed | 11 skipped (12)
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
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceRefinements.stories.svelte -t ColumnScrollbarsKeepAnOuterGutter --reporter=verbose
```

**Notes**

Output: .local/stack-header-drops-evidence/scroll-gutter-exact-final.log. Test Files  1 passed (1) | Tests  1 passed | 6 skipped (7)
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
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceHeaderDrops.stories.svelte --testNamePattern InlineFilesAndSelected --reporter=verbose
```

**Notes**

Output: .local/stack-header-drops-evidence/selection-story-before.log. Test Files  1 failed (1) | Tests  1 failed | 5 skipped (6)
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
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceHeaderDrops.stories.svelte -t InlineFilesAndSelectedRowsUseInsetSpacingAndLeftMarkers --reporter=verbose
```

**Notes**

Output: .local/stack-header-drops-evidence/selection-story-final.log. Test Files  1 passed (1) | Tests  1 passed | 5 skipped (6)
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
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceHeaderDrops.stories.svelte -t ExpandedBranchAndCommitFilesUseAShortHeading --reporter=verbose
```

**Notes**

Output: .local/stack-header-drops-evidence/short-heading-before.log. Test Files  1 failed (1) | Tests  1 failed | 9 skipped (10)
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
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceHeaderDrops.stories.svelte --testNamePattern CommitRowThresholds --reporter=verbose
```

**Notes**

Output: .local/stack-header-drops-evidence/sorting-story-before.log. Test Files  1 failed (1) | Tests  1 failed | 4 skipped (5)
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/stack-workspace-sorting.test.ts
```

**Notes**

Output: .local/stack-header-drops-evidence/sorting-unit-before.log. Test Files  1 failed (1) | Tests  2 failed (2)
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
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceHeaderDrops.stories.svelte --reporter=verbose
```

**Notes**

Output: .local/stack-header-drops-evidence/stories-after.log. Test Files  1 failed (1) | Tests  1 failed | 5 passed (6) One selection assertion retained a live CSSStyleDeclaration; copied primitive values before selecting another row. Marker assertions retained.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceHeaderDrops.stories.svelte --reporter=verbose
```

**Notes**

Output: .local/stack-header-drops-evidence/stories-before.log. Test Files  1 failed (1) | Tests  4 failed (4)
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
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace catalog/components/community-projects/CommunityProjectsRevisionGraph.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLogFiles.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLogDiff.stories.svelte catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.stories.svelte catalog/components/store-application/StoreRepositoryPicker.stories.svelte catalog/components/store-application/StoreRepositoryNavigation.stories.svelte catalog/components/store-application/StoreRepositoryBranchContext.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte --reporter=verbose
```

**Notes**

Output: .local/stack-header-drops-evidence/stories-consumers-final.log. Test Files  18 passed (18) | Tests  96 passed (96)
<!-- /run -->
<!-- /evidence-field -->

