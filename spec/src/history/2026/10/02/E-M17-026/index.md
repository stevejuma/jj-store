---
schema: 1
id: E-M17-026
---

# E-M17-026

<!-- evidence-field:id markdown -->
## Id

E-M17-026
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

opaque-codex-reader-frame-parking-and-current-admitted-lease-handoff
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-capture/src/source_registry.rs
- crates/jj-store-capture/src/source_registry/codex.rs
- crates/jj-store-capture/src/source_registry/parking.rs
- crates/jj-store-capture/tests/source_parking.rs
- crates/jj-store-server/src/capture_source.rs
- crates/jj-store-server/src/capture_source/acquire.rs
- crates/jj-store-server/src/capture_source/ingress.rs
- crates/jj-store-server/src/capture_source/pipeline.rs
- crates/jj-store-server/src/capture_source/parking.rs
- crates/jj-store-server/tests/capture_parking.rs
- spec/src/collaboration.md
- spec/src/cli.md
- spec/src/verification.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-server/tests/capture_parking.rs#parked_pipeline_retains_verification_and_releases_leases
- crates/jj-store-server/tests/capture_parking.rs#parked_backpressure_keeps_the_exact_batch_and_rechecks_admission
- crates/jj-store-server/tests/capture_parking.rs#another_owner_advancing_the_cursor_reopens_from_durable_progress
- crates/jj-store-server/tests/capture_parking.rs#replaced_source_cannot_reuse_a_parked_frame
- crates/jj-store-server/tests/capture_parking.rs#missing_or_regressive_durable_progress_never_resets_a_parked_reader
- crates/jj-store-capture/tests/source_parking.rs#parked_raw_batch_survives_append_and_transfers_across_overlapping_roots
- crates/jj-store-capture/tests/source_parking.rs#foreign_registered_channel_cannot_consume_a_parked_reader
- crates/jj-store-capture/tests/source_parking.rs#an_identical_replacement_or_changed_header_cannot_reuse_raw_state
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M17-026 details 2](details-02.md)
- [E-M17-026 details 3](details-03.md)
