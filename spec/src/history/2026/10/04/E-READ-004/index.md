---
schema: 1
id: E-READ-004
---

# E-READ-004

<!-- evidence-field:id markdown -->
## Id

E-READ-004
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

enclosing Jujutsu change rpzuyysk
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Files-first bounded reads and independent metadata
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; jj-lib 0.45.1; Node 26.9.0; pnpm 10.32.1. Registry Design Core 0.2.0 baseline checked; inherited temporary Design Core source 0.1.2 used for focused UI validation.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive path include_last_change flag defaults true; false returns not_requested. Local commit_summary and commit/path/file-scoped file_at added. Signed cursors bind metadata mode. Persisted native, WAL, manifest, local companion and credential formats unchanged.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/community-projects/CommunityProjectsRepositoryDirectory.stories.svelte
- crates/jj-store-cli/tests/local_browser/bounded_reads.rs
- crates/jj-store-cli/tests/local_browser.rs
- crates/jj-store-cli/tests/native_lifecycle/browsing.rs
- crates/jj-store-cli/tests/native_lifecycle/http_browsing.rs
- crates/jj-store-http/src/routes.rs
- crates/jj-store-jj/src/local_browser/mod.rs
- crates/jj-store-jj/src/local_browser/native.rs
- crates/jj-store-jj/src/local_browser/paths.rs
- crates/jj-store-jj/src/query/content/paths.rs
- crates/jj-store-service/src/pagination/browsing.rs
- schemas/openapi.json
- spec/src/api.md
- spec/src/cli.md
- spec/src/compatibility.md
- spec/src/jj-integration.md
- spec/src/local-jj.md
- spec/src/milestones.md
- spec/src/mutations.md
- spec/src/ui.md
- web/src/lib/api/generated.ts
- web/src/lib/controller/contracts.ts
- web/src/lib/controller/file-read-queue.ts
- web/src/lib/controller/http-file-reads.ts
- web/src/lib/controller/http-local-code-data-source.ts
- web/src/lib/controller/http-local-query.ts
- web/src/lib/controller/http-store-data-source.ts
- web/src/lib/controller/native-data-source.ts
- web/src/lib/controller/native-presentation.ts
- web/src/lib/controller/production-file-loaders.ts
- web/src/lib/controller/production-file-metadata.ts
- web/src/lib/controller/production-file-pages.ts
- web/src/lib/controller/production-file-reader.svelte.ts
- web/src/lib/controller/production-landing-documents.ts
- web/src/lib/controller/production-store-controller.svelte.ts
- web/src/lib/controller/production-store-loaders.ts
- web/src/lib/controller/production-store-reader.svelte.ts
- web/src/lib/controller/production-store-state.svelte.ts
- web/src/lib/controller/production-store-view.ts
- web/src/lib/controller/read-scope.ts
- web/src/lib/projects/repository-log.ts
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryDirectory.svelte
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryDirectoryRows.svelte
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryFilePanel.svelte
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryFilePanelHeader.svelte
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryFiles.svelte
- web/tests/browser/reads.spec.ts
- web/tests/vite/file-read-queue.test.ts
- web/tests/vite/production-file-reader.test.ts
- web/tests/vite/production-store-data-source.test.ts
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

- crates/jj-store-cli/tests/local_browser/bounded_reads.rs#local_file_rows_read_only_the_selected_commit_and_traversed_trees
- web/tests/vite/production-file-reader.test.ts#file rows become ready before
  commit metadata and landing documents
- web/tests/browser/reads.spec.ts#files remain usable while optional header and
  attribution are unavailable
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-READ-004 details 2](details-02.md)
- [E-READ-004 details 3](details-03.md)
