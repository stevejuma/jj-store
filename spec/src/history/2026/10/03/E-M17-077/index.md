---
schema: 1
id: E-M17-077
---

# E-M17-077

<!-- evidence-field:id markdown -->
## Id

E-M17-077
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

api-snapshot-native-receipt-publication-inspection
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change zwnotqlz
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-capture/src/reader_publication/catalog/tests.rs
- crates/jj-store-capture/src/reader_publication/catalog.rs
- crates/jj-store-capture/src/reader_publication/inspection.rs
- crates/jj-store-capture/src/reader_publication/projection.rs
- crates/jj-store-capture/src/reader_publication/ranges.rs
- crates/jj-store-capture/src/reader_publication/snapshot.rs
- crates/jj-store-capture/src/reader_publication.rs
- crates/jj-store-capture/tests/snapshot_publication.rs
- crates/jj-store-server/tests/opencode_publication.rs
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/verification.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-capture/tests/snapshot_publication.rs#empty_status_acknowledgement_and_identity_completion_do_not_prove_publication
- crates/jj-store-capture/tests/snapshot_publication.rs#unknown_or_mismatched_snapshot_receipts_keep_exact_retry_and_zero_frontier
- crates/jj-store-capture/tests/snapshot_publication.rs#generation_replacement_can_reduce_count_but_cannot_skip_earlier_holes
- crates/jj-store-capture/tests/snapshot_publication.rs#old_acknowledged_history_cannot_be_promoted_by_a_later_snapshot_receipt
- crates/jj-store-capture/tests/snapshot_publication.rs#selected_snapshot_boundary_survives_unavailable_later_reader_history
- crates/jj-store-capture/tests/snapshot_publication.rs#snapshot_inspection_does_not_read_pending_bodies_or_repair_selected_files
- crates/jj-store-capture/tests/snapshot_publication.rs#byte_and_snapshot_inspections_refuse_each_others_positions
- crates/jj-store-capture/tests/snapshot_publication.rs#ingress_pressure_does_not_select_snapshot_publication_evidence
- crates/jj-store-capture/tests/snapshot_publication.rs#missing_corrupt_or_foreign_snapshot_catalog_never_repairs_into_publication
- crates/jj-store-server/tests/opencode_publication.rs#empty_lifecycle_snapshot_coverage_requires_native_receipts_and_survives_reopen
- crates/jj-store-server/tests/opencode_publication.rs#later_snapshot_partition_receipt_cannot_cover_an_earlier_unpublished_status
- crates/jj-store-capture/src/reader_publication/catalog/tests.rs#capacity_keeps_every_hole_and_folding_one_exact_receipt_releases_one_slot
- crates/jj-store-capture/src/reader_publication/catalog/tests.rs#changed_digest_range_and_receipt_reuse_cannot_reauthor_selected_evidence
- crates/jj-store-capture/src/reader_publication/catalog/tests.rs#snapshot_capacity_keeps_status_holes_and_original_jsonl_encoding
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M17-077 details 2](details-02.md)
- [E-M17-077 details 3](details-03.md)
