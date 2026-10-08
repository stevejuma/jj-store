---
schema: 1
id: E-FLEET-011
---

# E-FLEET-011

<!-- evidence-field:id markdown -->
## Id

E-FLEET-011
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M24
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-019
- JST-UI-015
- JST-API-001
- JST-DOC-001
- JST-DOC-002
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-08
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Single-line agent request status and actionable recovery toasts
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change srvsznlw above bdbebe6c
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

The unknown-outcome and not-known recovery buttons rendered as siblings beneath the activity strip. A selected chat could duplicate the error in its composer. The new uncertain-request browser regression failed because Review workspace was absent from a toast (1 failed, 3 passed).
<!-- /evidence-field -->

<!-- evidence-field:findings markdown -->
## Findings

Recovery details and actions now use the existing application notification service and Design Core toast presentation. Pending requests suppress duplicate composer feedback. The activity strip does not wrap; desktop and 390px geometry agree. Dismissal preserves the pending request and draft across navigation-driven refresh, and Session options retains the action. Actions verify the exact request, receipt and state and are inert during dispatch. Review preserves unresolved history and opens the original session without a prompt; confirmed not-known requests retain their explicit exact retry. Receipt mismatch shows Checking instead of an older request outcome. All 58 direct browser stories and 16 unit tests pass. Type checks and documentation checks pass. The unchanged development-script line-limit failure remains separate.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

UI-only execution recovery, activity-strip layout, shared Session options and toast adapter. Escalation to 11 AgentExecutionFixture story families covers their shared notification/menu consumers. Cold Vitest browser processes cover lifecycle and keyboard/Axe acceptance; no actual provider prompts, fixture reset, server restart, schema changes, Rust lanes, root aggregate suites or screenshot baselines. The existing user tab was observed read-only and preserved; it had already moved to a different preparing session, so no live error was induced.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/AgentChatFeedback.stories.svelte
- catalog/components/store-application/AgentExecution.stories.svelte
- docs/maintenance/coverage.json
- docs/site/guides/agent-execution.md
- spec/src/current/contracts/api/41-interactive-agent-execution.md
- spec/src/current/contracts/documentation/08-interactive-agent-execution.md
- spec/src/current/contracts/ui/03-requirements.md
- spec/src/current/contracts/ui/06-application-notifications.md
- spec/src/current/contracts/ui/34-interactive-agent-sessions.md
- spec/src/current/progress/M24/history/M24/index.md
- spec/src/current/progress/M24/index.md
- web/src/lib/ui/community-agent-composer-activity/CommunityAgentComposerActivity.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentChatActions.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentConversation.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentExecutionNotice.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentRequestRecovery.svelte
- web/src/lib/ui/store-agent-execution/request-recovery-notice.ts
- web/tests/vite/agent-request-recovery-notice.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/vite/agent-request-recovery-notice.test.ts#review retains the
  unknown outcome and opens its original session without retrying
- web/tests/vite/agent-request-recovery-notice.test.ts#recovery actions refuse
  stale requests, changed outcomes and busy dispatch
- web/tests/vite/agent-request-recovery-notice.test.ts#dismissal survives
  refresh and navigation while a different request creates a new notice
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-FLEET-011 details 2](details-02.md)
- [E-FLEET-011 details 3](details-03.md)
