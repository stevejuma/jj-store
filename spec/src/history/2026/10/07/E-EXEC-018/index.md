---
schema: 1
id: E-EXEC-018
---

# E-EXEC-018

<!-- evidence-field:date markdown -->
## Date

2026-10-07
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M23
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/AgentChatFeedback.stories.svelte
- catalog/components/store-application/AgentChatLayout.stories.svelte
- catalog/components/store-application/AgentInboxNavigation.stories.svelte
- catalog/components/store-application/AgentSessions.stories.svelte
- catalog/fixtures/AgentExecutionCommandOutput.svelte
- catalog/fixtures/AgentExecutionFixture.svelte
- docs/maintenance/coverage.json
- docs/site/guides/agent-execution.md
- docs/ui-provenance.json
- scripts/agent-inbox-presentation.mjs
- spec/src/current/contracts/ui/34-interactive-agent-sessions.md
- spec/src/current/progress/M23/history/M23/index.md
- spec/src/current/progress/M23/index.md
- spec/src/history/2026/10/07/E-EXEC-018/SUMMARY.nav.md
- spec/src/history/2026/10/07/E-EXEC-018/details-02.md
- spec/src/history/2026/10/07/E-EXEC-018/details-03.md
- spec/src/history/2026/10/07/E-EXEC-018/index.md
- spec/src/history/2026/10/07/E-EXEC-019/SUMMARY.nav.md
- spec/src/history/2026/10/07/E-EXEC-019/index.md
- spec/src/history/2026/10/07/SUMMARY.nav.md
- spec/src/history/2026/10/07/index.md
- web/src/lib/ui/community-agent-activity/activity-feed.css
- web/src/lib/ui/community-agents/CommunityAgents.svelte
- web/src/lib/ui/community-agents/agents.css
- web/src/lib/ui/community-inbox/CommunityInbox.svelte
- web/src/lib/ui/community-inbox/CommunityInboxItem.svelte
- web/src/lib/ui/community-inbox/inbox-layout.ts
- web/src/lib/ui/community-inbox/inbox-resize.svelte.ts
- web/src/lib/ui/community-inbox/inbox.css
- web/src/lib/ui/community-inbox/model.ts
- web/src/lib/ui/store-agent-execution/StoreAgentExecutionHost.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentInboxMessage.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentTranscript.svelte
- web/src/lib/ui/store-agent-execution/presentation.ts
- web/src/lib/ui/store-agent-execution/session-presentation.ts
- web/tests/agent-execution.test.ts
- web/tests/agent-inbox-layout.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/agent-execution.test.ts#message times retain first observation
  across chunks and never fabricate legacy times
- web/tests/agent-execution.test.ts#inbox day dividers share thread labels and
  group genuinely unrecorded times
- web/tests/agent-inbox-layout.test.ts#inbox width survives reopening with an
  independent version and panel identity
- web/tests/agent-inbox-layout.test.ts#blocked device storage keeps inbox layout
  usable
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

[]
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Enclosing Jujutsu change xpopuyuk; macOS ARM64, Node 24.10.0, pnpm 10.32.1, Vitest 4.1.11. Community source 741b8285e2081f1c9af58afe5a9b72f538fad781, Apache-2.0.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

UI-only; existing event DTO and canonical stores unchanged. New private version-1 device layout key jj-store/layout/v1/agent-inbox/sessions, independent from repository shell/thread layout. No contract regeneration or Rust lane required.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Maintain qualified inbox behavior. Exact historical times cannot be reconstructed for events with no recorded time; pooling, native code publication qualification and visual baseline approval remain separate. Rust and root aggregate check/test/build lanes are out of scope under targeted UI verification policy.
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M23 original Done gates remain unchanged. This maintenance slice
  qualifies session navigation, honest timing presentation and layout
  persistence; it does not expand harness/publication scope or approve visual
  baselines.

```
<!-- /evidence-field -->

<!-- evidence-field:id markdown -->
## Id

E-EXEC-018
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-EXEC-018 details 2](details-02.md)
- [E-EXEC-018 details 3](details-03.md)
