---
schema: 1
id: E-UI-VIEW-READINESS-001
---

# E-UI-VIEW-READINESS-001

<!-- evidence-field:id markdown -->
## Id

E-UI-VIEW-READINESS-001
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-10
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M24
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-005
- JST-UI-010
- JST-UI-015
- JST-UI-017
- JST-UI-018
- JST-API-001
- JST-DOC-003
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-appearance/StoreAppearance.stories.svelte
- catalog/components/store-application/AgentLayout.stories.svelte
- catalog/components/store-application/AgentSessionSettings.stories.svelte
- catalog/components/store-application/AgentToolGroups.stories.svelte
- catalog/components/store-application/AgentTranscriptSettings.stories.svelte
- catalog/components/store-application/ConversationCaret.stories.svelte
- catalog/components/store-application/LocalCompute.stories.svelte
- catalog/components/store-application/NativeAgentMenus.stories.svelte
- catalog/components/store-application/NativeReviewDetail.stories.svelte
- catalog/components/store-application/RepositoryApplication.stories.svelte
- catalog/components/store-application/RepositoryLayout.stories.svelte
- catalog/components/store-application/RepositoryLoading.stories.svelte
- catalog/components/store-application/RepositoryNotices.stories.svelte
- catalog/components/store-application/StoreApplication.stories.svelte
- catalog/components/store-application/StoreCollaboration.stories.svelte
- catalog/components/store-application/StoreDirectoryNavigation.stories.svelte
- catalog/components/store-application/StoreEvolutionColumn.stories.svelte
- catalog/components/store-application/StoreFileEditor.stories.svelte
- catalog/components/store-application/StoreRepositoryApplication.stories.svelte
- catalog/components/store-application/StoreRepositoryBranchContext.stories.svelte
- catalog/components/store-application/StoreRepositoryFeedback.stories.svelte
- catalog/components/store-application/StoreRepositoryNavigation.stories.svelte
- catalog/components/store-application/StoreRepositorySearch.stories.svelte
- catalog/components/store-application/StoreWorkspaces.stories.svelte
- catalog/components/store-application/WorkspaceFiles.stories.svelte
- catalog/components/store-application/agent-sidebar-play.ts
- catalog/components/store-application/appearance-tool-discussion-play.ts
- catalog/components/store-application/capture-harness-play.ts
- catalog/components/store-application/repository-layout-story-assertions.ts
- catalog/components/store-application/settings-shell-play.ts
- catalog/components/store-stack-workspace/StoreStackWorkspaceLoading.stories.svelte
- catalog/components/store-stack-workspace/StoreStackWorkspaceSettings.stories.svelte
- catalog/components/store-threads/StoreThreadAlignment.stories.svelte
- catalog/components/store-threads/StoreThreads.stories.svelte
- catalog/components/store-work/ProjectsActivity.stories.svelte
- catalog/components/store-work/ProjectsAgentActions.stories.svelte
- catalog/components/store-work/ProjectsFilters.stories.svelte
- catalog/components/store-work/ProjectsHub.stories.svelte
- catalog/components/store-work/ProjectsReviewDetail.stories.svelte
- catalog/fixtures/AgentToolGroupsFixture.svelte
- docs/maintenance/coverage.json
- docs/site/guides/browsing.md
- docs/ui-provenance.json
- spec/src/current/contracts/api/16-browsing-ui-execution.md
- spec/src/current/contracts/documentation/03-delivery-and-validation.md
- spec/src/current/contracts/ui/07-application-startup.md
- spec/src/current/contracts/ui/37-document-conversations.md
- spec/src/current/contracts/ui/39-intentional-thread-inbox.md
- spec/src/current/governance/spec-governance.md
- spec/src/current/progress/M24/history/M24/index.md
- spec/src/current/progress/M24/index.md
- spec/src/current/progress/M24/view-loading-review.md
- spec/src/history/2026/10/10/E-UI-VIEW-READINESS-001/SUMMARY.nav.md
- spec/src/history/2026/10/10/E-UI-VIEW-READINESS-001/details-02.md
- spec/src/history/2026/10/10/E-UI-VIEW-READINESS-001/details-03.md
- spec/src/history/2026/10/10/E-UI-VIEW-READINESS-001/details-04.md
- spec/src/history/2026/10/10/E-UI-VIEW-READINESS-001/index.md
- spec/src/history/2026/10/10/SUMMARY.nav.md
- spec/src/history/2026/10/10/index.md
- web/src/lib/ui/StoreToolbarSearch.svelte
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryFileTree.svelte
- web/src/lib/ui/community-projects/repository-file-tree.css
- web/src/lib/ui/community-projects/repository-files.css
- web/tests/browser/appearance.spec.ts
- web/tests/browser/repository-navigation.spec.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/browser/repository-navigation.spec.ts#right rail destinations match
  repository tabs and preserve workspace snapshot
- web/tests/browser/repository-navigation.spec.ts#phone overflow supports
  keyboard selection, reload, resize and right rail navigation
- web/tests/browser/repository-navigation.spec.ts#request errors use
  notifications while the right rail stays minimal
- web/tests/browser/appearance.spec.ts#cold device appearance restores under the
  production CSP and propagates across tabs
- web/tests/browser/lazy-views.spec.ts#a failed view chunk offers explicit retry
  without reloading or replaying commands
- web/tests/browser/lazy-views.spec.ts#cold built shell loads selected views on
  demand and reuses completed imports
- web/tests/browser/lazy-views.spec.ts#navigation away from a delayed view
  cannot render its obsolete completion
- scripts/view-loading-assets.test.mjs#built browser defers page views and
  admits their complete dynamic asset closure
- scripts/web-manifest.test.mjs#browser manifest retains dynamic chunks and
  fonts while excluding unreferenced workers
- scripts/web-manifest.test.mjs#browser manifest rejects missing dependencies
  and unsafe asset paths
- scripts/web-manifest.test.mjs#browser manifest includes Vite's unsplit
  application stylesheet
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-UI-VIEW-READINESS-001 details 2](details-02.md)
- [E-UI-VIEW-READINESS-001 details 3](details-03.md)
- [E-UI-VIEW-READINESS-001 details 4](details-04.md)
