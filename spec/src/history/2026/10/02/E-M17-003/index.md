---
schema: 1
id: E-M17-003
---

# E-M17-003

<!-- evidence-field:id markdown -->
## Id

E-M17-003
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M17
- M17.1
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-02
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

bounded-transcript-append
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-agent/src/indexes.rs
- crates/jj-agent/src/objects.rs
- crates/jj-agent/src/transcript.rs
- crates/jj-agent/tests/activity.rs
- crates/jj-store-service/src/local_repository/agent_store.rs
- crates/jj-store-service/tests/local_agent.rs
- crates/jj-store-wal/src/publication/agent.rs
- crates/jj-store-wal/src/publication/agent_capture_transcript.rs
- crates/jj-store-wal/src/publication/agent_catalog.rs
- crates/jj-store-wal/src/publication/agent_io.rs
- crates/jj-store-wal/src/publication/agent_read_engine.rs
- crates/jj-store-wal/src/publication/agent_transcript_audit.rs
- crates/jj-store-wal/src/publication/agent_transcript_read.rs
- crates/jj-store-wal/src/publication/agent_transcript_tail.rs
- crates/jj-store-wal/src/publication.rs
- crates/jj-store-wal/tests/agent_catalog.rs
- crates/jj-store-wal/tests/native_copy.rs
- spec/src/collaboration.md
- spec/src/local-jj.md
- spec/src/milestones.md
- spec/src/mutations.md
- spec/src/publication.md
- spec/src/verification.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-wal/tests/agent_catalog.rs#transcript_append_reads_only_selected_tail_and_keeps_old_manifest_reconstructible
- crates/jj-store-wal/tests/agent_catalog.rs#legacy_prefix_append_requires_source_receipt_and_preserves_other_streams
- crates/jj-agent/tests/activity.rs#append_tail_pages_preserve_legacy_bytes_and_strong_prefix_dependencies
- crates/jj-store-service/tests/local_agent.rs#transcript_tail_rollover_preserves_pinned_history_after_local_restart
- crates/jj-store-wal/tests/agent_catalog.rs#legacy_transcript_ordering_cannot_be_inferred_from_an_unrelated_source_cursor
- crates/jj-store-wal/tests/native_copy.rs#native_copy_restores_code_agent_review_and_installs_manifest_last
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M17-003 details 2](details-02.md)
- [E-M17-003 details 3](details-03.md)
