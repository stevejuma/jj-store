---
schema: 1
id: E-M17-071
---

# E-M17-071

<!-- evidence-field:id markdown -->
## Id

E-M17-071
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

opencode-native-snapshot-acquisition
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change xrvytynr
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-agent/src/normalizers/mod.rs
- crates/jj-agent/src/normalizers/opencode.rs
- crates/jj-agent/src/normalizers/opencode_snapshot.rs
- crates/jj-agent/tests/opencode_snapshots.rs
- crates/jj-store-capture/src/lib.rs
- crates/jj-store-capture/src/opencode_snapshot.rs
- crates/jj-store-capture/tests/opencode_snapshot.rs
- crates/jj-store-server/src/lib.rs
- crates/jj-store-server/src/opencode_read_api.rs
- crates/jj-store-server/tests/opencode_read_api/fixture.rs
- crates/jj-store-server/tests/opencode_read_api.rs
- spec/src/collaboration.md
- spec/src/verification.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-agent/tests/opencode_snapshots.rs#native_snapshots_keep_messages_tools_and_explicit_api_lifecycle
- crates/jj-agent/tests/opencode_snapshots.rs#unsupported_parts_keep_safe_message_evidence_without_fictional_tools
- crates/jj-agent/tests/opencode_snapshots.rs#exported_snapshots_cannot_assert_running_and_unknown_roles_are_gaps
- crates/jj-agent/tests/opencode_snapshots.rs#mismatched_native_owners_versions_and_evidence_never_publish_aliases
- crates/jj-store-capture/tests/opencode_snapshot.rs#metadata_never_proves_activity_and_status_uses_pinned_idle_semantics
- crates/jj-store-capture/tests/opencode_snapshot.rs#api_and_export_share_native_message_shape_and_keep_per_record_context
- crates/jj-store-capture/tests/opencode_snapshot.rs#mismatched_owners_versions_paths_duplicates_and_oversize_pages_are_withheld
- crates/jj-store-server/tests/opencode_read_api.rs#metadata_discovery_selects_only_explicit_busy_or_retry_sessions
- crates/jj-store-server/tests/opencode_read_api.rs#acquisition_uses_only_context_bound_gets_and_opaque_message_cursors
- crates/jj-store-server/tests/opencode_read_api.rs#changed_version_redirect_and_streamed_body_overflow_withhold_acquisition
- crates/jj-store-server/tests/opencode_read_api.rs#changed_directory_and_repeated_cursor_preserve_the_acquisition_boundary
- crates/jj-store-server/tests/opencode_read_api.rs#invalid_endpoint_or_native_session_is_refused_before_native_reads
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M17-071 details 2](details-02.md)
