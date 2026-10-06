---
schema: 1
id: E-M21-018
---

# E-M21-018

<!-- evidence-field:id markdown -->
## Id

E-M21-018
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

vkkmyrqn
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Exact real local preparation preview/journal/witness/retention composition
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; JJ 0.45.1; Git 2.55.0; macOS ARM64 isolated stock-JJ fixtures; installed executables/services unchanged.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Private catalog version 2 with version-1 LocalExecutionV1. Legacy catalog version 1 remains byte-compatible and immutable. Pending full OperationArchiveV1 is selected with the exact server preview, recovery witness/digest and actual principal/request/intent/prepared retention key before reserved ref selection; acknowledgement keeps identical commit/archive digest and removes only the pending body. Existing 128-record/8-MiB limits include pending archives and receipt-growth reservation. Reserved ref V1/V2, companion/native/WAL/manifest and public transport formats unchanged.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-jj/src/local_execution/mod.rs
- crates/jj-store-jj/src/local_execution/recovery/retention.rs
- crates/jj-store-jj/src/local_execution/recovery/witness.rs
- crates/jj-store-jj/src/local_execution/recovery.rs
- crates/jj-store-server/src/lib.rs
- crates/jj-store-server/src/stack_command_journal/catalog.rs
- crates/jj-store-server/src/stack_command_journal/execution.rs
- crates/jj-store-server/src/stack_command_journal/transition.rs
- crates/jj-store-server/src/stack_command_journal.rs
- crates/jj-store-server/src/stack_preparation/report.rs
- crates/jj-store-server/src/stack_preparation.rs
- crates/jj-store-server/tests/stack_command_journal/admission.rs
- crates/jj-store-server/tests/stack_local_preparation/admission.rs
- crates/jj-store-server/tests/stack_local_preparation/lifecycle.rs
- crates/jj-store-server/tests/stack_local_preparation/persisted.rs
- crates/jj-store-server/tests/stack_local_preparation.rs
- docs/implementation-plan.md
- docs/maintenance/coverage.json
- docs/site/technical/lifecycle.md
- docs/site/technical/local.md
- spec/src/architecture.md
- spec/src/documentation.md
- spec/src/jj-integration.md
- spec/src/local-jj.md
- spec/src/verification.md
- spec/src/milestones.md
- spec/src/cli.md
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

stack-local-preparation-before.log records one missing stack_preparation import. Initial two-case implementation passes; expansion found one admission assertion failure without a captured error. Strengthened diagnostic assertion and later combination precondition retain the same stale contract; exact and full targets pass. Bounded repetitions recorded separately to investigate that initial intermittent failure.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

One private local preparation boundary across the existing server journal and JJ foreground retainer. Actual unpublished candidates and exact previews, private pending archives, original-request lookup before live context, immutable legacy slots, saved-archive retention/replay without source restoration, cancellation and real subprocess loss. No transport or writable production capability.
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-server/tests/stack_local_preparation.rs#real_preparation_joins_exact_preview_retention_and_private_recovery_witness
- crates/jj-store-server/tests/stack_local_preparation.rs#exact_preparation_replay_survives_source_loss_without_live_validation_or_reprepare
- crates/jj-store-server/tests/stack_local_preparation/admission.rs#authorization_stale_physical_selection_and_conflicting_identity_do_not_prepare
- crates/jj-store-server/tests/stack_local_preparation/admission.rs#local_execution_is_immutable_legacy_cannot_be_upgraded_and_terminal_replay_stays_terminal
- crates/jj-store-server/tests/stack_local_preparation/lifecycle.rs#cancel_after_pending_selection_releases_lease_and_resumes_exact_archive
- crates/jj-store-server/tests/stack_local_preparation/lifecycle.rs#process_loss_before_retention_replays_saved_candidate_after_source_loss_and_gc
- crates/jj-store-server/tests/stack_local_preparation/lifecycle.rs#process_loss_after_retention_replays_original_acknowledgement_after_source_loss_and_gc
- crates/jj-store-server/tests/stack_local_preparation/persisted.rs#malformed_saved_execution_is_rejected_without_repair_or_source_writes
- crates/jj-store-server/src/stack_command_journal/tests.rs#lost_coordination_after_atomic_replacement_is_unknown_and_replay_syncs_original
- crates/jj-store-server/tests/stack_command_journal.rs#retained_preparation_survives_reopen_and_conflicting_request_reuse
- crates/jj-store-jj/tests/stack_source_recovery.rs#restart_restores_exact_retained_history_without_heads_checkout_or_index_changes
- crates/jj-store-jj/tests/stack_preparation_retention.rs#retained_unpublished_stack_operation_survives_gc_and_source_state_loss
- crates/jj-store-jj/tests/stack_reword_preparation.rs#exact_reword_preparation_retains_change_tree_and_dirty_checkout_without_publication
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M21-018 details 2](details-02.md)
- [E-M21-018 details 3](details-03.md)
