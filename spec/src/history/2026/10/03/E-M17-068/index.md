---
schema: 1
id: E-M17-068
---

# E-M17-068

<!-- evidence-field:id markdown -->
## Id

E-M17-068
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

accepted-import-supervised-restart-recovery
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-server/src/capture_management.rs
- crates/jj-store-server/src/capture_management/authority.rs
- crates/jj-store-server/src/capture_management/import_dispatch.rs
- crates/jj-store-server/src/capture_management/import_policy.rs
- crates/jj-store-server/src/capture_management/import_recovery.rs
- crates/jj-store-server/src/capture_management/import_renewal.rs
- crates/jj-store-server/src/capture_management/imports.rs
- crates/jj-store-server/src/capture_management/tests/scheduled_import/restart.rs
- crates/jj-store-server/src/capture_management/tests/scheduled_import/restart/recovery.rs
- crates/jj-store-server/src/capture_source/import.rs
- crates/jj-store-server/src/capture_source/import/parking.rs
- crates/jj-store-server/src/capture_source/import/pipeline.rs
- crates/jj-store-server/src/capture_supervisor.rs
- crates/jj-store-server/src/capture_supervisor/lifetime.rs
- crates/jj-store-server/src/capture_supervisor/recovery.rs
- crates/jj-store-server/src/capture_supervisor/scheduling.rs
- crates/jj-store-server/src/capture_supervisor/source/import.rs
- crates/jj-store-server/src/process/capture.rs
- crates/jj-store-server/src/process/capture/selection.rs
- crates/jj-store-server/src/process/tests/capture_factory.rs
- crates/jj-store-server/src/process/tests/capture_factory/import_recovery.rs
- spec/src/api.md
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/compatibility.md
- spec/src/milestones.md
- spec/src/verification.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-server/src/capture_management/tests/scheduled_import/restart.rs#accepted_import_resumes_tail_after_restart_without_a_second_command
- crates/jj-store-server/src/capture_management/tests/scheduled_import/restart.rs#original_selector_reconciles_after_new_runtime_without_resetting_selected_history
- crates/jj-store-server/src/capture_management/tests/scheduled_import/restart/recovery.rs#accepted_restart_preserves_durable_pause_and_recovers_after_resume
- crates/jj-store-server/src/capture_management/tests/scheduled_import/restart/recovery.rs#accepted_restart_renews_repository_binding_before_capturing_tail
- crates/jj-store-server/src/capture_management/tests/scheduled_import/restart/recovery.rs#accepted_restart_cannot_transfer_consent_to_a_replacement_physical_file
- crates/jj-store-server/src/capture_management/tests/scheduled_import/restart/recovery.rs#recovery_withholds_disabled_and_missing_state_without_repair_or_enrollment
- crates/jj-store-server/src/capture_management/tests/scheduled_import/restart/recovery.rs#paused_recovery_settles_while_management_holds_its_policy_gate
- crates/jj-store-server/src/process/tests/capture_factory.rs#cold_capture_factory_prepares_private_resources_and_retains_ordered_lifetime
- crates/jj-store-server/src/process/tests/capture_factory.rs#capture_factory_helper
- crates/jj-store-server/src/process/tests/capture_factory/import_recovery.rs#cold_prepared_foreground_recovers_selected_history_without_an_http_command
- crates/jj-store-server/src/process/tests/capture_factory/import_recovery.rs#import_recovery_factory_helper
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M17-068 details 2](details-02.md)
- [E-M17-068 details 3](details-03.md)
- [E-M17-068 details 4](details-04.md)
