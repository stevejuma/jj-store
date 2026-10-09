---
schema: 1
id: E-CHAT-LIVE-011
---

# E-CHAT-LIVE-011

<!-- evidence-field:id markdown -->
## Id

E-CHAT-LIVE-011
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M24
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-001
- JST-UI-012
- JST-UI-015
- JST-UI-018
- JST-API-001
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-09
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Live inline Document input and square Advanced hover
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change zknxsomx
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Every live conversation mounted the framed Community composer in Document mode. The catalog used a separate inline next-message input. Advanced triggers used 13px rounding on hover. The exact live layout and hover regressions failed before the changes.
<!-- /evidence-field -->

<!-- evidence-field:findings markdown -->
## Findings

The body supplies its composition-deferred renderer choice to the shared composer. Document uses the owned inline next-message layout, small author avatar, primary caret and roomy editable minimum height; it has no framed composer or nested input ScrollArea. Classic retains its public Composer/ScrollArea layout. Both use the same Mira editor extensions, audience, emoji, formatting and submission logic. Document editor actions register with the fixed footer alongside runtime controls and Send. Selection is restored on presentation remount and composition defers view changes. Live/managed/recorded/thread/reply hosts bind the same session-owned message and comments. Status/recovery and versioned pending requests remain intact. Advanced hover areas are square; the owning rounded group boundary is retained. Extracted composer surfaces/send controls retain Community attribution. Source sizes were repaired by owned presentation/lifecycle helpers, without raising caps.
<!-- /evidence-field -->

<!-- evidence-field:live markdown -->
## Live

Fresh live page verified full-width Agent defaults without saving values. In retained sample ddcc5f115b95bf017564735592db880f, Document has one inline input and zero ai-chat-composer frames; Enter inserts a second line, and the temporary verification text was restored to its original empty value without sending. After the user selected sample 3aa8c81a8b5fbe2219498750ab014416, read-only inspection confirms a 444.59px editable region and toolbar in the fixed footer. Final screenshot: .local/document-inline-input-final.jpg. Warning/error logs are empty, including ResizeObserver on this live flow. Retained samples currently report interrupted/process_stopped_no_replay; this UI slice does not resume them or claim real-provider delivery acceptance.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/AgentChatChanges.stories.svelte
- catalog/components/store-application/AgentChatLayout.stories.svelte
- catalog/components/store-application/AgentChatRuntime.stories.svelte
- catalog/components/store-application/AgentProviderConfiguration.stories.svelte
- catalog/components/store-application/ConversationContinuity.stories.svelte
- catalog/fixtures/ConversationContinuityFixture.svelte
- docs/ui-provenance.json
- spec/src/current/contracts/api/41-interactive-agent-execution.md
- spec/src/current/contracts/ui/35-compute-defaults-and-sent-context.md
- spec/src/current/contracts/ui/37-document-conversations.md
- spec/src/current/governance/spec-governance.md
- spec/src/current/progress/M24/history/M24/index.md
- spec/src/current/progress/M24/index.md
- spec/src/history/2026/10/09/E-CHAT-LIVE-010/SUMMARY.nav.md
- spec/src/history/2026/10/09/E-CHAT-LIVE-010/index.md
- spec/src/history/2026/10/09/SUMMARY.nav.md
- spec/src/history/2026/10/09/index.md
- web/src/lib/ui/community-chat-composer/CommunityChatComposer.svelte
- web/src/lib/ui/community-chat-composer/CommunityComposerSendActions.svelte
- web/src/lib/ui/community-chat-composer/CommunityComposerSurface.svelte
- web/src/lib/ui/community-chat-composer/composer-document-actions.svelte.ts
- web/src/lib/ui/community-chat-composer/composer-selection.ts
- web/src/lib/ui/community-chat-composer/props.ts
- web/src/lib/ui/store-agent-chat/StoreAgentChatSidebar.svelte
- web/src/lib/ui/store-agent-chat/StoreLiveConversation.svelte
- web/src/lib/ui/store-agent-chat-body/ChatMessageFooter.svelte
- web/src/lib/ui/store-agent-chat-body/ChatNextMessage.svelte
- web/src/lib/ui/store-agent-chat-body/StoreAgentChatBody.svelte
- web/src/lib/ui/store-agent-chat-body/model.ts
- web/src/lib/ui/store-agent-execution/StoreAgentConversation.svelte
- web/src/lib/ui/store-collaboration/StoreRecordedConversation.svelte
- web/src/lib/ui/store-settings/StoreSettingsDisclosure.svelte
- web/src/lib/ui/store-threads/StoreThreadConversation.svelte
- web/src/lib/ui/store-threads/StoreThreadPendingActions.svelte
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-CHAT-LIVE-011 details 2](details-02.md)
- [E-CHAT-LIVE-011 details 3](details-03.md)
