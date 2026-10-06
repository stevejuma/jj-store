---
schema: 1
id: E-M17-063
---

# E-M17-063

<!-- evidence-field:id markdown -->
## Id

E-M17-063
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

retained-explicit-history-import-scheduling
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-server/src/capture_management.rs
- crates/jj-store-server/src/capture_management/import_dispatch.rs
- crates/jj-store-server/src/capture_management/import_renewal.rs
- crates/jj-store-server/src/capture_management/tests.rs
- crates/jj-store-server/src/capture_management/tests/scheduled_import.rs
- crates/jj-store-server/src/capture_management/tests/scheduled_import/handoff.rs
- crates/jj-store-server/src/capture_management/tests/scheduled_import/restart.rs
- crates/jj-store-server/src/capture_runtime.rs
- crates/jj-store-server/src/capture_runtime/control.rs
- crates/jj-store-server/src/capture_runtime/imports.rs
- crates/jj-store-server/src/capture_runtime/sources.rs
- crates/jj-store-server/src/capture_source/import.rs
- crates/jj-store-server/src/capture_source/import/parking.rs
- crates/jj-store-server/src/capture_source/import/pipeline.rs
- crates/jj-store-server/src/capture_supervisor.rs
- crates/jj-store-server/src/capture_supervisor/delivery.rs
- crates/jj-store-server/src/capture_supervisor/imports.rs
- crates/jj-store-server/src/capture_supervisor/scheduling.rs
- crates/jj-store-server/src/capture_supervisor/source.rs
- crates/jj-store-server/src/capture_supervisor/source/import.rs
- crates/jj-store-server/src/capture_supervisor/source/tests.rs
- crates/jj-store-server/src/capture_worker/executor.rs
- crates/jj-store-server/src/capture_worker/executor/tests.rs
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/milestones.md
- spec/src/verification.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-server/src/capture_management/tests/scheduled_import/handoff.rs#import_waits_for_existing_raw_active_reader_and_keeps_the_same_session_identity
- crates/jj-store-server/src/capture_management/tests/scheduled_import/handoff.rs#compact_active_handoff_reuses_cursor_alias_and_publishes_one_native_session
- crates/jj-store-server/src/capture_management/tests/scheduled_import/restart.rs#original_selector_reconciles_after_new_runtime_without_resetting_selected_history
- crates/jj-store-server/src/capture_management/tests/scheduled_import.rs#retained_runtime_import_publishes_inactive_history_with_existing_source_budgets
- crates/jj-store-server/src/capture_management/tests/scheduled_import.rs#scheduled_import_pause_replay_and_tail_preserve_original_receipt_and_cursor
- crates/jj-store-server/src/capture_management/tests/scheduled_import.rs#lost_caller_and_full_mailbox_keep_one_original_command_without_body_ownership
- crates/jj-store-server/src/capture_management/tests/scheduled_import.rs#accepted_source_refuses_other_request_without_replacing_original_owned_work
- crates/jj-store-server/src/capture_management/tests/scheduled_import.rs#multi_step_import_prefix_survives_worker_parking_without_starving_ingress
- crates/jj-store-server/src/capture_management/tests/scheduled_import.rs#queued_dispatch_renews_revoked_installation_before_enrollment_or_body_work
- crates/jj-store-server/src/capture_management/tests.rs#caller_cancellation_retains_journal_gate_and_operation_until_owned_application_finishes
- crates/jj-store-server/src/capture_management/tests.rs#revocation_after_selection_returns_unknown_and_exact_retry_recovers_receipt
- crates/jj-store-server/src/capture_management/tests.rs#application_task_panic_preserves_selected_receipt_for_exact_replay
- crates/jj-store-server/src/capture_management/tests.rs#policy_directory_rebinding_after_selection_withholds_application_until_exact_replay
- crates/jj-store-server/src/capture_supervisor/source/tests.rs#cancelled_live_raw_state_overrides_older_compact_snapshot_before_import_handoff
- crates/jj-store-server/src/capture_worker/executor/tests.rs#settlement_hook_follows_actual_cancelled_future_and_keeps_original_state
- crates/jj-store-server/src/capture_worker/executor/tests.rs#blocking_io_deadline_retains_owner_and_keeps_runtime_responsive
- crates/jj-store-server/src/capture_worker/executor/tests.rs#pause_resume_cannot_replace_an_undrained_owner
- crates/jj-store-server/src/capture_worker/executor/tests.rs#shutdown_timeout_retains_owner_until_settlement
- crates/jj-store-server/src/capture_worker/executor/tests.rs#completed_late_result_remains_known
- crates/jj-store-server/src/capture_worker/executor/tests.rs#task_loss_closes_the_owner_without_replacement
- crates/jj-store-server/src/capture_worker/executor/tests.rs#dropped_controller_keeps_running_resources_until_settlement
- crates/jj-store-server/src/capture_management/tests/native_import.rs#rewritten_selected_prefix_and_physical_replacement_cannot_inherit_consent
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M17-063 details 2](details-02.md)
- [E-M17-063 details 3](details-03.md)
- [E-M17-063 details 4](details-04.md)
