---
schema: 1
id: E-CHAT-BODY-009
---

# E-CHAT-BODY-009

<!-- evidence-field:id markdown -->
## Id

E-CHAT-BODY-009
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

Footer agent, thinking and model controls, wide menus and harness avatars
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change kyyuzynv above nwwsyrru in jj-store-chat-body
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

The fixed footer exposed only a text Send button and shortcut label. Captured and simulated assistant messages used initials. The first exact FooterRuntimeControls regression failed because no Model control existed. After implementing controls, the user reported narrow popovers. The width regression reproduced 128px where the thinking menu needed at least 223px; agent/model menus inherited the same trigger-dependent minimum.
<!-- /evidence-field -->

<!-- evidence-field:findings markdown -->
## Findings

ChatRuntimeControls composes the public Design Core DropdownMenu radio menu and Button with owned CommunityMemberAvatar. The host owns choices and selections; callbacks retain message and comment drafts. The fixed footer composes an optional controls snippet, compact thinking/model labels, an icon-only agent selector and icon-only Send with an accessible name and shortcut title. Agent/model menus use 18rem and thinking uses 14rem, bounded by the viewport with clearance. Narrow footer labels hide while controls keep their accessible names. Menu selection and Escape restore trigger focus. Catalog-only Codex, Cursor and OpenCode choices use existing bundled harness assets; historical Codex identities do not change with the selection. Simulated deliveries retain their agent/model/thinking snapshot, and linked simulated replies keep that harness after later selection changes. No live catalog, settings, dispatch, persistence, dependency changes or copied assets were introduced. Public menu layout props provide sizing; shared dependency files remain untouched.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/AgentChatBody.stories.svelte
- catalog/fixtures/ChatBodyFixture.svelte
- catalog/fixtures/delta-chat/adapter.ts
- catalog/fixtures/delta-chat/browser.acceptance.mjs
- catalog/fixtures/delta-chat/footer-runtime-native.mjs
- catalog/fixtures/delta-chat/footer-runtime-play.ts
- catalog/fixtures/delta-chat/next-message.svelte.ts
- catalog/fixtures/delta-chat/playground.ts
- catalog/fixtures/delta-chat/runtime.svelte.ts
- catalog/fixtures/delta-chat/runtime.test.ts
- spec/src/current/contracts/api/41-interactive-agent-execution.md
- spec/src/current/contracts/ui/36-commentable-chat-body-prototype.md
- spec/src/current/progress/M24/index.md
- spec/src/current/verification/ui-02.md
- spec/src/current/verification/ui.md
- spec/src/history/2026/10/08/E-CHAT-BODY-009/SUMMARY.nav.md
- spec/src/history/2026/10/08/E-CHAT-BODY-009/details-02.md
- spec/src/history/2026/10/08/E-CHAT-BODY-009/details-03.md
- spec/src/history/2026/10/08/E-CHAT-BODY-009/index.md
- spec/src/history/2026/10/08/SUMMARY.nav.md
- spec/src/history/2026/10/08/index.md
- web/src/lib/ui/store-agent-chat-body/ChatMessageFooter.svelte
- web/src/lib/ui/store-agent-chat-body/ChatRuntimeControls.svelte
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- catalog/fixtures/delta-chat/runtime.test.ts#simulated replies retain the
  harness and model selected at delivery
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-CHAT-BODY-009 details 2](details-02.md)
- [E-CHAT-BODY-009 details 3](details-03.md)
