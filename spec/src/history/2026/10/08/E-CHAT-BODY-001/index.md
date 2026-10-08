---
schema: 1
id: E-CHAT-BODY-001
---

# E-CHAT-BODY-001

<!-- evidence-field:id markdown -->
## Id

E-CHAT-BODY-001
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

Standalone commentable agent chat body, captured Delta comparison and cold UI acceptance
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change ulzpulnn above 6d3ee2e9 in chat-body workspace
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

The original workspace was clean. No standalone commentable body existed. The initial seven-story acceptance had five failures; inline composer labels, patch syntax contrast, native text selection and first-character focus required correction.
<!-- /evidence-field -->

<!-- evidence-field:findings markdown -->
## Findings

Public MiraCodeEditor owns editor lifecycle. Fixed reading decorations retain immutable content; first input, paste and composition route into anchored local discussions. DOM-keyed widgets clean up mounted Svelte views and observers and request measurement. The app body shell hosts captured reads in its right/mobile pane. Multi-file patch disclosures retain comments/drafts independently. Tool headers align left, have transparent backgrounds, no internal padding, trailing chevrons and focus clearance; Raw JSON is a named hover/focus icon. A dense turn rail groups the prompt with assistant segments and tools and shows a screenshot-style summary card with captured counts/time and local bookmarks. The raw export was obtained through the read-only Delta CLI because native UI automation repeatedly timed out; the user supplied the navigation screenshot. No typing behavior was independently verified in Delta.
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
- catalog/fixtures/delta-chat/example-source.ts
- catalog/fixtures/delta-chat/offsets.ts
- catalog/fixtures/delta-chat/patches.ts
- catalog/fixtures/delta-chat/playground.ts
- catalog/fixtures/delta-chat/reference.json
- scripts/delta-chat-fixture.py
- spec/src/current/contracts/api/41-interactive-agent-execution.md
- spec/src/current/contracts/ui/36-commentable-chat-body-prototype.md
- spec/src/current/contracts/ui/SUMMARY.nav.md
- spec/src/current/contracts/ui/index.md
- spec/src/current/governance/spec-governance.md
- spec/src/current/progress/M24/index.md
- spec/src/current/verification/ui-02.md
- spec/src/current/verification/ui.md
- spec/src/history/2026/10/08/E-CHAT-BODY-001/SUMMARY.nav.md
- spec/src/history/2026/10/08/E-CHAT-BODY-001/details-02.md
- spec/src/history/2026/10/08/E-CHAT-BODY-001/details-03.md
- spec/src/history/2026/10/08/E-CHAT-BODY-001/index.md
- spec/src/history/2026/10/08/SUMMARY.nav.md
- spec/src/history/2026/10/08/index.md
- web/src/lib/ui/store-agent-chat-body/ChatCommentArea.svelte
- web/src/lib/ui/store-agent-chat-body/ChatContent.svelte
- web/src/lib/ui/store-agent-chat-body/ChatNavigation.svelte
- web/src/lib/ui/store-agent-chat-body/ChatRawToggle.svelte
- web/src/lib/ui/store-agent-chat-body/ChatTextBlock.svelte
- web/src/lib/ui/store-agent-chat-body/ChatToolBlocks.svelte
- web/src/lib/ui/store-agent-chat-body/ChatToolPanel.svelte
- web/src/lib/ui/store-agent-chat-body/ChatTools.svelte
- web/src/lib/ui/store-agent-chat-body/ChatTurnCard.svelte
- web/src/lib/ui/store-agent-chat-body/StoreAgentChatBody.svelte
- web/src/lib/ui/store-agent-chat-body/anchors.ts
- web/src/lib/ui/store-agent-chat-body/comment-widgets.ts
- web/src/lib/ui/store-agent-chat-body/comments.svelte.ts
- web/src/lib/ui/store-agent-chat-body/custom-interaction.ts
- web/src/lib/ui/store-agent-chat-body/markdown-blocks.ts
- web/src/lib/ui/store-agent-chat-body/markdown-decorations.ts
- web/src/lib/ui/store-agent-chat-body/model.ts
- web/src/lib/ui/store-agent-chat-body/navigation-turns.ts
- web/src/lib/ui/store-agent-chat-body/text-editor.ts
- web/src/lib/ui/store-agent-chat-body/text-reader.css
- web/src/lib/ui/store-agent-chat-body/tool-blocks.ts
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
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-CHAT-BODY-001 details 2](details-02.md)
- [E-CHAT-BODY-001 details 3](details-03.md)
