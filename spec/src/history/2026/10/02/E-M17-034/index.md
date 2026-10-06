---
schema: 1
id: E-M17-034
---

# E-M17-034

<!-- evidence-field:id markdown -->
## Id

E-M17-034
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M17
- M17.1
- M17.2
- M13.3
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

- crates/jj-agent/src/objects.rs
- crates/jj-agent/src/transcript.rs
- crates/jj-agent/tests/activity.rs
- crates/jj-store-cli/tests/native_lifecycle/http_collaboration.rs
- crates/jj-store-service/src/local_repository/agent_descriptor.rs
- crates/jj-store-service/tests/agent_sessions.rs
- crates/jj-store-wal/src/publication/agent_transcript_audit.rs
- crates/jj-store-wal/src/publication/agent_transcript_read.rs
- crates/jj-store-wal/src/publication/agent_transcript_tail.rs
- crates/jj-store-wal/src/publication.rs
- crates/jj-store-wal/tests/agent_catalog.rs
- crates/jj-store-wal/tests/native_copy.rs
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/local-jj.md
- spec/src/milestones.md
- spec/src/mutations.md
- spec/src/publication.md
- spec/src/verification.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-agent/tests/activity.rs#only_current_tail_pages_decode_and_retain_strong_prefix_dependencies
- crates/jj-store-service/src/local_repository/agent_descriptor.rs#rejects_legacy_agent_descriptor_without_in_memory_upgrade
- crates/jj-store-service/src/local_repository/agent_descriptor.rs#current_agent_descriptor_requires_exact_version_identity_and_canonical_bytes
- crates/jj-store-wal/tests/agent_catalog.rs#tail_append_preserves_other_streams_and_reads_only_selected_tail
- crates/jj-store-wal/tests/agent_catalog.rs#tail_append_ordering_cannot_be_inferred_from_an_unrelated_source_cursor
- crates/jj-store-service/tests/local_agent.rs#transcript_tail_rollover_preserves_pinned_history_after_local_restart
- crates/jj-store-service/tests/agent_sessions.rs#transcript_pages_retain_exact_root_and_validate_stream_cursor
- crates/jj-store-cli/tests/native_lifecycle/http_collaboration.rs#local_trusted_http_lists_root_pinned_agent_sessions_without_bearer_credentials
- crates/jj-store-wal/tests/native_copy.rs#native_copy_restores_code_agent_review_and_installs_manifest_last
<!-- /evidence-field -->

<!-- evidence-field:removed_tests list -->
## Removed tests

- crates/jj-agent/tests/activity.rs#append_tail_pages_preserve_legacy_bytes_and_strong_prefix_dependencies
- crates/jj-store-wal/tests/agent_catalog.rs#legacy_prefix_append_requires_source_receipt_and_preserves_other_streams
- crates/jj-store-wal/tests/agent_catalog.rs#legacy_transcript_ordering_cannot_be_inferred_from_an_unrelated_source_cursor
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M17-034 details 2](details-02.md)
- [E-M17-034 details 3](details-03.md)
