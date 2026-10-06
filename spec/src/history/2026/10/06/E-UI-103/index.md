---
schema: 1
id: E-UI-103
---

# E-UI-103

<!-- evidence-field:id markdown -->
## Id

E-UI-103
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M11.4
- M4.31
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-015
- JST-UI-019
- JST-API-005
- JST-WORK-008
- JST-DOC-002
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-06
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change oqtvrqzv
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Projects tab read-error toast placement and hidden recovery
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

The exact new feedback family reproduced seven inline-error failures and one passing repository-metadata case before implementation. Reviews displayed the original unavailable error and Retry reviews inside its collection. Activity, Agents, scope reads, Projects and Tasks used equivalent inline blocks.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-work/ProjectsFeedback.stories.svelte
- catalog/components/store-work/ProjectsHub.stories.svelte
- catalog/fixtures/ProjectsFeedbackFixture.svelte
- docs/maintenance/coverage.json
- spec/src/current/contracts/api/21-workstore-service-and-transports.md
- spec/src/current/contracts/documentation/03-delivery-and-validation.md
- spec/src/current/contracts/ui/06-application-notifications.md
- spec/src/current/contracts/ui/18-installation-projects-hub.md
- web/src/lib/notifications/notification-recovery.svelte.ts
- web/src/lib/notifications/project-notification-recovery.svelte.ts
- web/src/lib/ui/store-work/StoreProjects.svelte
- web/src/lib/ui/store-work/StoreProjectsActivity.svelte
- web/src/lib/ui/store-work/StoreProjectsAgentDirectory.svelte
- web/src/lib/ui/store-work/StoreProjectsDirectory.svelte
- web/src/lib/ui/store-work/StoreProjectsReviews.svelte
- spec/src/current/progress/M11/history/M11.4/index.md
- spec/src/current/progress/M11/index.md
- spec/src/current/progress/M4/history/M4.31/index.md
- spec/src/current/progress/M4/index.md
- spec/src/history/2026/10/06/SUMMARY.nav.md
- spec/src/history/2026/10/06/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/vite/projects-directories.test.ts#missing review capability never
  reads transport while supported failures remain retryable
- web/tests/vite/projects-agents.test.ts#Denied reads and rediscovery clear
  retained agent sessions
- web/tests/vite/projects-freshness.test.ts#hidden Agents projections refresh
  only their dirty repository on return
- web/tests/vite/projects-freshness.test.ts#failed and superseded domain reads
  retain dirtiness independently
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-UI-103 details 2](details-02.md)
- [E-UI-103 details 3](details-03.md)
