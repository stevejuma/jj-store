---
schema: 1
id: E-DOCUMENT-SEND-001
---

# E-DOCUMENT-SEND-001

<!-- evidence-field:id markdown -->
## Id

E-DOCUMENT-SEND-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M24
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-018
- JST-API-001
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-11
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

mwxqqtwv
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/AgentSessionState.stories.svelte
- spec/src/current/contracts/api/41-interactive-agent-execution.md
- spec/src/current/contracts/ui/37-document-conversations.md
- spec/src/current/progress/M24/index.md
- spec/src/current/progress/M24/queued-messages-review.md
- spec/src/history/2026/10/11/E-DOCUMENT-SEND-001/SUMMARY.nav.md
- spec/src/history/2026/10/11/E-DOCUMENT-SEND-001/details-02.md
- spec/src/history/2026/10/11/E-DOCUMENT-SEND-001/index.md
- spec/src/history/2026/10/11/SUMMARY.nav.md
- spec/src/history/2026/10/11/index.md
- web/src/lib/ui/store-agent-chat-body/ChatMessageFooter.svelte
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
<!-- /evidence-field -->

<!-- evidence-field:story_files list -->
## Story files

- catalog/components/store-application/AgentSessionState.stories.svelte
- catalog/components/store-application/QueuedAgentMessages.stories.svelte
- catalog/components/store-application/ConversationFooterLayout.stories.svelte
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Correct the shared Document message-footer presentation in the main workspace: idle Send uses send-horizontal and a circular button; active empty drafts select the existing red/destructive rounded-square Stop; active content selects a circular arrow-up Queue. Preserve request, draft, cancellation and queue semantics. Classic composer remains owned by its existing component.
<!-- /evidence-field -->

<!-- evidence-field:before_state markdown -->
## Before state

ChatMessageFooter imported arrow-up as SendIcon for every non-Stop state. Default icon-sm buttons were rounded-square; Queue and idle Send lacked the requested circular appearance. Main contained 28 unrelated pending documentation paths. Work is inserted beneath that retained change, without a new workspace.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-DOCUMENT-SEND-001 details 2](details-02.md)
- [E-DOCUMENT-SEND-001 details 3](details-03.md)
