---
schema: 1
id: E-M17-057
---

# E-M17-057

<!-- evidence-field:id markdown -->
## Id

E-M17-057
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

private-durable-inactive-history-import-journal
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-server/src/capture_control_state/tests.rs
- crates/jj-store-server/src/capture_control_state.rs
- crates/jj-store-server/src/capture_import_state/catalog.rs
- crates/jj-store-server/src/capture_import_state/hex_codec.rs
- crates/jj-store-server/src/capture_import_state/tests.rs
- crates/jj-store-server/src/capture_import_state/types.rs
- crates/jj-store-server/src/capture_import_state.rs
- crates/jj-store-server/src/capture_journal_files/metadata.rs
- crates/jj-store-server/src/capture_journal_files/publication.rs
- crates/jj-store-server/src/capture_journal_files.rs
- crates/jj-store-server/src/lib.rs
- crates/jj-store-server/tests/capture_import_state/capacity.rs
- crates/jj-store-server/tests/capture_import_state/cold.rs
- crates/jj-store-server/tests/capture_import_state/safety.rs
- crates/jj-store-server/tests/capture_import_state.rs
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/verification.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:removed_files list -->
## Removed files

- crates/jj-store-server/src/capture_control_state/files.rs
- crates/jj-store-server/src/capture_control_state/files/metadata.rs
- crates/jj-store-server/src/capture_control_state/files/publication.rs
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-server/tests/capture_import_state.rs#import_receipt_survives_reopen_and_exact_replay_never_retargets_its_source
- crates/jj-store-server/tests/capture_import_state/capacity.rs#byte_capacity_preserves_exact_replay_and_refuses_new_intent_without_eviction
- crates/jj-store-server/tests/capture_import_state/capacity.rs#malformed_import_catalogs_fail_closed_without_repair
- crates/jj-store-server/tests/capture_import_state/cold.rs#import_receipt_survives_fresh_process_exit_and_exact_retry
- crates/jj-store-server/src/capture_import_state/tests.rs#lost_coordination_after_import_replacement_is_unknown_and_exact_receipt_recovers
- crates/jj-store-server/src/capture_import_state/tests.rs#import_callback_retains_gate_until_settlement
- crates/jj-store-server/src/capture_import_state/tests.rs#import_callback_rejects_byte_identical_selected_file_replacement
- crates/jj-store-server/tests/capture_control_state.rs#pause_receipts_survive_reopen_and_old_replay_cannot_reverse_newer_resume
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M17-057 details 2](details-02.md)
- [E-M17-057 details 3](details-03.md)
