---
schema: 1
id: E-M17-075
---

# E-M17-075

<!-- evidence-field:id markdown -->
## Id

E-M17-075
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

opencode-composite-lifecycle-checkpoint-exact-recovery
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change lrznprmr
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-capture/src/codex_transcript/poll.rs
- crates/jj-store-capture/src/codex_transcript.rs
- crates/jj-store-capture/src/lib.rs
- crates/jj-store-capture/src/opencode_checkpoint.rs
- crates/jj-store-capture/src/opencode_ingress.rs
- crates/jj-store-capture/src/opencode_snapshot.rs
- crates/jj-store-capture/src/reader_progress.rs
- crates/jj-store-capture/tests/codex_context.rs
- crates/jj-store-capture/tests/codex_import/reader.rs
- crates/jj-store-capture/tests/codex_recovery.rs
- crates/jj-store-capture/tests/codex_transcript.rs
- crates/jj-store-capture/tests/opencode_checkpoint.rs
- crates/jj-store-capture/tests/reader_progress.rs
- crates/jj-store-capture/tests/reader_publication/mod.rs
- crates/jj-store-capture/tests/source_parking/reservations.rs
- crates/jj-store-capture/tests/transcript_sanitization.rs
- crates/jj-store-server/src/capture_supervisor/opencode/source.rs
- crates/jj-store-server/src/capture_supervisor/opencode/tests.rs
- crates/jj-store-server/src/opencode_source/conversion.rs
- crates/jj-store-server/src/opencode_source/ingress.rs
- crates/jj-store-server/src/opencode_source/lifecycle.rs
- crates/jj-store-server/src/opencode_source/recovery.rs
- crates/jj-store-server/src/opencode_source/snapshot.rs
- crates/jj-store-server/src/opencode_source.rs
- crates/jj-store-server/tests/opencode_lifecycle.rs
- crates/jj-store-server/tests/opencode_source/fixture.rs
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/verification.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-server/tests/opencode_lifecycle.rs#empty_active_session_publishes_idle_after_restart_without_inventing_messages
- crates/jj-store-server/tests/opencode_lifecycle.rs#queued_status_and_message_proofs_survive_current_idle_without_recap_or_identity_change
- crates/jj-store-server/tests/opencode_lifecycle.rs#message_only_history_upgrades_without_changing_retained_intent
- crates/jj-store-server/tests/opencode_lifecycle.rs#retry_and_repeated_status_keep_zero_message_progress_and_current_authority
- crates/jj-store-server/tests/opencode_lifecycle.rs#lifecycle_spool_pressure_never_acknowledges_state
- crates/jj-store-server/tests/opencode_lifecycle.rs#rewritten_message_prefix_retains_status_but_withholds_original_message
- crates/jj-store-server/tests/opencode_lifecycle.rs#returning_witness_recovers_original_generation_above_newer_allocation_floors
- crates/jj-store-capture/tests/opencode_checkpoint.rs#empty_lifecycle_advances_without_fictional_messages_and_survives_reopen
- crates/jj-store-capture/tests/opencode_checkpoint.rs#invalid_composite_cursor_context_or_duplicate_state_never_advances
- crates/jj-store-server/src/capture_supervisor/opencode/tests.rs#blocked_api_publication_retains_drain_ownership_while_ingress_continues
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M17-075 details 2](details-02.md)
- [E-M17-075 details 3](details-03.md)
