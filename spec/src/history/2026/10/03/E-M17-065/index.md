---
schema: 1
id: E-M17-065
---

# E-M17-065

<!-- evidence-field:id markdown -->
## Id

E-M17-065
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

physical-publication-frontier-across-numeric-generations
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-capture/src/reader_publication.rs
- crates/jj-store-capture/src/reader_publication/projection.rs
- crates/jj-store-capture/tests/reader_publication.rs
- crates/jj-store-capture/tests/reader_publication/generations.rs
- spec/src/collaboration.md
- spec/src/milestones.md
- spec/src/verification.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-capture/tests/reader_publication.rs#later_repository_receipt_cannot_cover_an_earlier_withheld_range_after_restart
- crates/jj-store-capture/tests/reader_publication.rs#identity_only_completion_and_empty_queue_never_prove_publication
- crates/jj-store-capture/tests/reader_publication.rs#unknown_and_mismatched_receipts_keep_the_exact_prepared_retry_and_frontier
- crates/jj-store-capture/tests/reader_publication.rs#confirmed_evidence_precedes_body_removal_and_retry_does_not_count_it_twice
- crates/jj-store-capture/tests/reader_publication.rs#preexisting_or_lost_publication_history_is_not_promoted_by_a_later_receipt
- crates/jj-store-capture/tests/reader_publication.rs#corrupt_or_missing_selected_catalog_withholds_enqueue_and_is_never_repaired
- crates/jj-store-capture/tests/reader_publication.rs#selected_catalog_contains_no_body_path_or_native_session_key
- crates/jj-store-capture/tests/reader_publication/generations.rs#later_numeric_generation_cannot_hide_an_earlier_physical_history_hole
- crates/jj-store-capture/tests/reader_publication/generations.rs#unknown_old_generation_is_not_promoted_by_a_new_generation_receipt
- crates/jj-store-capture/tests/reader_publication/generations.rs#a_pending_continuation_preserves_the_prior_committed_frontier
- crates/jj-store-capture/tests/reader_publication/generations.rs#loss_of_an_old_numeric_channel_keeps_retained_physical_history_unqualified
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M17-065 details 2](details-02.md)
