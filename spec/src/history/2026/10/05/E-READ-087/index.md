---
schema: 1
id: E-READ-087
---

# E-READ-087

<!-- evidence-field:id markdown -->
## Id

E-READ-087
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
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Resumable JJ file-history attribution through existing managed indexes
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change wpvmvnlq
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

E-READ-086 left 22 root and 18 arch Linux rows permanently Budget-unavailable beyond 64 first-parent visits. Exact &gt;64 regression failed before implementation. Real acceptance also exposed repeated Git handle validation, synchronous JJ File polling exhausting Tokio cooperative budget, and read-only rejection of clean merged-parent synthetic file writes. Real block/root/arch limit-50 requests later hit 30-second proxy 502 deadlines; linux-ui-windows-final.log/json records this. The combined candidate/path cap removes the deadline failure; union filtering subsequently reduces repeated unrelated comparisons.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Keep filename-only reads unchanged. Use JJ File predicates over ancestry batches of at most 64 commits and 1024 candidate/path evaluations, with a union predicate excluding unrelated commits before per-row checks. Retain operation/commit/path-scoped bounded ID-only continuation and resolved-row reuse. Reuse the managed preparation queue to optionally extend upstream standard changed-path indexes; no watcher or authoritative indexing framework. Add Pending/NotFound outcomes and cancellable UI retry/backoff, preserving resolved rows and skeletons. Reuse one Git handle per local request. Supply bounded request-local canonical merge blobs without source/native publication. UI also preserves validated resolved metadata if an evicted server checkpoint returns Pending for the same immutable terms.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

HTTP V1 additive native/local Pending and NotFound unavailable reasons; generated OpenAPI/TypeScript updated. Pinned JJ 0.45.1 changed-path format-0 and commit format-6 remain upstream formats in disposable managed copies. No source/index/ref write, native stored-format, manifest/WAL/receipt/cursor/credential/installed CLI change.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; Rust 1.98.1; JJ/jj-lib/jj-core 0.45.1; Node 26.9.0; pnpm 10.32.1; Svelte 5.57.1; Vitest 4.1.11; Storybook 10.5.7.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/local_browser/managed_code.rs
- crates/jj-store-cli/tests/local_browser/managed_http.rs
- crates/jj-store-cli/tests/native_lifecycle/path_metadata.rs
- crates/jj-store-http/src/lib.rs
- crates/jj-store-jj/Cargo.toml
- crates/jj-store-jj/src/backend.rs
- crates/jj-store-jj/src/factories.rs
- crates/jj-store-jj/src/local_browser/code_index.rs
- crates/jj-store-jj/src/local_browser/git_read/commit.rs
- crates/jj-store-jj/src/local_browser/git_read.rs
- crates/jj-store-jj/src/local_browser/mod.rs
- crates/jj-store-jj/src/local_browser/paths.rs
- crates/jj-store-jj/src/local_browser/scoped.rs
- crates/jj-store-jj/src/query/cache/managed.rs
- crates/jj-store-jj/src/query/code_index/merge_files.rs
- crates/jj-store-jj/src/query/code_index/native.rs
- crates/jj-store-jj/src/query/code_index/path_history.rs
- crates/jj-store-jj/src/query/code_index/path_index.rs
- crates/jj-store-jj/src/query/code_index.rs
- crates/jj-store-jj/src/query/content/native.rs
- crates/jj-store-jj/src/query/content/paths.rs
- crates/jj-store-jj/src/query/content.rs
- crates/jj-store-jj/src/query/mod.rs
- crates/jj-store-service/src/code_indexes/jobs.rs
- crates/jj-store-service/src/code_indexes/paths.rs
- crates/jj-store-service/src/code_indexes.rs
- crates/jj-store-service/src/pagination/browsing.rs
- schemas/openapi.json
- spec/src/api.md
- spec/src/cli.md
- spec/src/compatibility.md
- spec/src/jj-integration.md
- spec/src/local-jj.md
- spec/src/mutations.md
- spec/src/ui.md
- web/src/lib/api/generated.ts
- web/src/lib/controller/native-presentation.ts
- web/src/lib/controller/production-file-metadata.ts
- web/src/lib/controller/production-file-reader.svelte.ts
- web/tests/vite/production-file-reader.test.ts
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/local_browser/managed_code.rs#indexed_path_history_resumes_beyond_sixty_four_commits
- crates/jj-store-cli/tests/local_browser/managed_code.rs#source_changed_path_index_is_copied_without_repair_or_history_tree_reads
- crates/jj-store-cli/tests/local_browser/managed_code.rs#indexed_merge_history_resolves_parent_contents_without_source_writes
- crates/jj-store-cli/tests/local_browser/managed_http.rs#managed_http_path_metadata_resumes_and_preserves_source_state
- crates/jj-store-cli/tests/native_lifecycle/path_metadata.rs#path_metadata_crosses_merges_and_retains_the_ancestry_bound
- crates/jj-store-cli/tests/local_browser/managed_code.rs#wide_path_history_skips_unrelated_commits_once_per_window
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-READ-087 details 2](details-02.md)
- [E-READ-087 details 3](details-03.md)
- [E-READ-087 details 4](details-04.md)
- [E-READ-087 details 5](details-05.md)
