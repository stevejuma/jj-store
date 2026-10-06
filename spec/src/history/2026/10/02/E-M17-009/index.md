---
schema: 1
id: E-M17-009
---

# E-M17-009

<!-- evidence-field:id markdown -->
## Id

E-M17-009
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

sanitized-native-codex-projection
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-agent/src/normalize.rs
- crates/jj-agent/src/normalizers/codex.rs
- crates/jj-agent/src/normalizers/codex_transcript.rs
- crates/jj-agent/src/normalizers/mod.rs
- crates/jj-agent/tests/codex_transcript.rs
- crates/jj-store-capture/src/agent_sanitization.rs
- crates/jj-store-capture/src/codex_transcript/file.rs
- crates/jj-store-capture/src/codex_transcript.rs
- crates/jj-store-capture/src/lib.rs
- crates/jj-store-capture/src/transcript_sanitization.rs
- crates/jj-store-capture/tests/transcript_sanitization.rs
- crates/jj-store-wal/tests/agent_catalog.rs
- spec/src/collaboration.md
- spec/src/compatibility.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-agent/tests/codex_transcript.rs#native_codex_transcripts_project_messages_and_lifecycle_without_inventing_execution
- crates/jj-agent/tests/codex_transcript.rs#metadata_unknown_records_and_unmatched_results_do_not_invent_activity
- crates/jj-agent/tests/codex_transcript.rs#native_message_identity_survives_updates_and_missing_ids_use_source_positions
- crates/jj-agent/tests/codex_transcript.rs#desktop_native_profile_does_not_authorize_legacy_hooks_or_untrusted_envelopes
- crates/jj-agent/tests/codex_transcript.rs#missing_message_ids_cannot_collide_with_real_native_aliases
- crates/jj-agent/tests/codex_transcript.rs#unrelated_hook_metadata_preserves_legacy_codex_interpretation
- crates/jj-store-capture/tests/transcript_sanitization.rs#transcript_sanitization_precedes_durability_and_covers_path_keys_and_values
- crates/jj-store-capture/tests/transcript_sanitization.rs#omissions_key_collisions_and_invalid_identity_retain_safe_source_progress
- crates/jj-store-capture/tests/transcript_sanitization.rs#ambiguous_privacy_roots_fail_before_cache_or_state_creation
- crates/jj-store-wal/tests/agent_catalog.rs#native_desktop_messages_publish_sanitized_evidence_without_asserting_execution
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M17-009 details 2](details-02.md)
- [E-M17-009 details 3](details-03.md)
