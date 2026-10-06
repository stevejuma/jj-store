---
schema: 1
id: E-M17-061
---

# E-M17-061

<!-- evidence-field:id markdown -->
## Id

E-M17-061
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M17
- M17.2
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-03
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

owned-import-native-admission-settlement-gates
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-server/src/capture_control_state.rs
- crates/jj-store-server/src/capture_control_state/pinned.rs
- crates/jj-store-server/src/capture_import_state.rs
- crates/jj-store-server/src/capture_import_state/selected.rs
- crates/jj-store-server/src/capture_management.rs
- crates/jj-store-server/src/capture_management/import_policy.rs
- crates/jj-store-server/src/capture_management/import_settlement.rs
- crates/jj-store-server/src/capture_management/tests.rs
- crates/jj-store-server/src/capture_management/tests/import_policy.rs
- crates/jj-store-server/src/capture_management/tests/import_settlement.rs
- spec/src/collaboration.md
- spec/src/cli.md
- spec/src/verification.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-server/src/capture_management/tests/import_settlement.rs#owned_settlement_retains_policy_and_receipt_gates_across_native_authorization
- crates/jj-store-server/src/capture_management/tests/import_settlement.rs#selected_settlement_replay_ignores_offered_metadata_and_releases_exact_locks
- crates/jj-store-server/src/capture_management/tests/import_settlement.rs#preselection_pause_disable_and_physical_loss_refuse_owned_settlement
- crates/jj-store-server/src/capture_management/tests/import_settlement.rs#postselection_policy_runtime_and_selected_file_loss_preserve_original_receipt
- crates/jj-store-server/src/capture_management/tests/import_settlement.rs#caller_cancellation_does_not_release_settlement_gates_held_by_owned_admission
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M17-061 details 2](details-02.md)
- [E-M17-061 details 3](details-03.md)
