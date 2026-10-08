---
schema: 1
id: E-CHAT-BODY-007
---

# E-CHAT-BODY-007

<!-- evidence-field:id markdown -->
## Id

E-CHAT-BODY-007
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

Delta-style queued comments, linked replies and consistent chat spacing
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change youwpzpm above b4e3bbab in jj-store-chat-body
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

The existing UI used immediate Add comment/Add reply actions and independent composer text. It lacked delivery batches, linked replies, the shared bottom queue and multi-turn comment-only reference. Tool/file/Thought gaps accumulated nested spacing. Before implementation the existing adapter/reader surface passed 16 tests and the original interaction/axe family passed 14 stories (.local/chat-body-before.log). Native Delta click/scroll inspection remained unavailable; supplied screenshots, exported thread data and the approved plan are the reference. Planning left this workspace clean.
<!-- /evidence-field -->

<!-- evidence-field:findings markdown -->
## Findings

One host-owned ChatBodyComments controller now owns stable-author discussion entries, draft/sending/sent/replied status, exact block/revision/UTF-16 range/quote anchors, message draft and disclosure state. Nonempty inline edits appear immediately in the bottom queue; either mounted editor updates the same entry without remounting its peer. Same-author exact-anchor continuation resumes pending text or appends after delivery. The structured send snapshots message and comments, prevents concurrent submissions, retains failed drafts and produces immutable historical batches. Delivered entries use a green Lucide double-check with an accessible Sent label; linked replies retain a distinct Replied label and View reply action. Source navigation opens outer/tool/file disclosures, raw/rich content, middle folds and the read sidebar before focusing the exact anchor. Unavailable/revised anchors keep their quote and explanation. Collapsed boundaries hoist discussions once per visible transcript branch; read rows retain their source summary. Queue/history/reply cards share owned avatar, Markdown and editor components. Quote Markdown uses the existing fullWidth API; icon-xs comment actions align with the 24px avatar and eliminate the reported asymmetric card inset. Mixed-content readers use Mira document mode with owned code styling so nested comment editors retain normal text typography. Widget identities and ResizeObserver cleanup remain stable. The new sanitized Hello World fixture preserves three turns, two comments-only delivered batches, three linked replies and one pending comment; verified source maps convert UTF-8 offsets through Markdown trimming/splitting. Simulated sends/replies and failure controls remain entirely catalog-owned. Tool flows use 8px gaps; messages/turns use 16px. Focus clearance, label-adjacent chevrons, transparent headers, code backgrounds, ScrollArea and the blank composer/fixed footer remain intact. Follow-up screenshots add explicit same-anchor pending entries inside one shared border while ordinary re-entry resumes the latest draft. Reply/Add another are hover/focus/touch icon actions on the comment row. Each inline entry links to its exact queued editor or sent batch card. Navigation enriches comment-only and mixed turns with actual authored comments and exposes a pending marker, with author/count/available agent/time metadata, keyboard navigation and bookmarks.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/AgentChatBody.stories.svelte
- catalog/fixtures/ChatBodyFixture.svelte
- catalog/fixtures/delta-chat/acceptance-helpers.ts
- catalog/fixtures/delta-chat/acceptance-plays.ts
- catalog/fixtures/delta-chat/adapter.ts
- catalog/fixtures/delta-chat/browser.acceptance.mjs
- catalog/fixtures/delta-chat/comment-followup-play.ts
- catalog/fixtures/delta-chat/comment-lifecycle-play.ts
- catalog/fixtures/delta-chat/comment-navigation-play.ts
- catalog/fixtures/delta-chat/comment-queue-native.mjs
- catalog/fixtures/delta-chat/comments.test.ts
- catalog/fixtures/delta-chat/discussions.test.ts
- catalog/fixtures/delta-chat/discussions.ts
- catalog/fixtures/delta-chat/example-source.ts
- catalog/fixtures/delta-chat/fixture.css
- catalog/fixtures/delta-chat/hello-world-sources.json
- catalog/fixtures/delta-chat/hello-world.json
- catalog/fixtures/delta-chat/large-capture-native.mjs
- catalog/fixtures/delta-chat/large-capture-play.ts
- catalog/fixtures/delta-chat/middle-fold.test.ts
- catalog/fixtures/delta-chat/model.ts
- catalog/fixtures/delta-chat/next-message.svelte.ts
- catalog/fixtures/delta-chat/reader-input-native.mjs
- catalog/fixtures/delta-chat/spacing-play.ts
- catalog/fixtures/delta-chat/turns.ts
- scripts/delta-chat-comments.py
- spec/src/current/contracts/api/41-interactive-agent-execution.md
- spec/src/current/contracts/ui/36-commentable-chat-body-prototype.md
- spec/src/current/governance/spec-governance.md
- spec/src/current/progress/M24/index.md
- spec/src/current/verification/ui-02.md
- spec/src/current/verification/ui.md
- spec/src/history/2026/10/08/E-CHAT-BODY-007/SUMMARY.nav.md
- spec/src/history/2026/10/08/E-CHAT-BODY-007/details-02.md
- spec/src/history/2026/10/08/E-CHAT-BODY-007/details-03.md
- spec/src/history/2026/10/08/E-CHAT-BODY-007/index.md
- spec/src/history/2026/10/08/SUMMARY.nav.md
- spec/src/history/2026/10/08/index.md
- web/src/lib/ui/store-agent-chat-body/ChatCollapsedComments.svelte
- web/src/lib/ui/store-agent-chat-body/ChatCommentArea.svelte
- web/src/lib/ui/store-agent-chat-body/ChatCommentBatch.svelte
- web/src/lib/ui/store-agent-chat-body/ChatCommentEditor.svelte
- web/src/lib/ui/store-agent-chat-body/ChatContent.svelte
- web/src/lib/ui/store-agent-chat-body/ChatMessageFooter.svelte
- web/src/lib/ui/store-agent-chat-body/ChatNavigation.svelte
- web/src/lib/ui/store-agent-chat-body/ChatNextMessage.svelte
- web/src/lib/ui/store-agent-chat-body/ChatTextBlock.svelte
- web/src/lib/ui/store-agent-chat-body/ChatToolBlocks.svelte
- web/src/lib/ui/store-agent-chat-body/ChatToolPanel.svelte
- web/src/lib/ui/store-agent-chat-body/ChatTools.svelte
- web/src/lib/ui/store-agent-chat-body/ChatTurnCard.svelte
- web/src/lib/ui/store-agent-chat-body/StoreAgentChatBody.svelte
- web/src/lib/ui/store-agent-chat-body/chat-body.css
- web/src/lib/ui/store-agent-chat-body/comment-card.css
- web/src/lib/ui/store-agent-chat-body/comment-navigation.ts
- web/src/lib/ui/store-agent-chat-body/comment-widgets.ts
- web/src/lib/ui/store-agent-chat-body/comments.svelte.ts
- web/src/lib/ui/store-agent-chat-body/custom-interaction.ts
- web/src/lib/ui/store-agent-chat-body/markdown-blocks.ts
- web/src/lib/ui/store-agent-chat-body/model.ts
- web/src/lib/ui/store-agent-chat-body/navigation-turns.ts
- web/src/lib/ui/store-agent-chat-body/navigation.css
- web/src/lib/ui/store-agent-chat-body/text-reader.css
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- catalog/fixtures/delta-chat/comments.test.ts#comment-only sends freeze
  snapshots, prevent double submission and retain failures
- catalog/fixtures/delta-chat/comments.test.ts#explicit new comments share an
  anchor while ordinary editing resumes the latest draft
- catalog/fixtures/delta-chat/discussions.test.ts#Hello World preserves three
  turns, two comments-only batches and three linked replies
- catalog/fixtures/delta-chat/discussions.test.ts#navigation previews delivered
  and pending comment batches without duplicating turns
- catalog/fixtures/delta-chat/discussions.test.ts#source spans retain trimming,
  Unicode and fenced-code offsets
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-CHAT-BODY-007 details 2](details-02.md)
- [E-CHAT-BODY-007 details 3](details-03.md)
