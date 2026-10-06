---
schema: 1
id: E-M4-008
---

# E-M4-008

<!-- evidence-field:id markdown -->
## Id

E-M4-008
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4
- M4.1
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-27
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

bounded-native-query-cache-retention-and-recovery
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/native_lifecycle/graph_cache.rs
- crates/jj-store-cli/tests/native_lifecycle/graphs.rs
- crates/jj-store-jj/src/factories.rs
- crates/jj-store-jj/src/query/cache.rs
- crates/jj-store-jj/src/query/mod.rs
- spec/src/api.md
- spec/src/cli.md
- spec/src/jj-integration.md
- crates/jj-store-cli/tests/administration_rpc.rs
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/native_lifecycle/graph_cache.rs#graph_cache_corrupt_operation_link_rebuilds_without_publication
- crates/jj-store-cli/tests/native_lifecycle/graph_cache.rs#graph_cache_checksums_missing_files_and_receipts_rebuild_exact_snapshots
- crates/jj-store-cli/tests/native_lifecycle/graph_cache.rs#graph_cache_retention_evicts_lru_and_bounds_bytes_files_and_slots
- crates/jj-store-cli/tests/native_lifecycle/graph_cache.rs#graph_cache_recovery_rechecks_permission_and_serializes_independent_executors
- crates/jj-store-cli/tests/native_lifecycle/graph_cache.rs#graph_cache_recovery_never_follows_symlinks_or_repairs_authoritative_objects
- crates/jj-store-cli/tests/native_lifecycle/graph_cache.rs#graph_cache_bindings_separate_repository_and_epoch_for_identical_root_ids
- crates/jj-store-cli/tests/native_lifecycle/graphs.rs#graph_query_three_histories_filtered_edges_and_snapshot_pagination
- crates/jj-store-cli/tests/native_lifecycle/graphs.rs#graph_query_divergent_operations_bookmarks_and_explicit_workspace
- crates/jj-store-cli/tests/native_lifecycle/graphs.rs#graph_query_rebuild_limits_staged_rejection_and_current_revocation
- crates/jj-store-cli/tests/native_lifecycle/graphs.rs#graph_query_cancellation_retains_worker_permit_until_exit_and_deadlines_bound_waits
- crates/jj-store-cli/tests/native_lifecycle/graphs.rs#graph_query_marks_absent_evolution_data_and_rejects_missing_continuations
- crates/jj-store-cli/tests/administration_rpc.rs#rpc_fixture_reserves_listener_across_restarts
- crates/jj-store-cli/tests/administration_rpc.rs#rpc_token_retries_and_competing_snapshots_preserve_accepted_policy
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M4-008 details 2](details-02.md)
- [E-M4-008 details 3](details-03.md)
