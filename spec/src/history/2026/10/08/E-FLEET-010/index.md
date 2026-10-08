---
schema: 1
id: E-FLEET-010
---

# E-FLEET-010

<!-- evidence-field:id markdown -->
## Id

E-FLEET-010
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M24
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-COLLAB-034
- JST-COLLAB-035
- JST-COLLAB-036
- JST-API-001
- JST-API-006
- JST-UI-005
- JST-UI-015
- JST-DOC-001
- JST-DOC-002
- JST-GOV-007
- JST-GOV-011
- JST-CLI-004
- JST-MUT-001
- JST-COMPAT-003
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-08
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Event-driven agent status, settings, targets and conversations
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change omtpvtvv above 44c6c979
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

AgentExecution reloaded status/settings/targets every two seconds, the directory added a five-second refresh, contextual chat polled every second, and recorded discussions polled every two seconds even with a healthy shared feed. The supervisor session broadcaster was not a producer of application events. The exact before regression failed twice: healthy idle continued GETs and scoped events did not refresh the appropriate reader.
<!-- /evidence-field -->

<!-- evidence-field:findings markdown -->
## Findings

LocalTrusted Admin receives separate body-free agent_execution and agent_settings notices through the existing shared SSE feed. The supervisor compares semantic state and suppresses private journal revision/publication retry churn; privacy completion and worker transitions also notify. Status, settings and targets are independent coalesced readers; repository/workspace/code notices invalidate targets. Contextual history follows adopted status; recorded discussion follows its repository notices. Healthy idle and unchanged renewal perform no periodic agent data GETs. Outage fallback backs off at jittered 5/10/20/30 seconds; dirty failed reads retry and preserve drafts and exact request identities. Explicit failed status and recorded feedback reads remain dirty. Catalog configure receipts settle independently and emit actual production-format notices instead of progressing on reads.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/fixtures/AgentExecutionFixture.svelte
- catalog/fixtures/agent-command-receipts.ts
- catalog/fixtures/agent-freshness.ts
- crates/jj-store-http/src/application_events/producers.rs
- crates/jj-store-http/src/application_events/tests.rs
- crates/jj-store-server/src/agent_execution/commands.rs
- crates/jj-store-server/src/agent_execution/fleet.rs
- crates/jj-store-server/src/agent_execution/freshness.rs
- crates/jj-store-server/src/agent_execution/mod.rs
- crates/jj-store-service/src/agent_execution.rs
- crates/jj-store-service/src/freshness.rs
- docs/maintenance/coverage.json
- docs/site/guides/browsing.md
- docs/site/reference/api/types.md
- docs/site/reference/transport.md
- docs/site/technical/lifecycle.md
- schemas/openapi.json
- spec/src/current/contracts/api/35-scoped-application-freshness.md
- spec/src/current/contracts/cli/35-interactive-agent-execution.md
- spec/src/current/contracts/collaboration/37-reusable-agent-conversations.md
- spec/src/current/contracts/compatibility/12-application-freshness-compatibility.md
- spec/src/current/contracts/documentation/08-interactive-agent-execution.md
- spec/src/current/contracts/mutations/20-scoped-application-freshness.md
- spec/src/current/contracts/ui/32-event-driven-freshness-migration.md
- spec/src/current/progress/M24/history/M24/index.md
- spec/src/current/progress/M24/index.md
- web/src/lib/api/application-events.ts
- web/src/lib/api/generated.ts
- web/src/lib/controller/agent-chat-types.ts
- web/src/lib/controller/agent-chat.svelte.ts
- web/src/lib/controller/agent-discussion-refresh.ts
- web/src/lib/controller/agent-discussions.svelte.ts
- web/src/lib/controller/agent-execution-refresh.ts
- web/src/lib/controller/agent-execution.svelte.ts
- web/src/lib/controller/agent-freshness.ts
- web/src/lib/controller/agent-settings.svelte.ts
- web/src/lib/controller/automatic-refresh.ts
- web/src/lib/ui/store-agent-execution/StoreAgentDiscussion.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentExecutionDirectory.svelte
- web/tests/browser/agent-chat-changes.spec.ts
- web/tests/real/agent-freshness.spec.ts
- web/tests/vite/agent-discussions.test.ts
- web/tests/vite/agent-freshness.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/vite/agent-freshness.test.ts#healthy idle agent state performs no
  periodic settings, targets or status reads
- web/tests/vite/agent-discussions.test.ts#discussion outage fallback runs
  independently and stops when the view closes
- web/tests/real/agent-freshness.spec.ts#cold agent directory is idle, follows
  scoped changes across tabs and recovers missed events
- crates/jj-store-http/src/application_events/tests.rs#agent_owner_notices_join_shared_feed_without_reads_and_remain_local_admin_only
- crates/jj-store-server/src/agent_execution/freshness.rs#internal_journal_revisions_do_not_invalidate_execution_but_visible_changes_do
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-FLEET-010 details 2](details-02.md)
- [E-FLEET-010 details 3](details-03.md)
