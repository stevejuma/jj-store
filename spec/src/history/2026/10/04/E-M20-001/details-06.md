---
schema: 1
id: E-M20-001
---

# E-M20-001 details 6

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

Output: .local/stack-workspace-ui-evidence/final-ownership.log. Specification validated: 1 validators, 23 chapters, 173 requirements. 
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
node scripts/check-progress.mjs
```

**Notes**

Output: .local/stack-workspace-ui-evidence/final-progress.log. Command completed. 
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
spec-validator first
```

**Notes**

Output: .local/stack-workspace-ui-evidence/final-spec-first.log. Spec-first gate passed: 55 protected file(s), 4 canonical chapter(s). 
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
spec-validator validate
```

**Notes**

Output: .local/stack-workspace-ui-evidence/final-spec.log. Specification validated: 5 validators, 23 chapters, 173 requirements. 
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 6
hasNote: true
noteEncoding: markdown

```

```shell
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace catalog/components/community-projects/CommunityProjectsRevisionGraph.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLogFiles.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLogDiff.stories.svelte catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.stories.svelte catalog/components/store-application/StoreRepositoryPicker.stories.svelte catalog/components/store-application/StoreRepositoryNavigation.stories.svelte catalog/components/store-application/StoreRepositoryBranchContext.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte
```

**Notes**

Output: .local/stack-workspace-ui-evidence/final-stories.log. ⎯⎯⎯⎯⎯⎯⎯ Failed Tests 6 ⎯⎯⎯⎯⎯⎯⎯ | Test Files  3 failed | 11 passed (14) | Tests  6 failed | 70 passed (76) 
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
svelte-check --tsconfig web/tsconfig.json
```

**Notes**

Output: .local/stack-workspace-ui-evidence/final-web.log. svelte-check found 0 errors and 0 warnings 
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
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspacePresentation.stories.svelte -t 'UnstagedHeaderAndStyledCanvas|LinkedBookmarksAndOptionalActions' --reporter=verbose
```

**Notes**

Output: .local/stack-workspace-ui-evidence/header-typography-final.log. Test Files  1 passed (1) | Tests  2 passed | 2 skipped (4) Exact centered Unstaged header geometry and branch title font family, size and weight passed; 2/2 stories.
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
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspace.stories.svelte -t Immutable --reporter=verbose
```

**Notes**

Output: .local/stack-workspace-ui-evidence/immutable-diagnostic.log. Test Files  1 passed (1) | Tests  1 passed | 13 skipped (14) 
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
node scripts/web-architecture.mjs
```

**Notes**

Output: .local/stack-workspace-ui-evidence/latest-architecture-first.log. Command completed. Stack story exceeded 400-line catalog budget; extracted presentation stories.
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
node scripts/web-architecture.mjs
```

**Notes**

Output: .local/stack-workspace-ui-evidence/latest-architecture-second.log. web architecture ok (995 modules) 
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

Output: .local/stack-workspace-ui-evidence/latest-catalog.log. svelte-check found 0 errors and 0 warnings | catalog boundaries ok (204 modules) 
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

Output: .local/stack-workspace-ui-evidence/latest-ownership.log. Specification validated: 1 validators, 23 chapters, 173 requirements. 
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
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace -t 'IconToolbar|UnstagedHeader|LinkedBookmarks|BookmarkActionBars|WorkspaceDraggable|WorkspaceInAppShell'
```

**Notes**

Output: .local/stack-workspace-ui-evidence/latest-stories-fifth.log. ⎯⎯⎯⎯⎯⎯⎯ Failed Tests 1 ⎯⎯⎯⎯⎯⎯⎯ | Test Files  1 failed | 2 passed | 2 skipped (5) | Tests  1 failed | 5 passed | 30 skipped (36) Workspace search input uses a label rather than combobox role; repaired against the rendered public InputGroup input.
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
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace -t 'IconToolbar|UnstagedHeader|LinkedBookmarks|BookmarkActionBars|WorkspaceDraggable'
```

**Notes**

Output: .local/stack-workspace-ui-evidence/latest-stories-first.log. ⎯⎯⎯⎯⎯⎯⎯ Failed Tests 2 ⎯⎯⎯⎯⎯⎯⎯ | Test Files  2 failed | 1 passed | 1 skipped (4) | Tests  2 failed | 3 passed | 23 skipped (28) Vite optimization reload and obsolete group/tooltip selectors; explicit icon prebundling and observed public selectors repaired.
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
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace -t 'IconToolbar|UnstagedHeader|LinkedBookmarks|BookmarkActionBars|WorkspaceDraggable'
```

**Notes**

Output: .local/stack-workspace-ui-evidence/latest-stories-fourth.log. ⎯⎯⎯⎯⎯⎯⎯ Failed Tests 2 ⎯⎯⎯⎯⎯⎯⎯ | Test Files  1 failed | 1 passed | 3 skipped (5) | Tests  2 failed | 3 passed | 31 skipped (36) Tooltip selector and 0.75-pixel graph connector mismatch; repaired exact public selector and removed false card-border assumption.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace -t 'IconToolbar|UnstagedHeader|LinkedBookmarks|BookmarkActionBars|WorkspaceDraggable'
```

**Notes**

Output: .local/stack-workspace-ui-evidence/latest-stories-second.log. ⎯⎯⎯⎯⎯⎯⎯ Failed Tests 2 ⎯⎯⎯⎯⎯⎯⎯ | Test Files  1 failed | 1 passed | 3 skipped (5) | Tests  2 failed | 3 passed | 31 skipped (36) Two remaining public group/tooltip selector failures.
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
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace -t 'IconToolbar|UnstagedHeader|LinkedBookmarks|BookmarkActionBars|WorkspaceDraggable|WorkspaceInAppShell'
```

**Notes**

Output: .local/stack-workspace-ui-evidence/latest-stories-seventh.log. Test Files  3 passed | 2 skipped (5) | Tests  6 passed | 30 skipped (36) Exact final refinements: picker search/switch/cancel, app-shell header centering, column keyboard resizing, sticky return, bookmark links/actions and hunk drag.
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
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace -t 'IconToolbar|UnstagedHeader|LinkedBookmarks|BookmarkActionBars|WorkspaceDraggable|WorkspaceInAppShell'
```

**Notes**

Output: .local/stack-workspace-ui-evidence/latest-stories-sixth.log. ⎯⎯⎯⎯⎯⎯⎯ Failed Tests 1 ⎯⎯⎯⎯⎯⎯⎯ | Test Files  1 failed | 2 passed | 2 skipped (5) | Tests  1 failed | 5 passed | 30 skipped (36) Five passes; new sticky geometry regression found the clipped external return rail upstream.
<!-- /run -->
<!-- /evidence-field -->

