---
schema: 1
id: E-CHAT-LIVE-013
---

# E-CHAT-LIVE-013

<!-- evidence-field:id markdown -->
## Id

E-CHAT-LIVE-013
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M24
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-012
- JST-API-001
- JST-GOV-008
- JST-GOV-002
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-09
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Repository-grouped session navigation
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change ovpmqyms
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

The live Session chats sidebar rendered a flat list. Three corrected repository-tree stories failed and the adapter identity regression failed. The initial story fixture separately had three unhandled errors from incorrect shell composition; these were corrected before the valid before run.
<!-- /evidence-field -->

<!-- evidence-field:findings markdown -->
## Findings

Loaded session rows group by exact repository ID, retaining source order, exact independent workspace/operation links, presence and activity-time tooltips. Same-name repositories remain separate. Public Collapsible/Button controls and Lucide icons adapt maintained Community navigation. Groups start expanded; disclosure state survives refresh, rename, reorder, an empty loaded list and hide/show. New active selections reveal their group without resetting drafts. Unbound drafts have a No repository group. Compact child rows preserve status and exact navigation. Long-name testing caught an intrinsic grid-width overflow; minmax(0, 1fr) and shrinkable group/label layout keep names within the sidebar. Test failures during disclosure teardown were fixed by awaiting the existing asynchronous visibility transition, retaining the same final visibility assertion. Community sources were inspected before Design Core. No upstream component defect or external import was introduced.
<!-- /evidence-field -->

<!-- evidence-field:live markdown -->
## Live

A fresh in-app browser page inspected preserved selected session 3aa8c81a8b5fbe2219498750ab014416. Its nine chats appear under two exact repository groups (seven and two). At 224px, the long label has 136px available for 405px of text; at 480px it has 392px, with ellipsis overflow and no row expansion. Loaded chat rows measure 53.140625px. Collapse and hide/show retain group state and current selection. Width was restored to approximately its prior 296px. A fresh page reload verifies the final implementation and repository-catalog name updates. No prompt, resume, installation or configuration save was performed. Warning/error logs are empty, including ResizeObserver for the inspected flows. Screenshot: .local/session-repository-tree-final.png. The supplemental Storybook server was started fresh on 6048 for inspection; its first attempted tab preceded startup and showed connection refused. Temporary browser tabs were closed or left to automatic tool cleanup.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- web/src/lib/ui/community-inbox/CommunityInbox.svelte
- web/src/lib/ui/community-inbox/CommunityInboxItem.svelte
- web/src/lib/ui/community-inbox/CommunityInboxSessions.svelte
- web/src/lib/ui/community-inbox/inbox.css
- web/src/lib/ui/community-inbox/model.ts
- web/src/lib/ui/store-agent-execution/session-presentation.ts
- web/tests/agent-inbox-groups.test.ts
- web/tests/managed-agent.test.ts
- catalog/fixtures/AgentInboxTreeFixture.svelte
- catalog/components/store-application/AgentInboxTree.stories.svelte
- docs/ui-provenance.json
- spec/src/current/contracts/ui/34-interactive-agent-sessions.md
- spec/src/current/contracts/api/41-interactive-agent-execution.md
- spec/src/current/governance/spec-governance.md
- spec/src/current/progress/M24/index.md
- spec/src/current/progress/M24/history/M24/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-CHAT-LIVE-013 details 2](details-02.md)
- [E-CHAT-LIVE-013 details 3](details-03.md)
