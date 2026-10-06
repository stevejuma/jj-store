---
schema: 1
id: E-M17-067
---

# E-M17-067

<!-- evidence-field:id markdown -->
## Id

E-M17-067
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

accepted-import-progress-http-cli
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-capture/src/agent_capture.rs
- crates/jj-store-capture/src/agent_capture_binding.rs
- crates/jj-store-capture/src/codex_import.rs
- crates/jj-store-capture/src/codex_import/reader.rs
- crates/jj-store-capture/src/reader_publication.rs
- crates/jj-store-capture/src/reader_publication/files.rs
- crates/jj-store-capture/src/reader_publication/inspection.rs
- crates/jj-store-capture/tests/codex_import.rs
- crates/jj-store-capture/tests/codex_import/reader.rs
- crates/jj-store-capture/tests/reader_publication.rs
- crates/jj-store-capture/tests/reader_publication/inspection.rs
- crates/jj-store-cli/src/agent_commands/capture.rs
- crates/jj-store-cli/src/agent_commands/capture/imports.rs
- crates/jj-store-cli/src/agent_commands/capture/sources.rs
- crates/jj-store-cli/tests/capture_cli.rs
- crates/jj-store-cli/tests/capture_cli/fixture.rs
- crates/jj-store-cli/tests/capture_cli/imports.rs
- crates/jj-store-cli/tests/capture_import_cli.rs
- crates/jj-store-http/src/capture_management.rs
- crates/jj-store-http/src/capture_management/imports.rs
- crates/jj-store-server/src/capture_import_state/reader.rs
- crates/jj-store-server/src/capture_import_state/tests.rs
- crates/jj-store-server/src/capture_import_state/tests/reader.rs
- crates/jj-store-server/src/capture_import_state/tests/reader_verification.rs
- crates/jj-store-server/src/capture_management.rs
- crates/jj-store-server/src/capture_management/import_dispatch.rs
- crates/jj-store-server/src/capture_management/import_progress.rs
- crates/jj-store-server/src/capture_management/import_settlement.rs
- crates/jj-store-server/src/capture_management/imports.rs
- crates/jj-store-server/src/capture_management/tests.rs
- crates/jj-store-server/src/capture_management/tests/import_policy.rs
- crates/jj-store-server/src/capture_management/tests/native_import.rs
- crates/jj-store-server/src/capture_management/tests/native_import/prefix_progress.rs
- crates/jj-store-server/src/capture_management/tests/scheduled_import/restart.rs
- crates/jj-store-server/src/capture_source/import.rs
- crates/jj-store-server/src/capture_source/import/pipeline.rs
- crates/jj-store-server/src/capture_supervisor/sources.rs
- crates/jj-store-server/src/process/capture.rs
- crates/jj-store-server/src/process/capture/selection.rs
- crates/jj-store-server/tests/capture_management.rs
- crates/jj-store-server/tests/capture_management/imports.rs
- crates/jj-store-server/tests/capture_management/imports/inspection.rs
- crates/jj-store-service/src/capture_management.rs
- crates/jj-store-service/src/capture_management/imports.rs
- crates/jj-store-service/src/collaboration/models.rs
- schemas/openapi.json
- scripts/contracts.mjs
- spec/src/api.md
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/compatibility.md
- spec/src/milestones.md
- spec/src/mutations.md
- spec/src/spec-governance.md
- spec/src/verification.md
- web/src/lib/api/generated.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-capture/tests/codex_import.rs#inactive_preparation_selects_complete_history_without_native_enrollment_or_raw_output
- crates/jj-store-capture/tests/codex_import.rs#partial_or_ambiguous_context_grants_no_current_directory
- crates/jj-store-capture/tests/codex_import/reader.rs#accepted_probe_rejects_a_retained_cursor_from_different_bytes_of_the_same_file
- crates/jj-store-capture/tests/codex_import/reader.rs#cursor_verification_shares_prefix_budget_and_retains_selected_boundary_on_append
- crates/jj-store-capture/tests/codex_import/reader.rs#same_snapshot_reader_verification_extends_prefix_without_rescanning_earlier_bytes
- crates/jj-store-capture/tests/reader_publication.rs#later_repository_receipt_cannot_cover_an_earlier_withheld_range_after_restart
- crates/jj-store-capture/tests/reader_publication.rs#identity_only_completion_and_empty_queue_never_prove_publication
- crates/jj-store-capture/tests/reader_publication.rs#unknown_and_mismatched_receipts_keep_the_exact_prepared_retry_and_frontier
- crates/jj-store-capture/tests/reader_publication.rs#confirmed_evidence_precedes_body_removal_and_retry_does_not_count_it_twice
- crates/jj-store-capture/tests/reader_publication.rs#preexisting_or_lost_publication_history_is_not_promoted_by_a_later_receipt
- crates/jj-store-capture/tests/reader_publication.rs#corrupt_or_missing_selected_catalog_withholds_enqueue_and_is_never_repaired
- crates/jj-store-capture/tests/reader_publication.rs#selected_catalog_contains_no_body_path_or_native_session_key
- crates/jj-store-capture/tests/reader_publication/inspection.rs#accepted_boundary_survives_unknown_later_history
- crates/jj-store-capture/tests/reader_publication/inspection.rs#progress_inspection_does_not_open_pending_bodies
- crates/jj-store-capture/tests/reader_publication/inspection.rs#inspection_keeps_earlier_holes_and_unknown_history_unqualified
- crates/jj-store-capture/tests/reader_publication/inspection.rs#earlier_qualified_generation_proves_boundary_and_missing_tail_stays_separate
- crates/jj-store-capture/tests/reader_publication/inspection.rs#inspection_neither_initializes_locks_nor_recovers_temporary_catalog_files
- crates/jj-store-cli/tests/capture_cli.rs#disabled_capture_status_requires_no_repository_or_device_state
- crates/jj-store-cli/tests/capture_cli.rs#capture_controls_preserve_exact_receipt_and_current_policy
- crates/jj-store-cli/tests/capture_cli.rs#capture_inputs_cannot_initialize_state_or_select_actor
- crates/jj-store-cli/tests/capture_cli/imports.rs#import_cli_forwards_once_and_withholds_foreign_or_impossible_acknowledgements
- crates/jj-store-cli/tests/capture_cli/imports.rs#inspection_cli_binds_the_exact_request_and_rejects_unaccepted_completion
- crates/jj-store-cli/tests/capture_import_cli.rs#explicit_cli_import_inspects_published_history_and_replays_without_a_repository
- crates/jj-store-server/src/capture_import_state/tests/reader.rs#synchronized_reader_selection_refreshes_the_held_stamp_without_changing_its_receipt
- crates/jj-store-server/src/capture_import_state/tests/reader.rs#numeric_continuation_reuses_original_association_and_changed_reuse_keeps_selected_bytes
- crates/jj-store-server/src/capture_import_state/tests/reader.rs#duplicate_foreign_invalid_or_oversized_reader_catalogs_fail_without_repair
- crates/jj-store-server/src/capture_import_state/tests/reader.rs#original_reader_link_survives_process_exit_while_selection_gate_is_held
- crates/jj-store-server/src/capture_import_state/tests/reader_verification.rs#old_reader_link_needs_explicit_prefix_proof_and_upgrade_retains_original_generation
- crates/jj-store-server/src/capture_management/tests/import_policy.rs#import_policy_retains_management_and_control_gates_without_enrolling_history
- crates/jj-store-server/src/capture_management/tests/import_policy.rs#exact_import_receipt_precedes_new_revision_and_runtime_selector_rules
- crates/jj-store-server/src/capture_management/tests/import_policy.rs#paused_stopped_and_requested_pause_runtime_withhold_import_work_but_keep_receipts
- crates/jj-store-server/src/capture_management/tests/native_import.rs#selected_history_enrolls_after_native_write_and_publishes_sanitized_current_store
- crates/jj-store-server/src/capture_management/tests/native_import.rs#unavailable_native_write_withholds_intent_source_allocation_and_progress
- crates/jj-store-server/src/capture_management/tests/native_import.rs#import_replay_uses_original_binding_and_existing_cursor_after_append
- crates/jj-store-server/src/capture_management/tests/native_import.rs#imported_parking_renews_consent_and_preserves_progress_while_paused_or_unbound
- crates/jj-store-server/src/capture_management/tests/native_import.rs#rewritten_selected_prefix_and_physical_replacement_cannot_inherit_consent
- crates/jj-store-server/src/capture_management/tests/native_import.rs#runtime_or_installation_revocation_withholds_ingestion_and_exact_queued_recovery
- crates/jj-store-server/src/capture_management/tests/native_import.rs#native_import_preparation_remains_bounded_and_selects_last_complete_record
- crates/jj-store-server/src/capture_management/tests/native_import/prefix_progress.rs#fresh_import_cannot_reuse_published_reader_history_from_a_rewritten_prefix
- crates/jj-store-server/src/capture_management/tests/scheduled_import/restart.rs#original_selector_reconciles_after_new_runtime_without_resetting_selected_history
- crates/jj-store-server/tests/capture_management.rs#current_authority_and_configuration_withhold_without_resetting_policy
- crates/jj-store-server/tests/capture_management.rs#old_receipt_replay_applies_latest_policy_and_cannot_restart_a_stopped_owner
- crates/jj-store-server/tests/capture_management.rs#disabled_status_and_missing_journal_never_initialize_capture_state
- crates/jj-store-server/tests/capture_management/imports.rs#import_acceptance_progress_and_replay_are_actual_http_contracts
- crates/jj-store-server/tests/capture_management/imports/inspection.rs#missing_reader_link_and_unqualified_publication_withhold_completion_without_repair
- crates/jj-store-server/tests/capture_management/imports/inspection.rs#rebound_spool_and_missing_import_state_are_unavailable_without_new_owners_or_files
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M17-067 details 2](details-02.md)
- [E-M17-067 details 3](details-03.md)
- [E-M17-067 details 4](details-04.md)
