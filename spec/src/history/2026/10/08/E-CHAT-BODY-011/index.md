---
schema: 1
id: E-CHAT-BODY-011
---

# E-CHAT-BODY-011

<!-- evidence-field:id markdown -->
## Id

E-CHAT-BODY-011
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

Representative sent discussions, matching quote typography and compact reply context
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change yvqxmrms above stllokrm in jj-store-chat-body
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

ChatBodyFixture initialized discussions only for the separate Hello World comments scenario. The main reference supplied captured items and no initial threads or delivered review turn. The user requested representative sent inline comments. During review of the populated example, the user identified monospace source quotes in the sent batch and requested the normal chat font and matching text size.
<!-- /evidence-field -->

<!-- evidence-field:findings markdown -->
## Findings

A catalog-owned referenceDiscussions overlay retains the untouched Delta reference JSON and captured items, adding four delivered comments at three explicit patch/read/prose anchors. The patch discussion groups two user comments with one linked simulated reply; the prose discussion has another linked reply; the README comment remains sent awaiting a reply. Each source requires its exact block ID and unique quote, then retains the captured revision and UTF-16 offsets. Stable demo IDs join the shared inline/sidebar threads, comments-only historical batch, reply-context cards and rail marker. Labels explicitly identify demo review comments and simulated replies. The initial draft queue stays empty and Send stays disabled until user input. Comment-card captured-code quotes now inherit the normal chat font and match the rendered comment body using the public Mira Markdown font-size/line-height tokens while preserving whitespace/highlighting; code readers retain monospace. When the preceding user batch is expanded, reply context uses a compact count/source link rather than repeating those comments. Collapsing the batch restores full quoted context; other turns and unavailable anchors retain full context. Distinct entries at one anchor are preserved. No live dispatch, persistence, dependencies or source assets were introduced.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/AgentChatBody.stories.svelte
- catalog/fixtures/ChatBodyFixture.svelte
- catalog/fixtures/delta-chat/browser.acceptance.mjs
- catalog/fixtures/delta-chat/numbered-code-gutter-native.mjs
- catalog/fixtures/delta-chat/reference-discussion-play.ts
- catalog/fixtures/delta-chat/reference-discussions.ts
- spec/src/current/contracts/api/41-interactive-agent-execution.md
- spec/src/current/contracts/ui/36-commentable-chat-body-prototype.md
- spec/src/current/progress/M24/index.md
- spec/src/current/verification/ui-02.md
- spec/src/current/verification/ui.md
- spec/src/history/2026/10/08/E-CHAT-BODY-011/SUMMARY.nav.md
- spec/src/history/2026/10/08/E-CHAT-BODY-011/details-02.md
- spec/src/history/2026/10/08/E-CHAT-BODY-011/details-03.md
- spec/src/history/2026/10/08/E-CHAT-BODY-011/index.md
- spec/src/history/2026/10/08/SUMMARY.nav.md
- spec/src/history/2026/10/08/index.md
- web/src/lib/ui/store-agent-chat-body/ChatReplyContext.svelte
- web/src/lib/ui/store-agent-chat-body/StoreAgentChatBody.svelte
- web/src/lib/ui/store-agent-chat-body/comment-card.css
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-CHAT-BODY-011 details 2](details-02.md)
- [E-CHAT-BODY-011 details 3](details-03.md)
