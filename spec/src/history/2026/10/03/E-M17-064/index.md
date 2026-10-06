---
schema: 1
id: E-M17-064
---

# E-M17-064

<!-- evidence-field:id markdown -->
## Id

E-M17-064
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

receipt-backed-automatic-reader-publication-coverage
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-capture/src/agent_capture.rs
- crates/jj-store-capture/src/agent_capture/budget.rs
- crates/jj-store-capture/src/agent_capture_enqueue.rs
- crates/jj-store-capture/src/agent_publication.rs
- crates/jj-store-capture/src/agent_publication/drain.rs
- crates/jj-store-capture/src/lib.rs
- crates/jj-store-capture/src/reader_progress/spool.rs
- crates/jj-store-capture/src/reader_publication.rs
- crates/jj-store-capture/src/reader_publication/catalog.rs
- crates/jj-store-capture/src/reader_publication/catalog/tests.rs
- crates/jj-store-capture/src/reader_publication/files.rs
- crates/jj-store-capture/src/reader_publication/ranges.rs
- crates/jj-store-capture/src/reader_publication/spool.rs
- crates/jj-store-capture/tests/reader_publication.rs
- crates/jj-store-capture/tests/reader_publication/mixed.rs
- crates/jj-store-capture/tests/reader_publication/mod.rs
- crates/jj-store-capture/tests/reader_publication/paths.rs
- crates/jj-store-capture/tests/reader_publication/process.rs
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/milestones.md
- spec/src/verification.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-capture/src/reader_publication/catalog/tests.rs#capacity_keeps_every_hole_and_folding_one_exact_receipt_releases_one_slot
- crates/jj-store-capture/src/reader_publication/catalog/tests.rs#changed_digest_range_and_receipt_reuse_cannot_reauthor_selected_evidence
- crates/jj-store-capture/tests/reader_publication/mixed.rs#shared_reader_adds_requirements_only_to_an_already_tracked_channel
- crates/jj-store-capture/tests/reader_publication/paths.rs#ingress_capacity_refusal_does_not_select_a_publication_requirement
- crates/jj-store-capture/tests/reader_publication/paths.rs#oversized_or_foreign_catalog_files_withhold_without_reading_transcripts_or_repair
- crates/jj-store-capture/tests/reader_publication/paths.rs#symlinked_or_public_selected_evidence_cannot_authorize_coverage
- crates/jj-store-capture/tests/reader_publication/process.rs#committed_frontier_survives_a_separate_process_exit
- crates/jj-store-capture/tests/reader_publication.rs#later_repository_receipt_cannot_cover_an_earlier_withheld_range_after_restart
- crates/jj-store-capture/tests/reader_publication.rs#identity_only_completion_and_empty_queue_never_prove_publication
- crates/jj-store-capture/tests/reader_publication.rs#unknown_and_mismatched_receipts_keep_the_exact_prepared_retry_and_frontier
- crates/jj-store-capture/tests/reader_publication.rs#confirmed_evidence_precedes_body_removal_and_retry_does_not_count_it_twice
- crates/jj-store-capture/tests/reader_publication.rs#preexisting_or_lost_publication_history_is_not_promoted_by_a_later_receipt
- crates/jj-store-capture/tests/reader_publication.rs#corrupt_or_missing_selected_catalog_withholds_enqueue_and_is_never_repaired
- crates/jj-store-capture/tests/reader_publication.rs#selected_catalog_contains_no_body_path_or_native_session_key
- crates/jj-store-server/tests/capture_recovery.rs#multiple_queued_partitions_recover_in_source_order_before_new_reading
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M17-064 details 2](details-02.md)
- [E-M17-064 details 3](details-03.md)
- [E-M17-064 details 4](details-04.md)
