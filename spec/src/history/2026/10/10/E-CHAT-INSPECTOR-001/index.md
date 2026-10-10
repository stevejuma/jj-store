---
schema: 1
id: E-CHAT-INSPECTOR-001
---

# E-CHAT-INSPECTOR-001

<!-- evidence-field:id markdown -->
## Id

E-CHAT-INSPECTOR-001
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-10
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M24
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-015
- JST-UI-018
- JST-API-001
- JST-API-004
- JST-API-006
- JST-JJ-003
- JST-JJ-004
- JST-MUT-001
- JST-CLI-004
- JST-DOC-001
- JST-DOC-002
- JST-DOC-003
- JST-COLLAB-034
- JST-COLLAB-035
- JST-COLLAB-036
- JST-COMPAT-003
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/AgentChatChanges.stories.svelte
- catalog/components/store-application/AgentSessionState.stories.svelte
- catalog/components/store-application/WorkspaceFiles.stories.svelte
- catalog/components/store-application/WorkspaceInspector.stories.svelte
- catalog/fixtures/AgentFleetApplicationFixture.svelte
- catalog/fixtures/WorkspaceFilesFixture.svelte
- catalog/fixtures/agent-fleet-changes.ts
- crates/jj-store-cli/tests/agent_execution/workspace_inspection.rs
- crates/jj-store-jj/src/agent_workspaces/inspection.rs
- crates/jj-store-server/src/agent_execution/workspace_inspection/content.rs
- crates/jj-store-server/src/agent_execution/workspace_inspection.rs
- crates/jj-store-service/src/agent_execution/workspace_inspection/files.rs
- crates/jj-store-service/src/agent_execution/workspace_inspection.rs
- docs/maintenance/coverage.json
- docs/site/guides/agent-execution.md
- docs/site/reference/api/types.md
- docs/site/reference/transport.md
- schemas/openapi.json
- spec/src/current/contracts/api/41-interactive-agent-execution.md
- spec/src/current/contracts/cli/35-interactive-agent-execution.md
- spec/src/current/contracts/collaboration/37-reusable-agent-conversations.md
- spec/src/current/contracts/compatibility/13-interactive-agent-execution.md
- spec/src/current/contracts/documentation/01-application-documentation.md
- spec/src/current/contracts/jj-integration/32-interactive-agent-workspaces.md
- spec/src/current/contracts/mutations/22-interactive-agent-execution.md
- spec/src/current/contracts/ui/37-document-conversations.md
- spec/src/current/progress/M24/SUMMARY.nav.md
- spec/src/current/progress/M24/index.md
- spec/src/current/progress/M24/workspace-inspector-review.md
- spec/src/history/2026/10/10/E-CHAT-INSPECTOR-001/details-02.md
- spec/src/history/2026/10/10/E-CHAT-INSPECTOR-001/details-03.md
- spec/src/history/2026/10/10/E-CHAT-INSPECTOR-001/index.md
- spec/src/history/2026/10/10/SUMMARY.nav.md
- spec/src/history/2026/10/10/index.md
- web/src/lib/api/generated.ts
- web/src/lib/controller/agent-turn-changes.ts
- web/src/lib/controller/agent-workspace-changes.ts
- web/src/lib/conversations/inspection.svelte.ts
- web/src/lib/conversations/workspace-bindings.ts
- web/src/lib/conversations/workspace-browser.svelte.ts
- web/src/lib/conversations/workspace-observation.ts
- web/src/lib/ui/community-agent-composer-activity/CommunityAgentComposerActivity.svelte
- web/src/lib/ui/community-diff-controls/CommunityDiffControls.svelte
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryFileTree.svelte
- web/src/lib/ui/store-agent-chat/StoreAgentBindings.svelte
- web/src/lib/ui/store-agent-chat/StoreAgentChangesPanel.svelte
- web/src/lib/ui/store-agent-chat/StoreAgentChatSidebar.svelte
- web/src/lib/ui/store-agent-chat/StoreConversationAgentSupplement.svelte
- web/src/lib/ui/store-agent-chat/StoreLinkedConversationActivity.svelte
- web/src/lib/ui/store-agent-chat/StoreLiveConversation.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentConversation.svelte
- web/src/lib/ui/store-agent-workspace/StoreWorkspaceFiles.svelte
- web/src/lib/ui/store-agent-workspace/StoreWorkspaceInspector.svelte
- web/src/lib/ui/store-application/StoreApplication.svelte
- web/src/lib/ui/store-application/view-tabs.css
- web/src/lib/ui/store-collaboration/StoreRecordedConversation.svelte
- web/src/lib/ui/store-threads/StoreThreadConversation.svelte
- web/tests/vite/agent-workspace-changes.test.ts
- web/tests/vite/workspace-browser.test.ts
- web/src/lib/ui/store-agent-chat/StoreAgentChangesDiff.svelte
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-jj/src/agent_workspaces/inspection.rs#text_counts_keep_shared_lines_and_refuse_unsupported_content
- crates/jj-store-service/src/agent_execution/workspace_inspection.rs#legacy_file_entries_keep_unknown_counts_and_new_counts_round_trip
- crates/jj-store-cli/tests/agent_execution/conversations.rs#repository_free_chat_reuses_isolated_bindings_and_never_replays_after_restart
- web/tests/vite/agent-workspace-changes.test.ts#historical turn cards preserve
  every completed request's own repository boundaries during later work
- web/tests/vite/workspace-browser.test.ts#saved turn rail and inspector reuse
  immutable bytes, preserve old boundaries and never read today's workspace
<!-- /evidence-field -->

<!-- evidence-field:story_cases list -->
## Story cases

- catalog/components/store-application/AgentChatChanges.stories.svelte#Selected
  live avatar and completed file panel
- catalog/components/store-application/WorkspaceInspector.stories.svelte#Changed
  file header owns responsive presentation tabs and icon diff controls
- catalog/components/store-application/WorkspaceInspector.stories.svelte#Files
  and Changes retain compact root icons and nested indentation
<!-- /evidence-field -->

