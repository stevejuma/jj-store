---
schema: 1
id: E-M17-015
---

# E-M17-015

<!-- evidence-field:id markdown -->
## Id

E-M17-015
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

codex-directory-context-partitions-and-source-wide-durable-cursors
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-capture/src/codex_transcript/context.rs
- crates/jj-store-capture/src/codex_transcript/poll.rs
- crates/jj-store-capture/src/codex_transcript.rs
- crates/jj-store-capture/src/reader_progress/history.rs
- crates/jj-store-capture/src/reader_progress/spool.rs
- crates/jj-store-capture/src/reader_progress.rs
- crates/jj-store-capture/src/transcript_sanitization.rs
- crates/jj-store-capture/tests/codex_context.rs
- crates/jj-store-capture/tests/reader_progress.rs
- crates/jj-store-capture/tests/transcript_sanitization.rs
- crates/jj-store-cli/tests/local_browser.rs
- crates/jj-store-server/src/capture_admission.rs
- crates/jj-store-server/tests/capture_admission.rs
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/verification.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-capture/tests/codex_context.rs#directory_changes_partition_exact_batches_and_global_resume_crosses_repositories
- crates/jj-store-capture/tests/codex_context.rs#malformed_context_withholds_later_content_until_explicit_valid_context
- crates/jj-store-capture/tests/codex_context.rs#restart_reconstructs_directory_across_bounded_prefix_steps_without_reemitting_history
- crates/jj-store-capture/tests/codex_context.rs#arbitrary_cwd_fields_do_not_rebind_and_repeated_session_headers_are_sticky_gaps
- crates/jj-store-capture/tests/codex_context.rs#malformed_record_context_is_reconstructed_on_restart_until_a_valid_new_turn
- crates/jj-store-capture/tests/reader_progress.rs#repository_partitions_share_one_durable_source_cursor_and_cannot_replay_elsewhere
- crates/jj-store-capture/tests/reader_progress.rs#partitioned_pending_recovery_repairs_all_repository_journals_and_rejects_conflicts
- crates/jj-store-capture/tests/transcript_sanitization.rs#unknown_directory_never_sanitizes_native_bodies_and_safe_gaps_allow_later_repository
- crates/jj-store-capture/tests/transcript_sanitization.rs#transcript_sanitization_precedes_durability_and_covers_path_keys_and_values
- crates/jj-store-server/tests/capture_admission.rs#batch_admission_uses_reconstructed_context_and_current_registry_membership
- crates/jj-store-cli/tests/workspace_management.rs#server_capture_admits_only_published_registry_bindings_and_revalidates_them
- crates/jj-store-cli/tests/local_browser.rs#local_code_reads_cover_graphs_diffs_roots_conflicts_and_multiple_heads
- crates/jj-store-capture/tests/codex_context.rs#desktop_profile_independently_qualifies_directory_changes_without_cli_header_adoption
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M17-015 details 2](details-02.md)
- [E-M17-015 details 3](details-03.md)
