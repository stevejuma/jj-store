---
schema: 1
id: E-M17-012
---

# E-M17-012

<!-- evidence-field:id markdown -->
## Id

E-M17-012
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

common-native-codex-alias-adoption
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-service/tests/local_agent/native_results.rs
- crates/jj-store-wal/src/publication/agent_capture.rs
- crates/jj-store-wal/src/publication/agent_capture_aliases/catalog.rs
- crates/jj-store-wal/src/publication/agent_capture_aliases/catalog_tests.rs
- crates/jj-store-wal/src/publication/agent_capture_aliases/remap.rs
- crates/jj-store-wal/src/publication/agent_capture_aliases.rs
- crates/jj-store-wal/src/publication/agent_capture_projection.rs
- crates/jj-store-wal/src/publication/agent_capture_results.rs
- crates/jj-store-wal/src/publication.rs
- crates/jj-store-wal/tests/agent_catalog/common_aliases.rs
- crates/jj-store-wal/tests/agent_catalog/native_results.rs
- crates/jj-store-wal/tests/agent_catalog.rs
- spec/src/collaboration.md
- spec/src/local-jj.md
- spec/src/mutations.md
- spec/src/publication.md
- spec/src/spec-governance.md
- spec/src/verification.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-wal/tests/agent_catalog/common_aliases.rs#hook_and_transcript_channels_adopt_selected_native_session_and_tool_ids
- crates/jj-store-wal/tests/agent_catalog/common_aliases.rs#result_only_new_channel_adopts_a_published_call_without_new_session_or_lifecycle
- crates/jj-store-wal/tests/agent_catalog/common_aliases.rs#transcript_first_ids_survive_later_hook_channel_and_an_independent_source_cursor
- crates/jj-store-wal/tests/agent_catalog/common_aliases.rs#conflicting_legacy_native_sessions_withhold_activity_but_keep_source_progress
- crates/jj-store-wal/tests/agent_catalog/common_aliases.rs#non_uuid_labels_remain_source_scoped_and_never_join_by_clock_or_content
- crates/jj-store-wal/tests/agent_catalog/common_aliases.rs#common_message_and_turn_aliases_use_uuid_identity_and_keep_missing_ids_source_scoped
- crates/jj-store-wal/tests/agent_catalog/common_aliases.rs#conflicting_tool_aliases_never_choose_one_or_replace_accepted_aliases
- crates/jj-store-wal/tests/agent_catalog/common_aliases.rs#concurrent_cross_profile_preparations_cannot_publish_duplicate_native_sessions
- crates/jj-store-wal/src/publication/agent_capture_aliases/catalog_tests.rs#incomplete_alias_or_page_scan_never_returns_a_partial_identity_proof
- crates/jj-store-service/tests/local_agent/native_results.rs#local_common_alias_adoption_survives_reopen_receipt_replay_and_later_source_results
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M17-012 details 2](details-02.md)
- [E-M17-012 details 3](details-03.md)
