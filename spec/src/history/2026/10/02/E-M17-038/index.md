---
schema: 1
id: E-M17-038
---

# E-M17-038

<!-- evidence-field:id markdown -->
## Id

E-M17-038
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

- crates/jj-store-capture/src/agent_capture_enqueue.rs
- crates/jj-store-capture/src/codex_source.rs
- crates/jj-store-capture/src/codex_transcript.rs
- crates/jj-store-capture/src/codex_transcript/poll.rs
- crates/jj-store-capture/src/codex_transcript/progress.rs
- crates/jj-store-capture/src/reader_progress.rs
- crates/jj-store-capture/src/reader_progress/source.rs
- crates/jj-store-capture/src/reader_progress/spool.rs
- crates/jj-store-capture/src/source_registry/codex.rs
- crates/jj-store-capture/src/source_registry/parking.rs
- crates/jj-store-capture/tests/reader_progress.rs
- crates/jj-store-capture/tests/reader_progress/source.rs
- crates/jj-store-capture/tests/source_parking.rs
- crates/jj-store-server/src/capture_source/acquire.rs
- crates/jj-store-server/src/capture_source/ingress.rs
- crates/jj-store-server/src/capture_source/parking.rs
- crates/jj-store-server/src/capture_source/parking/current.rs
- crates/jj-store-server/tests/capture_parking.rs
- crates/jj-store-server/tests/capture_parking/generations.rs
- crates/jj-store-server/tests/capture_pipeline.rs
- crates/jj-store-server/tests/capture_source/progress_capacity.rs
- crates/jj-store-server/tests/capture_supervisor.rs
- crates/jj-store-server/tests/capture_supervisor/generations.rs
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/milestones.md
- spec/src/verification.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-server/tests/capture_supervisor/generations.rs#retained_physical_generations_do_not_replay_unchanged_files
- crates/jj-store-server/tests/capture_supervisor/generations.rs#delayed_archived_writes_rewrites_and_restart_preserve_prefixes_and_channel_order
- crates/jj-store-capture/tests/reader_progress/source.rs#physical_selection_and_channel_floors_span_partitions_pending_and_restart
- crates/jj-store-capture/tests/reader_progress/source.rs#automatic_admission_refuses_stale_allocations_but_retains_exact_durable_retries
- crates/jj-store-capture/tests/source_parking.rs#independent_generation_advancement_withholds_an_exact_parked_raw_allocation
- crates/jj-store-server/tests/capture_parking/generations.rs#another_physical_reader_cannot_renumber_or_enqueue_a_parked_raw_batch
- crates/jj-store-server/tests/capture_source/progress_capacity.rs#progress_snapshot_retains_exact_capacity_refusal_from_an_independent_spool_handle
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M17-038 details 2](details-02.md)
- [E-M17-038 details 3](details-03.md)
- [E-M17-038 details 4](details-04.md)
