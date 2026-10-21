---
schema: 1
id: E-HISTORY-SERVICE-001
---

# E-HISTORY-SERVICE-001 details 2

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M33 and M33.4 stay In progress. Library extraction alone does not
  satisfy managed conversation persistence, downstream native or hosted browser
  gates.

```
<!-- /evidence-field -->

<!-- evidence-field:removed_files list -->
## Removed files

- crates/tauri-plugin-jj-store/src/annotations.rs
- crates/tauri-plugin-jj-store/src/file_history.rs
- crates/tauri-plugin-jj-store/src/frames.rs
- crates/tauri-plugin-jj-store/src/jobs.rs
- crates/tauri-plugin-jj-store/src/policy.rs
- crates/tauri-plugin-jj-store/src/scopes.rs
- crates/tauri-plugin-jj-store/src/service.rs
- crates/tauri-plugin-jj-store/src/wire.rs
- crates/tauri-plugin-jj-store/tests/admission.rs
- crates/tauri-plugin-jj-store/tests/blame/mod.rs
- crates/tauri-plugin-jj-store/tests/file_history/mod.rs
- crates/tauri-plugin-jj-store/tests/frames.rs
- crates/tauri-plugin-jj-store/tests/operations/mod.rs
- crates/tauri-plugin-jj-store/tests/wire.rs
<!-- /evidence-field -->

<!-- evidence-field:removed_tests list -->
## Removed tests

- crates/tauri-plugin-jj-store/tests/admission.rs#handles_bind_window_vault_origin_generation_and_exact_operation
- crates/tauri-plugin-jj-store/tests/admission.rs#close_cancels_and_waits_for_actual_preparation_exit
- crates/tauri-plugin-jj-store/tests/admission.rs#revoke_waits_for_inflight_reads_and_denies_their_completion
- crates/tauri-plugin-jj-store/tests/admission.rs#revocation_preserves_other_windows_and_failed_preparation_is_observable
- crates/tauri-plugin-jj-store/src/transport/commands/tests.rs#renderer_frames_cannot_supply_native_authority
- crates/tauri-plugin-jj-store/tests/frames.rs#renderer_frames_cannot_supply_native_authority
- crates/tauri-plugin-jj-store/tests/wire.rs#text_envelopes_preserve_full_width_integers_unicode_and_exact_escaping
- crates/tauri-plugin-jj-store/tests/wire.rs#envelope_version_shape_and_byte_limits_fail_closed
- crates/tauri-plugin-jj-store/tests/blame/mod.rs#blame_matches_jj_and_retains_pin_scope_and_source_bytes
- crates/tauri-plugin-jj-store/tests/blame/mod.rs#blame_close_waits_for_the_owned_read_to_exit
- crates/tauri-plugin-jj-store/tests/file_history/mod.rs#file_history_matches_jj_across_path_batches_merges_and_deletion
- crates/tauri-plugin-jj-store/tests/file_history/mod.rs#file_history_retains_native_conflicted_merge_semantics
- crates/tauri-plugin-jj-store/tests/file_history/mod.rs#file_history_keeps_spaces_quotes_and_pattern_characters_literal
<!-- /evidence-field -->

