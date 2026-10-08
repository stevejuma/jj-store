---
schema: 1
id: E-CHAT-BODY-002
---

# E-CHAT-BODY-002

<!-- evidence-field:id markdown -->
## Id

E-CHAT-BODY-002
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M24
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-001
- JST-UI-015
- JST-API-001
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-08
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Delta screenshot refinements, immutable middle folds, local reactions and inline next-message draft
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change znowylku above b114495e in chat-body workspace
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

The completed initial prototype had full-width trailing chevrons, a monospace file label, a rail hover outline and no end marker. Long captures rendered every line, returned XML was plain prose, hover actions lacked reactions, and the comparison had no next-message draft/footer.
<!-- /evidence-field -->

<!-- evidence-field:findings markdown -->
## Findings

Disclosure controls fit labels with adjacent 8px chevrons. Tool/file/Thought headers share proportional compact UI typography and muted title colors; header hover remains transparent. Thought spacing is reduced. The rail preserves keyboard focus outlines, omits hover outlines and ends with a compact Jump to End hint/target. Captured code/diffs have muted content backgrounds without a frame around whole tool groups. Code/patch captures over 32 lines replace only the middle with a CodeMirror block widget, retaining 12 first and 8 final lines and the complete document/UTF-16 offsets. Revealing is retained by block ID/revision; valid hidden discussion anchors force visibility. XML uses the existing safe prompt-element renderer with whole-section anchors. The bordered hover bar reuses the owned Community reaction picker/bar and has a distinct Comment hover treatment. The host owns memory-only reaction toggles. An optional document-tail composer and fixed footer use public Mira handles; Enter inserts newlines and Send/Cmd/Ctrl+Enter appends only a local demo user message. The reference subtitle is removed and the fixture fills the viewport. Blank draft space remains below the content; Jump to End focuses the draft, and submission brings it into view.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/AgentChatBody.stories.svelte
- catalog/fixtures/ChatBodyFixture.svelte
- catalog/fixtures/delta-chat/acceptance-helpers.ts
- catalog/fixtures/delta-chat/acceptance-plays.ts
- catalog/fixtures/delta-chat/adapter.test.ts
- catalog/fixtures/delta-chat/adapter.ts
- catalog/fixtures/delta-chat/browser.acceptance.mjs
- catalog/fixtures/delta-chat/large-capture-native.mjs
- catalog/fixtures/delta-chat/large-capture-play.ts
- catalog/fixtures/delta-chat/large-captures.ts
- catalog/fixtures/delta-chat/local-reactions.svelte.ts
- catalog/fixtures/delta-chat/message-action-native.mjs
- catalog/fixtures/delta-chat/message-action-play.ts
- catalog/fixtures/delta-chat/middle-fold.test.ts
- catalog/fixtures/delta-chat/next-message-native.mjs
- catalog/fixtures/delta-chat/next-message-play.ts
- catalog/fixtures/delta-chat/next-message.svelte.ts
- catalog/fixtures/delta-chat/playground.ts
- spec/src/current/contracts/api/41-interactive-agent-execution.md
- spec/src/current/contracts/ui/36-commentable-chat-body-prototype.md
- spec/src/current/progress/M24/index.md
- spec/src/history/2026/10/08/E-CHAT-BODY-002/SUMMARY.nav.md
- spec/src/history/2026/10/08/E-CHAT-BODY-002/details-02.md
- spec/src/history/2026/10/08/E-CHAT-BODY-002/details-03.md
- spec/src/history/2026/10/08/E-CHAT-BODY-002/index.md
- spec/src/history/2026/10/08/SUMMARY.nav.md
- spec/src/history/2026/10/08/index.md
- web/src/lib/ui/store-agent-chat-body/ChatContent.svelte
- web/src/lib/ui/store-agent-chat-body/ChatEndHint.svelte
- web/src/lib/ui/store-agent-chat-body/ChatHoverActions.svelte
- web/src/lib/ui/store-agent-chat-body/ChatMessageFooter.svelte
- web/src/lib/ui/store-agent-chat-body/ChatNavigation.svelte
- web/src/lib/ui/store-agent-chat-body/ChatNextMessage.svelte
- web/src/lib/ui/store-agent-chat-body/ChatTextBlock.svelte
- web/src/lib/ui/store-agent-chat-body/ChatToolBlocks.svelte
- web/src/lib/ui/store-agent-chat-body/ChatTools.svelte
- web/src/lib/ui/store-agent-chat-body/StoreAgentChatBody.svelte
- web/src/lib/ui/store-agent-chat-body/anchors.ts
- web/src/lib/ui/store-agent-chat-body/chat-body.css
- web/src/lib/ui/store-agent-chat-body/middle-fold.ts
- web/src/lib/ui/store-agent-chat-body/model.ts
- web/src/lib/ui/store-agent-chat-body/text-editor.ts
- web/src/lib/ui/store-agent-chat-body/text-reader.css
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- catalog/fixtures/delta-chat/adapter.test.ts#preserves all 30 tools and
  assistant segments between tool groups
- catalog/fixtures/delta-chat/adapter.test.ts#maps the full prompt/response and
  30 tools into one actual conversation turn
- catalog/fixtures/delta-chat/adapter.test.ts#converts Unicode byte offsets and
  rejects offsets inside characters
- catalog/fixtures/delta-chat/adapter.test.ts#keeps multi-file patch hunks
  without synthetic file coordinates
- catalog/fixtures/delta-chat/adapter.test.ts#uses paragraph or line targets for
  a collapsed caret
- catalog/fixtures/delta-chat/adapter.test.ts#preserves exact ranges and never
  retargets changed revisions or bytes
- catalog/fixtures/delta-chat/adapter.test.ts#separates selectable fenced code
  and Mira-rendered complex blocks
- catalog/fixtures/delta-chat/middle-fold.test.ts#folds only long code and
  patches at complete line boundaries
- catalog/fixtures/delta-chat/middle-fold.test.ts#retains the exact Unicode
  source and offsets across a folded middle
- catalog/fixtures/delta-chat/middle-fold.test.ts#reveals a folded discussion
  while stale anchors cannot reveal a new revision
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-CHAT-BODY-002 details 2](details-02.md)
- [E-CHAT-BODY-002 details 3](details-03.md)
