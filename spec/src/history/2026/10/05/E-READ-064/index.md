---
schema: 1
id: E-READ-064
---

# E-READ-064

<!-- evidence-field:id markdown -->
## Id

E-READ-064
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M13.4
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-05
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

mmyxnoro
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

prepared-evolution-checkpoints-and-point-graph-visibility
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust1.98.1;jj-lib/jj-core0.45.1;macOS ARM64.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Authoritative object,WAL,DTO,cursor,cache receipt and upstream format6 index unchanged.Disposable operation-history JSTOPG2 adds a checksum-bound64KiB repeated-ID bitmap;JSTOPG1 stays readable with evolution checkpoints disabled.No installed CLI or migration.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Complete query library and CLI native_lifecycle/local_browser binaries:shared native/local graph descriptions and local scoped operation admission consume visibility/predecessor changes;own topology/capture framing changes affect operation-history consumers. Exact regressions first. Package/format/boundary/architecture and changed-document checks. Root aggregate,WAL/RPC/HTTP/UI/generation and complete scale/fault/cold-browser/RustFS/release qualification out of scope.
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-JJ-003
- JST-JJ-005
- JST-LOCAL-004
- JST-API-002
- JST-API-004
- JST-CLI-003
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/local_browser.rs
- crates/jj-store-cli/tests/local_browser/preparation.rs
- crates/jj-store-cli/tests/local_browser/prepared_evolution_pages.rs
- crates/jj-store-cli/tests/native_lifecycle/graphs.rs
- crates/jj-store-cli/tests/native_lifecycle/prepared_evolution_pages.rs
- crates/jj-store-jj/src/local_browser/graphs.rs
- crates/jj-store-jj/src/local_browser/operation_store.rs
- crates/jj-store-jj/src/local_browser/preparation/graph.rs
- crates/jj-store-jj/src/local_browser/preparation/visibility.rs
- crates/jj-store-jj/src/query/graphs.rs
- crates/jj-store-jj/src/query/prepared/mod.rs
- crates/jj-store-jj/src/query/prepared/operations/history.rs
- crates/jj-store-jj/src/query/prepared/operations/history/evolution.rs
- crates/jj-store-jj/src/query/prepared/operations/history/tests.rs
- crates/jj-store-jj/src/query/projection.rs
- docs/implementation-plan.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/jj-integration.md
- spec/src/local-jj.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/native_lifecycle/prepared_evolution_pages.rs#prepared_evolution_continuations_resume_without_preceding_body_reads
- crates/jj-store-cli/tests/local_browser/prepared_evolution_pages.rs#prepared_evolution_continuations_resume_without_preceding_body_reads
- crates/jj-store-jj/src/query/prepared/operations/history/evolution.rs#evolution_state_preserves_branch_order_shared_predecessors_and_private_forks
- crates/jj-store-jj/src/query/prepared/operations/history/evolution.rs#evolution_state_preserves_legacy_duplicates_and_rejects_cycles_and_work_exhaustion
- crates/jj-store-jj/src/query/prepared/operations/history/tests.rs#evolution_anchor_proof_rejects_repeated_ids_and_keeps_format_one_readable
- crates/jj-store-cli/tests/local_browser/preparation.rs#local_preparation_matches_upstream_without_trees_or_source_index_writes
- crates/jj-store-cli/tests/local_browser/preparation.rs#prepared_local_log_preserves_conflicts_and_filtered_native_edges
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-READ-064 details 2](details-02.md)
- [E-READ-064 details 3](details-03.md)
