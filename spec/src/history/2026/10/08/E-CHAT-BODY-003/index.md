---
schema: 1
id: E-CHAT-BODY-003
---

# E-CHAT-BODY-003

<!-- evidence-field:id markdown -->
## Id

E-CHAT-BODY-003
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

Final Delta reference code/screenshot review and patch classification regression
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change pxqwrkql above fdf28eca in chat-body workspace
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

The native comparison route had previously timed out. The saved story capture and supplied Delta screenshots show leading success icons and horizontal tool/file offsets absent from Delta, and the code used only a gutter stripe for changed lines. Turn totals excluded every +++/--- prefix, including actual hunk content; the exact regression failed 1/1 before edits (expected two additions, observed one).
<!-- /evidence-field -->

<!-- evidence-field:findings markdown -->
## Findings

Successful tool titles now start at the same text edge as group/file titles; the success icon and horizontal offsets are removed while vertical clearance and offset focus rings remain. Error/running icons remain visible, and all statuses have an aria-describedby relationship. Changed diff lines use full-line public Design Core diff background tokens with theme-aware fallbacks. One owned classifier supplies both reader decorations and turn totals: file headers are neutral before a hunk, triple-prefix lines inside hunks remain changes, and offsets retain exact UTF-16 source positions. The reference remains +175/-31 captured lines. A fresh read-only Delta CLI export has the same buffer, 38 projected records, 30 tools and draft anchor as the fixture; only native comment_set and snapshot metadata changed. No new thread content or private metadata was imported.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/fixtures/delta-chat/adapter.test.ts
- catalog/fixtures/delta-chat/patch-presentation.test.ts
- spec/src/current/contracts/api/41-interactive-agent-execution.md
- spec/src/current/contracts/ui/36-commentable-chat-body-prototype.md
- spec/src/current/progress/M24/index.md
- spec/src/history/2026/10/08/E-CHAT-BODY-003/SUMMARY.nav.md
- spec/src/history/2026/10/08/E-CHAT-BODY-003/details-02.md
- spec/src/history/2026/10/08/E-CHAT-BODY-003/details-03.md
- spec/src/history/2026/10/08/E-CHAT-BODY-003/index.md
- spec/src/history/2026/10/08/SUMMARY.nav.md
- spec/src/history/2026/10/08/index.md
- web/src/lib/ui/store-agent-chat-body/ChatToolBlocks.svelte
- web/src/lib/ui/store-agent-chat-body/ChatTools.svelte
- web/src/lib/ui/store-agent-chat-body/navigation-turns.ts
- web/src/lib/ui/store-agent-chat-body/patch-lines.ts
- web/src/lib/ui/store-agent-chat-body/text-editor.ts
- web/src/lib/ui/store-agent-chat-body/text-reader.css
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- catalog/fixtures/delta-chat/patch-presentation.test.ts#turn totals distinguish
  file headers from triple-prefix content inside hunks
- catalog/fixtures/delta-chat/patch-presentation.test.ts#captured hunks keep
  triple prefixes as content and preserve Unicode offsets
- catalog/fixtures/delta-chat/patch-presentation.test.ts#a new diff resets
  header classification without changing captured source
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

- [E-CHAT-BODY-003 details 2](details-02.md)
- [E-CHAT-BODY-003 details 3](details-03.md)
