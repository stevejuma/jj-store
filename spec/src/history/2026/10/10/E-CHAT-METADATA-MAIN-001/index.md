---
schema: 1
id: E-CHAT-METADATA-MAIN-001
---

# E-CHAT-METADATA-MAIN-001

<!-- evidence-field:id markdown -->
## Id

E-CHAT-METADATA-MAIN-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M24
- M25
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-COLLAB-034
- JST-COLLAB-035
- JST-COLLAB-036
- JST-UI-012
- JST-API-001
- JST-COMPAT-001
- JST-DOC-001
- JST-MUT-001
- JST-PUB-001
- JST-GOV-008
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-10
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/AgentDefaults.stories.svelte
- catalog/components/store-application/AgentFleet.stories.svelte
- catalog/components/store-application/AgentInboxNavigation.stories.svelte
- catalog/components/store-application/AgentSessionSettings.stories.svelte
- catalog/components/store-application/AgentSessions.stories.svelte
- catalog/components/store-application/ConversationMetadata.stories.svelte
- catalog/controller/conversation-metadata.test.ts
- catalog/fixtures/AgentFleetApplicationFixture.svelte
- catalog/fixtures/ConversationMetadataFixture.svelte
- catalog/fixtures/WorkspaceFilesFixture.svelte
- catalog/fixtures/agent-execution-configuration.svelte.ts
- catalog/fixtures/agent-execution-session.ts
- catalog/fixtures/agent-fleet-changes.ts
- catalog/fixtures/agent-settings-fixture.svelte.ts
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
- crates/jj-store-cli/tests/agent_execution/metadata.rs
- crates/jj-store-cli/tests/agent_execution/metadata_recovery.rs
- crates/jj-store-cli/tests/agent_execution.rs
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
- crates/jj-store-service/src/agent_execution/conversations.rs
- crates/jj-store-service/src/agent_execution/metadata.rs
- crates/jj-store-service/src/agent_execution/settings.rs
- crates/jj-store-service/src/agent_execution.rs
- crates/jj-store-service/src/collab_reads.rs
- crates/jj-store-service/src/collaboration/agent_details.rs
- crates/jj-store-service/src/collaboration/models.rs
- crates/jj-store-service/src/collaboration.rs
- crates/jj-store-service/src/content_search/index.rs
- crates/jj-store-service/src/conversation_discovery.rs
- crates/jj-store-service/src/conversion/collab.rs
- crates/jj-store-service/src/lib.rs
- crates/jj-store-service/src/local_repository/browser_agents.rs
- crates/jj-store-service/src/local_repository/browser_collab.rs
- crates/jj-store-service/src/local_repository/collab_commands.rs
- crates/jj-store-wal/src/publication/collab_commands.rs
- docs/maintenance/coverage.json
- docs/site/cli/index.md
- docs/site/cli/jj-store.md
- docs/site/concepts/collaboration.md
- docs/site/guides/agent-execution.md
- docs/site/guides/settings.md
- docs/site/reference/api/collaboration.md
- docs/site/reference/api/installation.md
- docs/site/reference/api/types.md
- docs/site/reference/transport.md
- docs/site/technical/architecture.md
- docs/site/technical/lifecycle.md
- docs/site/technical/publication.md
- runtime/local-agent/UPSTREAM.md
- runtime/local-agent/src/main.rs
- runtime/local-agent/src/metadata.rs
- runtime/local-agent/tests/metadata.rs
- runtime/local-agent/vendor/buzz-agent/src/lib.rs
- runtime/local-agent/vendor/buzz-agent/src/llm.rs
- schemas/openapi.json
- scripts/check-boundaries.mjs
- scripts/conversation-metadata-browser.mjs
- scripts/conversation-metadata.acceptance.py
- scripts/governance.test.mjs
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
- spec/src/current/contracts/ui/38-conversation-metadata.md
- spec/src/current/contracts/ui/SUMMARY.nav.md
- spec/src/current/contracts/ui/index.md
- spec/src/current/progress/Bootstrap/history/Bootstrap/index.md
- spec/src/current/progress/M13/history/M13.3/index.md
- spec/src/current/progress/M19/history/M19/index.md
- spec/src/current/progress/M24/SUMMARY.nav.md
- spec/src/current/progress/M24/history/M24/index.md
- spec/src/current/progress/M24/index.md
- spec/src/current/progress/M24/metadata-review.md
- spec/src/current/progress/M25/history/M25/index.md
- spec/src/current/progress/M25/index.md
- web/src/lib/api/generated.ts
- web/src/lib/controller/agent-execution.svelte.ts
- web/src/lib/controller/agent-resource-disposal.ts
- web/src/lib/controller/agent-settings.svelte.ts
- web/src/lib/controller/conversation-metadata.svelte.ts
- web/src/lib/ui/community-chat-composer/CommunityChatComposer.svelte
- web/src/lib/ui/community-chat-composer/composer-document-actions.svelte.ts
- web/src/lib/ui/community-chat-composer/composer-submission.ts
- web/src/lib/ui/community-chat-composer/props.ts
- web/src/lib/ui/community-inbox/CommunityInbox.svelte
- web/src/lib/ui/store-agent-chat/StoreAgentChatSidebar.svelte
- web/src/lib/ui/store-agent-chat/StoreLiveConversation.svelte
- web/src/lib/ui/store-agent-chat-body/StoreAgentChatBody.svelte
- web/src/lib/ui/store-agent-chat-body/comment-submission.ts
- web/src/lib/ui/store-agent-chat-body/model.ts
- web/src/lib/ui/store-agent-execution/StoreAgentChatActions.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentConfiguration.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentExecutionDirectory.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentExecutionHost.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentModelSelect.svelte
- web/src/lib/ui/store-agent-execution/StoreConversationMetadata.svelte
- web/src/lib/ui/store-agent-execution/session-presentation.ts
- web/src/lib/ui/store-collaboration/StoreAgentContext.svelte
- web/src/lib/ui/store-collaboration/StoreAgentSessionMenu.svelte
- web/src/lib/ui/store-collaboration/StoreAgentSessionRow.svelte
- web/src/lib/ui/store-collaboration/StoreAgentSessions.svelte
- web/src/lib/ui/store-collaboration/StoreAgentSidebar.svelte
- web/src/lib/ui/store-settings/StoreAgentDefaults.svelte
- web/src/lib/ui/store-settings/StoreConversationMetadataSettings.svelte
- web/tests/vite/agent-mention-settings.test.ts
- web/tests/vite/agent-settings.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/agent_execution/metadata.rs#metadata_generates_once_preserves_custom_titles_and_recovers_repository_state
- crates/jj-store-cli/tests/agent_execution/metadata.rs#metadata_does_not_backfill_and_cancellation_retains_titles_without_replaying_inference
- crates/jj-store-cli/tests/agent_execution/metadata_recovery.rs#metadata_stale_generation_and_unknown_publication_recover_without_new_inference
- crates/jj-store-cli/tests/agent_execution/metadata_recovery.rs#metadata_acp_suggestions_are_repository_backed_without_a_generation_provider
- scripts/contracts.test.mjs#openapi_document_validates_without_external_references
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-CHAT-METADATA-MAIN-001 details 2](details-02.md)
- [E-CHAT-METADATA-MAIN-001 details 3](details-03.md)
