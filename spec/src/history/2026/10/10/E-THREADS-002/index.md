---
schema: 1
id: E-THREADS-002
---

# E-THREADS-002

<!-- evidence-field:id markdown -->
## Id

E-THREADS-002
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M24
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-012
- JST-UI-015
- JST-GOV-009
- JST-COLLAB-035
- JST-DOC-001
- JST-DOC-002
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

- catalog/components/store-application/AgentInboxNavigation.stories.svelte
- catalog/components/store-application/StoreRepositoryPicker.stories.svelte
- catalog/components/store-application/WorkspaceFiles.stories.svelte
- catalog/components/store-threads/StoreThreadAlignment.stories.svelte
- catalog/components/store-threads/StoreThreadNavigation.stories.svelte
- catalog/components/store-threads/StoreThreadSearch.stories.svelte
- catalog/components/store-threads/StoreThreads.stories.svelte
- catalog/fixtures/StoreThreadsFixture.svelte
- catalog/fixtures/thread-reaction-play.ts
- catalog/fixtures/thread-transport.ts
- crates/jj-collab/src/commands.rs
- crates/jj-collab/src/graph.rs
- crates/jj-collab/src/lib.rs
- crates/jj-collab/src/thread_codebase.rs
- crates/jj-collab/src/threads.rs
- crates/jj-collab/tests/target_resolution.rs
- crates/jj-collab/tests/thread_codebase.rs
- crates/jj-store-cli/tests/agent_execution/comment_answers.rs
- crates/jj-store-cli/tests/agent_execution/context_delivery.rs
- crates/jj-store-cli/tests/agent_execution/conversations.rs
- crates/jj-store-cli/tests/agent_execution/metadata.rs
- crates/jj-store-cli/tests/agent_execution/real.rs
- crates/jj-store-cli/tests/agent_execution/thread_workspace.rs
- crates/jj-store-cli/tests/agent_execution.rs
- crates/jj-store-http/src/agent_execution.rs
- crates/jj-store-http/src/collab/commands.rs
- crates/jj-store-http/src/local_browser/collaboration.rs
- crates/jj-store-server/src/agent_execution/commands.rs
- crates/jj-store-server/src/agent_execution/comment_submissions.rs
- crates/jj-store-server/src/agent_execution/conversation_links.rs
- crates/jj-store-server/src/agent_execution/conversation_origin.rs
- crates/jj-store-server/src/agent_execution/conversations.rs
- crates/jj-store-server/src/agent_execution/journal.rs
- crates/jj-store-server/src/agent_execution/metadata_store.rs
- crates/jj-store-server/src/agent_execution/mod.rs
- crates/jj-store-server/src/agent_execution/thread_workspace.rs
- crates/jj-store-server/src/agent_execution/workspace_turns.rs
- crates/jj-store-service/src/agent_execution/conversations.rs
- crates/jj-store-service/src/agent_execution/thread_workspace.rs
- crates/jj-store-service/src/agent_execution.rs
- crates/jj-store-service/src/collab_reads.rs
- crates/jj-store-service/src/content_search/index.rs
- crates/jj-store-service/src/local_repository/browser_collab.rs
- docs/maintenance/coverage.json
- docs/site/concepts/collaboration.md
- docs/site/guides/browsing.md
- docs/site/reference/api/collaboration.md
- docs/site/reference/api/installation.md
- docs/site/reference/api/types.md
- docs/site/reference/transport.md
- docs/site/technical/lifecycle.md
- schemas/openapi.json
- scripts/dev-demo-threads.mjs
- scripts/dev-demo-threads.test.mjs
- scripts/dev-demo.mjs
- spec/src/current/contracts/api/44-thread-workspaces.md
- spec/src/current/contracts/api/SUMMARY.nav.md
- spec/src/current/contracts/api/index.md
- spec/src/current/contracts/cli/37-conversation-metadata.md
- spec/src/current/contracts/collaboration/03-requirements.md
- spec/src/current/contracts/collaboration/37-reusable-agent-conversations.md
- spec/src/current/contracts/collaboration/43-intentional-threads-and-codebases.md
- spec/src/current/contracts/collaboration/SUMMARY.nav.md
- spec/src/current/contracts/collaboration/index.md
- spec/src/current/contracts/compatibility/37-intentional-thread-formats.md
- spec/src/current/contracts/compatibility/SUMMARY.nav.md
- spec/src/current/contracts/compatibility/index.md
- spec/src/current/contracts/documentation/01-application-documentation.md
- spec/src/current/contracts/mutations/22-interactive-agent-execution.md
- spec/src/current/contracts/ui/39-intentional-thread-inbox.md
- spec/src/current/contracts/ui/SUMMARY.nav.md
- spec/src/current/contracts/ui/index.md
- spec/src/current/governance/spec-governance.md
- spec/src/current/progress/M24/SUMMARY.index-index/002/index.md
- spec/src/current/progress/M24/SUMMARY.nav.md
- spec/src/current/progress/M24/history/M24/index.md
- spec/src/current/progress/M24/index.md
- spec/src/current/progress/M24/intentional-threads-review.md
- web/src/lib/api/generated.ts
- web/src/lib/community/community-link-normalize.ts
- web/src/lib/controller/agent-chat-context.ts
- web/src/lib/controller/agent-chat-selections.ts
- web/src/lib/controller/agent-chat-types.ts
- web/src/lib/controller/agent-chat.svelte.ts
- web/src/lib/controller/contracts.ts
- web/src/lib/controller/http-store-data-source.ts
- web/src/lib/controller/production-entity-search.ts
- web/src/lib/controller/production-profile-route.ts
- web/src/lib/controller/production-store-connect.ts
- web/src/lib/controller/production-store-controller.svelte.ts
- web/src/lib/controller/production-store-lifecycle.svelte.ts
- web/src/lib/controller/production-store-view.ts
- web/src/lib/controller/production-threads-integration.ts
- web/src/lib/controller/production-threads.svelte.ts
- web/src/lib/controller/store-route-codec.ts
- web/src/lib/controller/store-route-types.ts
- web/src/lib/controller/thread-committed-refresh.ts
- web/src/lib/controller/thread-conversations.svelte.ts
- web/src/lib/controller/thread-workspaces.svelte.ts
- web/src/lib/controller/threads-route-codec.ts
- web/src/lib/conversations/inspection.svelte.ts
- web/src/lib/conversations/repository-comparison.ts
- web/src/lib/conversations/repository-inspection.ts
- web/src/lib/conversations/session.svelte.ts
- web/src/lib/conversations/thread-send.ts
- web/src/lib/conversations/workspace-bindings.ts
- web/src/lib/conversations/workspace-browser.svelte.ts
- web/src/lib/ui/community-channel-settings/CommunityChannelSettings.svelte
- web/src/lib/ui/community-inbox/CommunityInbox.svelte
- web/src/lib/ui/community-inbox/CommunityInboxItem.svelte
- web/src/lib/ui/community-inbox/CommunityInboxSessions.svelte
- web/src/lib/ui/community-inbox/inbox.css
- web/src/lib/ui/community-inbox/model.ts
- web/src/lib/ui/store-agent-chat/StoreAgentChatRepositories.svelte
- web/src/lib/ui/store-agent-chat-body/ChatContent.svelte
- web/src/lib/ui/store-agent-chat-body/ChatHoverActions.svelte
- web/src/lib/ui/store-agent-chat-body/ChatTextBlock.svelte
- web/src/lib/ui/store-agent-chat-body/StoreAgentChatBody.svelte
- web/src/lib/ui/store-agent-chat-body/chat-body.css
- web/src/lib/ui/store-agent-chat-body/model.ts
- web/src/lib/ui/store-agent-workspace/StoreWorkspaceButton.svelte
- web/src/lib/ui/store-application/StoreApplication.svelte
- web/src/lib/ui/store-application/StoreApplicationMain.svelte
- web/src/lib/ui/store-application/StoreApplicationNavigation.svelte
- web/src/lib/ui/store-application/StoreApplicationRightPanels.svelte
- web/src/lib/ui/store-application/StoreRepositoryPicker.svelte
- web/src/lib/ui/store-application/entity-context.ts
- web/src/lib/ui/store-application/store-navigation.ts
- web/src/lib/ui/store-search/StoreSearchOverlay.svelte
- web/src/lib/ui/store-threads/StoreConversationMessageActions.svelte
- web/src/lib/ui/store-threads/StoreThreadAgentRepositories.svelte
- web/src/lib/ui/store-threads/StoreThreadCodebase.svelte
- web/src/lib/ui/store-threads/StoreThreadConversation.svelte
- web/src/lib/ui/store-threads/StoreThreadCreate.svelte
- web/src/lib/ui/store-threads/StoreThreadSettings.svelte
- web/src/lib/ui/store-threads/StoreThreads.svelte
- web/src/lib/ui/store-threads/StoreThreadsBreadcrumb.svelte
- web/src/lib/ui/store-threads/thread-inbox.ts
- web/src/lib/ui/store-threads/thread-navigation.ts
- web/tests/native-entity-links.test.ts
- web/tests/vite/intentional-thread-workspaces.test.ts
- web/tests/vite/production-threads.test.ts
- web/tests/vite/repository-inspection.test.ts
- web/tests/vite/thread-navigation-integration.test.ts
- web/tests/vite/thread-navigation.test.ts
- web/tests/vite/thread-workspace-publication.test.ts
- web/tests/vite/workspace-browser.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- scripts/dev-demo-threads.test.mjs#explicit demo Threads cover empty and
  multiple codebases without replaying after startup
- web/tests/native-entity-links.test.ts#Thread references normalize legacy
  channels while Review thread identity remains distinct
- web/tests/native-entity-links.test.ts#slash picker inserts useful native
  titles and keeps complete IDs in Markdown routes
- web/tests/vite/intentional-thread-workspaces.test.ts#Thread inbox routes
  retain canonical metadata home, repository filter and reply identity
- web/tests/vite/intentional-thread-workspaces.test.ts#unknown Thread
  publication retains the exact request, busy refusal releases it for editing
- web/tests/vite/production-threads.test.ts#human replies retain exact parent
  revision and edit identity
- web/tests/vite/production-threads.test.ts#unknown outcomes retain and retry
  the original request, then refresh
- web/tests/vite/repository-inspection.test.ts#pre-execution browsing pins
  source reads, caches them and never creates agent authority
- web/tests/vite/repository-inspection.test.ts#repository comparison reads both
  exact file revisions and refuses unavailable turn context
- web/tests/vite/thread-navigation-integration.test.ts#selecting a
  repository-owned child binds its owner before opening its exact thread
- web/tests/vite/thread-navigation-integration.test.ts#opening the Threads inbox
  leaves repository code unconnected
- web/tests/vite/thread-navigation.test.ts#repository aliases retain their
  canonical owner and omit Browse controls
- web/tests/vite/thread-navigation.test.ts#thread selection forwards its
  repository instead of reusing the active connection
- web/tests/vite/workspace-browser.test.ts#retained workspace requests keep
  their authority when the active binding changes
- web/tests/vite/workspace-browser.test.ts#file selection joins the existing
  queue and sends exact UTF-8 ranges without transcript rows
- web/tests/vite/thread-workspace-publication.test.ts#Codebase publication
  refreshes the cached metadata-home directory before displaying repository
  aliases
- web/tests/vite/production-threads.test.ts#committed removal returns to the
  inbox without reading the hidden thread
<!-- /evidence-field -->

