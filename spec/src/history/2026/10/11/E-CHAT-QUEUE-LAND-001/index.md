---
schema: 1
id: E-CHAT-QUEUE-LAND-001
---

# E-CHAT-QUEUE-LAND-001

<!-- evidence-field:id markdown -->
## Id

E-CHAT-QUEUE-LAND-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M24
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-018
- JST-GOV-002
- JST-GOV-008
- JST-API-001
- JST-API-006
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

tlqzqqrn
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- web/src/lib/ui/store-agent-chat-body/text-reader.css
- spec/src/current/progress/M24/index.md
- spec/src/current/progress/M24/queued-messages-review.md
- spec/src/history/2026/10/11/E-CHAT-QUEUE-LAND-001/index.md
- spec/src/history/2026/10/11/E-CHAT-QUEUE-LAND-001/details-02.md
- spec/src/history/2026/10/11/E-CHAT-QUEUE-LAND-001/SUMMARY.nav.md
- spec/src/history/2026/10/11/index.md
- spec/src/history/2026/10/11/SUMMARY.nav.md
- spec/src/history/2026/10/11/E-CHAT-QUEUE-LAND-001/details-03.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/vite/queued-agent-messages.test.ts#queue captures immutable reviews
  and advances recipients independently across views
- web/tests/vite/agent-chat.test.ts#controller queue acknowledgement refreshes
  the retained receipt without dispatching again
- catalog/fixtures/delta-chat/markdown-reading.test.ts#reading history renders
  angle-bracket repository file destinations as links
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Land all queued-message changes from this session into default/main, retain concurrent changes and retire only this session’s temporary JJ workspaces. Existing Document/Classic queue, exact snapshots/cancellation, comment-style cards, full-row opacity, prose parity, avatar alignment and See More / See Less remain unchanged. Storybook moves to the main checkout on the existing literal-loopback port 6048.
<!-- /evidence-field -->

<!-- evidence-field:before_state markdown -->
## Before state

Queue commit tszmluon/66619f00 was isolated in queued-messages. Main had 83 changed files at f8ac8f06541a5b2769d53ae6978aa8c46b526b2f; other sessions committed Markdown navigation and column acceptance during integration. Parent baseline and exact main-file hashes retained before rebase.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-CHAT-QUEUE-LAND-001 details 2](details-02.md)
- [E-CHAT-QUEUE-LAND-001 details 3](details-03.md)
