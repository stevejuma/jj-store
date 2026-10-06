---
schema: 1
id: E-READ-065
---

# E-READ-065

<!-- evidence-field:id markdown -->
## Id

E-READ-065
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

xwntxpvz
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

prepared-predecessor-occurrence-seek
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; jj-lib/jj-core 0.45.1; macOS ARM64.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Authoritative storage, WAL, DTO, cursor, cache receipt and upstream format 6 unchanged. Disposable operation-history JSTOPG3 adds map-presence/discovery-position flags and exact commit identity plus four-byte occurrence references. JSTOPG1/2 stay readable with bounded replay. No migration or installed executable change.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Exact sparse-history regressions and lookup units first. Complete query library and native_lifecycle/local_browser test binaries because shared native/local prepared evolution, history decoding and topology scratch affect both profiles and their graph/preparation consumers. Focused package, Rust format, boundary, architecture and changed-document checks. Root aggregates, RPC/HTTP/UI/contract generation and complete scale/RSS/latency/fault/cold-browser/real-RustFS/release qualification out of scope.
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

- crates/jj-store-cli/tests/local_browser/preparation.rs
- crates/jj-store-cli/tests/local_browser/prepared_evolution_pages.rs
- crates/jj-store-cli/tests/native_lifecycle/prepared_evolution_pages.rs
- crates/jj-store-jj/src/query/prepared/mod.rs
- crates/jj-store-jj/src/query/prepared/operations/history/evolution.rs
- crates/jj-store-jj/src/query/prepared/operations/history/predecessors.rs
- crates/jj-store-jj/src/query/prepared/operations/history/tests.rs
- crates/jj-store-jj/src/query/prepared/operations/history/traversal.rs
- crates/jj-store-jj/src/query/prepared/operations/history.rs
- docs/implementation-plan.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/jj-integration.md
- spec/src/local-jj.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/native_lifecycle/prepared_evolution_pages.rs#prepared_evolution_seeks_predecessors_across_unrelated_bookmark_operations
- crates/jj-store-cli/tests/local_browser/prepared_evolution_pages.rs#prepared_evolution_seeks_predecessors_across_unrelated_bookmark_operations
- crates/jj-store-jj/src/query/prepared/operations/history/tests.rs#predecessor_lookup_seeks_ranked_repeated_occurrences_pending_branches_and_legacy_boundary
- crates/jj-store-jj/src/query/prepared/operations/history/tests.rs#predecessor_lookup_rejects_corrupt_presence_length_order_and_operation_membership
- crates/jj-store-jj/src/query/prepared/operations/history/tests.rs#predecessor_occurrences_do_not_multiply_topology_scratch
- crates/jj-store-jj/src/query/prepared/operations/history/tests.rs#evolution_anchor_proof_rejects_repeated_ids_and_keeps_format_one_readable
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-READ-065 details 2](details-02.md)
- [E-READ-065 details 3](details-03.md)
