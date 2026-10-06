---
schema: 1
id: E-READ-053
---

# E-READ-053

<!-- evidence-field:id markdown -->
## Id

E-READ-053
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

enclosing Jujutsu change nvypupmw
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Prepared local operation child selectors seek bounded retained reverse edges
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust1.98.1;jj-lib/jj-cli0.45.1;local Git SHA-1 browser profile;native index format-6 unchanged;macOS ARM64.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Adds disposable local-operation-children projection framing1 (JSTLCH1, exact selected operation, sorted128-byte parent/child records, SHA256 checksum). Existing JSTLOP2 identity/fingerprint framing and native JJ index bytes remain unchanged. Resident admission charges flat edge capacities and buffer before decoding; disk output stays under existing preparation quota. Missing older projection requires explicit reconstruction for historical prefix-based child selection; current-relative selectors retain the bounded cold fallback. No authoritative object, public DTO/schema, signed cursor, RPC, receipt, WAL or manifest format change.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/local_browser/preparation.rs
- crates/jj-store-cli/tests/local_browser/prepared_operations.rs
- crates/jj-store-cli/tests/local_browser/prepared_points_http.rs
- crates/jj-store-jj/src/local_browser/graphs.rs
- crates/jj-store-jj/src/local_browser/mod.rs
- crates/jj-store-jj/src/local_browser/preparation/graph.rs
- crates/jj-store-jj/src/query/language/operations.rs
- crates/jj-store-jj/src/query/prepared/mod.rs
- crates/jj-store-jj/src/query/prepared/operations/children/tests.rs
- crates/jj-store-jj/src/query/prepared/operations/children.rs
- crates/jj-store-jj/src/query/prepared/operations.rs
- docs/implementation-plan.md
- spec/src/api.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/jj-integration.md
- spec/src/local-jj.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Commit verified local child selection, then prioritize native indexed neighbors and deeper operation/predecessor/copy/search continuations. Main retains E-READ-052; follow-up stays in bounded-reads. Full scale/RSS/latency/fault/cold browser/RustFS/release and Help/source-review qualification stays deferred.
<!-- /evidence-field -->

<!-- evidence-field:commands list -->
## Commands

[]
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/local_browser/prepared_operations.rs#prepared_operation_prefixes_resolve_with_only_selected_metadata
- crates/jj-store-cli/tests/local_browser/prepared_operations.rs#prepared_child_operation_selectors_reject_retained_branch_ambiguity_and_later_children
- crates/jj-store-jj/src/query/prepared/operations/children/tests.rs#child_projection_preserves_unique_empty_ambiguous_and_missing_artifact_outcomes
- crates/jj-store-jj/src/query/prepared/operations/children/tests.rs#child_projection_rejects_corruption_outside_membership_and_preallocation_limits
- crates/jj-store-cli/tests/local_browser/prepared_points_http.rs#local_point_http_uses_ready_membership_and_only_falls_back_when_cold
- crates/jj-store-cli/tests/local_browser/preparation.rs#local_preparation_matches_upstream_without_trees_or_source_index_writes
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-READ-053 details 2](details-02.md)
- [E-READ-053 details 3](details-03.md)
