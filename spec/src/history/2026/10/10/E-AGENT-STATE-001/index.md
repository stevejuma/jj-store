---
schema: 1
id: E-AGENT-STATE-001
---

# E-AGENT-STATE-001

<!-- evidence-field:id markdown -->
## Id

E-AGENT-STATE-001
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
- JST-UI-015
- JST-UI-018
- JST-CLI-004
- JST-API-001
- JST-API-006
- JST-GOV-002
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

- catalog/components/store-application/AgentChatFeedback.stories.svelte
- catalog/components/store-application/AgentExecution.stories.svelte
- catalog/components/store-application/AgentSessionState.stories.svelte
- catalog/components/store-application/ConversationCaret.stories.svelte
- catalog/fixtures/AgentExecutionFixture.svelte
- catalog/fixtures/AgentExecutionPendingControls.svelte
- catalog/fixtures/agent-command-receipts.ts
- catalog/fixtures/agent-session-recovery.ts
- crates/jj-store-cli/tests/agent_execution/conversation_failures.rs
- crates/jj-store-cli/tests/agent_execution.rs
- crates/jj-store-server/src/agent_execution/commands.rs
- crates/jj-store-server/src/agent_execution/conversation_frames.rs
- crates/jj-store-server/src/agent_execution/fleet.rs
- docs/ui-provenance.json
- spec/src/current/contracts/api/41-interactive-agent-execution.md
- spec/src/current/contracts/cli/35-interactive-agent-execution.md
- spec/src/current/contracts/collaboration/37-reusable-agent-conversations.md
- spec/src/current/contracts/ui/34-interactive-agent-sessions.md
- spec/src/current/contracts/ui/37-document-conversations.md
- spec/src/current/governance/spec-governance.md
- spec/src/current/progress/M24/footer-review.md
- spec/src/current/progress/M24/index.md
- web/src/lib/controller/agent-execution-refresh.ts
- web/src/lib/controller/agent-execution.svelte.ts
- web/src/lib/controller/agent-session-activity.ts
- web/src/lib/controller/agent-session-stop.svelte.ts
- web/src/lib/ui/community-agent-composer-activity/CommunityAgentComposerActivity.svelte
- web/src/lib/ui/community-chat-composer/CommunityComposerSurface.svelte
- web/src/lib/ui/store-agent-chat/StoreAgentChatSidebar.svelte
- web/src/lib/ui/store-agent-chat/StoreAgentMessageFooter.svelte
- web/src/lib/ui/store-agent-chat/StoreLinkedConversationActivity.svelte
- web/src/lib/ui/store-agent-chat/StoreLiveConversation.svelte
- web/src/lib/ui/store-agent-chat-body/ChatHistoryStatus.svelte
- web/src/lib/ui/store-agent-chat-body/ChatMessageFooter.svelte
- web/src/lib/ui/store-agent-chat-body/ChatNextMessage.svelte
- web/src/lib/ui/store-agent-chat-body/StoreAgentChatBody.svelte
- web/src/lib/ui/store-agent-chat-body/chat-body.css
- web/src/lib/ui/store-agent-chat-body/chat-caret.ts
- web/src/lib/ui/store-agent-chat-body/text-reader.css
- web/src/lib/ui/store-agent-execution/StoreAgentConversation.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentRequestRecovery.svelte
- web/src/lib/ui/store-agent-execution/session-presentation.ts
- web/src/lib/ui/store-collaboration/StoreRecordedConversation.svelte
- web/src/lib/ui/store-threads/StoreThreadConversation.svelte
- web/tests/agent-inbox-groups.test.ts
- web/tests/agent-session-activity.test.ts
- web/tests/vite/agent-session-stop.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/agent_execution/conversation_failures.rs#rejected_resume_finishes_receipt_retires_worker_and_never_replays
- crates/jj-store-cli/tests/agent_execution/conversation_failures.rs#rejected_resume_configuration_settles_its_correlated_receipt
- crates/jj-store-cli/tests/agent_execution/conversation_failures.rs#stop_during_resume_cancels_opening_without_resurrection_or_prompt_replay
- web/tests/agent-session-activity.test.ts#interruption outranks a stale
  dispatch receipt and does not animate Starting
- web/tests/agent-session-activity.test.ts#Stop uses the exact uncompleted turn,
  ignores handshake and terminal history, and never targets disconnected
  sessions
- web/tests/vite/agent-session-stop.test.ts#Stop bypasses a pending Send without
  overwriting it, its draft, or another selected session
- web/tests/vite/agent-session-stop.test.ts#an uncertain Stop retains one
  identity; a second click only checks its outcome
- web/tests/agent-session-activity.test.ts#an opening handshake can be stopped
  before the session becomes connected
- web/tests/agent-inbox-groups.test.ts#repository-backed conversations group by
  code attachments with their own operation and workspace, never their metadata
  home
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-AGENT-STATE-001 details 2](details-02.md)
- [E-AGENT-STATE-001 details 3](details-03.md)
- [E-AGENT-STATE-001 details 4](details-04.md)
