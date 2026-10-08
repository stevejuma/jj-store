---
schema: 1
id: E-CHAT-BODY-005
---

# E-CHAT-BODY-005

<!-- evidence-field:id markdown -->
## Id

E-CHAT-BODY-005
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

Disclosure icon hover regions and captured numbered-file syntax
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change tplyxxqu above ef33f926 in chat-body workspace
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

The user requested hover fill on disclosure icons themselves while preserving transparent headers. The supplied README sidebar screenshot has square heading corners and unhighlighted Markdown. Retained read_file output contains five-space line-number/tab prefixes, which make the original Markdown parser treat headings as indented code. The new exact reference assertion reproduced missing heading syntax before integration: 1 failed, 13 skipped.
<!-- /evidence-field -->

<!-- evidence-field:findings markdown -->
## Findings

An owned reusable disclosure icon gives tool/group/file/Thought/context/handoff headers a rounded 22px hover region using the public muted surface. Keyboard-visible focus gets the same icon fill; the parent button stays transparent and remains the sole named/tabbable control. The glyph stays 14px with its existing 8px label gap and rotates from the public disclosure state. File heading top corners inherit the content radius without clipping hover controls or comments. The catalog explicitly marks known Delta numbered read captures in an optional UI-only lineNumbered flag. Syntax parsing removes number/tab prefixes only in a temporary projection; Mira public syntax classes map back to the exact immutable capture, with every multiline token split around neutral prefixes. UTF-16 offsets, raw text, folds and comment anchors remain unchanged. Ordinary code and raw JSON retain their existing language path. Public Mira and CodeMirror APIs are sufficient; no upstream or dependency changes were needed.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/AgentChatBody.stories.svelte
- catalog/fixtures/delta-chat/adapter.ts
- catalog/fixtures/delta-chat/captured-code.test.ts
- spec/src/current/contracts/api/41-interactive-agent-execution.md
- spec/src/current/contracts/ui/36-commentable-chat-body-prototype.md
- spec/src/current/progress/M24/index.md
- spec/src/history/2026/10/08/E-CHAT-BODY-005/SUMMARY.nav.md
- spec/src/history/2026/10/08/E-CHAT-BODY-005/details-02.md
- spec/src/history/2026/10/08/E-CHAT-BODY-005/index.md
- spec/src/history/2026/10/08/SUMMARY.nav.md
- spec/src/history/2026/10/08/index.md
- web/src/lib/ui/store-agent-chat-body/ChatDisclosureIcon.svelte
- web/src/lib/ui/store-agent-chat-body/ChatToolBlocks.svelte
- web/src/lib/ui/store-agent-chat-body/ChatTools.svelte
- web/src/lib/ui/store-agent-chat-body/StoreAgentChatBody.svelte
- web/src/lib/ui/store-agent-chat-body/captured-code.ts
- web/src/lib/ui/store-agent-chat-body/chat-body.css
- web/src/lib/ui/store-agent-chat-body/model.ts
- web/src/lib/ui/store-agent-chat-body/text-editor.ts
- web/src/lib/ui/store-agent-chat-body/text-reader.css
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- catalog/fixtures/delta-chat/captured-code.test.ts#highlights numbered Markdown
  without changing captured source or prefixes
- catalog/fixtures/delta-chat/captured-code.test.ts#splits multiline syntax
  around numbered prefixes and keeps UTF-16 anchors
- catalog/fixtures/delta-chat/captured-code.test.ts#leaves unknown formats plain
  and supports unnumbered lines in a capture
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
- catalog/fixtures/delta-chat/patch-presentation.test.ts#turn totals distinguish
  file headers from triple-prefix content inside hunks
- catalog/fixtures/delta-chat/patch-presentation.test.ts#captured hunks keep
  triple prefixes as content and preserve Unicode offsets
- catalog/fixtures/delta-chat/patch-presentation.test.ts#a new diff resets
  header classification without changing captured source
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-CHAT-BODY-005 details 2](details-02.md)
- [E-CHAT-BODY-005 details 3](details-03.md)
