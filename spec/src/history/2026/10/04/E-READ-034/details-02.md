---
schema: 1
id: E-READ-034
---

# E-READ-034 details 2

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/native_lifecycle/graph_cache.rs#cold_historical_indexes_leave_the_primary_generation_immutable
- crates/jj-store-cli/tests/native_lifecycle/graph_cache.rs#warm_filtered_graph_starts_while_another_operation_is_preparing
- crates/jj-store-cli/tests/native_lifecycle/graph_cache.rs#private_historical_construction_reserves_quota_without_blocking_warm_graphs
- crates/jj-store-cli/tests/native_lifecycle/graph_cache.rs#graph_cache_recovery_rechecks_permission_and_serializes_independent_executors
- crates/jj-store-jj/src/query/cache/constructed.rs#private_output_requires_exact_operation_link_and_segment_checksum
<!-- /evidence-field -->

