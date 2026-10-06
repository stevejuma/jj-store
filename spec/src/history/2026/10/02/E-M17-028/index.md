---
schema: 1
id: E-M17-028
---

# E-M17-028

<!-- evidence-field:id markdown -->
## Id

E-M17-028
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

bounded-configured-codex-supervisor-native-service-composition-and-local-fifo-ownership
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-capture/src/agent_capture.rs
- crates/jj-store-capture/src/agent_capture_lock.rs
- crates/jj-store-capture/src/codex_activity.rs
- crates/jj-store-capture/src/codex_activity/scan.rs
- crates/jj-store-capture/src/codex_source.rs
- crates/jj-store-capture/src/codex_transcript.rs
- crates/jj-store-capture/src/lib.rs
- crates/jj-store-capture/src/source_registry.rs
- crates/jj-store-capture/src/source_registry/codex.rs
- crates/jj-store-capture/src/source_registry/parking.rs
- crates/jj-store-capture/tests/codex_activity.rs
- crates/jj-store-capture/tests/reader_progress.rs
- crates/jj-store-capture/tests/source_parking.rs
- crates/jj-store-server/src/capture_admission.rs
- crates/jj-store-server/src/capture_source/acquire.rs
- crates/jj-store-server/src/capture_source/parking.rs
- crates/jj-store-server/src/capture_source/pipeline.rs
- crates/jj-store-server/src/capture_source/resume.rs
- crates/jj-store-server/src/capture_supervisor.rs
- crates/jj-store-server/src/capture_supervisor/delivery.rs
- crates/jj-store-server/src/capture_supervisor/discovery.rs
- crates/jj-store-server/src/capture_supervisor/scheduling.rs
- crates/jj-store-server/src/capture_supervisor/source.rs
- crates/jj-store-server/src/capture_worker.rs
- crates/jj-store-server/src/capture_worker/executor.rs
- crates/jj-store-server/src/lib.rs
- crates/jj-store-server/tests/capture_supervisor.rs
- crates/jj-store-server/tests/capture_supervisor/blocked.rs
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/milestones.md
- spec/src/verification.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-server/tests/capture_supervisor.rs#enabled_supervisor_backfills_new_start_without_ingesting_inactive_history
- crates/jj-store-server/tests/capture_supervisor.rs#disabled_selection_does_not_open_roots_or_dispatch
- crates/jj-store-server/tests/capture_supervisor.rs#overlap_and_hardlinks_are_deduplicated_and_tracking_is_bounded
- crates/jj-store-server/tests/capture_supervisor.rs#pause_preserves_armed_activity_and_shutdown_releases_resources
- crates/jj-store-server/tests/capture_supervisor.rs#activity_without_current_native_write_cannot_enroll_or_acknowledge
- crates/jj-store-server/tests/capture_supervisor.rs#admitted_offline_tail_delivers_b_while_long_a_proof_is_withheld
- crates/jj-store-server/tests/capture_supervisor.rs#multi_step_inactive_probes_share_bounded_frames_and_all_reach_compact_eof
- crates/jj-store-server/tests/capture_supervisor/blocked.rs#blocked_directory_keeps_its_owner_while_another_source_delivers
- crates/jj-store-capture/tests/codex_activity.rs#compact_eof_does_not_admit_an_old_partial_start_and_renews_a_genuine_start
- crates/jj-store-capture/tests/source_parking.rs#compact_partial_eof_retains_exact_record_after_later_append
- crates/jj-store-capture/tests/reader_progress.rs#forked_spool_handles_keep_compatibility_lease_and_shared_durable_progress
- crates/jj-store-capture/src/agent_capture_lock.rs#queued_fork_mutations_settle_in_arrival_order
- crates/jj-store-capture/src/codex_activity/scan.rs#eof_compaction_releases_allocation_without_rewinding_the_complete_prefix
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M17-028 details 2](details-02.md)
- [E-M17-028 details 3](details-03.md)
- [E-M17-028 details 4](details-04.md)
