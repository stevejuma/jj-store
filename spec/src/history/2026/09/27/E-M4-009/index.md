---
schema: 1
id: E-M4-009
---

# E-M4-009

<!-- evidence-field:id markdown -->
## Id

E-M4-009
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

native-query-cancellation-and-release-resource-qualification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/native_lifecycle/graphs.rs
- crates/jj-store-cli/tests/native_lifecycle/query_resources.rs
- crates/jj-store-jj/examples/query_qualification.rs
- crates/jj-store-jj/src/query/access.rs
- crates/jj-store-jj/src/query/content.rs
- crates/jj-store-jj/src/query/mod.rs
- crates/jj-store-jj/src/runtime.rs
- docs/evidence/m4-native-query-audit.md
- docs/evidence/m4-native-query-budget-before.json
- docs/evidence/m4-native-query-budget.json
- scripts/qualify-query-resources.py
- spec/src/api.md
- spec/src/cli.md
- spec/src/compatibility.md
- spec/src/jj-integration.md
- spec/src/spec-governance.md
- docs/evidence/m4-runtime-rustfs.json
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/native_lifecycle/query_resources.rs#query_cancel_drops_pending_graph_authorization_and_releases_cache
- crates/jj-store-cli/tests/native_lifecycle/query_resources.rs#query_cancel_drops_pending_content_authorization_and_releases_permit
- crates/jj-store-cli/tests/native_lifecycle/query_resources.rs#query_cancel_drops_remote_object_bodies_for_graphs_and_content
- crates/jj-store-jj/src/runtime.rs#native_runtime_future_progress_does_not_require_foreign_executor_cooperative_reset
- crates/jj-store-jj/src/runtime.rs#native_runtime_stream_progress_does_not_require_foreign_executor_cooperative_reset
- crates/jj-store-cli/tests/native_lifecycle/graphs.rs#graph_query_cancellation_retains_worker_permit_until_exit_and_deadlines_bound_waits
- crates/jj-store-jj/examples/query_qualification.rs#main
- crates/jj-store-jj/tests/rustfs.rs#real_rustfs_transport_and_native_blob_contract
- crates/jj-store-jj/tests/rustfs.rs#real_rustfs_manifest_publication_and_cold_recovery
- crates/jj-store-cli/tests/server_executable.rs#real_rustfs_server_cli_restart_and_revocation
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M4-009 details 2](details-02.md)
- [E-M4-009 details 3](details-03.md)
