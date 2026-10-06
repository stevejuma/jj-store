---
schema: 1
id: E-M20-001
---

# E-M20-001 details 9

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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/stack-workspace.test.ts
```

**Notes**

Output: .local/stack-workspace-ui-evidence/controller-first.log. Test Files  1 passed (1) | Tests  17 passed (17)
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
pnpm exec prettier --write web/src/lib/controller/stack-workspace/controller.svelte.ts web/src/lib/controller/stack-workspace/drag.ts web/src/lib/controller/stack-workspace/files.ts web/src/lib/controller/stack-workspace/presentation.ts web/src/lib/controller/stack-workspace/types.ts web/src/lib/ui/store-stack-workspace/stack-workspace.css web/src/lib/ui/store-stack-workspace/StoreStackCommitActions.svelte web/src/lib/ui/store-stack-workspace/StoreStackLane.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspace.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceCanvas.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceChoice.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceConflict.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceDiff.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceForm.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspacePreview.svelte catalog/fixtures/stack-workspace/adapter.ts catalog/fixtures/stack-workspace/changes.ts catalog/fixtures/stack-workspace/data.ts catalog/fixtures/stack-workspace/reducer.ts catalog/fixtures/stack-workspace/story-actions.ts catalog/fixtures/stack-workspace/validation.ts catalog/fixtures/StackWorkspaceFixture.svelte catalog/components/store-stack-workspace/StoreStackWorkspace.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceRecovery.stories.svelte web/tests/vite/stack-workspace.test.ts spec/src/ui.md spec/src/api.md spec/src/spec-governance.md
```

**Notes**

Output: .local/stack-workspace-ui-evidence/format.log. Command completed; formatter output or installation completion retained.
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
pnpm exec prettier --write web/src/lib/controller/stack-workspace/controller.svelte.ts web/src/lib/controller/stack-workspace/drag.ts web/src/lib/controller/stack-workspace/files.ts web/src/lib/controller/stack-workspace/presentation.ts web/src/lib/controller/stack-workspace/types.ts web/src/lib/ui/store-stack-workspace/stack-workspace.css web/src/lib/ui/store-stack-workspace/StoreStackCommitActions.svelte web/src/lib/ui/store-stack-workspace/StoreStackLane.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspace.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceCanvas.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceChoice.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceConflict.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceDiff.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceForm.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspacePreview.svelte web/tests/vite/stack-workspace.test.ts docs/ui-provenance.json
```

**Notes**

Output: .local/stack-workspace-ui-evidence/format-refinement.log. Command completed; formatter output or installation completion retained.
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
pnpm exec prettier --write catalog/components/store-stack-workspace/StoreStackWorkspaceCommands.stories.svelte
```

**Notes**

Output: .local/stack-workspace-ui-evidence/format-stories.log. Command completed; formatter output or installation completion retained.
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
pnpm exec prettier --write catalog/components/store-stack-workspace/StoreStackWorkspace.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceCommands.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceRecovery.stories.svelte catalog/fixtures/stack-workspace/story-actions.ts
```

**Notes**

Output: .local/stack-workspace-ui-evidence/format-selectors.log. Command completed; formatter output or installation completion retained.
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
pnpm exec prettier --write catalog/fixtures/stack-workspace/adapter.ts catalog/fixtures/stack-workspace/changes.ts catalog/fixtures/stack-workspace/data.ts catalog/fixtures/stack-workspace/preview.ts catalog/fixtures/stack-workspace/reducer.ts catalog/fixtures/stack-workspace/story-actions.ts catalog/fixtures/stack-workspace/validation.ts web/src/lib/controller/stack-workspace/controller.svelte.ts web/src/lib/controller/stack-workspace/drag.ts web/src/lib/controller/stack-workspace/files.ts web/src/lib/controller/stack-workspace/presentation.ts web/src/lib/controller/stack-workspace/types.ts web/src/lib/ui/store-stack-workspace/stack-workspace.css web/src/lib/ui/store-stack-workspace/StoreStackCommitActions.svelte web/src/lib/ui/store-stack-workspace/StoreStackLane.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspace.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceCanvas.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceChoice.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceConflict.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceDiff.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceForm.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspacePreview.svelte catalog/components/store-stack-workspace/StoreStackWorkspace.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceCommands.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceRecovery.stories.svelte
```

**Notes**

Output: .local/stack-workspace-ui-evidence/format-preview.log. Command completed; formatter output or installation completion retained.
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
pnpm exec prettier --write web/tests/vite/stack-workspace.test.ts catalog/fixtures/stack-workspace/validation.ts web/src/lib/ui/store-stack-workspace/stack-workspace.css spec/src/ui.md
```

**Notes**

Output: .local/stack-workspace-ui-evidence/format-final-product.log. Command completed; formatter output or installation completion retained.
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
pnpm exec prettier --write catalog/fixtures/StackWorkspaceFixture.svelte catalog/components/store-stack-workspace/StoreStackWorkspace.stories.svelte
```

**Notes**

Output: .local/stack-workspace-ui-evidence/format-narrow.log. Command completed; formatter output or installation completion retained.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/stack-workspace.test.ts web/tests/vite/store-route-history.test.ts
```

**Notes**

Output: .local/stack-workspace-ui-evidence/unit-shell-first.log. Test Files  2 passed (2) | Tests  31 passed (31)
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte
```

**Notes**

Output: .local/stack-workspace-ui-evidence/shell-stories-first.log. Test Files  1 failed (1) | Tests  no tests
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte
```

**Notes**

Output: .local/stack-workspace-ui-evidence/shell-stories-second.log. ⎯⎯⎯⎯⎯⎯⎯ Failed Tests 4 ⎯⎯⎯⎯⎯⎯⎯ | Test Files  1 failed (1) | Tests  4 failed | 1 passed (5)
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
pnpm exec prettier --write web/src/lib/controller/stack-workspace/controller.svelte.ts web/src/lib/controller/stack-workspace/drag.ts web/src/lib/controller/stack-workspace/files.ts web/src/lib/controller/stack-workspace/presentation.ts web/src/lib/controller/stack-workspace/types.ts web/src/lib/ui/store-stack-workspace/stack-workspace.css web/src/lib/ui/store-stack-workspace/StoreStackBookmarkCard.svelte web/src/lib/ui/store-stack-workspace/StoreStackCommitActions.svelte web/src/lib/ui/store-stack-workspace/StoreStackLane.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkingCopy.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspace.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceCanvas.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceChoice.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceConflict.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceDiff.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceForm.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspacePreview.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceToolbar.svelte web/tests/vite/stack-workspace.test.ts spec/src/ui.md
```

**Notes**

Output: .local/stack-workspace-ui-evidence/diffs-format.log. Command completed; formatter output or installation completion retained.
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
pnpm exec prettier --write web/tests/vite/stack-workspace.test.ts catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte
```

**Notes**

Output: .local/stack-workspace-ui-evidence/diffs-story-format.log. Command completed; formatter output or installation completion retained.
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
pnpm exec prettier --write web/src/lib/ui/community-projects/CommunityProjectsRevisionGraph.svelte web/src/lib/ui/community-projects/CommunityProjectsRevisionGraphCopy.svelte web/src/lib/ui/store-stack-workspace/stack-workspace.css web/src/lib/ui/store-stack-workspace/StoreStackBookmarkCard.svelte web/src/lib/ui/store-stack-workspace/StoreStackCommitActions.svelte web/src/lib/ui/store-stack-workspace/StoreStackLane.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkingCopy.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspace.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceCanvas.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceChoice.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceConflict.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceDiff.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceForm.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspacePreview.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceToolbar.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte spec/src/ui.md
```

**Notes**

Output: .local/stack-workspace-ui-evidence/alignment-format.log. Command completed; formatter output or installation completion retained.
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
pnpm exec prettier --write web/src/lib/controller/stack-workspace/controller.svelte.ts web/src/lib/controller/stack-workspace/drag.ts web/src/lib/controller/stack-workspace/files.ts web/src/lib/controller/stack-workspace/presentation.ts web/src/lib/controller/stack-workspace/types.ts web/src/lib/ui/store-stack-workspace/stack-workspace.css web/src/lib/ui/store-stack-workspace/StoreStackBookmarkCard.svelte web/src/lib/ui/store-stack-workspace/StoreStackCommitActions.svelte web/src/lib/ui/store-stack-workspace/StoreStackLane.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkingCopy.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspace.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceCanvas.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceChoice.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceConflict.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceDiff.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceForm.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspacePreview.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceToolbar.svelte web/src/lib/ui/community-projects/CommunityProjectsRevisionGraph.svelte catalog/fixtures/StackWorkspaceApplicationFixture.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte spec/src/ui.md
```

**Notes**

Output: .local/stack-workspace-ui-evidence/header-files-format.log. Command completed; formatter output or installation completion retained.
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
pnpm exec prettier --write web/src/lib/ui/community-projects/CommunityProjectsRevisionGraph.svelte web/tests/vite/stack-workspace.test.ts docs/ui-provenance.json
```

**Notes**

Output: .local/stack-workspace-ui-evidence/continuity-format.log. Command completed; formatter output or installation completion retained.
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
pnpm exec prettier --write web/src/lib/ui/community-projects/CommunityProjectsRevisionGraph.svelte web/src/lib/ui/store-stack-workspace/StoreStackBookmarkCard.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte
```

**Notes**

Output: .local/stack-workspace-ui-evidence/continuity-fix-format.log. Command completed; formatter output or installation completion retained.
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
pnpm exec prettier --write web/src/lib/ui/community-projects/CommunityProjectsRevisionGraph.svelte web/src/lib/ui/community-projects/CommunityProjectsRevisionGraphSvg.svelte
```

**Notes**

Output: .local/stack-workspace-ui-evidence/graph-extraction-format.log. Command completed; formatter output or installation completion retained.
<!-- /run -->
<!-- /evidence-field -->

