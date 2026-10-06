---
schema: 1
id: E-M17-066
---

# E-M17-066

<!-- evidence-field:id markdown -->
## Id

E-M17-066
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

original-import-receipt-to-reader-association
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-capture/src/codex_import/binding.rs
- crates/jj-store-capture/tests/codex_import/identity.rs
- crates/jj-store-server/src/capture_import_state.rs
- crates/jj-store-server/src/capture_import_state/catalog.rs
- crates/jj-store-server/src/capture_import_state/reader.rs
- crates/jj-store-server/src/capture_import_state/selected.rs
- crates/jj-store-server/src/capture_import_state/tests.rs
- crates/jj-store-server/src/capture_import_state/tests/reader.rs
- crates/jj-store-server/src/capture_management/import_settlement.rs
- crates/jj-store-server/src/capture_management/tests/native_import.rs
- crates/jj-store-server/src/capture_management/tests/native_import/reader_binding.rs
- crates/jj-store-server/src/capture_source/import.rs
- crates/jj-store-server/src/capture_source/import/pipeline.rs
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/milestones.md
- spec/src/verification.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-capture/tests/codex_import/identity.rs#accepted_header_digest_matches_only_its_exact_reader_generation_witness
- crates/jj-store-capture/tests/codex_import/identity.rs#binding_rejects_byte_identical_physical_replacement_and_changed_header_or_profile
- crates/jj-store-capture/tests/codex_import/identity.rs#qualified_desktop_surface_has_the_same_complete_record_preparation_bounds
- crates/jj-store-capture/tests/codex_import/identity.rs#accepted_binding_reconciles_in_a_fresh_process_without_retargeting_appended_history
- crates/jj-store-server/src/capture_import_state/tests/reader.rs#synchronized_reader_selection_refreshes_the_held_stamp_without_changing_its_receipt
- crates/jj-store-server/src/capture_import_state/tests/reader.rs#numeric_continuation_reuses_original_association_and_changed_reuse_keeps_selected_bytes
- crates/jj-store-server/src/capture_import_state/tests/reader.rs#duplicate_foreign_invalid_or_oversized_reader_catalogs_fail_without_repair
- crates/jj-store-server/src/capture_import_state/tests/reader.rs#original_reader_link_survives_process_exit_while_selection_gate_is_held
- crates/jj-store-server/src/capture_management/tests/native_import.rs#selected_history_enrolls_after_native_write_and_publishes_sanitized_current_store
- crates/jj-store-server/src/capture_management/tests/native_import.rs#unavailable_native_write_withholds_intent_source_allocation_and_progress
- crates/jj-store-server/src/capture_management/tests/native_import.rs#import_replay_uses_original_binding_and_existing_cursor_after_append
- crates/jj-store-server/src/capture_management/tests/native_import.rs#imported_parking_renews_consent_and_preserves_progress_while_paused_or_unbound
- crates/jj-store-server/src/capture_management/tests/native_import.rs#rewritten_selected_prefix_and_physical_replacement_cannot_inherit_consent
- crates/jj-store-server/src/capture_management/tests/native_import.rs#runtime_or_installation_revocation_withholds_ingestion_and_exact_queued_recovery
- crates/jj-store-server/src/capture_management/tests/native_import.rs#native_import_preparation_remains_bounded_and_selects_last_complete_record
- crates/jj-store-server/src/capture_management/tests/native_import/reader_binding.rs#changed_reader_association_withholds_body_work_and_cannot_retarget_original_intent
- crates/jj-store-server/src/capture_management/tests/native_import/reader_binding.rs#accepted_import_selects_a_durable_reader_association_before_returning_its_pipeline
- crates/jj-store-server/src/capture_management/tests/native_import/reader_binding.rs#exact_reader_association_survives_reopen_tail_and_absent_transcript_lookup
- crates/jj-store-server/src/capture_management/tests/native_import/reader_binding.rs#receipt_only_import_is_unqualified_until_original_source_admission_reconstructs_reader_link
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M17-066 details 2](details-02.md)
- [E-M17-066 details 3](details-03.md)
