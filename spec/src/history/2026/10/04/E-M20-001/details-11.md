---
schema: 1
id: E-M20-001
---

# E-M20-001 details 11

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
pnpm exec prettier --write catalog/components/store-stack-workspace/StoreStackWorkspace.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceRecovery.stories.svelte spec/src/ui.md
```

**Notes**

Output: .local/stack-workspace-ui-evidence/composer-recovery-format.log. Command completed; formatter output or installation completion retained.
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
pnpm exec prettier --write web/src/lib/ui/community-projects/CommunityProjectsRepositoryLogFiles.svelte
```

**Notes**

Output: .local/stack-workspace-ui-evidence/composer-shared-format.log. Command completed; formatter output or installation completion retained.
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
pnpm exec prettier --write web/src/lib/ui/community-projects/CommunityProjectsRevisionGraph.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte spec/src/ui.md docs/ui-provenance.json
```

**Notes**

Output: .local/stack-workspace-ui-evidence/composer-observer-format.log. Command completed; formatter output or installation completion retained.
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
pnpm exec prettier --check .storybook/vite.config.ts catalog/components/store-stack-workspace/StoreStackWorkspace.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceCommands.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspacePresentation.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceRecovery.stories.svelte catalog/fixtures/StackWorkspaceApplicationFixture.svelte catalog/fixtures/StackWorkspaceFixture.svelte catalog/fixtures/stack-workspace/adapter.ts catalog/fixtures/stack-workspace/changes.ts catalog/fixtures/stack-workspace/data.ts catalog/fixtures/stack-workspace/preview.ts catalog/fixtures/stack-workspace/reducer.ts catalog/fixtures/stack-workspace/story-actions.ts catalog/fixtures/stack-workspace/validation.ts docs/ui-provenance.json spec/src/api.md spec/src/spec-governance.md spec/src/ui.md spec/src/verification.md web/src/lib/controller/stack-workspace/commit-drafts.svelte.ts web/src/lib/controller/stack-workspace/controller.svelte.ts web/src/lib/controller/stack-workspace/drag.ts web/src/lib/controller/stack-workspace/files.ts web/src/lib/controller/stack-workspace/presentation.ts web/src/lib/controller/stack-workspace/types.ts web/src/lib/controller/store-route-codec.ts web/src/lib/ui/community-commit-diff/CommunityCommitDiff.svelte web/src/lib/ui/community-navigation/icons.ts web/src/lib/ui/community-navigation/model.ts web/src/lib/ui/community-projects/CommunityProjectsRepositoryLogFiles.svelte web/src/lib/ui/community-projects/CommunityProjectsRevisionGraph.svelte web/src/lib/ui/community-projects/CommunityProjectsRevisionGraphCopy.svelte web/src/lib/ui/community-projects/CommunityProjectsRevisionGraphSvg.svelte web/src/lib/ui/community-repository-branch-command/CommunityRepositoryBranchCommand.svelte web/src/lib/ui/store-application/StoreApplication.svelte web/src/lib/ui/store-application/StoreApplicationMain.svelte web/src/lib/ui/store-application/StoreApplicationNavigation.svelte web/src/lib/ui/store-application/StoreRepositoryApplication.svelte web/src/lib/ui/store-application/StoreRepositoryPicker.svelte web/src/lib/ui/store-application/application-props.ts web/src/lib/ui/store-application/store-navigation.ts web/src/lib/ui/store-stack-workspace/StoreStackBookmarkActions.svelte web/src/lib/ui/store-stack-workspace/StoreStackBookmarkCard.svelte web/src/lib/ui/store-stack-workspace/StoreStackCommitActions.svelte web/src/lib/ui/store-stack-workspace/StoreStackCommitComposer.svelte web/src/lib/ui/store-stack-workspace/StoreStackDiffHunk.svelte web/src/lib/ui/store-stack-workspace/StoreStackLane.svelte web/src/lib/ui/store-stack-workspace/StoreStackUnstaged.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspace.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceActions.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceCanvas.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceChoice.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceConflict.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceDiff.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceForm.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspacePicker.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspacePreview.svelte web/src/lib/ui/store-stack-workspace/stack-workspace.css web/tests/vite/stack-workspace.test.ts web/tests/vite/store-route-history.test.ts
```

**Notes**

Output: .local/stack-workspace-ui-evidence/composer-format-final.log. Command completed; formatter output or installation completion retained.
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
pnpm exec prettier --write spec/src/verification.md
```

**Notes**

Output: .local/stack-workspace-ui-evidence/composer-verification-format.log. Command completed; formatter output or installation completion retained.
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
pnpm install --lockfile=false
```

**Notes**

Output: .local/stack-workspace-ui-evidence/composer-divider-refresh.log. Command completed; formatter output or installation completion retained.
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
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspace.stories.svelte -t NarrowLayout --reporter=verbose
```

**Notes**

Output: .local/stack-workspace-ui-evidence/composer-narrow-after.log. Test Files  1 passed (1) | Tests  1 passed | 13 skipped (14) Exact NarrowLayout rerun passed after the source layout fix.
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
pnpm exec prettier --check .storybook/vite.config.ts catalog/components/store-stack-workspace/StoreStackWorkspace.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceCommands.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspacePresentation.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceRecovery.stories.svelte catalog/fixtures/StackWorkspaceApplicationFixture.svelte catalog/fixtures/StackWorkspaceFixture.svelte catalog/fixtures/stack-workspace/adapter.ts catalog/fixtures/stack-workspace/changes.ts catalog/fixtures/stack-workspace/data.ts catalog/fixtures/stack-workspace/preview.ts catalog/fixtures/stack-workspace/reducer.ts catalog/fixtures/stack-workspace/story-actions.ts catalog/fixtures/stack-workspace/validation.ts docs/ui-provenance.json spec/src/api.md spec/src/spec-governance.md spec/src/ui.md spec/src/verification.md web/src/lib/controller/stack-workspace/commit-drafts.svelte.ts web/src/lib/controller/stack-workspace/controller.svelte.ts web/src/lib/controller/stack-workspace/drag.ts web/src/lib/controller/stack-workspace/files.ts web/src/lib/controller/stack-workspace/presentation.ts web/src/lib/controller/stack-workspace/types.ts web/src/lib/controller/store-route-codec.ts web/src/lib/ui/community-commit-diff/CommunityCommitDiff.svelte web/src/lib/ui/community-navigation/icons.ts web/src/lib/ui/community-navigation/model.ts web/src/lib/ui/community-projects/CommunityProjectsRepositoryLogFiles.svelte web/src/lib/ui/community-projects/CommunityProjectsRevisionGraph.svelte web/src/lib/ui/community-projects/CommunityProjectsRevisionGraphCopy.svelte web/src/lib/ui/community-projects/CommunityProjectsRevisionGraphSvg.svelte web/src/lib/ui/community-repository-branch-command/CommunityRepositoryBranchCommand.svelte web/src/lib/ui/store-application/StoreApplication.svelte web/src/lib/ui/store-application/StoreApplicationMain.svelte web/src/lib/ui/store-application/StoreApplicationNavigation.svelte web/src/lib/ui/store-application/StoreRepositoryApplication.svelte web/src/lib/ui/store-application/StoreRepositoryPicker.svelte web/src/lib/ui/store-application/application-props.ts web/src/lib/ui/store-application/store-navigation.ts web/src/lib/ui/store-stack-workspace/StoreStackBookmarkActions.svelte web/src/lib/ui/store-stack-workspace/StoreStackBookmarkCard.svelte web/src/lib/ui/store-stack-workspace/StoreStackCommitActions.svelte web/src/lib/ui/store-stack-workspace/StoreStackCommitComposer.svelte web/src/lib/ui/store-stack-workspace/StoreStackDiffHunk.svelte web/src/lib/ui/store-stack-workspace/StoreStackLane.svelte web/src/lib/ui/store-stack-workspace/StoreStackUnstaged.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspace.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceActions.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceCanvas.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceChoice.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceConflict.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceDiff.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceForm.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspacePicker.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspacePreview.svelte web/src/lib/ui/store-stack-workspace/stack-workspace.css web/tests/vite/stack-workspace.test.ts web/tests/vite/store-route-history.test.ts
```

**Notes**

Output: .local/stack-workspace-ui-evidence/composer-format-final-second.log. All matched files use Prettier code style!
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
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace catalog/components/community-projects/CommunityProjectsRevisionGraph.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLogFiles.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLogDiff.stories.svelte catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.stories.svelte catalog/components/store-application/StoreRepositoryPicker.stories.svelte catalog/components/store-application/StoreRepositoryNavigation.stories.svelte catalog/components/store-application/StoreRepositoryBranchContext.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte --reporter=verbose
```

**Notes**

Output: .local/stack-workspace-ui-evidence/composer-divider-consumers-final.log. Test Files  14 passed (14) | Tests  77 passed (77)
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
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspacePresentation.stories.svelte -t 'Diff header and wide action menus'
```

**Notes**

Output: .local/stack-workspace-ui-evidence/menu-diff-header-before.log. Test Files  1 skipped (1) | Tests  5 skipped (5) All five tests skipped because the filter used the display name; no acceptance claim. Exact transformed-name rerun reproduced the regression.
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
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspacePresentation.stories.svelte -t DiffHeader
```

**Notes**

Output: .local/stack-workspace-ui-evidence/menu-diff-header-before-exact.log. ⎯⎯⎯⎯⎯⎯⎯ Failed Tests 1 ⎯⎯⎯⎯⎯⎯⎯ | Test Files  1 failed (1) | Tests  1 failed | 4 skipped (5)
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspacePresentation.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte -t 'WholeStackHeaders|WorkspaceInAppShell'
```

**Notes**

Output: .local/stack-workspace-ui-evidence/stack-header-gap-before.log. ⎯⎯⎯⎯⎯⎯⎯ Failed Tests 2 ⎯⎯⎯⎯⎯⎯⎯ | Test Files  2 failed (2) | Tests  2 failed | 12 skipped (14)
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
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceCommands.stories.svelte -t InlineCommit
```

**Notes**

Output: .local/stack-workspace-ui-evidence/inline-composer-before.log. Test Files  1 failed (1) | Tests  no tests One test-file import failure; no tests executed. Exact retry then reproduced the actual title-height regression.
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
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceCommands.stories.svelte -t InlineCommit
```

**Notes**

Output: .local/stack-workspace-ui-evidence/inline-composer-before-exact.log. ⎯⎯⎯⎯⎯⎯⎯ Failed Tests 1 ⎯⎯⎯⎯⎯⎯⎯ | Test Files  1 failed (1) | Tests  1 failed | 5 skipped (6) One failing measured title-height regression: 36px exceeded the required 28px.
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
pnpm install --lockfile=false
```

**Notes**

Output: .local/stack-workspace-ui-evidence/drag-local-refresh.log. Command completed; formatter output or installation completion retained.
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
pnpm exec prettier --write .storybook/vite.config.ts catalog/components/store-stack-workspace/StoreStackWorkspace.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceCommands.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspacePresentation.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceRecovery.stories.svelte catalog/fixtures/StackWorkspaceApplicationFixture.svelte catalog/fixtures/StackWorkspaceFixture.svelte catalog/fixtures/stack-workspace/adapter.ts catalog/fixtures/stack-workspace/changes.ts catalog/fixtures/stack-workspace/data.ts catalog/fixtures/stack-workspace/preview.ts catalog/fixtures/stack-workspace/reducer.ts catalog/fixtures/stack-workspace/story-actions.ts catalog/fixtures/stack-workspace/validation.ts docs/ui-provenance.json spec/src/api.md spec/src/spec-governance.md spec/src/ui.md spec/src/verification.md web/src/lib/controller/stack-workspace/commit-drafts.svelte.ts web/src/lib/controller/stack-workspace/controller.svelte.ts web/src/lib/controller/stack-workspace/drag-session.svelte.ts web/src/lib/controller/stack-workspace/drag.ts web/src/lib/controller/stack-workspace/file-drag.ts web/src/lib/controller/stack-workspace/files.ts web/src/lib/controller/stack-workspace/presentation.ts web/src/lib/controller/stack-workspace/types.ts web/src/lib/controller/store-route-codec.ts web/src/lib/ui/community-commit-diff/CommunityCommitDiff.svelte web/src/lib/ui/community-navigation/icons.ts web/src/lib/ui/community-navigation/model.ts web/src/lib/ui/community-projects/CommunityProjectsRepositoryLogFiles.svelte web/src/lib/ui/community-projects/CommunityProjectsRevisionGraph.svelte web/src/lib/ui/community-projects/CommunityProjectsRevisionGraphCopy.svelte web/src/lib/ui/community-projects/CommunityProjectsRevisionGraphSvg.svelte web/src/lib/ui/community-repository-branch-command/CommunityRepositoryBranchCommand.svelte web/src/lib/ui/store-application/StoreApplication.svelte web/src/lib/ui/store-application/StoreApplicationMain.svelte web/src/lib/ui/store-application/StoreApplicationNavigation.svelte web/src/lib/ui/store-application/StoreRepositoryApplication.svelte web/src/lib/ui/store-application/StoreRepositoryPicker.svelte web/src/lib/ui/store-application/application-props.ts web/src/lib/ui/store-application/store-navigation.ts web/src/lib/ui/store-stack-workspace/StoreStackBookmarkActions.svelte web/src/lib/ui/store-stack-workspace/StoreStackBookmarkCard.svelte web/src/lib/ui/store-stack-workspace/StoreStackCommitActions.svelte web/src/lib/ui/store-stack-workspace/StoreStackCommitComposer.svelte web/src/lib/ui/store-stack-workspace/StoreStackDiffHunk.svelte web/src/lib/ui/store-stack-workspace/StoreStackDropGuide.svelte web/src/lib/ui/store-stack-workspace/StoreStackLane.svelte web/src/lib/ui/store-stack-workspace/StoreStackUnstaged.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspace.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceActions.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceCanvas.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceChoice.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceConflict.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceDiff.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceForm.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspacePicker.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspacePreview.svelte web/src/lib/ui/store-stack-workspace/stack-workspace.css web/tests/vite/stack-workspace.test.ts web/tests/vite/store-route-history.test.ts
```

**Notes**

Output: .local/stack-workspace-ui-evidence/drag-format.log. Command completed; formatter output or installation completion retained.
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
svelte-check --tsconfig web/tsconfig.json
```

**Notes**

Output: .local/stack-workspace-ui-evidence/drag-web-check.log. svelte-check found 0 errors and 0 warnings
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
vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/stack-workspace.test.ts web/tests/vite/store-route-history.test.ts
```

**Notes**

Output: .local/stack-workspace-ui-evidence/drag-unit-first.log. Test Files  2 passed (2) | Tests  43 passed (43)
<!-- /run -->
<!-- /evidence-field -->

