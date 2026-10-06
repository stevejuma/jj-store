---
schema: 1
id: E-M4-011
---

# E-M4-011

<!-- evidence-field:id markdown -->
## Id

E-M4-011
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

native-revset-predicate-admission-and-reader-runtime
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/native_lifecycle/content.rs
- crates/jj-store-cli/tests/native_lifecycle/query_predicates.rs
- crates/jj-store-jj/src/backend.rs
- crates/jj-store-jj/src/query/access.rs
- crates/jj-store-jj/src/query/content.rs
- crates/jj-store-jj/src/query/graphs.rs
- crates/jj-store-jj/src/query/mod.rs
- crates/jj-store-jj/src/query/predicates.rs
- scripts/qualify-query-resources.py
- spec/src/api.md
- spec/src/cli.md
- spec/src/compatibility.md
- spec/src/jj-integration.md
- spec/src/spec-governance.md
- docs/evidence/m4-native-predicate-audit.md
- docs/evidence/m4-native-predicate-budget.json
- docs/evidence/m4-native-reader-rustfs.json
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/native_lifecycle/query_predicates.rs#native_predicates_bound_file_bytes_before_materialization
- crates/jj-store-cli/tests/native_lifecycle/query_predicates.rs#native_predicates_bound_line_comparison_work_before_evaluation
- crates/jj-store-cli/tests/native_lifecycle/query_predicates.rs#native_predicates_bound_derived_merge_terms_without_limiting_plain_log
- crates/jj-store-cli/tests/native_lifecycle/query_predicates.rs#native_predicates_preserve_admitted_file_and_recursive_merge_semantics
- crates/jj-store-cli/tests/native_lifecycle/query_predicates.rs#native_predicates_reject_recursive_criss_cross_expansion
- crates/jj-store-cli/tests/native_lifecycle/query_predicates.rs#native_predicates_bound_recursive_tree_depth_for_graphs_and_content
- crates/jj-store-cli/tests/native_lifecycle/query_predicates.rs#native_predicates_admit_boundary_files_and_charge_repeated_tree_paths
- crates/jj-store-jj/src/backend.rs#native_file_reader_progress_does_not_require_foreign_executor_cooperative_reset
- crates/jj-store-jj/tests/rustfs.rs#real_rustfs_transport_and_native_blob_contract
- crates/jj-store-jj/tests/rustfs.rs#real_rustfs_manifest_publication_and_cold_recovery
- crates/jj-store-cli/tests/server_executable.rs#real_rustfs_server_cli_restart_and_revocation
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M4-011 details 2](details-02.md)
- [E-M4-011 details 3](details-03.md)
