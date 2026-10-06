---
schema: 1
id: E-UI-014
---

# E-UI-014

<!-- evidence-field:id markdown -->
## Id

E-UI-014
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4
- M4.3
- M4.13
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-28
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

implementation
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/community-projects/repository-browser.test.ts
- catalog/controller/in-memory-store-controller.svelte.ts
- spec/src/api.md
- spec/src/milestones.md
- spec/src/ui.md
- web/src/lib/controller/contracts.ts
- web/src/lib/controller/http-store-data-source.ts
- web/src/lib/controller/native-data-source.ts
- web/src/lib/controller/native-presentation.ts
- web/src/lib/controller/production-file-pages.ts
- web/src/lib/controller/production-file-reader.svelte.ts
- web/src/lib/controller/production-route-restorer.ts
- web/src/lib/controller/production-store-controller.svelte.ts
- web/src/lib/controller/production-store-loaders.ts
- web/src/lib/controller/production-store-reader.svelte.ts
- web/src/lib/controller/production-store-state.svelte.ts
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryFileTree.svelte
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryFiles.svelte
- web/src/lib/ui/community-projects/repository-browser.ts
- web/tests/browser/native-browser-fixture.ts
- web/tests/browser/reads.spec.ts
- web/tests/production-store-loaders.test.ts
- web/tests/vite/production-store-data-source.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/vite/production-store-data-source.test.ts#path reads preserve
  directory, recursive, and opaque cursor queries
- web/tests/production-store-loaders.test.ts#native presentation preserves
  conflicting targets and exact identifiers
- web/tests/production-store-loaders.test.ts#path pages isolate lazy directories
  from recursive listing views
- web/tests/browser/reads.spec.ts#repository directories load lazily and flat
  views request recursive paths
- web/tests/browser/reads.spec.ts#graph_and_directory_pages_retain_exact_snapshots_and_query_context
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-UI-014 details 2](details-02.md)
