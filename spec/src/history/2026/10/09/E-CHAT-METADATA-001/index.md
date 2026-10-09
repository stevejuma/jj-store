---
schema: 1
id: E-CHAT-METADATA-001
---

# E-CHAT-METADATA-001

<!-- evidence-field:id markdown -->
## Id

E-CHAT-METADATA-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M24
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-COLLAB-034
- JST-COLLAB-035
- JST-COLLAB-036
- JST-API-001
- JST-CLI-004
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-09
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-collab/src/catalog.rs
- crates/jj-collab/src/commands.rs
- crates/jj-collab/src/conversation_metadata.rs
- crates/jj-collab/src/graph.rs
- crates/jj-collab/src/lib.rs
- crates/jj-collab/src/objects.rs
- crates/jj-collab/src/threads.rs
- crates/jj-collab/tests/conversation_metadata.rs
- crates/jj-store-cli/src/conversation_metadata.rs
- crates/jj-store-cli/src/lib.rs
- crates/jj-store-cli/src/server_commands.rs
- crates/jj-store-cli/tests/agent_execution.rs
- crates/jj-store-cli/tests/agent_execution/metadata.rs
- crates/jj-store-cli/tests/agent_execution/metadata_recovery.rs
- crates/jj-store-http/src/agent_execution.rs
- crates/jj-store-server/src/agent_execution/commands.rs
- crates/jj-store-server/src/agent_execution/configuration.rs
- crates/jj-store-server/src/agent_execution/conversations.rs
- crates/jj-store-server/src/agent_execution/journal.rs
- crates/jj-store-server/src/agent_execution/launch.rs
- crates/jj-store-server/src/agent_execution/metadata_cancel.rs
- crates/jj-store-server/src/agent_execution/metadata_directory.rs
- crates/jj-store-server/src/agent_execution/metadata_jobs.rs
- crates/jj-store-server/src/agent_execution/metadata_runtime.rs
- crates/jj-store-server/src/agent_execution/metadata_sources.rs
- crates/jj-store-server/src/agent_execution/metadata_store.rs
- crates/jj-store-server/src/agent_execution/metadata_suggestions.rs
- crates/jj-store-server/src/agent_execution/metadata_text.rs
- crates/jj-store-server/src/agent_execution/mod.rs
- crates/jj-store-server/src/agent_execution/publication.rs
- crates/jj-store-service/src/agent_execution.rs
- crates/jj-store-service/src/agent_execution/conversations.rs
- crates/jj-store-service/src/agent_execution/metadata.rs
- crates/jj-store-service/src/agent_execution/settings.rs
- crates/jj-store-service/src/collab_reads.rs
- crates/jj-store-service/src/collaboration.rs
- crates/jj-store-service/src/collaboration/agent_details.rs
- crates/jj-store-service/src/collaboration/models.rs
- crates/jj-store-service/src/content_search/index.rs
- crates/jj-store-service/src/conversation_discovery.rs
- crates/jj-store-service/src/conversion/collab.rs
- crates/jj-store-service/src/lib.rs
- crates/jj-store-service/src/local_repository/browser_agents.rs
- crates/jj-store-service/src/local_repository/browser_collab.rs
- crates/jj-store-service/src/local_repository/collab_commands.rs
- crates/jj-store-wal/src/publication/collab_commands.rs
- runtime/local-agent/UPSTREAM.md
- runtime/local-agent/src/main.rs
- runtime/local-agent/src/metadata.rs
- runtime/local-agent/tests/metadata.rs
- runtime/local-agent/vendor/buzz-agent/src/lib.rs
- runtime/local-agent/vendor/buzz-agent/src/llm.rs
- schemas/openapi.json
- spec/src/current/contracts/api/43-conversation-metadata.md
- spec/src/current/contracts/api/SUMMARY.nav.md
- spec/src/current/contracts/api/index.md
- spec/src/current/contracts/architecture/20-interactive-agent-execution.md
- spec/src/current/contracts/cli/37-conversation-metadata.md
- spec/src/current/contracts/cli/SUMMARY.nav.md
- spec/src/current/contracts/cli/index.md
- spec/src/current/contracts/collaboration/42-conversation-metadata.md
- spec/src/current/contracts/collaboration/SUMMARY.nav.md
- spec/src/current/contracts/collaboration/index.md
- web/src/lib/api/generated.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-collab/tests/conversation_metadata.rs#legacy_root_bytes_and_metadata_preconditions_are_preserved
- crates/jj-collab/tests/conversation_metadata.rs#suggestions_require_snapshot_provenance_and_generated_fields_are_bounded
- crates/jj-collab/tests/conversation_metadata.rs#metadata_owner_is_immutable_and_tombstoned_threads_refuse_new_revisions
- crates/jj-store-cli/tests/agent_execution/metadata.rs#metadata_generates_once_preserves_custom_titles_and_recovers_repository_state
- crates/jj-store-cli/tests/agent_execution/metadata_recovery.rs#metadata_stale_generation_and_unknown_publication_recover_without_new_inference
- crates/jj-store-cli/tests/agent_execution/metadata_recovery.rs#metadata_acp_suggestions_are_repository_backed_without_a_generation_provider
- runtime/local-agent/vendor/buzz-agent/src/llm.rs#metadata_summary_never_retries_an_unknown_inference
- runtime/local-agent/tests/metadata.rs#metadata_accepts_bounded_transcript_after_json_escape_expansion
- runtime/local-agent/tests/metadata.rs#metadata_uses_one_tool_free_request_and_strict_result
- runtime/local-agent/tests/metadata.rs#metadata_rejects_foreign_and_fenced_results_without_repair
- runtime/local-agent/tests/metadata.rs#metadata_does_not_open_databricks_sign_in
- crates/jj-store-server/src/agent_execution/metadata_text.rs#owned_sent_context_keeps_authored_text_without_bootstrap_or_context_bodies
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-CHAT-METADATA-001 details 2](details-02.md)
