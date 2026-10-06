---
schema: 1
id: E-M20-001
---

# E-M20-001

<!-- evidence-field:id markdown -->
## Id

E-M20-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M20
- M20.1
- M20.2
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-04
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change vmozymrt
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

frontend-controller-and-storybook
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ 0.45.1, Node 26.9.0, pnpm 10.32.1, Svelte 5.57.1, Storybook 10.5.7, Vitest 4.1.11; Design Core source xmmtwsry/3a2ba27c through the existing ignored development package hook; local 0.1.2 acceptance archive retained, registry release qualification separate
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Frontend snapshots, requests and outcomes only; no persisted format or HTTP/RPC/Rust contract change
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- .storybook/vite.config.ts
- catalog/components/store-stack-workspace/StoreStackWorkspace.stories.svelte
- catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte
- catalog/components/store-stack-workspace/StoreStackWorkspaceCommands.stories.svelte
- catalog/components/store-stack-workspace/StoreStackWorkspaceDrag.stories.svelte
- catalog/components/store-stack-workspace/StoreStackWorkspaceInline.stories.svelte
- catalog/components/store-stack-workspace/StoreStackWorkspaceMenus.stories.svelte
- catalog/components/store-stack-workspace/StoreStackWorkspacePresentation.stories.svelte
- catalog/components/store-stack-workspace/StoreStackWorkspaceRecovery.stories.svelte
- catalog/fixtures/StackWorkspaceApplicationFixture.svelte
- catalog/fixtures/StackWorkspaceFixture.svelte
- catalog/fixtures/stack-workspace/adapter.ts
- catalog/fixtures/stack-workspace/changes.ts
- catalog/fixtures/stack-workspace/data.ts
- catalog/fixtures/stack-workspace/menu-reducer.ts
- catalog/fixtures/stack-workspace/menu-validation.ts
- catalog/fixtures/stack-workspace/preview.ts
- catalog/fixtures/stack-workspace/reducer.ts
- catalog/fixtures/stack-workspace/story-actions.ts
- catalog/fixtures/stack-workspace/validation.ts
- docs/ui-provenance.json
- spec/src/api.md
- spec/src/milestones.md
- spec/src/spec-governance.md
- spec/src/ui.md
- spec/src/verification.md
- web/src/lib/controller/stack-workspace/commit-drafts.svelte.ts
- web/src/lib/controller/stack-workspace/controller.svelte.ts
- web/src/lib/controller/stack-workspace/drag-session.svelte.ts
- web/src/lib/controller/stack-workspace/drag.ts
- web/src/lib/controller/stack-workspace/file-drag.ts
- web/src/lib/controller/stack-workspace/files.ts
- web/src/lib/controller/stack-workspace/presentation.ts
- web/src/lib/controller/stack-workspace/types.ts
- web/src/lib/controller/store-route-codec.ts
- web/src/lib/ui/community-commit-diff/CommunityCommitDiff.svelte
- web/src/lib/ui/community-navigation/icons.ts
- web/src/lib/ui/community-navigation/model.ts
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryLogFiles.svelte
- web/src/lib/ui/community-projects/CommunityProjectsRevisionGraph.svelte
- web/src/lib/ui/community-projects/CommunityProjectsRevisionGraphCopy.svelte
- web/src/lib/ui/community-projects/CommunityProjectsRevisionGraphSvg.svelte
- web/src/lib/ui/community-repository-branch-command/CommunityRepositoryBranchCommand.svelte
- web/src/lib/ui/store-application/StoreApplication.svelte
- web/src/lib/ui/store-application/StoreApplicationMain.svelte
- web/src/lib/ui/store-application/StoreApplicationNavigation.svelte
- web/src/lib/ui/store-application/StoreRepositoryApplication.svelte
- web/src/lib/ui/store-application/StoreRepositoryPicker.svelte
- web/src/lib/ui/store-application/application-props.ts
- web/src/lib/ui/store-application/store-navigation.ts
- web/src/lib/ui/store-stack-workspace/StoreStackBookmarkActions.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackBookmarkCard.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackCommitActions.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackCommitComposer.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackCopyMenu.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackDiffHunk.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackDropGuide.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackLane.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackRelativeActions.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackStagedFiles.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackUnstaged.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackWorkspace.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceActions.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceCanvas.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceChoice.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceConflict.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceDiff.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceForm.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackWorkspacePicker.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackWorkspacePreview.svelte
- web/src/lib/ui/store-stack-workspace/stack-workspace.css
- web/tests/vite/stack-workspace-branches.test.ts
- web/tests/vite/stack-workspace-menus.test.ts
- web/tests/vite/stack-workspace.test.ts
- web/tests/vite/store-route-history.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/vite/store-route-history.test.ts#Stacks round-trips as a separate
  application route
- web/tests/vite/store-route-history.test.ts#workspace names survive browser
  Back, Forward, reload and repository switches
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M20-001 details 2](details-02.md)
- [E-M20-001 details 3](details-03.md)
- [E-M20-001 details 4](details-04.md)
- [E-M20-001 details 5](details-05.md)
- [E-M20-001 details 6](details-06.md)
- [E-M20-001 details 7](details-07.md)
- [E-M20-001 details 8](details-08.md)
- [E-M20-001 details 9](details-09.md)
- [E-M20-001 details 10](details-10.md)
- [E-M20-001 details 11](details-11.md)
- [E-M20-001 details 12](details-12.md)
- [E-M20-001 details 13](details-13.md)
- [E-M20-001 details 14](details-14.md)
- [E-M20-001 details 15](details-15.md)
