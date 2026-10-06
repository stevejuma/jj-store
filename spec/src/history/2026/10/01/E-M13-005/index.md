---
schema: 1
id: E-M13-005
---

# E-M13-005

<!-- evidence-field:id markdown -->
## Id

E-M13-005
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M13
- M13.3
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-01
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

filesystem-git
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

vrunmlxs
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/src/agent_capture.rs
- crates/jj-store-cli/src/agent_capture_intent.rs
- crates/jj-store-cli/src/agent_commands.rs
- crates/jj-store-cli/src/agent_commands/backend.rs
- crates/jj-store-cli/src/agent_publication.rs
- crates/jj-store-cli/src/lib.rs
- crates/jj-store-cli/src/local_agent_publication.rs
- crates/jj-store-cli/src/local_diagnostics.rs
- crates/jj-store-cli/tests/local_agent.rs
- crates/jj-store-cli/tests/local_agent_recovery.rs
- crates/jj-store-service/src/local_repository.rs
- crates/jj-store-service/src/local_repository/actor.rs
- crates/jj-store-service/src/local_repository/agent_commands.rs
- crates/jj-store-service/src/local_repository/agent_pages.rs
- crates/jj-store-service/tests/local_agent_pages.rs
- docs/cli/agent.md
- spec/src/cli.md
- spec/src/local-jj.md
- spec/src/milestones.md
- spec/src/mutations.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/local_agent.rs#local_agent_reads_and_initialization_accept_all_git_layouts_and_workspaces
- crates/jj-store-cli/tests/local_agent.rs#cold_hook_sanitizes_and_publishes_to_local_git_without_native_connection
- crates/jj-store-cli/tests/local_agent_recovery.rs#unknown_local_capture_restarts_with_exact_intent_before_later_batches
- crates/jj-store-cli/tests/local_agent_recovery.rs#mismatched_receipts_and_repository_binding_never_complete_local_captures
- crates/jj-store-service/tests/local_agent_pages.rs#local_session_cursor_survives_restart_and_later_capture
- crates/jj-store-service/tests/local_agent_pages.rs#forged_agent_cursor_cannot_select_a_staged_metadata_candidate
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M13-005 details 2](details-02.md)
- [E-M13-005 details 3](details-03.md)
