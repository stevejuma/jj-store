---
schema: 1
id: E-READ-046
---

# E-READ-046

<!-- evidence-field:id markdown -->
## Id

E-READ-046
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

enclosing Jujutsu change ypqxwvkt
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Compact native prepared visible identities and lexical reference groups under unchanged resident admission
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust1.98.1; jj-lib/jj-cli source pinned0.45.1; macOS ARM64.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No persisted native/index6/receipt/companion/WAL, public DTO/schema, RPC or cursor format changes. Native selected visibility uses a sorted Vec of full64-byte commit IDs; ordered reference groups use sorted80-byte native change/commit pairs and4-byte group offsets. Conservative identity reservation is192bytes/indexed commit, down from512; the44byte difference beyond148byte array capacities covers bounded upstream prefix/traversal scratch, with heads/reverse labels/fixed overhead admitted separately. Construction reserves maximum capacities before growth, rejects duplicate visibility, and uses in-place heapsort with ordinary cancellation/deadline/work checks at most every1024comparisons. Arbitrary lexical anchors compare39-byte stack keys without heap formatting; selected groups alone allocate source IDs/metadata. Upstream prefix ambiguity/hidden offsets and fresh native access remain unchanged. Full resource and frontier qualification stays deferred.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-jj/src/query/projection.rs
- crates/jj-store-jj/src/query/projection/changes.rs
- crates/jj-store-jj/src/query/projection/references.rs
- crates/jj-store-jj/src/query/projection/references/tests.rs
- crates/jj-store-jj/src/query/projection/tests.rs
- crates/jj-store-jj/src/query/content/references/prepared.rs
- spec/src/jj-integration.md
- spec/src/architecture.md
- spec/src/milestones.md
- docs/implementation-plan.md
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Continue native indexed filters with retained quota-accounted filter-file lifetime, then different-operation/deep/comparison/search/RPC delivery. Keep complete combined qualification separate.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Private native prepared visibility/reference representation and construction admission. Exact resource/lexical/sort regressions plus focused native graph, reference, prefix/history, shared worker, cache/recovery, upstream-byte and signed HTTP consumers. JJ/service/HTTP/RPC/CLI checks cover direct callers. No full repository/UI/contract/release or combined qualification.
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-JJ-003
- JST-JJ-005
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-jj/src/query/projection/tests.rs#native_projection_admits_two_million_identities_with_index_headroom
- crates/jj-store-jj/src/query/projection/references/tests.rs#flat_native_groups_preserve_lexical_windows_and_divergent_target_order
- crates/jj-store-jj/src/query/projection/references/tests.rs#native_projection_sort_is_exact_and_stops_at_work_cancellation_and_deadline_bounds
- crates/jj-store-cli/tests/native_lifecycle/reference_windows.rs#native_prepared_references_select_visible_windows_before_target_reads
- crates/jj-store-cli/tests/native_lifecycle/graphs.rs#graph_query_three_histories_filtered_edges_and_snapshot_pagination
- crates/jj-store-cli/tests/native_lifecycle/graphs.rs#graph_query_divergent_operations_bookmarks_and_explicit_workspace
- crates/jj-store-cli/tests/native_lifecycle/graph_cache.rs#historical_predicates_reuse_separately_prepared_residents_without_slot_growth
- crates/jj-store-cli/tests/native_lifecycle/graph_cache.rs#resident_indexes_preserve_multiple_historical_operation_scopes
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-READ-046 details 2](details-02.md)
- [E-READ-046 details 3](details-03.md)
