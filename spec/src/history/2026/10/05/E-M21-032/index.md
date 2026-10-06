---
schema: 1
id: E-M21-032
---

# E-M21-032

<!-- evidence-field:id markdown -->
## Id

E-M21-032
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

qvlnowzo
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Actual retained physical file installation with provider-owned interrupted progress
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; pinned JJ 0.45.1/Git SHA-1 profile; macOS ARM64, Rustix 1.1.5 native no-replace rename. Isolated real-native/scanner folders and private reword runtime fixtures; CARGO_INCREMENTAL=0. No installed binary/service replacement.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

New installation-private canonical progress version 1, 32-MiB bound, exact plan/inventory/private-directory identities and synced pending transition. Existing plan catalog version 2 and command journal version 5 unchanged. Original and maximum intermediate namespace remain within 4097 records. Original native checkout/tree-state, public wire, companion, WAL and durable combination formats unchanged.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/stack_checkout_plans/candidate.rs
- crates/jj-store-cli/tests/stack_checkout_plans/installation.rs
- crates/jj-store-cli/tests/stack_checkout_plans/installation_faults.rs
- crates/jj-store-cli/tests/stack_checkout_plans/installation_process.rs
- crates/jj-store-cli/tests/stack_checkout_plans.rs
- crates/jj-store-jj/src/local_execution/mod.rs
- crates/jj-store-jj/src/local_execution/recovery/checkout.rs
- crates/jj-store-jj/src/local_execution/recovery/tree_checkout/effects.rs
- crates/jj-store-jj/src/local_execution/recovery/tree_checkout/installation/files.rs
- crates/jj-store-jj/src/local_execution/recovery/tree_checkout/installation/progress.rs
- crates/jj-store-jj/src/local_execution/recovery/tree_checkout/installation/storage.rs
- crates/jj-store-jj/src/local_execution/recovery/tree_checkout/installation/validation.rs
- crates/jj-store-jj/src/local_execution/recovery/tree_checkout/installation.rs
- crates/jj-store-jj/src/local_execution/recovery/tree_checkout/plan.rs
- crates/jj-store-jj/src/local_execution/recovery/tree_checkout.rs
- crates/jj-store-jj/src/local_execution/recovery.rs
- crates/jj-store-service/src/working_tree/guard.rs
- crates/jj-store-service/tests/working_tree_inventory/guard.rs
- docs/implementation-plan.md
- docs/maintenance/coverage.json
- docs/site/technical/architecture.md
- docs/site/technical/local.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/documentation.md
- spec/src/jj-integration.md
- spec/src/local-jj.md
- spec/src/verification.md
- spec/src/milestones.md
- spec/src/mutations.md
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-LOCAL-005
- JST-JJ-002
- JST-JJ-004
- JST-GOV-002
- JST-GOV-003
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

The exact requested real installation test fails with a missing provider method. Initial implementation passes actual physical effects. Three provider/one test collapsible-if lint diagnostics and one platform RawMode compilation mismatch are repaired; final exact/focused tests and strict lint pass.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Explicit provider file effects only after sealed exact retained publication. Verify original metadata/effects/access; prepare candidate files/new parents, persist intent before atomic no-replace moves, retain originals as private backups, validate complete fresh inventory with restricted exact transition normalization. Replay checks known inodes/content/modes/mtime, handles partial staging and metadata acknowledgement without rewriting identity, and never reconstructs missing completed progress from current files. Cancellation retains foreground/admission until worker exit. No native checkout/tree-state update, command acknowledgement, combination selection or writable transport follows.
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/stack_checkout_plans/installation.rs#retained_tree_files_install_with_durable_progress_and_unchanged_native_metadata
- crates/jj-store-cli/tests/stack_checkout_plans/installation.rs#installed_tree_replay_retains_inodes_and_all_original_backups
- crates/jj-store-cli/tests/stack_checkout_plans/installation.rs#binary_executable_additions_deletions_and_nested_parents_preserve_other_files
- crates/jj-store-cli/tests/stack_checkout_plans/installation.rs#editor_created_target_after_backup_is_never_replaced
- crates/jj-store-cli/tests/stack_checkout_plans/installation.rs#changed_ignored_file_after_move_blocks_with_backups_and_native_state_retained
- crates/jj-store-cli/tests/stack_checkout_plans/installation_faults.rs#replaced_staging_files_and_private_directory_fail_before_physical_changes
- crates/jj-store-cli/tests/stack_checkout_plans/installation_faults.rs#replaced_parent_backup_symlink_and_changed_mode_never_acknowledge_installation
- crates/jj-store-cli/tests/stack_checkout_plans/installation_faults.rs#interrupted_initial_progress_prefix_reuses_exact_staged_file_identities
- crates/jj-store-cli/tests/stack_checkout_plans/installation_faults.rs#partial_candidate_prefix_is_resumed_but_corrupt_prefix_is_preserved
- crates/jj-store-cli/tests/stack_checkout_plans/installation_faults.rs#missing_completed_progress_never_reconstructs_an_installation_from_new_files
- crates/jj-store-cli/tests/stack_checkout_plans/installation_faults.rs#authorization_loss_between_moves_retains_pending_progress_and_backups
- crates/jj-store-cli/tests/stack_checkout_plans/installation_faults.rs#canceled_worker_keeps_provider_admission_and_foreground_lock_until_exit
- crates/jj-store-cli/tests/stack_checkout_plans/installation_process.rs#abrupt_loss_before_and_after_every_physical_move_replays_original_input
- crates/jj-store-cli/tests/stack_checkout_plans/installation_process.rs#partial_move_acknowledgement_replays_only_exact_saved_next_prefix
- crates/jj-store-cli/tests/stack_checkout_plans/installation_process.rs#exit_at_physical_boundary
- crates/jj-store-service/tests/working_tree_inventory/guard.rs#registered_access_guard_captures_complete_inventory_and_preserves_validation
- crates/jj-store-service/tests/working_tree_inventory/guard.rs#revoked_and_replaced_binding_cannot_capture_or_initialize_inventory
- crates/jj-store-service/tests/working_tree_inventory/guard.rs#validation_only_guard_does_not_gain_inventory_capture
- crates/jj-store-service/tests/working_tree_inventory/guard.rs#fresh_observation_preserves_scope_authorization_and_original_validation
- crates/jj-store-service/tests/working_tree_inventory/guard.rs#inventory_capture_cancellation_retains_shared_admission_until_worker_exits
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M21-032 details 2](details-02.md)
- [E-M21-032 details 3](details-03.md)
- [E-M21-032 details 4](details-04.md)
