---
schema: 1
id: E-TEST-REPAIR-LAND-001
---

# E-TEST-REPAIR-LAND-001

<!-- evidence-field:schema yaml -->
## Schema

```yaml
1

```
<!-- /evidence-field -->

<!-- evidence-field:id markdown -->
## Id

E-TEST-REPAIR-LAND-001
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

- JST-UI-001
- JST-UI-005
- JST-UI-015
- JST-UI-018
- JST-UI-019
- JST-API-001
- JST-API-006
- JST-CLI-004
- JST-GOV-002
- JST-GOV-008
- JST-COLLAB-034
- JST-COLLAB-035
- JST-COLLAB-036
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/AgentChatFeedback.stories.svelte
- catalog/components/store-application/AgentChatRuntime.stories.svelte
- catalog/components/store-application/AgentDiscussion.stories.svelte
- catalog/components/store-application/AgentExecution.stories.svelte
- catalog/components/store-application/AgentReplyIndependence.stories.svelte
- catalog/components/store-application/AgentSessionState.stories.svelte
- catalog/components/store-application/ConversationCaret.stories.svelte
- catalog/components/store-application/ManagedAgentIdentity.stories.svelte
- catalog/components/store-application/RepositoryReview.stories.svelte
- catalog/components/store-application/repository-directory-story-assertions.ts
- catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte
- catalog/components/store-stack-workspace/StoreStackWorkspaceForms.stories.svelte
- catalog/components/store-stack-workspace/StoreStackWorkspacePresentation.stories.svelte
- catalog/components/store-stack-workspace/StoreStackWorkspaceSetup.stories.svelte
- catalog/components/store-stack-workspace/StoreStackWorkspaceSpacer.stories.svelte
- catalog/fixtures/AgentExecutionFixture.svelte
- catalog/fixtures/AgentExecutionPendingControls.svelte
- catalog/fixtures/AgentInlineCommentsFixture.svelte
- catalog/fixtures/StackWorkspaceProductionFixture.svelte
- catalog/fixtures/agent-command-receipts.ts
- catalog/fixtures/agent-inline-comment-play.ts
- catalog/fixtures/agent-inline-comment-transport.ts
- catalog/fixtures/agent-session-recovery.ts
- catalog/fixtures/stack-workspace/production-http.ts
- crates/jj-store-cli/tests/agent_execution/conversation_failures.rs
- crates/jj-store-cli/tests/agent_execution.rs
- crates/jj-store-server/src/agent_execution/commands.rs
- crates/jj-store-server/src/agent_execution/conversation_frames.rs
- crates/jj-store-server/src/agent_execution/fleet.rs
- crates/jj-store-server/src/agent_execution/mod.rs
- crates/jj-store-server/src/agent_execution/participation_resume.rs
- docs/maintenance/coverage.json
- docs/site/guides/agent-execution.md
- docs/site/guides/settings.md
- docs/site/technical/lifecycle.md
- docs/ui-provenance.json
- scripts/dev-agent-probe.mjs
- scripts/dev-agent-runtime.mjs
- scripts/dev-agent-runtime.test.mjs
- scripts/dev-ui.test.mjs
- spec/src/current/contracts/api/41-interactive-agent-execution.md
- spec/src/current/contracts/cli/35-interactive-agent-execution.md
- spec/src/current/contracts/collaboration/37-reusable-agent-conversations.md
- spec/src/current/contracts/documentation/01-application-documentation.md
- spec/src/current/contracts/ui/13-shared-application.md
- spec/src/current/contracts/ui/34-interactive-agent-sessions.md
- spec/src/current/contracts/ui/37-document-conversations.md
- spec/src/current/governance/spec-governance.md
- spec/src/current/progress/M24/SUMMARY.index-index/002/index.md
- spec/src/current/progress/M24/SUMMARY.nav.md
- spec/src/current/progress/M24/footer-review.md
- spec/src/current/progress/M24/history/M24/index.md
- spec/src/current/progress/M24/index.md
- spec/src/current/progress/M24/test-repairs-review.md
- spec/src/history/2026/10/10/E-AGENT-RESUME-001/SUMMARY.nav.md
- spec/src/history/2026/10/10/E-AGENT-RESUME-001/details-02.md
- spec/src/history/2026/10/10/E-AGENT-RESUME-001/details-03.md
- spec/src/history/2026/10/10/E-AGENT-RESUME-001/details-04.md
- spec/src/history/2026/10/10/E-AGENT-RESUME-001/index.md
- spec/src/history/2026/10/10/E-AGENT-STATE-001/SUMMARY.nav.md
- spec/src/history/2026/10/10/E-AGENT-STATE-001/details-02.md
- spec/src/history/2026/10/10/E-AGENT-STATE-001/details-03.md
- spec/src/history/2026/10/10/E-AGENT-STATE-001/details-04.md
- spec/src/history/2026/10/10/E-AGENT-STATE-001/index.md
- spec/src/history/2026/10/10/E-TEST-REPAIR-001/SUMMARY.nav.md
- spec/src/history/2026/10/10/E-TEST-REPAIR-001/details-02.md
- spec/src/history/2026/10/10/E-TEST-REPAIR-001/details-03.md
- spec/src/history/2026/10/10/E-TEST-REPAIR-001/details-04.md
- spec/src/history/2026/10/10/E-TEST-REPAIR-001/index.md
- spec/src/history/2026/10/10/E-TEST-REPAIR-002/SUMMARY.nav.md
- spec/src/history/2026/10/10/E-TEST-REPAIR-002/details-02.md
- spec/src/history/2026/10/10/E-TEST-REPAIR-002/details-03.md
- spec/src/history/2026/10/10/E-TEST-REPAIR-002/index.md
- spec/src/history/2026/10/10/E-TEST-REPAIR-LAND-001/SUMMARY.nav.md
- spec/src/history/2026/10/10/E-TEST-REPAIR-LAND-001/details-02.md
- spec/src/history/2026/10/10/E-TEST-REPAIR-LAND-001/details-03.md
- spec/src/history/2026/10/10/E-TEST-REPAIR-LAND-001/index.md
- spec/src/history/2026/10/10/SUMMARY.nav.md
- spec/src/history/2026/10/10/index.md
- web/src/lib/controller/agent-execution-refresh.ts
- web/src/lib/controller/agent-execution.svelte.ts
- web/src/lib/controller/agent-runtime-observation.ts
- web/src/lib/controller/agent-selection-history.ts
- web/src/lib/controller/agent-session-activity.ts
- web/src/lib/controller/agent-session-stop.svelte.ts
- web/src/lib/controller/production-store-mutations.ts
- web/src/lib/controller/production-store-search-controller.ts
- web/src/lib/conversations/ordered-activity.ts
- web/src/lib/conversations/prepare-standalone-runtime.ts
- web/src/lib/conversations/standalone-send.svelte.ts
- web/src/lib/ui/community-agent-composer-activity/CommunityAgentComposerActivity.svelte
- web/src/lib/ui/community-chat-composer/CommunityComposerSurface.svelte
- web/src/lib/ui/community-projects/repository-directory.css
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
- web/src/lib/ui/store-agent-execution/conversation-error.ts
- web/src/lib/ui/store-agent-execution/runtime-preparation.ts
- web/src/lib/ui/store-agent-execution/session-presentation.ts
- web/src/lib/ui/store-collaboration/StoreRecordedConversation.svelte
- web/src/lib/ui/store-threads/StoreThreadConversation.svelte
- web/tests/agent-inbox-groups.test.ts
- web/tests/agent-session-activity.test.ts
- web/tests/conversation-ordering.test.ts
- web/tests/vite/agent-freshness.test.ts
- web/tests/vite/agent-runtime-preparation.test.ts
- web/tests/vite/agent-session-stop.test.ts
- web/tests/vite/conversation-create.test.ts
- web/tests/vite/local-browser-merge.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/vite/agent-discussions.test.ts#an ended agent accepts ordinary
  replies after feedback refresh fails
- web/tests/vite/agent-discussions.test.ts#discussion without a feed loads once
  and never polls
- web/tests/vite/conversation-projection.test.ts#several validated answers
  retain ordinary prose in one turn and shared comments use one destination
- web/tests/vite/conversation-projection.test.ts#foreign or malformed
  unvalidated tags stay readable and changed captures disable selection
- web/tests/vite/conversation-projection.test.ts#multiple agents merge immutable
  comment revisions while preserving separate answer links
- web/tests/vite/conversation-projection.test.ts#patch file and numbered-read
  slices preserve exact captured pointers and Unicode offsets
- web/tests/vite/conversation-projection.test.ts#recorded rich captures without
  a plain-text mapping support whole sections without inventing ranges
- web/tests/vite/conversation-projection.test.ts#shared answers repeated in
  feedback and spanning tool boundaries have stable unique fragments
- web/tests/vite/conversation-projection.test.ts#committed comments-only user
  rows retain their canonical batch and turn
- web/tests/vite/conversation-projection.test.ts#execute output uses committed
  stdout parts instead of the terminal snapshot envelope
- web/tests/vite/conversation-projection.test.ts#execute output aliases preserve
  empty and stderr captures without fabricating terminal text
- web/tests/vite/conversation-projection.test.ts#terminal-only committed
  captures show retained execution output with whole-section comments
- web/tests/vite/conversation-projection.test.ts#recorded tools retain command
  and output without writable capture targets
- web/tests/vite/conversation-reactions.test.ts#linked answer reactions use the
  capture's repository reader and original row identity
- web/tests/vite/conversation-session.test.ts#session drafts survive remounts
  and exact publication reconciles without duplicates
- web/tests/vite/conversation-session.test.ts#unknown publication freezes the
  original snapshot and rejects duplicate sends until receipt recovery
- web/tests/vite/conversation-session.test.ts#editing a sent comment retains its
  old answered revision and prepares a checked replacement
- web/tests/vite/conversation-session.test.ts#published mixed submission with
  failed delivery retains one request without replaying delivered recipients
- web/tests/vite/conversation-session.test.ts#successful receipt clears only the
  original text and retains the canonical publication ID
- web/tests/vite/conversation-session.test.ts#authoritative tombstones remove
  published revisions while unavailable reads and local drafts retain them
- web/tests/vite/agent-runtime-preparation.test.ts#old ready capabilities cannot
  bypass a pending exact configure receipt
- web/tests/vite/agent-runtime-preparation.test.ts#Send prepares a disconnected
  session once and observes readiness without dispatching a prompt
- web/tests/vite/agent-runtime-preparation.test.ts#changing selection during
  opening does not retarget the captured submission
- web/tests/vite/agent-runtime-preparation.test.ts#accepted unrelated receipt
  cannot authorize the captured prompt
- web/tests/agent-execution.test.ts#queue cancellation targets only prompts that
  have not started or settled
- web/tests/agent-execution.test.ts#runtime choices prefer advertised
  configuration values and retain groups
- web/tests/agent-execution.test.ts#live text and tool deltas retain turn
  identity and exact permission lifetimes
- web/tests/agent-execution.test.ts#message times retain first observation
  across chunks and never fabricate legacy times
- web/tests/agent-execution.test.ts#inbox day dividers share thread labels and
  group genuinely unrecorded times
- web/tests/agent-execution.test.ts#resume history is not a new message or
  feedback target
- web/tests/production-store-mutations.test.ts#preview and apply retain one
  exact request through commitment
- web/tests/production-store-mutations.test.ts#unknown outcomes keep the request
  identity for status recovery
- web/tests/production-store-mutations.test.ts#late preview responses cannot
  repopulate a departed session
- web/tests/production-store-mutations.test.ts#reconciliation captures fresh
  heads without changing browsing and retains unknown intent
- web/tests/production-store-search.test.ts#preserves source strings and full
  native selection identities
- web/tests/production-store-search.test.ts#routes commit and operation results
  without confusing identities
- web/tests/production-store-search.test.ts#rejects search pages from another
  snapshot
- web/tests/production-store-search.test.ts#ignores a late response from an
  older query
- web/tests/production-store-search.test.ts#continues with the opaque cursor and
  deduplicates overlapping pages
- web/tests/production-store-search.test.ts#presents cursor reset failures
  without retaining a refreshing state
- web/tests/conversation-ordering.test.ts#assistant chunks preserve tool
  boundaries and exact whole-message offsets
- web/tests/conversation-ordering.test.ts#legacy answers retain explicit unknown
  ordering, inconsistent streams have no exact selection
- web/tests/conversation-ordering.test.ts#each assistant segment retains its
  first valid time without borrowing neighboring times
- web/tests/conversation-ordering.test.ts#legacy final answers use only a valid
  observation time from their own record
- web/tests/conversation-source-map.test.ts#exact source maps trimmed Markdown,
  fences and Unicode in both directions
- web/tests/conversation-source-map.test.ts#source maps refuse stale revisions,
  invented text, split surrogate pairs and changed hashes
- web/tests/agent-session-activity.test.ts#an opening handshake can be stopped
  before the session becomes connected
- web/tests/agent-session-activity.test.ts#interruption outranks a stale
  dispatch receipt and does not animate Starting
- web/tests/agent-session-activity.test.ts#Stop uses the exact uncompleted turn,
  ignores handshake and terminal history, and never targets disconnected
  sessions
- web/tests/agent-inbox-groups.test.ts#repository-backed conversations group by
  code attachments with their own operation and workspace, never their metadata
  home
- web/tests/agent-inbox-groups.test.ts#repository groups retain service order,
  distinct identities and exact per-chat links
- web/tests/agent-inbox-groups.test.ts#empty inboxes do not invent repository
  groups
- web/tests/agent-inbox-groups.test.ts#pins form the first expandable group and
  collapse repository aliases into one canonical row
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-TEST-REPAIR-LAND-001 details 2](details-02.md)
- [E-TEST-REPAIR-LAND-001 details 3](details-03.md)
- [E-TEST-REPAIR-LAND-001 details 4](details-04.md)
