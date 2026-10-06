---
schema: 1
id: E-EXEC-008
---

# E-EXEC-008

<!-- evidence-field:id markdown -->
## Id

E-EXEC-008
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-06
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M23
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-COLLAB-009
- JST-COLLAB-010
- JST-COLLAB-031
- JST-COLLAB-032
- JST-COLLAB-033
- JST-UI-015
- JST-LOCAL-001
- JST-LOCAL-003
- JST-LOCAL-006
- JST-GOV-002
- JST-GOV-008
- JST-ARCH-001
- JST-ARCH-004
- JST-DOC-002
- JST-MUT-001
- JST-CLI-004
- JST-COMPAT-001
- JST-API-004
- JST-API-006
- JST-COLLAB-027
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/AgentDiscussion.stories.svelte
- catalog/fixtures/AgentExecutionCommandOutput.svelte
- catalog/fixtures/AgentExecutionFixture.svelte
- catalog/fixtures/agent-discussion-fixture.ts
- catalog/fixtures/agent-execution-session.ts
- catalog/fixtures/agent-execution-status.ts
- catalog/fixtures/thread-transport.ts
- crates/jj-collab/src/commands.rs
- crates/jj-collab/src/objects.rs
- crates/jj-collab/src/threads.rs
- crates/jj-collab/tests/agent_discussion.rs
- crates/jj-collab/tests/target_resolution.rs
- crates/jj-store-http/src/agent_execution/tests.rs
- crates/jj-store-http/src/agent_execution.rs
- crates/jj-store-server/src/agent_execution/commands.rs
- crates/jj-store-server/src/agent_execution/feedback.rs
- crates/jj-store-server/src/agent_execution/mod.rs
- crates/jj-store-server/src/agent_execution/publication.rs
- crates/jj-store-service/src/agent_execution.rs
- crates/jj-store-service/src/agent_feedback/tests.rs
- crates/jj-store-service/src/agent_feedback.rs
- crates/jj-store-service/src/collab_reads/agent_feedback.rs
- crates/jj-store-service/src/collab_reads.rs
- crates/jj-store-service/src/conversion/collab.rs
- crates/jj-store-service/src/lib.rs
- crates/jj-store-service/src/local_repository/agent_feedback.rs
- crates/jj-store-service/src/local_repository/collab_commands.rs
- crates/jj-store-service/src/local_repository.rs
- crates/jj-store-service/tests/conversion_agent_discussion.rs
- crates/jj-store-service/tests/local_collab/agent_discussion.rs
- crates/jj-store-service/tests/local_collab.rs
- crates/jj-store-wal/src/publication/collab_commands.rs
- docs/agent-execution.md
- docs/maintenance/coverage.json
- docs/site/concepts/collaboration.md
- docs/site/guides/agent-execution.md
- docs/site/reference/api/collaboration.md
- docs/site/reference/api/installation.md
- docs/site/reference/api/types.md
- docs/ui-provenance.json
- schemas/openapi.json
- scripts/agent-execution-acceptance.py
- scripts/agent-execution-browser-input.mjs
- scripts/agent-execution-browser.mjs
- scripts/agent-execution-discussion.mjs
- spec/src/current/contracts/api/41-interactive-agent-execution.md
- spec/src/current/contracts/collaboration/16-persistent-threads-and-reactions-m15.md
- spec/src/current/contracts/collaboration/36-interactive-agent-execution.md
- spec/src/current/contracts/local-jj/07-m15-additive-collaboration-publication.md
- spec/src/current/contracts/ui/34-interactive-agent-sessions.md
- spec/src/current/progress/M23/history/M23/index.md
- spec/src/current/progress/M23/index.md
- spec/src/history/2026/10/06/E-EXEC-005/index.md
- spec/src/history/2026/10/06/E-EXEC-006/index.md
- spec/src/history/2026/10/06/E-EXEC-007/index.md
- spec/src/history/2026/10/06/E-EXEC-008/SUMMARY.nav.md
- spec/src/history/2026/10/06/E-EXEC-008/details-02.md
- spec/src/history/2026/10/06/E-EXEC-008/index.md
- spec/src/history/2026/10/06/E-EXEC-009/SUMMARY.nav.md
- spec/src/history/2026/10/06/E-EXEC-009/index.md
- spec/src/history/2026/10/06/SUMMARY.nav.md
- spec/src/history/2026/10/06/index.md
- spec/src/previous-locations.md
- web/src/lib/api/generated.ts
- web/src/lib/controller/agent-discussions.svelte.ts
- web/src/lib/controller/agent-execution-launch-context.ts
- web/src/lib/controller/agent-execution.svelte.ts
- web/src/lib/controller/production-local-configuration.ts
- web/src/lib/controller/production-threads.svelte.ts
- web/src/lib/controller/thread-post.svelte.ts
- web/src/lib/ui/store-agent-execution/StoreAgentInboxMessage.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentTranscript.svelte
- web/tests/vite/agent-execution-controller.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-collab/tests/agent_discussion.rs#agent_discussion_retains_exact_target_and_legacy_bytes
- crates/jj-store-service/tests/local_collab/agent_discussion.rs#saved_agent_replies_and_reactions_survive_restart_and_keep_exact_targets
- crates/jj-store-service/src/agent_feedback/tests.rs#prompt_feedback_is_exact_bounded_and_omits_tombstones
- crates/jj-store-http/src/agent_execution/tests.rs#feedback_is_read_only_local_admin_and_rejects_unbound_queries
- crates/jj-store-service/tests/conversion_agent_discussion.rs#conversion_preserves_agent_discussion_identity_and_requires_exact_witness_mapping
<!-- /evidence-field -->

