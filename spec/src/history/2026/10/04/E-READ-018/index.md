---
schema: 1
id: E-READ-018
---

# E-READ-018

<!-- evidence-field:id markdown -->
## Id

E-READ-018
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4.1
- M13.5
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-04
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change mnyolwqt
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Local prepared-history HTTP/browser delivery and bounded read-only metadata
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust1.98.1; jj-lib0.45.1; Node26.9.0; pnpm10.32.1; macOS ARM64.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Persistence and collaboration capacity unchanged. Additive local index_tasks capability, local_snapshot error pin and start/status/bodyless-cancel routes; regenerated OpenAPI and TypeScript.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/src/local_browser.rs
- crates/jj-store-cli/tests/local_browser/bounded_reads.rs
- crates/jj-store-cli/tests/local_browser/preparation.rs
- crates/jj-store-cli/tests/local_browser.rs
- crates/jj-store-http/src/dto.rs
- crates/jj-store-http/src/error.rs
- crates/jj-store-http/src/lib.rs
- crates/jj-store-http/src/local_browser/index_tasks.rs
- crates/jj-store-http/src/local_browser.rs
- crates/jj-store-jj/src/local_browser/git_read/commit.rs
- crates/jj-store-jj/src/local_browser/git_read/extras.rs
- crates/jj-store-jj/src/local_browser/git_read.rs
- crates/jj-store-jj/src/local_browser/mod.rs
- crates/jj-store-jj/src/local_browser/preparation/graph.rs
- crates/jj-store-jj/src/local_browser/preparation.rs
- crates/jj-store-jj/src/local_browser/scoped.rs
- crates/jj-store-jj/src/local_metadata/mod.rs
- crates/jj-store-jj/src/local_metadata/types.rs
- crates/jj-store-jj/tests/local_metadata.rs
- schemas/openapi.json
- spec/src/api.md
- spec/src/cli.md
- spec/src/compatibility.md
- spec/src/jj-integration.md
- spec/src/local-jj.md
- spec/src/ui.md
- web/src/lib/api/api-error.ts
- web/src/lib/api/generated.ts
- web/src/lib/api/local-client.ts
- web/src/lib/controller/http-local-index-data-source.ts
- web/src/lib/controller/http-local-query.ts
- web/src/lib/controller/http-local-store-data-source.ts
- web/src/lib/controller/native-data-source.ts
- web/src/lib/controller/repository-profile.ts
- web/tests/real/local-browser.spec.ts
- web/tests/vite/local-browser.test.ts
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:removed_files list -->
## Removed files

- crates/jj-store-jj/src/local_browser/preparation/commit.rs
- crates/jj-store-jj/src/local_browser/preparation/extras.rs
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-JJ-003
- JST-JJ-005
- JST-LOCAL-001
- JST-LOCAL-002
- JST-LOCAL-004
- JST-API-004
- JST-API-006
- JST-UI-019
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/local_browser.rs#filesystem_runtime_registers_and_reads_without_native_storage
- crates/jj-store-cli/tests/local_browser/bounded_reads.rs#ordinary_point_reads_do_not_import_missing_extra_metadata
- crates/jj-store-jj/src/local_browser/git_read/extras.rs#point_lookup_seeks_without_reading_unrelated_large_values
- crates/jj-store-jj/tests/local_metadata.rs#identity_observation_does_not_admit_or_read_domain_payloads
- web/tests/vite/local-browser.test.ts#local_preparation_preserves_the_failed_query_pin_and_rejects_mismatched_task_status
- web/tests/real/local-browser.spec.ts#local_repository_opens_files_history_agents_and_accessible_folder_controls
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-READ-018 details 2](details-02.md)
- [E-READ-018 details 3](details-03.md)
