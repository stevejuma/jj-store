---
schema: 1
id: E-READ-005
---

# E-READ-005

<!-- evidence-field:id markdown -->
## Id

E-READ-005
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4.1
- M13.4
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-04
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change kqmuytop
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Independent workspace windows and exact reference startup selection
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; jj-lib 0.45.1; Node 26.9.0; pnpm 10.32.1. Inherited temporary Design Core source validation from E-READ-004; registry lockfile unchanged.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive workspaces reference kind and exact selector for native/local requests. Signed cursors bind exact selector. Persisted native objects, manifest/WAL and local companion formats unchanged.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/local_browser.rs
- crates/jj-store-cli/tests/local_browser/bounded_reads.rs
- crates/jj-store-cli/tests/native_lifecycle/browsing.rs
- crates/jj-store-cli/tests/native_lifecycle/http_browsing.rs
- crates/jj-store-cli/tests/native_lifecycle/reference_windows.rs
- crates/jj-store-http/src/routes.rs
- crates/jj-store-jj/src/local_browser/mod.rs
- crates/jj-store-jj/src/local_browser/references.rs
- crates/jj-store-jj/src/query/content/references.rs
- crates/jj-store-service/src/pagination/browsing.rs
- schemas/openapi.json
- spec/src/api.md
- spec/src/cli.md
- spec/src/compatibility.md
- spec/src/jj-integration.md
- spec/src/local-jj.md
- spec/src/milestones.md
- spec/src/ui.md
- web/src/lib/api/generated.ts
- web/src/lib/controller/contracts.ts
- web/src/lib/controller/http-local-code-data-source.ts
- web/src/lib/controller/http-store-data-source.ts
- web/src/lib/controller/native-data-source.ts
- web/src/lib/controller/native-presentation.ts
- web/src/lib/controller/production-reference-reader.ts
- web/src/lib/controller/production-route-restorer.ts
- web/src/lib/controller/production-store-actions.ts
- web/src/lib/controller/production-store-loaders.ts
- web/src/lib/controller/production-store-reader.svelte.ts
- web/src/lib/controller/production-store-state.svelte.ts
- web/src/lib/controller/production-store-view.ts
- web/src/lib/controller/store-workspaces.ts
- web/src/lib/ui/store-application/StoreWorkspacePicker.svelte
- web/tests/browser/repository-navigation.spec.ts
- web/tests/browser/workspaces.spec.ts
- web/tests/vite/production-reference-reader.test.ts
- web/tests/vite/store-workspaces.test.ts
- spec/src/mutations.md
- web/src/lib/controller/http-reference-reads.ts
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-LOCAL-004
- JST-JJ-003
- JST-API-001
- JST-UI-005
- JST-UI-006
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/local_browser/bounded_reads.rs#local_metadata_and_bookmark_pages_do_not_load_history_or_index
- crates/jj-store-cli/tests/native_lifecycle/reference_windows.rs#workspace_windows_seek_filter_and_resolve_exact_names_without_history
- web/tests/vite/store-workspaces.test.ts#workspace pages do not drain bookmarks
  or later workspace pages
- web/tests/vite/production-reference-reader.test.ts#exact bookmark outside the
  first window stays selected when pickers arrive
- web/tests/browser/workspaces.spec.ts#workspace names sharing a commit survive
  cold reload, tabs, history and explicit Refresh
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-READ-005 details 2](details-02.md)
- [E-READ-005 details 3](details-03.md)
