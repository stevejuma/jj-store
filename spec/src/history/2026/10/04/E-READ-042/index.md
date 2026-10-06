---
schema: 1
id: E-READ-042
---

# E-READ-042

<!-- evidence-field:id markdown -->
## Id

E-READ-042
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M13.4
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-04
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change volopptk
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Prepared retained local operation-prefix lookup with canonical source fingerprints and bounded disk/resident admission
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust1.98.1 (48a229cea); jj-lib0.45.1; stock JJ0.45.1; macOS ARM64.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Disposable local-operations artifact: JSTLOP1 NUL magic, selected64-byte operation, sorted96-byte operation/fingerprint records and SHA256 checksum. Export reuses ordered retained-operation scratch; reserves bytes before writing and resident192bytes/record plus64KiB before allocation. Native index segment/link bytes, public routes/schemas/cursors, native/WAL/companion/receipt formats and ordinary read limits unchanged.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/local_browser/preparation.rs
- crates/jj-store-cli/tests/local_browser/prepared_operations.rs
- crates/jj-store-cli/tests/local_browser/prepared_points_http.rs
- crates/jj-store-cli/tests/local_browser.rs
- crates/jj-store-jj/src/local_browser/graphs.rs
- crates/jj-store-jj/src/local_browser/mod.rs
- crates/jj-store-jj/src/local_browser/operation_store.rs
- crates/jj-store-jj/src/local_browser/preparation/graph.rs
- crates/jj-store-jj/src/local_browser/preparation.rs
- crates/jj-store-jj/src/query/language/operations.rs
- crates/jj-store-jj/src/query/prepared/mod.rs
- crates/jj-store-jj/src/query/prepared/operations.rs
- docs/implementation-plan.md
- spec/src/api.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/compatibility.md
- spec/src/jj-integration.md
- spec/src/local-jj.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-LOCAL-004
- JST-JJ-003
- JST-API-001
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/local_browser/prepared_operations.rs#prepared_operation_prefixes_resolve_with_only_selected_metadata
- crates/jj-store-cli/tests/local_browser/prepared_points_http.rs#local_point_http_uses_ready_membership_and_only_falls_back_when_cold
- crates/jj-store-cli/tests/local_browser/preparation.rs#local_preparation_matches_upstream_without_trees_or_source_index_writes
- crates/jj-store-cli/tests/local_browser/preparation.rs#prepared_local_log_preserves_conflicts_and_filtered_native_edges
- crates/jj-store-jj/src/query/prepared/operations.rs#operation_identity_writer_refuses_growth_before_exceeding_reserved_bytes
- crates/jj-store-jj/src/query/prepared/operations.rs#lookup_preserves_exact_odd_missing_and_ambiguous_prefixes_under_admission
- crates/jj-store-jj/src/query/prepared/operations.rs#lookup_rejects_corrupt_framing_checksum_order_membership_and_source_selection
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-READ-042 details 2](details-02.md)
- [E-READ-042 details 3](details-03.md)
