---
schema: 1
id: E-M17-039
---

# E-M17-039

<!-- evidence-field:id markdown -->
## Id

E-M17-039
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

- crates/jj-store-capture/src/codex_source.rs
- crates/jj-store-capture/src/codex_transcript.rs
- crates/jj-store-capture/src/source_registry/files.rs
- crates/jj-store-capture/src/source_registry/parking/pinned.rs
- crates/jj-store-capture/src/source_registry/parking.rs
- crates/jj-store-capture/src/source_registry.rs
- crates/jj-store-capture/tests/source_parking/reservations.rs
- crates/jj-store-capture/tests/source_parking.rs
- crates/jj-store-server/src/capture_source/parking/acquire.rs
- crates/jj-store-server/src/capture_source/parking.rs
- crates/jj-store-server/src/capture_supervisor/source.rs
- crates/jj-store-server/tests/capture_supervisor/raw_channels.rs
- crates/jj-store-server/tests/capture_supervisor/raw_handoff.rs
- crates/jj-store-server/tests/capture_supervisor.rs
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/verification.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-capture/tests/source_parking/reservations.rs#raw_reservation_excludes_alias_and_rotated_readers_but_releases_physical_ownership
- crates/jj-store-capture/tests/source_parking/reservations.rs#changed_admission_or_allocation_retains_exact_raw_channel_for_retry
- crates/jj-store-capture/tests/source_parking/reservations.rs#foreign_registry_or_replaced_channel_lock_cannot_transfer_a_raw_reservation
- crates/jj-store-capture/tests/source_parking/reservations.rs#physical_exclusion_failed_reopen_and_source_replacement_retain_raw_reservation
- crates/jj-store-capture/tests/source_parking/reservations.rs#parked_owner_death_releases_ephemeral_channel_without_serializing_raw_state
- crates/jj-store-capture/tests/source_parking/reservations.rs#automatic_partial_scan_parking_releases_both_leases_and_preserves_record_bytes
- crates/jj-store-capture/tests/source_parking/reservations.rs#durable_exact_raw_acceptance_reopens_with_prefix_verification_and_releases_channel_at_idle
- crates/jj-store-server/tests/capture_supervisor/raw_channels.rs#raw_channel_reservation_survives_pause_and_resumes_after_queue_relief
- crates/jj-store-server/tests/capture_supervisor/raw_channels.rs#shutdown_releases_raw_channel_without_acknowledging_tail_and_restart_recovers
- crates/jj-store-server/tests/capture_supervisor/raw_handoff.rs#overdue_and_cancelled_admission_keeps_raw_channel_before_and_after_physical_acquisition
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M17-039 details 2](details-02.md)
- [E-M17-039 details 3](details-03.md)
