---
schema: 1
id: E-CHAT-WORKSPACE-001
---

# E-CHAT-WORKSPACE-001

<!-- evidence-field:id markdown -->
## Id

E-CHAT-WORKSPACE-001
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
- JST-JJ-003
- JST-JJ-004
- JST-MUT-001
- JST-CLI-004
- JST-COMPAT-001
- JST-COMPAT-003
- JST-COMPAT-005
- JST-LOCAL-004
- JST-LOCAL-005
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-09
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/agent_execution/conversations.rs
- crates/jj-store-cli/tests/agent_execution/tooling.rs
- crates/jj-store-cli/tests/agent_execution/workspace_inspection.rs
- crates/jj-store-cli/tests/agent_execution.rs
- crates/jj-store-http/src/agent_execution/tests.rs
- crates/jj-store-http/src/agent_execution.rs
- crates/jj-store-jj/src/agent_workspaces/ignores.rs
- crates/jj-store-jj/src/agent_workspaces/inspection.rs
- crates/jj-store-jj/src/agent_workspaces/mod.rs
- crates/jj-store-jj/src/local_stacks/mod.rs
- crates/jj-store-server/src/agent_execution/commands.rs
- crates/jj-store-server/src/agent_execution/driver.rs
- crates/jj-store-server/src/agent_execution/fleet.rs
- crates/jj-store-server/src/agent_execution/mod.rs
- crates/jj-store-server/src/agent_execution/runtime.rs
- crates/jj-store-server/src/agent_execution/workspace_inspection/comparison.rs
- crates/jj-store-server/src/agent_execution/workspace_inspection/content.rs
- crates/jj-store-server/src/agent_execution/workspace_inspection/source.rs
- crates/jj-store-server/src/agent_execution/workspace_inspection.rs
- crates/jj-store-server/src/agent_execution/workspace_turns.rs
- crates/jj-store-service/src/agent_execution/workspace_inspection/files.rs
- crates/jj-store-service/src/agent_execution/workspace_inspection.rs
- crates/jj-store-service/src/agent_execution.rs
- crates/jj-store-service/tests/agent_workspace_files.rs
- schemas/openapi.json
- spec/src/current/contracts/api/41-interactive-agent-execution.md
- spec/src/current/contracts/cli/35-interactive-agent-execution.md
- spec/src/current/contracts/collaboration/37-reusable-agent-conversations.md
- spec/src/current/contracts/compatibility/13-interactive-agent-execution.md
- spec/src/current/contracts/jj-integration/32-interactive-agent-workspaces.md
- spec/src/current/contracts/local-jj/12-bounded-local-code-reads.md
- spec/src/current/contracts/mutations/22-interactive-agent-execution.md
- spec/src/current/progress/M24/history/M24/index.md
- spec/src/current/progress/M24/index.md
- web/src/lib/api/generated.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-service/tests/agent_workspace_files.rs#listing_includes_unchanged_and_untracked_files_but_not_ignored_or_metadata
- crates/jj-store-service/tests/agent_workspace_files.rs#inspection_request_rejects_arbitrary_roots_and_revision_expressions
- crates/jj-store-http/src/agent_execution/tests.rs#workspace_inspection_is_local_admin_and_accepts_no_filesystem_root
- crates/jj-store-cli/tests/agent_execution/conversations.rs#repository_free_chat_reuses_isolated_bindings_and_never_replays_after_restart
- crates/jj-store-server/src/agent_execution/workspace_turns.rs#queued_turns_wait_for_the_exact_active_completion_and_cancel_without_dispatch
- crates/jj-store-server/src/agent_execution/workspace_inspection/comparison.rs#latest_completed_turn_requires_its_own_exact_repository_boundaries
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-CHAT-WORKSPACE-001 details 2](details-02.md)
- [E-CHAT-WORKSPACE-001 details 3](details-03.md)
