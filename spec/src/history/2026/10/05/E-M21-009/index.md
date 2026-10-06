---
schema: 1
id: E-M21-009
---

# E-M21-009

<!-- evidence-field:id markdown -->
## Id

E-M21-009
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M21
- M21.2
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-05
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

sxvmtmun
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Private current checkout/sparse/ignore observation integration
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; pinned JJ 0.45.1 library and explicit existing development executable; pnpm 10.32.1; macOS ARM64. Documentation source-derived example build only; no installed executable/service replacement.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No HTTP/RPC/native/WAL/companion/private persisted format change. Private current witness uses bounded upstream existing checkout protobuf, exact ordered WC tree terms/labels and sparse prefixes; framed metadata/policy/directory/head fingerprint. No new wire route, content token or command capability.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- Cargo.lock
- crates/jj-store-jj/Cargo.toml
- crates/jj-store-jj/src/local_browser/working_tree/current/capture.rs
- crates/jj-store-jj/src/local_browser/working_tree/current/decode.rs
- crates/jj-store-jj/src/local_browser/working_tree/current/physical.rs
- crates/jj-store-jj/src/local_browser/working_tree/current.rs
- crates/jj-store-jj/src/local_browser/working_tree/ignores.rs
- crates/jj-store-jj/src/local_browser/working_tree.rs
- crates/jj-store-jj/tests/working_tree_current.rs
- crates/jj-store-service/src/working_tree/mod.rs
- crates/jj-store-service/src/working_tree/observer.rs
- crates/jj-store-service/src/working_tree/projection.rs
- crates/jj-store-service/tests/working_tree_observer.rs
- crates/jj-store-service/tests/working_tree_projection.rs
- docs/implementation-plan.md
- docs/maintenance/coverage.json
- docs/site/technical/architecture.md
- docs/site/technical/local.md
- spec/src/architecture.md
- spec/src/documentation.md
- spec/src/jj-integration.md
- spec/src/local-jj.md
- spec/src/mutations.md
- spec/src/ui.md
- spec/src/verification.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

No existing-checkout/sparse/actual-global-exclude witness or integrated service observation existed; historical WC baseline alone could not prove current physical state. Exact current target initially failed with 11 missing API/type diagnostics.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Private descriptor-relative current WC and policy capture plus repeated metadata/head proof around two-pass physical scan; sparse-aware projection; eight-second observation deadline; bounded non-queuing admission retained until a canceled scan worker exits. 37 distinct focused regressions pass, including five explicit fresh real CLI fixtures.
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-jj/tests/working_tree_current.rs#current_read_proves_checkout_tree_sparse_and_captured_base_policy_without_writes
- crates/jj-store-jj/tests/working_tree_current.rs#current_read_rejects_stale_missing_unsafe_and_unbounded_checkout_state
- crates/jj-store-jj/tests/working_tree_current.rs#current_read_preserves_sparse_scope_and_detects_ignore_policy_edits
- crates/jj-store-jj/tests/working_tree_current.rs#current_read_rejects_metadata_and_policy_races_and_revoked_access
- crates/jj-store-jj/tests/working_tree_current.rs#current_read_bounds_repeated_state_and_rejects_conflict_label_mismatch
- crates/jj-store-service/tests/working_tree_observer.rs#observation_reads_current_dirty_files_and_complete_ignore_policy_without_writes
- crates/jj-store-service/tests/working_tree_observer.rs#observation_respects_real_sparse_checkout_and_outside_obstacles
- crates/jj-store-service/tests/working_tree_observer.rs#observation_rejects_changed_policy_metadata_and_revoked_access_during_scan
- crates/jj-store-service/tests/working_tree_observer.rs#observation_cancellation_retains_admission_until_scan_worker_exits
- crates/jj-store-service/tests/working_tree_projection.rs#sparse_projection_keeps_excluded_paths_as_obstacles_without_false_changes
- crates/jj-store-service/tests/working_tree_local.rs#real_folder_comparison_preserves_operations_metadata_and_unsnapshotted_files
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M21-009 details 2](details-02.md)
- [E-M21-009 details 3](details-03.md)
