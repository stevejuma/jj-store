---
schema: 1
id: E-EXEC-020
---

# E-EXEC-020

<!-- evidence-field:date markdown -->
## Date

2026-10-07
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M23
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/AgentInboxNavigation.stories.svelte
- catalog/components/store-application/AgentPromptMarkup.stories.svelte
- catalog/components/store-application/ManagedAgentIdentity.stories.svelte
- catalog/fixtures/AgentExecutionFixture.svelte
- catalog/fixtures/ManagedAgentRepositoryFixture.svelte
- catalog/fixtures/agent-execution-session.ts
- crates/jj-store-server/src/agent_execution/commands.rs
- crates/jj-store-server/src/agent_execution/feedback.rs
- crates/jj-store-server/src/agent_execution/journal.rs
- crates/jj-store-server/src/agent_execution/launch.rs
- crates/jj-store-service/src/agent_execution.rs
- docs/maintenance/coverage.json
- docs/site/guides/agent-execution.md
- docs/site/reference/api/types.md
- docs/site/reference/configuration.md
- docs/site/reference/transport.md
- docs/site/technical/architecture.md
- schemas/openapi.json
- scripts/agent-inbox-presentation.mjs
- scripts/managed-agent-identity.mjs
- spec/src/current/contracts/api/41-interactive-agent-execution.md
- spec/src/current/contracts/cli/35-interactive-agent-execution.md
- spec/src/current/contracts/collaboration/36-interactive-agent-execution.md
- spec/src/current/contracts/compatibility/13-interactive-agent-execution.md
- spec/src/current/contracts/mutations/22-interactive-agent-execution.md
- spec/src/current/contracts/ui/34-interactive-agent-sessions.md
- spec/src/current/progress/M23/history/M23/index.md
- spec/src/current/progress/M23/index.md
- spec/src/history/2026/10/07/E-EXEC-020/SUMMARY.nav.md
- spec/src/history/2026/10/07/E-EXEC-020/details-02.md
- spec/src/history/2026/10/07/E-EXEC-020/details-03.md
- spec/src/history/2026/10/07/E-EXEC-020/index.md
- spec/src/history/2026/10/07/E-EXEC-021/SUMMARY.nav.md
- spec/src/history/2026/10/07/E-EXEC-021/index.md
- spec/src/history/2026/10/07/SUMMARY.nav.md
- spec/src/history/2026/10/07/index.md
- web/src/lib/api/generated.ts
- web/src/lib/collaboration/managed-agent.ts
- web/src/lib/collaboration/presentation.ts
- web/src/lib/ui/store-agent-execution/StoreAgentExecutionHost.svelte
- web/src/lib/ui/store-agent-execution/session-presentation.ts
- web/src/lib/ui/store-application/StoreApplication.svelte
- web/src/lib/ui/store-application/StoreWorkspacePicker.svelte
- web/src/lib/ui/store-collaboration/StoreAgentContext.svelte
- web/src/lib/ui/store-collaboration/StoreAgentSessionRow.svelte
- web/src/lib/ui/store-collaboration/StoreAgentSessions.svelte
- web/src/lib/ui/store-collaboration/StoreAgentSidebar.svelte
- web/tests/collaboration-presentation.test.ts
- web/tests/managed-agent.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/collaboration-presentation.test.ts#managed repository sessions use
  captured identity without changing message authors
- web/tests/managed-agent.test.ts#recorded identity requires exact repository,
  integration and a unique admitted alias
- web/tests/managed-agent.test.ts#workspace avatars require exact repository and
  ready retained workspace names
- web/tests/managed-agent.test.ts#last activity remains available without a
  first activity timestamp
- web/tests/managed-agent.test.ts#inbox repository links name the repository and
  round trip exact workspace and operation
- crates/jj-store-server/src/agent_execution/feedback.rs#committed_session_aliases_preserve_capture_identity_and_isolate_instances
- crates/jj-store-server/src/agent_execution/journal.rs#journal_restart_retains_agent_association_but_never_connection
<!-- /evidence-field -->


<!-- generated-navigation -->
## Pages

- [E-EXEC-020 details](details-02.md)
- [E-EXEC-020 details](details-03.md)
