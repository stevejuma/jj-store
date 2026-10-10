---
schema: 1
id: E-CHAT-QUEUE-001
---

# E-CHAT-QUEUE-001

<!-- evidence-field:id markdown -->
## Id

E-CHAT-QUEUE-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M24
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-018
- JST-COLLAB-034
- JST-GOV-002
- JST-GOV-008
- JST-API-001
- JST-API-006
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-10
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

tszmluon
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/AgentReplyIndependence.stories.svelte
- catalog/components/store-application/AgentSessionState.stories.svelte
- catalog/components/store-application/QueuedAgentMessages.stories.svelte
- catalog/fixtures/QueuedAgentMessagesFixture.svelte
- catalog/fixtures/queued-message-checks.ts
- docs/ui-provenance.json
- spec/src/current/contracts/api/41-interactive-agent-execution.md
- spec/src/current/contracts/ui/37-document-conversations.md
- spec/src/current/governance/spec-governance.md
- spec/src/current/progress/M24/SUMMARY.nav.md
- spec/src/current/progress/M24/index.md
- spec/src/current/progress/M24/queued-messages-review.md
- spec/src/history/2026/10/10/E-CHAT-QUEUE-001/SUMMARY.nav.md
- spec/src/history/2026/10/10/E-CHAT-QUEUE-001/details-02.md
- spec/src/history/2026/10/10/E-CHAT-QUEUE-001/details-03.md
- spec/src/history/2026/10/10/E-CHAT-QUEUE-001/details-04.md
- spec/src/history/2026/10/10/E-CHAT-QUEUE-001/details-05.md
- spec/src/history/2026/10/10/E-CHAT-QUEUE-001/index.md
- spec/src/history/2026/10/10/SUMMARY.nav.md
- spec/src/history/2026/10/10/index.md
- web/src/lib/controller/agent-chat-receipts.ts
- web/src/lib/controller/agent-execution.svelte.ts
- web/src/lib/conversations/legacy-recorded.ts
- web/src/lib/conversations/linked-conversation.svelte.ts
- web/src/lib/conversations/managed-send.ts
- web/src/lib/conversations/prompt-delivery.ts
- web/src/lib/conversations/queued-agent-messages.svelte.ts
- web/src/lib/conversations/recorded-send.ts
- web/src/lib/conversations/standalone-send.svelte.ts
- web/src/lib/conversations/submission-result.ts
- web/src/lib/conversations/thread-send.ts
- web/src/lib/ui/community-member-avatar/CommunityMemberAvatar.svelte
- web/src/lib/ui/community-member-avatar/community-member-avatar.css
- web/src/lib/ui/store-agent-chat/QueuedPromptText.svelte
- web/src/lib/ui/store-agent-chat/StoreAgentChatSidebar.svelte
- web/src/lib/ui/store-agent-chat/StoreAgentMessageFooter.svelte
- web/src/lib/ui/store-agent-chat/StoreAgentPromptQueue.svelte
- web/src/lib/ui/store-agent-chat/StoreConversationRecovery.svelte
- web/src/lib/ui/store-agent-chat/StoreLiveConversation.svelte
- web/src/lib/ui/store-agent-chat-body/ChatMessageFooter.svelte
- web/src/lib/ui/store-agent-chat-body/StoreAgentChatBody.svelte
- web/src/lib/ui/store-agent-chat-body/comment-submission.ts
- web/src/lib/ui/store-agent-chat-body/comments.svelte.ts
- web/src/lib/ui/store-agent-chat-body/model.ts
- web/src/lib/ui/store-agent-execution/StoreAgentConversation.svelte
- web/src/lib/ui/store-agent-execution/presentation.ts
- web/src/lib/ui/store-collaboration/StoreRecordedConversation.svelte
- web/src/lib/ui/store-threads/StoreThreadConversation.svelte
- web/tests/agent-execution.test.ts
- web/tests/vite/agent-chat.test.ts
- web/tests/vite/conversation-session.test.ts
- web/tests/vite/queued-agent-messages.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/agent-execution.test.ts#controller context delivery acknowledges a
  queued prompt before ACP starts it
- web/tests/vite/agent-chat.test.ts#live acknowledgement refreshes the retained
  receipt without dispatching again
- web/tests/vite/agent-chat.test.ts#controller queue acknowledgement refreshes
  the retained receipt without dispatching again
- web/tests/vite/agent-execution-controller.test.ts#stopping an agent targets
  its session without changing another chat or replaying a prompt
- web/tests/vite/queued-agent-messages.test.ts#queue captures immutable reviews
  and advances recipients independently across views
- web/tests/vite/queued-agent-messages.test.ts#later exact start releases a
  queue after an unknown status read
- web/tests/vite/queued-agent-messages.test.ts#late context acknowledgements
  cannot revive a started or completed request
- web/tests/vite/conversation-session.test.ts#a deferred review queue
  acknowledgement frees the composer without claiming publication
- web/tests/vite/conversation-session.test.ts#a proven queue refusal retains
  editable text and comments rather than freezing an unknown request
- web/tests/vite/conversation-projection.test.ts#recorded rich captures without
  a plain-text mapping support whole sections without inventing ranges
- catalog/fixtures/delta-chat/comments.test.ts#comment-only sends freeze
  snapshots, prevent double submission and retain failures
<!-- /evidence-field -->

<!-- evidence-field:story_cases list -->
## Story cases

- catalog/components/store-application/QueuedAgentMessages.stories.svelte (8
  cases)
- catalog/components/store-application/AgentSessionState.stories.svelte
- catalog/components/store-application/AgentDiscussion.stories.svelte
- catalog/components/store-application/AgentReplyIndependence.stories.svelte
- catalog/components/store-application/AgentFleet.stories.svelte
- catalog/components/store-application/ConversationContinuity.stories.svelte
- catalog/components/store-application/ConversationFooterLayout.stories.svelte
- catalog/components/store-application/AgentChatFeedback.stories.svelte
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

UI-only acceptance: active Document and Classic composers send exact queued submissions, show captured prompts/comments above the input, advance each recipient only after start and preserve drafts and unknown outcomes. Queued cards reuse pending-comment disclosure, theme-background card, captured author avatar and hover/focus controls; metadata is outside below the card and the whole row, including avatar and metadata, uses 0.7 opacity with optional stronger initials-avatar contrast. Enclosing Jujutsu change tszmluon in new workspace queued-messages; parent nzltvopw/0df7d007. Queued prose matches normal responses (body token, sans-serif family, line height 1.65). Short text is centred beside the avatar with symmetric 8px card block padding. Prompts above 600 characters or eight lines offer presentation-only See More / See Less; full captured text, draft, exact request and disclosure survive view/navigation changes. While an exact queued Send awaits confirmation, the shared footer exposes a separate Stop action beside disabled Queue, preserving the independent active-turn Stop lane.
<!-- /evidence-field -->

<!-- evidence-field:before_state markdown -->
## Before state

The shared footer unconditionally preferred Stop while an active session existed. Queue projection and managed receipt refresh ignored context_delivery acknowledgements with sentContext.delivery=queued. Before-state regressions failed once each at those two boundaries; accepted queued review snapshots could keep the composer unresolved until start.
<!-- /evidence-field -->


<!-- generated-navigation -->
## Pages

- [Validation and review 2](details-02.md)
- [Validation and review 3](details-03.md)
- [Validation and review 4](details-04.md)
- [Validation and review 5](details-05.md)
