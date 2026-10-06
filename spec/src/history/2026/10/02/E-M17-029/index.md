---
schema: 1
id: E-M17-029
---

# E-M17-029

<!-- evidence-field:id markdown -->
## Id

E-M17-029
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M17
- M17.2
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-02
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

implementation
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-capture/src/codex_resume.rs
- crates/jj-store-capture/src/codex_transcript/context.rs
- crates/jj-store-capture/tests/codex_resume.rs
- crates/jj-store-server/src/capture_source/parking/current.rs
- crates/jj-store-server/src/capture_source/parking.rs
- crates/jj-store-server/src/capture_source.rs
- crates/jj-store-server/src/capture_supervisor/source.rs
- crates/jj-store-server/tests/capture_reanchor/progress.rs
- crates/jj-store-server/tests/capture_reanchor/race.rs
- crates/jj-store-server/tests/capture_reanchor.rs
- crates/jj-store-server/tests/capture_supervisor.rs
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/verification.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-server/tests/capture_reanchor.rs#current_directory_handoff_preserves_the_exact_backpressured_batch_after_old_anchor_revocation
- crates/jj-store-server/tests/capture_reanchor.rs#partial_current_context_and_missing_write_do_not_acquire_or_advance
- crates/jj-store-server/tests/capture_reanchor.rs#bounded_preparation_keeps_an_unfinished_reader_prefix_verification
- crates/jj-store-server/tests/capture_reanchor.rs#failed_current_handoffs_keep_exact_frame_across_foreign_spool_and_source_replacement
- crates/jj-store-server/tests/capture_reanchor/race.rs#append_during_each_admission_boundary_releases_acquisition_and_retains_the_frame
- crates/jj-store-server/tests/capture_reanchor/progress.rs#missing_regressive_and_rebound_durable_state_cannot_authorize_current_handoff
- crates/jj-store-server/tests/capture_reanchor/progress.rs#advanced_current_cursor_reopens_instead_of_reusing_the_old_raw_batch
- crates/jj-store-server/tests/capture_supervisor.rs#live_parked_session_hands_off_to_b_while_exact_a_intent_remains_withheld
- crates/jj-store-capture/tests/codex_resume.rs#compact_eof_rebuilds_partial_context_after_append_and_same_length_rewrite
- crates/jj-store-capture/src/codex_transcript/context.rs#eof_compaction_releases_partial_allocation_and_keeps_complete_context
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M17-029 details 2](details-02.md)
- [E-M17-029 details 3](details-03.md)
