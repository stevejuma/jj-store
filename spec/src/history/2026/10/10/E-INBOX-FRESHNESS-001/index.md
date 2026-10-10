---
schema: 1
id: E-INBOX-FRESHNESS-001
---

# E-INBOX-FRESHNESS-001

<!-- evidence-field:id markdown -->
## Id

E-INBOX-FRESHNESS-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M24
- M25
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-005
- JST-UI-011
- JST-UI-012
- JST-UI-015
- JST-UI-017
- JST-API-001
- JST-COLLAB-034
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

- catalog/components/store-application/AgentChatChanges.stories.svelte
- catalog/components/store-application/AgentInboxNavigation.stories.svelte
- catalog/components/store-application/StoreRepositoryNavigation.stories.svelte
- catalog/components/store-threads/StoreThreadNavigation.stories.svelte
- catalog/fixtures/AgentExecutionFixture.svelte
- catalog/fixtures/AgentFleetApplicationFixture.svelte
- catalog/fixtures/StoreThreadsFixture.svelte
- catalog/fixtures/agent-execution-lifecycle.svelte.ts
- catalog/fixtures/agent-fleet-changes.ts
- spec/src/current/contracts/api/35-scoped-application-freshness.md
- spec/src/current/contracts/ui/22-automatic-application-updates.md
- spec/src/current/contracts/ui/32-event-driven-freshness-migration.md
- spec/src/current/progress/M24/SUMMARY.nav.md
- spec/src/current/progress/M24/history/M24/index.md
- spec/src/current/progress/M24/inbox-freshness-review.md
- spec/src/current/progress/M24/index.md
- spec/src/current/progress/M25/history/M25/index.md
- spec/src/current/progress/M25/index.md
- spec/src/history/2026/10/10/E-INBOX-FRESHNESS-001/SUMMARY.nav.md
- spec/src/history/2026/10/10/E-INBOX-FRESHNESS-001/details-02.md
- spec/src/history/2026/10/10/E-INBOX-FRESHNESS-001/details-03.md
- spec/src/history/2026/10/10/E-INBOX-FRESHNESS-001/details-04.md
- spec/src/history/2026/10/10/E-INBOX-FRESHNESS-001/index.md
- spec/src/history/2026/10/10/SUMMARY.nav.md
- spec/src/history/2026/10/10/index.md
- web/src/lib/api/local-client.ts
- web/src/lib/controller/agent-changes-cache.ts
- web/src/lib/controller/agent-chat-navigation.svelte.ts
- web/src/lib/controller/agent-chat.svelte.ts
- web/src/lib/controller/agent-compute.svelte.ts
- web/src/lib/controller/agent-discussion-reader.ts
- web/src/lib/controller/agent-discussion-refresh.ts
- web/src/lib/controller/agent-discussions.svelte.ts
- web/src/lib/controller/agent-execution-launch-context.ts
- web/src/lib/controller/agent-execution-refresh.ts
- web/src/lib/controller/agent-execution-stream.ts
- web/src/lib/controller/agent-execution.svelte.ts
- web/src/lib/controller/agent-freshness.ts
- web/src/lib/controller/agent-history-cache.ts
- web/src/lib/controller/agent-resource-disposal.ts
- web/src/lib/controller/agent-runtimes.svelte.ts
- web/src/lib/controller/agent-selection-history.ts
- web/src/lib/controller/application-freshness.ts
- web/src/lib/controller/automatic-refresh.ts
- web/src/lib/controller/capture-refresh.ts
- web/src/lib/controller/conversation-metadata.svelte.ts
- web/src/lib/controller/http-threads-data-source.ts
- web/src/lib/controller/local-inbox-threads.ts
- web/src/lib/controller/production-automatic-refresh.ts
- web/src/lib/controller/production-projects-hub.svelte.ts
- web/src/lib/controller/production-projects-integration.ts
- web/src/lib/controller/production-store-connect.ts
- web/src/lib/controller/production-store-controller.svelte.ts
- web/src/lib/controller/projects-command-state.ts
- web/src/lib/controller/projects-navigation.svelte.ts
- web/src/lib/controller/projects-refresh-scope.ts
- web/src/lib/controller/repository-thread-navigation.svelte.ts
- web/src/lib/controller/retained-reads.ts
- web/src/lib/controller/settings-refresh.ts
- web/src/lib/ui/community-inbox/CommunityInbox.svelte
- web/src/lib/ui/community-navigation/CommunityNavigation.svelte
- web/src/lib/ui/store-agent-chat/StoreAgentBindings.svelte
- web/src/lib/ui/store-agent-chat/StoreAgentChangesPanel.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentConversation.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentExecutionDirectory.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentExecutionHost.svelte
- web/src/lib/ui/store-agent-execution/StoreConversationMetadata.svelte
- web/src/lib/ui/store-application/StoreApplicationNavigation.svelte
- web/src/lib/ui/store-threads/thread-navigation-binding.svelte.ts
- web/src/lib/ui/store-threads/thread-navigation.ts
- web/tests/vite/agent-changes-cache.test.ts
- web/tests/vite/agent-discussions.test.ts
- web/tests/vite/agent-execution-controller.test.ts
- web/tests/vite/agent-freshness.test.ts
- web/tests/vite/application-freshness.test.ts
- web/tests/vite/automatic-refresh.test.ts
- web/tests/vite/capture-freshness.test.ts
- web/tests/vite/inbox-request-lifecycle.test.ts
- web/tests/vite/production-automatic-refresh.test.ts
- web/tests/vite/sidebar-read-lifecycle.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/vite/agent-changes-cache.test.ts#shared immutable previews survive
  one observer leaving and are reused on remount
- web/tests/vite/agent-discussions.test.ts#an ended agent accepts ordinary
  replies after feedback refresh fails
- web/tests/vite/agent-execution-controller.test.ts#Activity log opens the
  associated active chat, retains exact historical links and never launches
- web/tests/vite/agent-freshness.test.ts#dirty failed settings reads stop while
  the advertised feed is offline
- web/tests/vite/application-freshness.test.ts#split frames and heartbeats
  validate before advancing continuity
- web/tests/vite/automatic-refresh.test.ts#live events coalesce and an event
  during a reload schedules another reload
- web/tests/vite/capture-freshness.test.ts#busy capture observations wait
  without reporting unavailable or replacing snapshots
- web/tests/vite/inbox-request-lifecycle.test.ts#returning to a chat reuses
  history and feedback, opens one stream and retains drafts
- web/tests/vite/production-automatic-refresh.test.ts#repository views remain
  idle without an event feed
- web/tests/vite/sidebar-read-lifecycle.test.ts#removed repository dirtiness
  cannot keep project navigation refreshing
- web/tests/vite/inbox-request-lifecycle.test.ts#history captured before initial
  status reuses its exact committed boundary
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-INBOX-FRESHNESS-001 details 2](details-02.md)
- [E-INBOX-FRESHNESS-001 details 3](details-03.md)
- [E-INBOX-FRESHNESS-001 details 4](details-04.md)
- [E-INBOX-FRESHNESS-001 details 5](details-05.md)
