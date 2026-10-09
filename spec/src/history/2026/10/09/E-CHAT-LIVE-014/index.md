---
schema: 1
id: E-CHAT-LIVE-014
---

# E-CHAT-LIVE-014

<!-- evidence-field:id markdown -->
## Id

E-CHAT-LIVE-014
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M24
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-012
- JST-API-001
- JST-GOV-002
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-09
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Repository header actions and matching loading layout
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change svvwqkpp
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Two exact regressions failed for repeated chat-row links and session-pinned destinations. Subsequent requested surface, icon-inset and grouped-skeleton regressions each failed once: the header wrapper was transparent, the action measured 32px with no outer inset, and loading retained the old flat message skeleton.
<!-- /evidence-field -->

<!-- evidence-field:findings markdown -->
## Findings

Each exact repository group now has one independent, tooltipped normal repository file link between its label disclosure and a separate final chevron disclosure. Chat rows no longer contain repository actions or reserved action padding. Both label and chevron toggle the same group; repository navigation remains available when collapsed and never selects an arbitrary session workspace/operation. The full muted header surface uses theme tokens, with stronger action hover/focus treatment. Public icon-xs sizing gives 24px controls, 6px vertical inset and 4px outer end clearance. The existing maintained inbox component composes public Skeleton primitives into anonymous repository-header placeholders and five indented avatar/name/context rows. Loaded and placeholder row heights agree within 2px. One busy status retains the selected chat and draft, and the empty state waits until loading finishes. Community navigation/skeleton sources were inspected before Design Core Button, Collapsible and Skeleton; existing provenance remains valid, with no new copied file or checkout dependency.
<!-- /evidence-field -->

<!-- evidence-field:live markdown -->
## Live

Fresh in-app browser inspection used preserved session 3aa8c81a8b5fbe2219498750ab014416, with nine chats under two groups. DOM inspection confirms two header links and zero child-row links. The collapsed group action opens its existing repository file route; an initial short visibility wait elapsed while loading, then the next observation confirmed the metadata home's normal empty file table. Back returns to the selected chat. A real CDP pointer move, grounded in the action bounds, confirms :hover and a distinct action background (srgb 0.845043/0.8474/0.849757) over the shared row (0.909014/0.9098/0.910586). Action bounds are 24x24, top/bottom clearances 6px; the final chevron is after the action and 4px inside the outer right edge. Keyboard Tab reaches the action and shows its tooltip without collapsing the group. Screenshot: .local/session-header-repository-link-final.png. No prompt, resume, configuration save or other execution was performed. Warning/error logs are empty, including ResizeObserver for this inspected flow. The temporary browser tab was closed.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- web/src/lib/ui/community-inbox/CommunityInbox.svelte
- web/src/lib/ui/community-inbox/CommunityInboxSessions.svelte
- web/src/lib/ui/community-inbox/CommunityInboxItem.svelte
- web/src/lib/ui/community-inbox/inbox.css
- catalog/components/store-application/AgentInboxTree.stories.svelte
- catalog/components/store-application/AgentInboxNavigation.stories.svelte
- catalog/fixtures/AgentInboxTreeFixture.svelte
- spec/src/current/contracts/ui/34-interactive-agent-sessions.md
- spec/src/current/contracts/api/41-interactive-agent-execution.md
- spec/src/current/progress/M24/index.md
- spec/src/current/progress/M24/history/M24/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-CHAT-LIVE-014 details 2](details-02.md)
- [E-CHAT-LIVE-014 details 3](details-03.md)
