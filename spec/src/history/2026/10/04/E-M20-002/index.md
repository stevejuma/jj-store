---
schema: 1
id: E-M20-002
---

# E-M20-002

<!-- evidence-field:id markdown -->
## Id

E-M20-002
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

enclosing Jujutsu change mrymlyux
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

frontend-controller-and-storybook-refinements
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ 0.45.1, Node 26.9.0, pnpm 10.32.1, Svelte 5.57.1, Storybook 10.5.7, Vitest 4.1.11; existing ignored Design Core source validation setup at xmmtwsry/3a2ba27c plus zlyuvqyv/d7ebf3e1; registry release qualification remains separate
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Frontend mock commands only: optional exact destination branch and exact above/below sorting target. No HTTP/RPC/Rust schema or persisted format change.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.stories.svelte
- catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte
- catalog/components/store-stack-workspace/StoreStackWorkspaceBranches.stories.svelte
- catalog/components/store-stack-workspace/StoreStackWorkspaceCommands.stories.svelte
- catalog/components/store-stack-workspace/StoreStackWorkspaceFooter.stories.svelte
- catalog/components/store-stack-workspace/StoreStackWorkspaceForms.stories.svelte
- catalog/components/store-stack-workspace/StoreStackWorkspaceHeaderDrops.stories.svelte
- catalog/components/store-stack-workspace/StoreStackWorkspaceInline.stories.svelte
- catalog/components/store-stack-workspace/StoreStackWorkspaceMenus.stories.svelte
- catalog/components/store-stack-workspace/StoreStackWorkspacePresentation.stories.svelte
- catalog/components/store-stack-workspace/StoreStackWorkspaceRefinements.stories.svelte
- catalog/fixtures/stack-workspace/preview.ts
- catalog/fixtures/stack-workspace/reducer.ts
- catalog/fixtures/stack-workspace/validation.ts
- docs/ui-provenance.json
- spec/src/api.md
- spec/src/milestones.md
- spec/src/spec-governance.md
- spec/src/ui.md
- spec/src/verification.md
- web/src/lib/controller/stack-workspace/commit-placement.ts
- web/src/lib/controller/stack-workspace/drag-session.svelte.ts
- web/src/lib/controller/stack-workspace/drag.ts
- web/src/lib/controller/stack-workspace/types.ts
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryLogFiles.svelte
- web/src/lib/ui/community-projects/CommunityProjectsRevisionGraph.svelte
- web/src/lib/ui/community-repository-branch-command/CommunityRepositoryBranchCommand.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackBookmarkActions.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackBookmarkCard.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackCommitActions.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackCommitComposer.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackLane.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackStagedFiles.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackWorkspace.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceCanvas.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceForm.svelte
- web/src/lib/ui/store-stack-workspace/stack-workspace-drag.css
- web/src/lib/ui/store-stack-workspace/stack-workspace-layout.css
- web/src/lib/ui/store-stack-workspace/stack-workspace.css
- web/tests/vite/stack-workspace-header-drops.test.ts
- web/tests/vite/stack-workspace-sorting.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/vite/stack-workspace-header-drops.test.ts#commit header drops retain
  the exact branch and isolate feedback per header
- web/tests/vite/stack-workspace-header-drops.test.ts#moving into a dependent
  header preserves upper commits and supports undo
- web/tests/vite/stack-workspace-header-drops.test.ts#commits move between
  branch segments but reject their current segment
- web/tests/vite/stack-workspace-header-drops.test.ts#stale and immutable branch
  header targets reject drops and previews
- web/tests/vite/stack-workspace-header-drops.test.ts#header moves can append
  above immutable ancestors without rewriting them
- web/tests/vite/stack-workspace-sorting.test.ts#commit row edge drops preserve
  above and below placement instead of squashing
- web/tests/vite/stack-workspace-sorting.test.ts#exact row sorting retains
  identities and rejects immutable rewrites and redundant positions
- web/tests/vite/stack-workspace-sorting.test.ts#sorting thresholds use rendered
  quarters and leave file drags in the center
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M20-002 details 2](details-02.md)
- [E-M20-002 details 3](details-03.md)
- [E-M20-002 details 4](details-04.md)
- [E-M20-002 details 5](details-05.md)
- [E-M20-002 details 6](details-06.md)
- [E-M20-002 details 7](details-07.md)
- [E-M20-002 details 8](details-08.md)
- [E-M20-002 details 9](details-09.md)
