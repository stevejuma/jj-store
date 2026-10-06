---
schema: 1
id: E-M20-001
---

# E-M20-001 details 12

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
pnpm exec prettier --write web/src/lib/controller/stack-workspace/commit-drafts.svelte.ts web/src/lib/controller/stack-workspace/controller.svelte.ts web/src/lib/controller/stack-workspace/drag-session.svelte.ts web/src/lib/controller/stack-workspace/drag.ts web/src/lib/controller/stack-workspace/file-drag.ts web/src/lib/controller/stack-workspace/files.ts web/src/lib/controller/stack-workspace/presentation.ts web/src/lib/controller/stack-workspace/types.ts web/src/lib/ui/store-stack-workspace/stack-workspace.css web/src/lib/ui/store-stack-workspace/StoreStackBookmarkActions.svelte web/src/lib/ui/store-stack-workspace/StoreStackBookmarkCard.svelte web/src/lib/ui/store-stack-workspace/StoreStackCommitActions.svelte web/src/lib/ui/store-stack-workspace/StoreStackCommitComposer.svelte web/src/lib/ui/store-stack-workspace/StoreStackDiffHunk.svelte web/src/lib/ui/store-stack-workspace/StoreStackDropGuide.svelte web/src/lib/ui/store-stack-workspace/StoreStackLane.svelte web/src/lib/ui/store-stack-workspace/StoreStackUnstaged.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspace.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceActions.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceCanvas.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceChoice.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceConflict.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceDiff.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceForm.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspacePicker.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspacePreview.svelte web/src/lib/ui/community-projects/CommunityProjectsRevisionGraph.svelte web/src/lib/ui/community-projects/CommunityProjectsRepositoryLogFiles.svelte catalog/fixtures/stack-workspace/adapter.ts catalog/fixtures/stack-workspace/changes.ts catalog/fixtures/stack-workspace/data.ts catalog/fixtures/stack-workspace/preview.ts catalog/fixtures/stack-workspace/reducer.ts catalog/fixtures/stack-workspace/story-actions.ts catalog/fixtures/stack-workspace/validation.ts catalog/fixtures/StackWorkspaceFixture.svelte catalog/fixtures/StackWorkspaceApplicationFixture.svelte catalog/components/store-stack-workspace/StoreStackWorkspace.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceCommands.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceDrag.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspacePresentation.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceRecovery.stories.svelte web/tests/vite/stack-workspace.test.ts spec/src/ui.md
```

**Notes**

Output: .local/stack-workspace-ui-evidence/drag-format-final.log. Command completed; formatter output or installation completion retained.
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
vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/stack-workspace.test.ts web/tests/vite/store-route-history.test.ts
```

**Notes**

Output: .local/stack-workspace-ui-evidence/drag-unit-final.log. Test Files  2 passed (2) | Tests  46 passed (46)
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace -t 'DiffHeader|WholeStackHeaders|WorkspaceInAppShell|LinkedBookmarks|InlineCommit|DropChangesOnto|UnstagedFilesStage|FolderDrops|RejectedAndCancelled'
```

**Notes**

Output: .local/stack-workspace-ui-evidence/drag-stories-exact.log. ⎯⎯⎯⎯⎯⎯⎯ Failed Tests 3 ⎯⎯⎯⎯⎯⎯⎯ | Test Files  1 failed | 3 passed | 2 skipped (6) | Tests  3 failed | 6 passed | 33 skipped (42) Three drag regressions: initial cursor feedback and two stale public selectors; all three exact drag stories passed after repair.
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
pnpm exec prettier --write .storybook/vite.config.ts web/src/lib/ui/store-stack-workspace/StoreStackDropGuide.svelte web/src/lib/controller/stack-workspace/controller.svelte.ts catalog/components/store-stack-workspace/StoreStackWorkspaceDrag.stories.svelte web/tests/vite/stack-workspace.test.ts docs/ui-provenance.json
```

**Notes**

Output: .local/stack-workspace-ui-evidence/drag-regressions-format.log. Command completed; formatter output or installation completion retained.
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
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceDrag.stories.svelte
```

**Notes**

Output: .local/stack-workspace-ui-evidence/drag-stories-after.log. Test Files  1 passed (1) | Tests  3 passed (3)
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/stack-workspace.test.ts web/tests/vite/store-route-history.test.ts
```

**Notes**

Output: .local/stack-workspace-ui-evidence/drag-unit-final2.log. ⎯⎯⎯⎯⎯⎯⎯ Failed Tests 1 ⎯⎯⎯⎯⎯⎯⎯ | Test Files  1 failed | 1 passed (2) | Tests  1 failed | 46 passed (47) New drag-session test lacked its import; one failure, repaired in drag-unit-final3.log.
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
svelte-check --tsconfig web/tsconfig.json
```

**Notes**

Output: .local/stack-workspace-ui-evidence/drag-web-final.log. svelte-check found 0 errors and 0 warnings
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
pnpm exec prettier --write web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceForm.svelte web/tests/vite/stack-workspace.test.ts catalog/components/store-stack-workspace/StoreStackWorkspaceDrag.stories.svelte spec/src/api.md
```

**Notes**

Output: .local/stack-workspace-ui-evidence/drag-final-format-write.log. Command completed; formatter output or installation completion retained.
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
node scripts/web-architecture.mjs
```

**Notes**

Output: .local/stack-workspace-ui-evidence/drag-architecture-final.log. web architecture ok (1002 modules)
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
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceDrag.stories.svelte
```

**Notes**

Output: .local/stack-workspace-ui-evidence/drag-stories-final.log. Test Files  1 passed (1) | Tests  3 passed (3)
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
svelte-check --tsconfig catalog/tsconfig.json --fail-on-warnings && node scripts/check-catalog.mjs
```

**Notes**

Output: .local/stack-workspace-ui-evidence/drag-catalog-final.log. Command completed; formatter output or installation completion retained. One unsupported exact property on ByRoleOptions; repaired against the public role API.
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

Output: .local/stack-workspace-ui-evidence/drag-ownership-final.log. Specification validated: 1 validators, 23 chapters, 173 requirements.
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

Output: .local/stack-workspace-ui-evidence/drag-catalog-final2.log. svelte-check found 0 errors and 0 warnings | catalog boundaries ok (205 modules)
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
pnpm exec prettier --write catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceCommands.stories.svelte
```

**Notes**

Output: .local/stack-workspace-ui-evidence/drag-consumer-tests-format.log. Command completed; formatter output or installation completion retained.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace catalog/components/community-projects/CommunityProjectsRevisionGraph.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLogFiles.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLogDiff.stories.svelte catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.stories.svelte catalog/components/store-application/StoreRepositoryPicker.stories.svelte catalog/components/store-application/StoreRepositoryNavigation.stories.svelte catalog/components/store-application/StoreRepositoryBranchContext.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte --reporter=verbose
```

**Notes**

Output: .local/stack-workspace-ui-evidence/drag-direct-consumers-final.log. ⎯⎯⎯⎯⎯⎯⎯ Failed Tests 4 ⎯⎯⎯⎯⎯⎯⎯ | Test Files  3 failed | 12 passed (15) | Tests  4 failed | 78 passed (82) 78/82 passed. Immutable menu availability and changed source/header selectors exposed four failures; exact regressions passed before the final 90-story consumer run.
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
svelte-check --tsconfig web/tsconfig.json
```

**Notes**

Output: .local/stack-workspace-ui-evidence/drag-web-final2.log. svelte-check found 0 errors and 0 warnings
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
pnpm exec prettier --write catalog/components/store-stack-workspace/StoreStackWorkspace.stories.svelte
```

**Notes**

Output: .local/stack-workspace-ui-evidence/drag-fixture-tests-format.log. Command completed; formatter output or installation completion retained.
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
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceCommands.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceDrag.stories.svelte -t 'DragAHunkToACommit|WholeStackFilesAndContinuousGraph|FolderDrops'
```

**Notes**

Output: .local/stack-workspace-ui-evidence/drag-consumer-regressions-final.log. Test Files  3 passed (3) | Tests  3 passed | 14 skipped (17)
<!-- /run -->
<!-- /evidence-field -->

