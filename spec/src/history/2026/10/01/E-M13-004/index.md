---
schema: 1
id: E-M13-004
---

# E-M13-004

<!-- evidence-field:id markdown -->
## Id

E-M13-004
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M13
- M13.3
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-01
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

filesystem-git
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

npsqzsms
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-jj/src/local_metadata/blobs.rs
- crates/jj-store-service/src/local_repository.rs
- crates/jj-store-service/src/local_repository/agent_commands.rs
- crates/jj-store-service/src/local_repository/agent_io.rs
- crates/jj-store-service/src/local_repository/agent_store.rs
- crates/jj-store-service/src/local_repository/errors.rs
- crates/jj-store-service/src/local_repository/records.rs
- crates/jj-store-service/tests/local_agent.rs
- crates/jj-store-wal/src/publication.rs
- crates/jj-store-wal/src/publication/agent_capture.rs
- crates/jj-store-wal/src/publication/agent_capture_projection.rs
- crates/jj-store-wal/src/publication/agent_capture_records.rs
- crates/jj-store-wal/src/publication/agent_capture_transcript.rs
- crates/jj-store-wal/src/publication/agent_catalog.rs
- crates/jj-store-wal/src/publication/agent_catalog_engine.rs
- crates/jj-store-wal/src/publication/agent_io.rs
- crates/jj-store-wal/src/publication/agent_read_engine.rs
- crates/jj-store-wal/src/publication/agent_reads.rs
- spec/src/architecture.md
- spec/src/collaboration.md
- spec/src/jj-integration.md
- spec/src/local-jj.md
- spec/src/milestones.md
- spec/src/mutations.md
- spec/src/publication.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-service/tests/local_agent.rs#local_agent_reads_do_not_initialize_metadata_or_actor
- crates/jj-store-service/tests/local_agent.rs#local_capture_replays_original_receipt_after_restart_and_git_gc
- crates/jj-store-service/tests/local_agent.rs#local_transcripts_are_cumulative_and_old_reads_remain_pinned
- crates/jj-store-service/tests/local_agent.rs#agent_and_work_publications_preserve_each_other
- crates/jj-store-service/tests/local_agent.rs#invalid_or_unsupported_capture_never_selects_partial_metadata
- crates/jj-store-service/tests/local_agent.rs#concurrent_disjoint_captures_preserve_both_sources
- crates/jj-store-service/tests/local_agent.rs#git_carrier_preserves_native_record_bytes_and_repository_bound_ids
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M13-004 details 2](details-02.md)
- [E-M13-004 details 3](details-03.md)
