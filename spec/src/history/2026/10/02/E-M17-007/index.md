---
schema: 1
id: E-M17-007
---

# E-M17-007

<!-- evidence-field:id markdown -->
## Id

E-M17-007
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

durable-reader-progress
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-capture/src/agent_capture.rs
- crates/jj-store-capture/src/agent_capture_enqueue.rs
- crates/jj-store-capture/src/agent_capture_intent.rs
- crates/jj-store-capture/src/lib.rs
- crates/jj-store-capture/src/reader_progress.rs
- crates/jj-store-capture/src/reader_progress_files.rs
- crates/jj-store-capture/tests/reader_progress.rs
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/compatibility.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-capture/tests/reader_progress.rs#reader_cursor_is_durable_before_acknowledgement_and_survives_reopening
- crates/jj-store-capture/tests/reader_progress.rs#native_and_local_prepared_intents_recover_missing_progress_and_keep_it_after_completion
- crates/jj-store-capture/tests/reader_progress.rs#cursor_failure_keeps_exact_prepared_native_and_local_requests
- crates/jj-store-capture/tests/reader_progress.rs#inconsistent_or_regressive_readers_never_admit_another_pending_batch
- crates/jj-store-capture/tests/reader_progress.rs#generations_and_older_retained_retries_do_not_regress_progress
- crates/jj-store-capture/tests/reader_progress.rs#live_handles_clean_unacknowledged_temporaries_and_legacy_bytes_stay_unchanged
- crates/jj-store-capture/tests/reader_progress.rs#corrupt_oversized_and_symlinked_progress_fails_closed
- crates/jj-store-capture/tests/reader_progress.rs#full_progress_journal_retains_old_generations_and_allows_existing_updates
- crates/jj-store-capture/tests/reader_progress.rs#concurrent_reader_handles_share_mutation_and_pending_limits
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M17-007 details 2](details-02.md)
- [E-M17-007 details 3](details-03.md)
