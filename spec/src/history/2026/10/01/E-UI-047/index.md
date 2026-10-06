---
schema: 1
id: E-UI-047
---

# E-UI-047

<!-- evidence-field:id markdown -->
## Id

E-UI-047
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M11.4
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-01
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

community-projects-repository-scope-markdown-and-activity-filters
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

Enclosing Jujutsu change lvpkolvw.
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Main change follows verified soynnsrx / 74186666. Repository Projects treated as the hub, custom Markdown left backticks literal, project Activity followed Tasks and no native context filter existed. Exact document regression fails once and repository route/browse regressions fail twice. Settings workspace excluded.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/community-projects/RepositoryDocuments.stories.svelte
- catalog/components/store-application/StoreRepositoryApplication.stories.svelte
- catalog/components/store-work/ProjectsActivityControls.stories.svelte
- catalog/fixtures/ProjectsHubFixture.svelte
- spec/src/api.md
- spec/src/milestones.md
- spec/src/ui.md
- spec/src/verification.md
- web/src/lib/controller/contracts.ts
- web/src/lib/controller/native-activity-filter.ts
- web/src/lib/controller/production-store-connect.ts
- web/src/lib/controller/production-store-controller.svelte.ts
- web/src/lib/controller/production-store-lifecycle.svelte.ts
- web/src/lib/controller/projects-hub-types.ts
- web/src/lib/controller/projects-route-codec.ts
- web/src/lib/controller/projects-view-cache.ts
- web/src/lib/controller/store-route-history.svelte.ts
- web/src/lib/ui/community-projects/CommunityProjectsMarkdownPreview.svelte
- web/src/lib/ui/community-projects/repository-documents.css
- web/src/lib/ui/store-application/StoreApplication.svelte
- web/src/lib/ui/store-application/StoreRepositoryApplication.svelte
- web/src/lib/ui/store-work/StoreActivityFilter.svelte
- web/src/lib/ui/store-work/StoreProjects.svelte
- web/src/lib/ui/store-work/StoreProjectsActivity.svelte
- web/src/lib/ui/store-work/StoreProjectsFilters.svelte
- web/src/lib/ui/store-work/StoreRepositoryProjects.svelte
- web/tests/real/project-polish.spec.ts
- web/tests/store-route-codec.test.ts
- web/tests/vite/native-activity-filter.test.ts
- web/tests/vite/projects-code-navigation.test.ts
- web/tests/vite/projects-hub.test.ts
- web/tests/vite/store-route-history.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/vite/projects-hub.test.ts#repository Projects browses linked
  projects across homes and keeps a separate cache
- web/tests/vite/projects-hub.test.ts#missing project never broadens Activity to
  unrelated repositories
- web/tests/vite/store-route-history.test.ts#repository Projects reload retains
  its repository destination and code pin
- web/tests/vite/projects-code-navigation.test.ts#repository Projects external
  routes authorize their separate code connection
- web/tests/vite/native-activity-filter.test.ts#all activity keeps discovery
  lazy and agent filters never load session bodies
- web/tests/vite/native-activity-filter.test.ts#association classification
  bounds readers and excludes unknown facts from both categories
- web/tests/vite/native-activity-filter.test.ts#cancelled activity
  classification stops queued reads and discards delayed results
- web/tests/store-route-codec.test.ts#activity filters round trip and reject
  invalid or repeated classifications
- web/tests/real/project-polish.spec.ts#repository documents linked projects and
  Activity filters retain native scope
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-UI-047 details 2](details-02.md)
- [E-UI-047 details 3](details-03.md)
