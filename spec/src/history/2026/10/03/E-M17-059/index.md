---
schema: 1
id: E-M17-059
---

# E-M17-059

<!-- evidence-field:id markdown -->
## Id

E-M17-059
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

current-installation-policy-import-guard
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-server/src/capture_control_state.rs
- crates/jj-store-server/src/capture_import_state/types.rs
- crates/jj-store-server/src/capture_import_state.rs
- crates/jj-store-server/src/capture_journal_files.rs
- crates/jj-store-server/src/capture_management/authority.rs
- crates/jj-store-server/src/capture_management/import_policy.rs
- crates/jj-store-server/src/capture_management/tests/import_policy/safety.rs
- crates/jj-store-server/src/capture_management/tests/import_policy.rs
- crates/jj-store-server/src/capture_management/tests.rs
- crates/jj-store-server/src/capture_management.rs
- crates/jj-store-server/src/capture_runtime/control.rs
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/verification.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-server/src/capture_management/tests/import_policy.rs#import_policy_retains_management_and_control_gates_without_enrolling_history
- crates/jj-store-server/src/capture_management/tests/import_policy.rs#exact_import_receipt_precedes_new_revision_and_runtime_selector_rules
- crates/jj-store-server/src/capture_management/tests/import_policy.rs#paused_stopped_and_requested_pause_runtime_withhold_import_work_but_keep_receipts
- crates/jj-store-server/src/capture_management/tests/import_policy/safety.rs#disabled_denied_and_foreign_import_guards_read_no_unselected_capture_state
- crates/jj-store-server/src/capture_management/tests/import_policy/safety.rs#byte_identical_control_file_replacement_before_application_withholds_import
- crates/jj-store-server/src/capture_management/tests/import_policy/safety.rs#post_application_authority_loss_is_unknown_and_exact_selected_intent_survives
- crates/jj-store-server/src/capture_management/tests/import_policy/safety.rs#caller_cancellation_retains_import_policy_and_selection_gates_until_settlement
- crates/jj-store-server/src/capture_runtime/control.rs#requested_pause_withholds_import_even_before_cached_observation_changes
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M17-059 details 2](details-02.md)
- [E-M17-059 details 3](details-03.md)
