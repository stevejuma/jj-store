---
schema: 1
id: E-UI-011
---

# E-UI-011

<!-- evidence-field:id markdown -->
## Id

E-UI-011
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4
- M4.3
- M4.11
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-27
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

implementation
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/RepositoryReview.stories.svelte
- catalog/components/store-application/StoreApplication.stories.svelte
- catalog/components/store-application/StoreConnection.stories.svelte
- catalog/components/store-application/StoreRepositoryApplication.stories.svelte
- catalog/components/store-application/column-canvas-story-assertions.ts
- catalog/controller/in-memory-store-controller.svelte.ts
- catalog/vitest.config.ts
- crates/jj-store-http/src/assets.rs
- crates/jj-store-http/tests/assets.rs
- package.json
- pnpm-lock.yaml
- scripts/web-manifest.mjs
- spec/src/milestones.md
- spec/src/ui.md
- spec/src/verification.md
- web/src/App.svelte
- web/src/lib/controller/contracts.ts
- web/src/lib/controller/http-store-data-source.ts
- web/src/lib/controller/native-data-source.ts
- web/src/lib/controller/native-presentation.ts
- web/src/lib/controller/production-store-controller.svelte.ts
- web/src/lib/controller/production-store-loaders.ts
- web/src/lib/controller/production-store-reader.svelte.ts
- web/src/lib/controller/production-store-state.svelte.ts
- web/src/lib/controller/store-query-client.ts
- web/src/lib/controller/store-query-keys.ts
- web/src/lib/controller/store-route-history.svelte.ts
- web/src/lib/ui/community-app-shell/CommunityAppShell.svelte
- web/src/lib/ui/community-profile-menu/CommunityProfileMenu.svelte
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryDirectory.svelte
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryFiles.svelte
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryLog.svelte
- web/src/lib/ui/store-application/StoreApplication.svelte
- web/src/lib/ui/store-application/StoreConnection.svelte
- web/src/lib/ui/store-application/StoreRepositoryApplication.svelte
- web/src/lib/ui/store-application/StoreRepositoryDetails.svelte
- web/src/lib/ui/store-application/repository-breadcrumb.css
- web/src/style.css
- web/tests/browser/reads.spec.ts
- web/tests/production-store-loaders.test.ts
- web/tests/vite/production-store-data-source.test.ts
- web/vite.config.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/vite/production-store-data-source.test.ts#query cache reuses reads
  within one authentication session
- web/tests/vite/production-store-data-source.test.ts#query keys isolate
  credentials that reconnect to the same repository
- web/tests/vite/production-store-data-source.test.ts#authorization and
  nontransient API failures are never retried
- web/tests/vite/production-store-data-source.test.ts#closing a source aborts
  requests and clears protected cache entries
- web/tests/production-store-loaders.test.ts#native presentation preserves
  conflicting targets and exact identifiers
- web/tests/production-store-loaders.test.ts#paged reads reject a continuation
  from another captured snapshot
- web/tests/production-store-loaders.test.ts#ambiguous operations require a
  parent before loading change groups
- web/tests/production-store-loaders.test.ts#explicit operation parent maps each
  changed path to its comparison
- web/tests/browser/reads.spec.ts#native_graphs_files_and_source_rendering_are_accessible
- web/tests/browser/reads.spec.ts#disconnect_discards_inflight_history_and_revocation_clears_data
- web/tests/browser/reads.spec.ts#graph_and_directory_pages_retain_exact_snapshots_and_query_context
- web/tests/browser/reads.spec.ts#event_reconnect_resumes_and_reset_requires_explicit_refresh
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-UI-011 details 2](details-02.md)
