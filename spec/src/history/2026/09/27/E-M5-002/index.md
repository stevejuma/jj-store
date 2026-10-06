---
schema: 1
id: E-M5-002
---

# E-M5-002

<!-- evidence-field:id markdown -->
## Id

E-M5-002
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M5
- M5.1
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-27
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

durable-native-command-service
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-wal/src/publication.rs
- crates/jj-store-wal/src/publication/application.rs
- crates/jj-store-wal/tests/publication.rs
- crates/jj-store-wal/tests/publication/application.rs
- crates/jj-store-service/src/lib.rs
- crates/jj-store-service/src/mutations.rs
- crates/jj-store-jj/src/commands/mod.rs
- crates/jj-store-jj/src/query/mod.rs
- crates/jj-store-cli/tests/native_lifecycle.rs
- crates/jj-store-cli/tests/native_lifecycle/mutations.rs
- docs/evidence/m5-durable-commands.md
- spec/src/publication.md
- spec/src/mutations.md
- spec/src/jj-integration.md
- spec/src/cli.md
- spec/src/compatibility.md
- spec/src/verification.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-wal/tests/publication/application.rs#application_receipt_survives_regenerated_candidates_restart_and_checkpoint
- crates/jj-store-wal/tests/publication/application.rs#application_competing_candidates_return_only_the_manifest_selected_result
- crates/jj-store-wal/tests/publication/application.rs#application_unknown_outcomes_retain_identity_without_claiming_rejection
- crates/jj-store-wal/tests/publication/application.rs#application_identity_changes_conflict_and_native_digest_profile_is_unchanged
- crates/jj-store-wal/tests/publication/application.rs#application_guard_checks_staging_and_reauthorizes_receipt_replay
- crates/jj-store-wal/tests/publication/application.rs#application_receipt_lookup_rejects_selected_identity_position_and_index_corruption
- crates/jj-store-cli/tests/native_lifecycle/mutations.rs#typed_apply_returns_original_native_result_after_restart_and_later_publication
- crates/jj-store-cli/tests/native_lifecycle/mutations.rs#typed_mutations_reject_stale_intent_conflicts_readers_and_protected_previews
- crates/jj-store-cli/tests/native_lifecycle/mutations.rs#typed_mutation_status_is_principal_scoped_and_preview_does_not_authorize_apply
- crates/jj-store-cli/tests/native_lifecycle/mutations.rs#typed_identical_apply_races_return_one_result_during_preparation_or_manifest_cas
- crates/jj-store-cli/tests/native_lifecycle/mutations.rs#typed_apply_unknown_status_and_retry_recover_the_original_selected_result
- crates/jj-store-cli/tests/native_lifecycle/mutations.rs#typed_bookmark_receipts_replay_after_move_and_delete_without_resurrection
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M5-002 details 2](details-02.md)
- [E-M5-002 details 3](details-03.md)
