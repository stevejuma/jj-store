---
schema: 1
id: E-UI-032
---

# E-UI-032

<!-- evidence-field:id markdown -->
## Id

E-UI-032
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4.19
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-30
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

repository-workspace-picker-and-pinned-route-context
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

Enclosing Jujutsu change oxqznotm.
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

The repository body had no workspace picker and owned hashes could not retain a native workspace name. The exact new workspace round-trip regression first produced four passing route tests and one failure. A same-commit working-copy callback could select the first workspace rather than the requested name.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.stories.svelte
- catalog/components/store-application/StoreApplication.stories.svelte
- catalog/components/store-application/StoreWorkspaces.stories.svelte
- catalog/controller/in-memory-store-branch-actions.ts
- catalog/controller/in-memory-store-controller.svelte.ts
- catalog/controller/in-memory-store-navigation.ts
- catalog/controller/in-memory-store-options.ts
- catalog/controller/in-memory-store-route.ts
- catalog/fixtures/StoreRepositoryApplicationFixture.svelte
- spec/src/api.md
- spec/src/milestones.md
- spec/src/ui.md
- spec/src/verification.md
- web/src/lib/controller/contracts.ts
- web/src/lib/controller/production-branch-actions.ts
- web/src/lib/controller/production-route-restorer.ts
- web/src/lib/controller/production-store-actions.ts
- web/src/lib/controller/production-store-loaders.ts
- web/src/lib/controller/production-store-reader.svelte.ts
- web/src/lib/controller/store-route-codec.ts
- web/src/lib/controller/store-route-history.svelte.ts
- web/src/lib/controller/store-workspaces.ts
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryFiles.svelte
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryLog.svelte
- web/src/lib/ui/community-projects/workspace-snapshots.ts
- web/src/lib/ui/community-repository-branch-command/CommunityRepositoryBranchCommand.svelte
- web/src/lib/ui/community-repository-branch-command/CommunityWorkingCopyCard.svelte
- web/src/lib/ui/store-application/StoreApplication.svelte
- web/src/lib/ui/store-application/StoreRepositoryApplication.svelte
- web/src/lib/ui/store-application/StoreWorkspacePicker.svelte
- web/src/lib/ui/store-application/store-application.css
- web/src/lib/ui/store-application/workspace-editing.svelte.ts
- web/src/lib/ui/store-application/workspace-navigation.ts
- web/tests/browser/workspaces.spec.ts
- web/tests/store-route-codec.test.ts
- web/tests/vite/store-route-history.test.ts
- web/tests/vite/store-workspaces.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/store-route-codec.test.ts#workspace names round-trip independently
  of shared commit identities
- web/tests/vite/store-workspaces.test.ts#workspace context resolves the working
  copy and survives tab, history and reload
- web/tests/vite/store-workspaces.test.ts#a missing workspace fails without
  silently loading a different working copy
- web/tests/vite/store-workspaces.test.ts#workspace context permits independent
  explicit revision inspection
- web/tests/vite/store-workspaces.test.ts#working-copy cards distinguish two
  workspace names sharing the same commit
- web/tests/vite/store-workspaces.test.ts#workspace discovery reads one pinned
  window and rejects a mismatched operation
- web/tests/vite/store-workspaces.test.ts#explicit Refresh follows the
  workspace's new working copy at the captured operation
- web/tests/vite/store-workspaces.test.ts#workspace discovery continues the
  displayed first page without recapturing its snapshot
- web/tests/browser/workspaces.spec.ts#workspace names sharing a commit survive
  cold reload, tabs, history and explicit Refresh
- web/tests/browser/workspaces.spec.ts#missing workspace stays explicit and can
  recover to repository overview
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-UI-032 details 2](details-02.md)
- [E-UI-032 details 3](details-03.md)
