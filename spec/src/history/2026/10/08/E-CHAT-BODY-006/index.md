---
schema: 1
id: E-CHAT-BODY-006
---

# E-CHAT-BODY-006

<!-- evidence-field:id markdown -->
## Id

E-CHAT-BODY-006
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

ScrollArea viewport, full-frame shell and themed draft/reading carets
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change tkxpllop above d8956a14 in chat-body workspace
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

The chat section itself owned native overflow. Although the stories requested fullscreen, the Design Core root retained its normal 1.5rem component inset because this family had not opted into the existing jjStoreShell setting. The draft showed a placeholder and inherited smaller document typography; carets used ordinary foreground colors. An added exact InlineNextMessageAndFixedSendActions regression failed once before implementation because there was no ScrollArea viewport (1 failed, 13 skipped).
<!-- /evidence-field -->

<!-- evidence-field:findings markdown -->
## Findings

StoreAgentChatBody now composes the existing public Design Core shadcn ScrollArea, retaining its automatic engine strategy and hover scrollbar policy. The accessible Agent chat body region wraps that component; its bound viewportRef is the actual scrollport for append retention, turn navigation and Jump to End. The document no longer owns native overflow, and overscroll containment targets the viewport. The footer remains outside it. The story family uses the existing jjStoreShell parameter, making Canvas edge-to-edge and bounded without changing global catalog CSS or fixtures. Enter still inserts a newline; submission remains caller-owned. The draft has no placeholder and uses public Mira font-size/line-height tokens (1rem/1.75) with a 2px avatar inset to center the first 28px line beside the 24px avatar. An app-owned public CodeMirror theme shares a 2px, 1.2-height caret: muted-foreground for captured readers, primary for the draft. The qualified selector and logical border property give it precedence over Mira cursor defaults, without modifying foundational packages or node_modules. Cold diagnostics report primary oklch(0.45 0.2 295), reading grey oklch(0.5 0.05 295), identical first-line/avatar centers and no placeholder. No data or anchor projection changed.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/AgentChatBody.stories.svelte
- catalog/fixtures/delta-chat/browser.acceptance.mjs
- catalog/fixtures/delta-chat/next-message-native.mjs
- catalog/fixtures/delta-chat/next-message-play.ts
- spec/src/current/contracts/api/41-interactive-agent-execution.md
- spec/src/current/contracts/ui/36-commentable-chat-body-prototype.md
- spec/src/current/progress/M24/index.md
- spec/src/history/2026/10/08/E-CHAT-BODY-006/SUMMARY.nav.md
- spec/src/history/2026/10/08/E-CHAT-BODY-006/details-02.md
- spec/src/history/2026/10/08/E-CHAT-BODY-006/details-03.md
- spec/src/history/2026/10/08/E-CHAT-BODY-006/index.md
- spec/src/history/2026/10/08/SUMMARY.nav.md
- spec/src/history/2026/10/08/index.md
- web/src/lib/ui/store-agent-chat-body/ChatNextMessage.svelte
- web/src/lib/ui/store-agent-chat-body/StoreAgentChatBody.svelte
- web/src/lib/ui/store-agent-chat-body/chat-body.css
- web/src/lib/ui/store-agent-chat-body/chat-caret.ts
- web/src/lib/ui/store-agent-chat-body/text-editor.ts
- web/src/lib/ui/store-agent-chat-body/text-reader.css
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-CHAT-BODY-006 details 2](details-02.md)
- [E-CHAT-BODY-006 details 3](details-03.md)
