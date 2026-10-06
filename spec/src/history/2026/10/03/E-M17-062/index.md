---
schema: 1
id: E-M17-062
---

# E-M17-062

<!-- evidence-field:id markdown -->
## Id

E-M17-062
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

native-explicit-history-source-import
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-capture/src/codex_import/binding.rs
- crates/jj-store-server/src/capture_authority/publisher.rs
- crates/jj-store-server/src/capture_import_state/types.rs
- crates/jj-store-server/src/capture_management/import_renewal.rs
- crates/jj-store-server/src/capture_management/import_settlement.rs
- crates/jj-store-server/src/capture_management/tests/native_import/ownership.rs
- crates/jj-store-server/src/capture_management/tests/native_import.rs
- crates/jj-store-server/src/capture_management/tests.rs
- crates/jj-store-server/src/capture_management.rs
- crates/jj-store-server/src/capture_runtime/control.rs
- crates/jj-store-server/src/capture_runtime/sources.rs
- crates/jj-store-server/src/capture_runtime/targets.rs
- crates/jj-store-server/src/capture_runtime.rs
- crates/jj-store-server/src/capture_source/import/parking.rs
- crates/jj-store-server/src/capture_source/import/pipeline.rs
- crates/jj-store-server/src/capture_source/import.rs
- crates/jj-store-server/src/capture_source/ingress.rs
- crates/jj-store-server/src/capture_source/parking.rs
- crates/jj-store-server/src/capture_source/recovery.rs
- crates/jj-store-server/src/capture_source.rs
- crates/jj-store-server/src/capture_supervisor/sources.rs
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/milestones.md
- spec/src/verification.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-server/src/capture_management/tests/native_import/ownership.rs#parked_raw_import_retains_channel_and_recovers_exact_batch_after_native_refusal
- crates/jj-store-server/src/capture_management/tests/native_import/ownership.rs#cancelled_caller_cannot_release_owned_native_admission_gates_or_erase_selected_intent
- crates/jj-store-server/src/capture_management/tests/native_import.rs#selected_history_enrolls_after_native_write_and_publishes_sanitized_current_store
- crates/jj-store-server/src/capture_management/tests/native_import.rs#unavailable_native_write_withholds_intent_source_allocation_and_progress
- crates/jj-store-server/src/capture_management/tests/native_import.rs#import_replay_uses_original_binding_and_existing_cursor_after_append
- crates/jj-store-server/src/capture_management/tests/native_import.rs#imported_parking_renews_consent_and_preserves_progress_while_paused_or_unbound
- crates/jj-store-server/src/capture_management/tests/native_import.rs#rewritten_selected_prefix_and_physical_replacement_cannot_inherit_consent
- crates/jj-store-server/src/capture_management/tests/native_import.rs#runtime_or_installation_revocation_withholds_ingestion_and_exact_queued_recovery
- crates/jj-store-server/src/capture_management/tests/native_import.rs#native_import_preparation_remains_bounded_and_selects_last_complete_record
- crates/jj-store-server/src/capture_management/tests/native_import/ownership.rs#revocation_during_publisher_native_await_preserves_exact_prepared_intent_without_commit
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M17-062 details 2](details-02.md)
- [E-M17-062 details 3](details-03.md)
