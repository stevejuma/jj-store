---
schema: 1
id: E-M21-034
---

# E-M21-034

<!-- evidence-field:id markdown -->
## Id

E-M21-034
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

tvrnzuxv
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Original combination binding and ordered native completion acknowledgement
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; pinned JJ 0.45.1/Git SHA-1; macOS ARM64. Disposable real native/scanner and stock-JJ proof fixtures; CARGO_INCREMENTAL=0.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Private command journal version 6 adds absent-by-default scope-bound combination digest and bounded native completion references. Local execution version 1, combination snapshot/catalog version 1, plan catalog version 2, provider progress/completion version 1 and all public wire/native/companion/WAL formats retain their interpretation. Journal 1–5 absent-field serialization remains compatible; 32-KiB execution growth reservation is unchanged.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/stack_checkout_plans/coordination.rs
- crates/jj-store-cli/tests/stack_checkout_plans/coordination_process.rs
- crates/jj-store-cli/tests/stack_checkout_plans/coordination_unchanged.rs
- crates/jj-store-cli/tests/stack_checkout_plans.rs
- crates/jj-store-server/src/stack_combinations/linkage.rs
- crates/jj-store-server/src/stack_combinations/tests.rs
- crates/jj-store-server/src/stack_combinations.rs
- crates/jj-store-server/src/stack_command_journal/catalog.rs
- crates/jj-store-server/src/stack_command_journal/checkout.rs
- crates/jj-store-server/src/stack_command_journal/combination/tests.rs
- crates/jj-store-server/src/stack_command_journal/combination.rs
- crates/jj-store-server/src/stack_command_journal/execution.rs
- crates/jj-store-server/src/stack_command_journal.rs
- crates/jj-store-server/tests/stack_combinations.rs
- docs/implementation-plan.md
- docs/maintenance/coverage.json
- docs/site/technical/local.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/documentation.md
- spec/src/local-jj.md
- spec/src/verification.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-LOCAL-005
- JST-GOV-002
- JST-GOV-003
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Four legacy combination metadata cases pass. Baseline completes a journal checkout before selecting a combination, leaving a possible false-success interruption window. No exact staged binding or pending native evidence joins these boundaries. New metadata fixtures initially reused an occupied request slot (three failures); corrected identities and strict lint diagnostics are recorded and repaired.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Authenticate original journal before private store access. Preflight capacity, stage/sync exact provider after-state, save complete scope/principal/preview/retention-bound digest before publication. Retain sealed unchanged/tree native proofs as pending, select/sync only matching original combination under journal-first coordination and acknowledge final success last. Missing selected state cannot be recreated; public transitions and legacy completion cannot bypass a bound command; exact retired/completed histories do not select older state. Unpublished matching rejection remains discardable. New production command engines/routes are not enabled.
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/stack_checkout_plans/coordination.rs#native_completion_remains_pending_until_exact_combination_selection
- crates/jj-store-cli/tests/stack_checkout_plans/coordination.rs#changed_editor_files_and_missing_original_state_cannot_finish_a_pending_command
- crates/jj-store-cli/tests/stack_checkout_plans/coordination.rs#proof_linkage_and_cross_actor_checks_cannot_change_native_or_device_state
- crates/jj-store-cli/tests/stack_checkout_plans/coordination_process.rs#process_loss_after_native_evidence_and_final_ack_resumes_original_join
- crates/jj-store-cli/tests/stack_checkout_plans/coordination_process.rs#exit_after_join_boundary
- crates/jj-store-cli/tests/stack_checkout_plans/coordination_unchanged.rs#actual_reword_checkout_requires_combination_before_final_acknowledgement
- crates/jj-store-server/src/stack_command_journal/combination/tests.rs#loss_after_combination_selection_keeps_receipt_pending_and_exact_retry_finishes
- crates/jj-store-server/src/stack_command_journal/combination/tests.rs#exit_after_combination_before_receipt
- crates/jj-store-server/src/stack_command_journal/combination/tests.rs#final_receipt_sync_loss_is_unknown_and_replay_keeps_selected_combination
- crates/jj-store-server/src/stack_command_journal/combination/tests.rs#version_six_rejects_missing_binding_changed_proof_and_false_success
- crates/jj-store-server/tests/stack_combinations.rs#reads_never_initialize_and_selected_combinations_survive_reopen
- crates/jj-store-server/tests/stack_combinations.rs#wrong_principal_scope_intent_and_unretained_code_fail_without_selection
- crates/jj-store-server/tests/stack_combinations.rs#rejected_update_retains_original_outcome_without_initializing_setup
- crates/jj-store-server/tests/stack_combinations.rs#selected_metadata_and_retained_code_reopen_after_source_loss_and_gc
- crates/jj-store-server/tests/stack_combinations.rs#original_combination_binding_is_durable_but_never_a_success_receipt
- crates/jj-store-server/tests/stack_combinations.rs#interrupted_staging_resumes_original_binding_and_checks_actor_before_store
- crates/jj-store-server/tests/stack_combinations.rs#terminal_requests_cannot_gain_new_combination_bindings
- crates/jj-store-server/tests/stack_combinations.rs#bound_unpublished_rejection_discards_only_the_matching_pending_update
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M21-034 details 2](details-02.md)
- [E-M21-034 details 3](details-03.md)
- [E-M21-034 details 4](details-04.md)
