---
schema: 1
id: E-COMPUTE-004
---

# E-COMPUTE-004

<!-- evidence-field:id markdown -->
## Id

E-COMPUTE-004
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-08
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

oklpnryt
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M25
- M24
- M4.8
- M4.19
- M15.5
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-002
- JST-UI-004
- JST-UI-005
- JST-UI-012
- JST-UI-013
- JST-API-001
- JST-DOC-002
- JST-GOV-008
- JST-UI-017
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/AgentFleet.stories.svelte
- catalog/components/store-application/ComputeModelPicker.stories.svelte
- catalog/components/store-application/LocalCompute.stories.svelte
- catalog/components/store-application/ManagedAgentIdentity.stories.svelte
- catalog/components/store-application/RepositoryLoading.stories.svelte
- catalog/components/store-application/RepositoryReview.stories.svelte
- catalog/components/store-application/StoreFileEditor.stories.svelte
- catalog/components/store-application/StoreRepositoryApplication.stories.svelte
- catalog/components/store-application/repository-directory-story-assertions.ts
- catalog/components/store-application/repository-layout-story-assertions.ts
- catalog/components/store-work/ProjectsAgentActions.stories.svelte
- catalog/fixtures/LocalComputeFixture.svelte
- catalog/fixtures/ProjectsAgentActionsFixture.svelte
- catalog/fixtures/ProjectsHubFixture.svelte
- docs/maintenance/coverage.json
- docs/site/guides/agent-execution.md
- docs/ui-provenance.json
- spec/src/current/contracts/api/42-compute-and-agent-settings.md
- spec/src/current/contracts/documentation/08-interactive-agent-execution.md
- spec/src/current/contracts/ui/13-shared-application.md
- spec/src/current/contracts/ui/15-workspace-browsing-context.md
- spec/src/current/contracts/ui/18-installation-projects-hub.md
- spec/src/current/contracts/ui/34-interactive-agent-sessions.md
- spec/src/current/contracts/ui/35-compute-defaults-and-sent-context.md
- spec/src/current/governance/spec-governance.md
- spec/src/current/progress/M15/history/M15.5/index.md
- spec/src/current/progress/M15/index.md
- spec/src/current/progress/M24/history/M24/index.md
- spec/src/current/progress/M24/index.md
- spec/src/current/progress/M25/history/M25/index.md
- spec/src/current/progress/M25/index.md
- spec/src/current/progress/M4/history/M4.19/index.md
- spec/src/current/progress/M4/history/M4.8/index.md
- spec/src/current/progress/M4/index.md
- spec/src/history/2026/10/08/SUMMARY.nav.md
- spec/src/history/2026/10/08/index.md
- web/src/lib/ui/community-app-shell/right-sidebar.css
- web/src/lib/ui/layout-storage.ts
- web/src/lib/ui/store-agent-chat/StoreAgentChatSidebar.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentExecutionDirectory.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentHeaderActions.svelte
- web/src/lib/ui/store-application/StoreWorkspaceSectionHeader.svelte
- web/src/lib/ui/store-application/store-application.css
- web/src/lib/ui/store-settings/StoreComputeModelMetadata.svelte
- web/src/lib/ui/store-settings/StoreComputeModelPicker.svelte
- web/src/lib/ui/store-settings/StoreComputeModels.svelte
- web/src/lib/ui/store-settings/compute-model-catalog.ts
- web/src/lib/ui/store-work/StoreProjects.svelte
- web/src/lib/ui/store-work/StoreProjectsActions.svelte
- web/src/lib/ui/store-work/StoreWorkTaskDetail.svelte
- web/tests/compute-model-catalog.test.ts
- web/tests/layout-storage.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

UI-only follow-up in the local-compute JJ workspace. Workspace icon matches Projects; new Files layout starts collapsed while valid saved preferences restore; Files body has zero padding, 1px top divider and 8px top margin. Repository agent actions move into the Workspace title row with icon-only shadcn tooltips. Chat header has no bottom border. Compute uses a searchable Command View with owned curated/advanced GGUF references, estimated weight size and RAM fit, comfortable recommendations, installed status and exact custom/imported IDs. Selection only updates drafts. Before state: native Model select exposed two built-in choices and current/installed identities without size/fit; body agent text buttons; Files default open. No Rust/API schema changes, downloads, running model mutation or backend restart. Browser surface checks cover all eight direct consumer families, not unrelated product suites. Follow-up also applies the shared icon launcher to every Projects hub/project tab and task header, moves Manage agents into the Agents title row and removes duplicate action rendering from both directory callers. Task gets its manager explicitly from the owning Projects controller; existing repository/task/project context is retained. The independently reproduced parent Op Log case is outside this selected slice, with its failure preserved in this record.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-COMPUTE-004 details 2](details-02.md)
- [E-COMPUTE-004 details 3](details-03.md)
- [E-COMPUTE-004 details 4](details-04.md)
