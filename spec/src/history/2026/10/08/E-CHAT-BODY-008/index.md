---
schema: 1
id: E-CHAT-BODY-008
---

# E-CHAT-BODY-008

<!-- evidence-field:id markdown -->
## Id

E-CHAT-BODY-008
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

Full-height resizable captured-read sidebar, aligned headers and line gutters
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change nwwsyrru above youwpzpm in jj-store-chat-body
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

The read body pane had a fixed width, a redundant Captured read label and embedded number/tab prefixes that caused wrapped code to return to the number column. The fixture title added a second inset bottom border. The first exact resize regression failed because no resize handle existed. Follow-up requests selected the real full-height shell sidebar used by thread replies, joined edges, matching header dividers and a separate captured-number gutter.
<!-- /evidence-field -->

<!-- evidence-field:findings markdown -->
## Findings

The fixture composes the owned CommunityAppShell wrapper, Main and the standard sibling AppShell.Sidebar used by thread replies. The wrapper supplies joined edges; no fixture gap override or new resize implementation remains. Built-in pointer/keyboard resizing uses 280px/720px bounds and retains controller-owned widths across file changes and close/reopen. Constrained desktop uses replace-main with 360px protected main width; mobile uses the shell panel. The fixed Sidebar.Header and ScrollArea body match existing composition. Reference title and sidebar headings have matching height/bottom coordinates and a single title divider. The extra label and nested title border are removed. A CodeMirror gutter presents recorded labels from explicitly numbered captures, including noncontiguous numbering and folded trailing lines. Hidden atomic prefix decorations preserve the immutable document and exact UTF-16 anchors; projected syntax shares the same parser. Wrapped code starts at the content edge; mounted comment widgets keep their identity and editor focus during resizing. Community and Design Core shell ownership and Mira gutter APIs were inspected; no package edits, source copies or dependency changes were needed.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/AgentChatBody.stories.svelte
- catalog/fixtures/ChatBodyFixture.svelte
- catalog/fixtures/delta-chat/browser.acceptance.mjs
- catalog/fixtures/delta-chat/captured-code.test.ts
- catalog/fixtures/delta-chat/fixture.css
- catalog/fixtures/delta-chat/numbered-code-gutter-native.mjs
- catalog/fixtures/delta-chat/numbered-code-gutter-play.ts
- catalog/fixtures/delta-chat/read-panel-resize-native.mjs
- catalog/fixtures/delta-chat/read-panel-resize-play.ts
- spec/src/current/contracts/api/41-interactive-agent-execution.md
- spec/src/current/contracts/ui/36-commentable-chat-body-prototype.md
- spec/src/current/progress/M24/index.md
- spec/src/current/verification/ui-02.md
- spec/src/current/verification/ui.md
- spec/src/history/2026/10/08/E-CHAT-BODY-008/SUMMARY.nav.md
- spec/src/history/2026/10/08/E-CHAT-BODY-008/details-02.md
- spec/src/history/2026/10/08/E-CHAT-BODY-008/details-03.md
- spec/src/history/2026/10/08/E-CHAT-BODY-008/index.md
- spec/src/history/2026/10/08/SUMMARY.nav.md
- spec/src/history/2026/10/08/index.md
- web/src/lib/ui/store-agent-chat-body/ChatToolPanel.svelte
- web/src/lib/ui/store-agent-chat-body/captured-code.ts
- web/src/lib/ui/store-agent-chat-body/numbered-code-gutter.ts
- web/src/lib/ui/store-agent-chat-body/text-editor.ts
- web/src/lib/ui/store-agent-chat-body/text-reader.css
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- catalog/fixtures/delta-chat/captured-code.test.ts#maps captured gutter labels
  without renumbering gaps or shifting Unicode offsets
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-CHAT-BODY-008 details 2](details-02.md)
- [E-CHAT-BODY-008 details 3](details-03.md)
