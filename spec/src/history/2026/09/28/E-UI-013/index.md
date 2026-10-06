---
schema: 1
id: E-UI-013
---

# E-UI-013

<!-- evidence-field:id markdown -->
## Id

E-UI-013
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

- README.md
- catalog/components/store-application/RepositoryLoading.stories.svelte
- catalog/components/store-application/RepositoryReview.stories.svelte
- catalog/components/store-application/StoreRepositoryApplication.stories.svelte
- catalog/components/store-application/column-canvas-story-assertions.ts
- catalog/controller/in-memory-store-controller.svelte.ts
- catalog/controller/loading-store-controller.ts
- catalog/fixtures/LayoutPersistenceFixture.svelte
- catalog/fixtures/RepositoryFixture.svelte
- catalog/fixtures/StoreApplicationFixture.svelte
- catalog/fixtures/StoreRepositoryApplicationFixture.svelte
- catalog/fixtures/large-repository.ts
- catalog/fixtures/memory-storage.ts
- docs/browser.md
- docs/catalog.md
- docs/implementation-plan.md
- package.json
- scripts/catalog-browser.test.mjs
- scripts/dev-demo-services.mjs
- scripts/dev-demo.mjs
- spec/src/architecture.md
- spec/src/api.md
- spec/src/milestones.md
- spec/src/spec-governance.md
- spec/src/ui.md
- spec/src/verification.md
- web/src/lib/projects/repository-log-canvas.ts
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryBranchSelector.svelte
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryDirectory.svelte
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryDirectoryRows.svelte
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryFileTree.svelte
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryFiles.svelte
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryLog.svelte
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryLogDiff.svelte
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryLogFiles.svelte
- web/src/lib/ui/community-projects/CommunityProjectsRepositorySkeleton.svelte
- web/src/lib/ui/community-projects/CommunityProjectsRevisionGraph.svelte
- web/src/lib/ui/community-projects/CommunityProjectsVirtualRows.svelte
- web/src/lib/ui/community-projects/repository-directory.css
- web/src/lib/ui/community-projects/repository-file-tree-resize.css
- web/src/lib/ui/community-projects/repository-files.css
- web/src/lib/ui/community-projects/repository-log-diff.css
- web/src/lib/ui/community-projects/repository-skeleton.css
- web/src/lib/ui/community-repository-branch-command/CommunityRepositoryBranchCommand.svelte
- web/src/lib/ui/layout-storage.ts
- web/src/lib/ui/store-application/StoreApplication.svelte
- web/src/lib/ui/store-application/StoreRepositoryApplication.svelte
- web/src/lib/ui/store-application/repository-breadcrumb.css
- web/src/lib/ui/store-application/store-application.css
- web/tests/layout-storage.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- scripts/catalog-browser.test.mjs#cold_catalog_contract
- web/tests/browser/reads.spec.ts#native_graphs_files_and_source_rendering_are_accessible
- web/tests/browser/reads.spec.ts#graph_and_directory_pages_retain_exact_snapshots_and_query_context
- web/tests/layout-storage.test.ts#layout keys are versioned and isolate shell,
  files and every canvas page
- web/tests/layout-storage.test.ts#file-tree layout restores only a valid
  versioned width and open state
- web/tests/layout-storage.test.ts#shell layout is available synchronously
  without trusting malformed panels
- web/tests/layout-storage.test.ts#storage failures fall back without leaking
  layout errors into rendering
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-UI-013 details 2](details-02.md)
- [E-UI-013 details 3](details-03.md)
