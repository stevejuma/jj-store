---
schema: 1
id: E-READ-088
---

# E-READ-088

<!-- evidence-field:id markdown -->
## Id

E-READ-088
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-05
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4.1
- M4.6
- M4.9
- M4.10
- M13.4
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-API-001
- JST-API-005
- JST-UI-003
- JST-JJ-003
- JST-LOCAL-004
- JST-GOV-008
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Quick jj-lib metadata with Snapshot provenance and automatic bounded history refinement
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change vqzozuvl
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Exact local HTTP regression returns Pending instead of terminal snapshot metadata (before.log). Earlier pinned Linux investigation measured 1.5-1.7s names-only and 2.1-2.9s per unfinished metadata batch. Git pack reopening and operation fingerprinting remain outside this slice.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Native/local quick mode shares a 64-step first-parent scan, comparing grouped tree values and accepting integration commits. Selected-commit fallback is genuine Snapshot metadata. Per user follow-up, Files displays quick values first then automatically refines using the existing bounded JJ file-history reader, preserving visible values through pending/errors and canceling obsolete work. No Files-triggered changed-path indexing. First-line descriptions truncate before Updated; camera icon replaces Snapshot chip. Direct local metadata path seeks work after page one without overriding signed cursors.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive HTTP V1 available last_change.attribution (path_change or snapshot) and optional last_change_mode (quick default, history). Older responses without attribution retain path-change presentation and do not trigger unsupported refinement. Existing routes, source formats and native identities remain. Cursor query digest binds mode; no source/index repair or installed CLI replacement.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; JJ/jj-lib 0.45.1; Rust 1.98.1; Node 26.9.0; pnpm 10.32.1; Svelte 5.57.1; Vitest 4.1.11; Storybook 10.5.7.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/community-projects/CommunityProjectsRepositoryDirectory.stories.svelte
- crates/jj-store-cli/tests/local_browser/bounded_reads.rs
- crates/jj-store-cli/tests/local_browser/managed_code.rs
- crates/jj-store-cli/tests/local_browser/managed_http.rs
- crates/jj-store-cli/tests/local_browser/prepared_points.rs
- crates/jj-store-cli/tests/local_browser.rs
- crates/jj-store-cli/tests/native_lifecycle/browsing.rs
- crates/jj-store-cli/tests/native_lifecycle/path_metadata.rs
- crates/jj-store-http/src/local_browser.rs
- crates/jj-store-http/src/routes.rs
- crates/jj-store-jj/src/local_browser/paths.rs
- crates/jj-store-jj/src/query/content/paths.rs
- crates/jj-store-service/src/code_indexes/paths.rs
- crates/jj-store-service/src/pagination/browsing.rs
- docs/ui-provenance.json
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
- web/src/lib/controller/http-file-reads.ts
- web/src/lib/controller/http-local-code-data-source.ts
- web/src/lib/controller/native-data-source.ts
- web/src/lib/controller/native-presentation.ts
- web/src/lib/controller/production-file-loaders.ts
- web/src/lib/controller/production-file-metadata.ts
- web/src/lib/projects/repository-log.ts
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryDirectory.svelte
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryDirectoryAttribution.svelte
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryDirectoryRows.svelte
- web/src/lib/ui/community-projects/repository-directory.css
- web/tests/vite/production-file-reader.test.ts
- spec/src/spec-governance.md
<!-- /evidence-field -->

<!-- evidence-field:removed_tests list -->
## Removed tests

- crates/jj-store-cli/tests/local_browser/managed_code.rs#wide_path_history_skips_unrelated_commits_once_per_window
- crates/jj-store-cli/tests/local_browser/managed_http.rs#managed_http_path_metadata_resumes_and_preserves_source_state
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/local_browser/managed_http.rs#managed_http_path_metadata_finishes_with_snapshot_and_preserves_source_state
- crates/jj-store-cli/tests/local_browser/managed_code.rs#indexed_path_metadata_finishes_with_snapshot_beyond_sixty_four_commits
- crates/jj-store-cli/tests/local_browser/managed_code.rs#wide_path_metadata_finishes_in_one_shared_scan
- crates/jj-store-cli/tests/local_browser/managed_code.rs#indexed_merge_metadata_uses_integration_without_source_writes
- crates/jj-store-cli/tests/native_lifecycle/path_metadata.rs#path_metadata_crosses_merges_and_retains_the_ancestry_bound
- crates/jj-store-cli/tests/local_browser/managed_code.rs#indexed_path_history_resumes_beyond_sixty_four_commits
- crates/jj-store-cli/tests/local_browser/managed_code.rs#indexed_merge_history_resolves_parent_contents_without_source_writes
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-READ-088 details 2](details-02.md)
- [E-READ-088 details 3](details-03.md)
- [E-READ-088 details 4](details-04.md)
- [E-READ-088 details 5](details-05.md)
