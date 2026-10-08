---
schema: 1
id: E-CHAT-LIVE-005
---

# E-CHAT-LIVE-005

<!-- evidence-field:id markdown -->
## Id

E-CHAT-LIVE-005
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M24
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-001
- JST-UI-015
- JST-UI-018
- JST-COLLAB-034
- JST-COLLAB-035
- JST-COLLAB-036
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-09
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Shared production document/classic conversations and device preference
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change zlulznuw
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Only the catalog prototype had range comments and pending queues. Production concatenated assistant chunks; normal thread and agent rendering did not share durable batches, linked answers or session drafts.
<!-- /evidence-field -->

<!-- evidence-field:findings markdown -->
## Findings

Production hosts now adopt exact immutable projections into session-owned state. Ordered assistant segments retain tools and answer boundaries; validated reply links drive answer sections and the double-check destination chooser. Device Document/Classic selection preserves composition and drafts independently of Focus/Split. Thread and recorded bodies observe scoped agent participation, with repository-owned reactions, queued cancellation and saved changes. Exact source maps handle Markdown, UTF-8/UTF-16, numbered reads and patches. The local missing-thread response is recognized without ignoring other refusals. Streaming host updates preserve editor identity and bypass only the immutable reader edit filter. CodeMirror offscreen tests explicitly reveal target lines before asserting gutters/content.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/AgentChatFeedback.stories.svelte
- catalog/components/store-application/AgentChatLayout.stories.svelte
- catalog/components/store-application/AgentChatRuntime.stories.svelte
- catalog/components/store-application/AgentExecution.stories.svelte
- catalog/components/store-application/ConversationContinuity.stories.svelte
- catalog/components/store-threads/StoreCapturedReactions.stories.svelte
- catalog/components/store-threads/StoreThreadAlignment.stories.svelte
- catalog/components/store-threads/StoreThreads.stories.svelte
- catalog/fixtures/ConversationContinuityFixture.svelte
- catalog/fixtures/agent-discussion-fixture.ts
- catalog/fixtures/delta-chat/comment-lifecycle-play.ts
- catalog/fixtures/delta-chat/comment-queue-native.mjs
- catalog/fixtures/delta-chat/large-capture-play.ts
- catalog/fixtures/delta-chat/numbered-code-gutter-play.ts
- catalog/fixtures/thread-reaction-play.ts
- catalog/fixtures/thread-submission.ts
- catalog/fixtures/thread-transport.ts
- docs/maintenance/coverage.json
- docs/site/concepts/collaboration.md
- docs/site/guides/settings.md
- docs/site/reference/api/collaboration.md
- docs/site/reference/api/installation.md
- docs/site/reference/api/types.md
- docs/site/reference/transport.md
- docs/site/technical/architecture.md
- docs/site/technical/publication.md
- docs/ui-provenance.json
- spec/src/current/contracts/ui/37-document-conversations.md
- spec/src/current/progress/M24/index.md
- web/src/lib/appearance/preferences.ts
- web/src/lib/controller/agent-chat-context.ts
- web/src/lib/controller/agent-chat-selections.ts
- web/src/lib/controller/agent-chat.svelte.ts
- web/src/lib/controller/agent-execution-types.ts
- web/src/lib/controller/agent-execution.svelte.ts
- web/src/lib/controller/controller-actions.ts
- web/src/lib/controller/http-collaboration-data-source.ts
- web/src/lib/controller/http-store-data-source.ts
- web/src/lib/controller/native-data-source.ts
- web/src/lib/controller/production-agent-loaders.ts
- web/src/lib/controller/production-store-actions.ts
- web/src/lib/controller/production-store-collaboration.svelte.ts
- web/src/lib/controller/production-threads.svelte.ts
- web/src/lib/controller/thread-conversations.svelte.ts
- web/src/lib/controller/thread-directory.svelte.ts
- web/src/lib/controller/thread-history.ts
- web/src/lib/conversations/agent-adapter.ts
- web/src/lib/conversations/answer-segments.ts
- web/src/lib/conversations/discussion-reader.svelte.ts
- web/src/lib/conversations/discussions.ts
- web/src/lib/conversations/inspection.svelte.ts
- web/src/lib/conversations/linked-conversation.svelte.ts
- web/src/lib/conversations/managed-send.ts
- web/src/lib/conversations/merge-projections.ts
- web/src/lib/conversations/navigation.ts
- web/src/lib/conversations/ordered-activity.ts
- web/src/lib/conversations/reactions.svelte.ts
- web/src/lib/conversations/read-conversation.ts
- web/src/lib/conversations/recorded-adapter.ts
- web/src/lib/conversations/recorded-send.ts
- web/src/lib/conversations/session.svelte.ts
- web/src/lib/conversations/source-map.ts
- web/src/lib/conversations/standalone-send.svelte.ts
- web/src/lib/conversations/submission-result.ts
- web/src/lib/conversations/thread-adapter.ts
- web/src/lib/conversations/thread-discussions.ts
- web/src/lib/conversations/thread-send.ts
- web/src/lib/conversations/tool-adapter.ts
- web/src/lib/ui/community-chat-composer/CommunityChatComposer.svelte
- web/src/lib/ui/community-chat-composer/composer-extension.ts
- web/src/lib/ui/community-chat-composer/composer-submission.ts
- web/src/lib/ui/community-chat-composer/composer.css
- web/src/lib/ui/community-chat-composer/props.ts
- web/src/lib/ui/store-agent-chat/StoreAgentChatSidebar.svelte
- web/src/lib/ui/store-agent-chat/StoreConversationAgentSupplement.svelte
- web/src/lib/ui/store-agent-chat/StoreConversationRecovery.svelte
- web/src/lib/ui/store-agent-chat/StoreConversationRuntime.svelte
- web/src/lib/ui/store-agent-chat/StoreLinkedConversationActivity.svelte
- web/src/lib/ui/store-agent-chat/StoreLiveConversation.svelte
- web/src/lib/ui/store-agent-chat-body/ChatCommentArea.svelte
- web/src/lib/ui/store-agent-chat-body/ChatCommentBatch.svelte
- web/src/lib/ui/store-agent-chat-body/ChatCommentDelivery.svelte
- web/src/lib/ui/store-agent-chat-body/ChatCommentEditor.svelte
- web/src/lib/ui/store-agent-chat-body/ChatContent.svelte
- web/src/lib/ui/store-agent-chat-body/ChatHoverActions.svelte
- web/src/lib/ui/store-agent-chat-body/ChatInlineReference.svelte
- web/src/lib/ui/store-agent-chat-body/ChatTextBlock.svelte
- web/src/lib/ui/store-agent-chat-body/StoreAgentChatBody.svelte
- web/src/lib/ui/store-agent-chat-body/anchors.ts
- web/src/lib/ui/store-agent-chat-body/comment-navigation.ts
- web/src/lib/ui/store-agent-chat-body/comment-submission.ts
- web/src/lib/ui/store-agent-chat-body/comments.svelte.ts
- web/src/lib/ui/store-agent-chat-body/inline-references.ts
- web/src/lib/ui/store-agent-chat-body/markdown-decorations.ts
- web/src/lib/ui/store-agent-chat-body/model.ts
- web/src/lib/ui/store-agent-chat-body/text-editor.ts
- web/src/lib/ui/store-agent-chat-body/tool-blocks.ts
- web/src/lib/ui/store-agent-execution/StoreAgentChatRows.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentConversation.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentExecutionHost.svelte
- web/src/lib/ui/store-agent-execution/presentation.ts
- web/src/lib/ui/store-appearance/AppearancePreferences.svelte
- web/src/lib/ui/store-application/StoreApplication.svelte
- web/src/lib/ui/store-application/StoreApplicationRightPanels.svelte
- web/src/lib/ui/store-collaboration/StoreAgentContext.svelte
- web/src/lib/ui/store-collaboration/StoreAgentTranscript.svelte
- web/src/lib/ui/store-collaboration/StoreRecordedConversation.svelte
- web/src/lib/ui/store-threads/StoreConversationMessageActions.svelte
- web/src/lib/ui/store-threads/StoreThreadConversation.svelte
- web/src/lib/ui/store-threads/StoreThreadReplies.svelte
- web/src/lib/ui/store-threads/StoreThreads.svelte
- web/src/lib/ui/store-threads/thread-context.ts
- web/tests/conversation-merge.test.ts
- web/tests/conversation-ordering.test.ts
- web/tests/conversation-source-map.test.ts
- web/tests/vite/agent-chat.test.ts
- web/tests/vite/appearance.test.ts
- web/tests/vite/conversation-create.test.ts
- web/tests/vite/conversation-projection.test.ts
- web/tests/vite/conversation-reactions.test.ts
- web/tests/vite/conversation-refresh.test.ts
- web/tests/vite/conversation-session.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-CHAT-LIVE-005 details 2](details-02.md)
- [E-CHAT-LIVE-005 details 3](details-03.md)
