---
schema: 1
id: E-M20-001
---

# E-M20-001 details 4

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
svelte-check --tsconfig catalog/tsconfig.json --fail-on-warnings && node scripts/check-catalog.mjs
```

**Notes**

Output: .local/stack-workspace-ui-evidence/composer-final-catalog.log. svelte-check found 0 errors and 0 warnings | catalog boundaries ok (204 modules) 
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

Output: .local/stack-workspace-ui-evidence/composer-final-web.log. svelte-check found 0 errors and 0 warnings 
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
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/community-projects/CommunityProjectsRepositoryLogFiles.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte -t 'Paged|LargeVirtualizedRepository'
```

**Notes**

Output: .local/stack-workspace-ui-evidence/composer-list-regressions.log. Test Files  2 passed (2) | Tests  2 passed | 8 skipped (10) 
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

Output: .local/stack-workspace-ui-evidence/composer-observer-architecture.log. web architecture ok (998 modules) 
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
svelte-check --tsconfig catalog/tsconfig.json --fail-on-warnings && node scripts/check-catalog.mjs
```

**Notes**

Output: .local/stack-workspace-ui-evidence/composer-observer-catalog.log. svelte-check found 0 errors and 0 warnings | catalog boundaries ok (204 modules) 
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
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace catalog/components/community-projects/CommunityProjectsRevisionGraph.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLogFiles.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLogDiff.stories.svelte catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.stories.svelte catalog/components/store-application/StoreRepositoryPicker.stories.svelte catalog/components/store-application/StoreRepositoryNavigation.stories.svelte catalog/components/store-application/StoreRepositoryBranchContext.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte --reporter=verbose
```

**Notes**

Output: .local/stack-workspace-ui-evidence/composer-observer-consumers-final.log. Test Files  14 passed (14) | Tests  77 passed (77) 77/77 direct-consumer interaction/accessibility assertions passed. One unsuppressed ResizeObserver notice remains attributed to NarrowLayout; broader qualification retains this diagnostic.
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
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace catalog/components/community-projects/CommunityProjectsRevisionGraph.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLogFiles.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLogDiff.stories.svelte catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.stories.svelte catalog/components/store-application/StoreRepositoryPicker.stories.svelte catalog/components/store-application/StoreRepositoryNavigation.stories.svelte catalog/components/store-application/StoreRepositoryBranchContext.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte --reporter=verbose
```

**Notes**

Output: .local/stack-workspace-ui-evidence/composer-observer-diagnostic.log. Command completed. Verbose follow-up exposed one graph endpoint timing assertion failure; interrupted before completion to coalesce observer measurements and wait for the final geometry. No full acceptance claim.
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
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte -t WholeStackFilesAndContinuousGraph --reporter=verbose
```

**Notes**

Output: .local/stack-workspace-ui-evidence/composer-observer-exact.log. Test Files  1 passed (1) | Tests  1 passed | 7 skipped (8) Fresh process: the exact continuous graph endpoint regression passed with the unchanged geometry tolerance.
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
spec-validator first
```

**Notes**

Output: .local/stack-workspace-ui-evidence/composer-observer-spec-first.log. Spec-first gate passed: 57 protected file(s), 4 canonical chapter(s). 
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
spec-validator validate
```

**Notes**

Output: .local/stack-workspace-ui-evidence/composer-observer-spec.log. Specification validated: 5 validators, 23 chapters, 173 requirements. 
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
svelte-check --tsconfig web/tsconfig.json
```

**Notes**

Output: .local/stack-workspace-ui-evidence/composer-observer-web.log. svelte-check found 0 errors and 0 warnings 
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
spec-validator validate --only ui-ownership
```

**Notes**

Output: .local/stack-workspace-ui-evidence/composer-ownership.log. Specification validated: 1 validators, 23 chapters, 173 requirements. 
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
node scripts/check-progress.mjs
```

**Notes**

Output: .local/stack-workspace-ui-evidence/composer-progress-first.log. Command completed. New evidence test references omitted their required named-test suffix; repaired without changing historical evidence.
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
node scripts/check-progress.mjs
```

**Notes**

Output: .local/stack-workspace-ui-evidence/composer-progress-second.log. Command completed. 
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
spec-validator validate --only ui-ownership
```

**Notes**

Output: .local/stack-workspace-ui-evidence/composer-provenance-final.log. Specification validated: 1 validators, 23 chapters, 173 requirements. 
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
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspace.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceRecovery.stories.svelte -t 'CommitOneHunkAndUndo|StalePreviewRetainsDraft'
```

**Notes**

Output: .local/stack-workspace-ui-evidence/composer-recovery-exact.log. Test Files  2 passed (2) | Tests  2 passed | 17 skipped (19) 
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
spec-validator first
```

**Notes**

Output: .local/stack-workspace-ui-evidence/composer-spec-first.log. Spec-first gate passed: 57 protected file(s), 4 canonical chapter(s). 
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
spec-validator validate
```

**Notes**

Output: .local/stack-workspace-ui-evidence/composer-spec.log. Specification validated: 5 validators, 23 chapters, 173 requirements. 
<!-- /run -->
<!-- /evidence-field -->

