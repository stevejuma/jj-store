---
schema: 1
id: E-READ-006
---

# E-READ-006

<!-- evidence-field:id markdown -->
## Id

E-READ-006
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

enclosing Jujutsu change zlozvqup
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Operation metadata pages independent of commit indexes
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; jj-lib 0.45.1; Node 26.9.0; pnpm 10.32.1. Existing temporary Design Core source qualification; registry release gate remains pending.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive graph cache state not_used. Zero counters indicate no cache observation. Native identities, manifest, WAL and companion formats unchanged.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/local_browser/bounded_reads.rs
- crates/jj-store-cli/tests/native_lifecycle/graphs.rs
- crates/jj-store-cli/tests/native_lifecycle/index_tasks.rs
- crates/jj-store-cli/tests/native_lifecycle/reference_windows.rs
- crates/jj-store-jj/src/local_browser/graphs.rs
- crates/jj-store-jj/src/local_browser/mod.rs
- crates/jj-store-jj/src/local_browser/operation_store.rs
- crates/jj-store-jj/src/query/cache.rs
- crates/jj-store-jj/src/query/graphs.rs
- crates/jj-store-jj/src/query/language/operations.rs
- crates/jj-store-jj/src/query/mod.rs
- schemas/openapi.json
- spec/src/api.md
- spec/src/cli.md
- spec/src/compatibility.md
- spec/src/jj-integration.md
- spec/src/local-jj.md
- spec/src/milestones.md
- spec/src/mutations.md
- web/src/lib/api/generated.ts
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-LOCAL-004
- JST-JJ-003
- JST-JJ-005
- JST-API-001
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/local_browser/bounded_reads.rs#operation_pages_ignore_corrupt_commit_indexes
- crates/jj-store-cli/tests/native_lifecycle/reference_windows.rs#operation_pages_do_not_admit_or_build_a_commit_index
- crates/jj-store-cli/tests/local_browser/bounded_reads.rs#registered_large_repository_metadata_smoke
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-READ-006 details 2](details-02.md)
