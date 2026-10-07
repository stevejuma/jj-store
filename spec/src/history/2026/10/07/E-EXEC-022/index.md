---
schema: 1
id: E-EXEC-022
---

# E-EXEC-022

<!-- evidence-field:id markdown -->
## Id

E-EXEC-022
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M23
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-COLLAB-031
- JST-COLLAB-032
- JST-COLLAB-033
- JST-UI-012
- JST-API-001
- JST-CLI-004
- JST-MUT-001
- JST-ARCH-001
- JST-ARCH-004
- JST-COMPAT-001
- JST-COMPAT-003
- JST-COMPAT-005
- JST-DOC-002
- JST-GOV-002
- JST-GOV-008
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-07
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

lmwxwnrz (enclosing Jujutsu change; parent b8698e2f)
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/AgentRuntimes.stories.svelte
- catalog/fixtures/AgentRuntimesFixture.svelte
- crates/jj-store-cli/tests/agent_execution/tooling.rs
- crates/jj-store-cli/tests/agent_execution.rs
- crates/jj-store-http/src/agent_execution/tests.rs
- crates/jj-store-http/src/agent_execution.rs
- crates/jj-store-server/src/agent_execution/commands.rs
- crates/jj-store-server/src/agent_execution/journal.rs
- crates/jj-store-server/src/agent_execution/mod.rs
- crates/jj-store-server/src/agent_execution/runtime.rs
- crates/jj-store-server/src/agent_execution/tooling/install.rs
- crates/jj-store-server/src/agent_execution/tooling/mod.rs
- crates/jj-store-server/src/agent_execution/tooling/paths.rs
- crates/jj-store-server/src/agent_execution/tooling/status.rs
- crates/jj-store-server/src/agent_execution/tooling/tests.rs
- crates/jj-store-service/src/agent_execution/tooling.rs
- crates/jj-store-service/src/agent_execution.rs
- docs/agent-execution.md
- docs/maintenance/coverage.json
- docs/site/guides/agent-execution.md
- docs/site/guides/settings.md
- docs/site/reference/api/installation.md
- docs/site/reference/api/types.md
- docs/site/reference/configuration.md
- docs/site/reference/transport.md
- docs/site/technical/architecture.md
- schemas/openapi.json
- scripts/agent-runtimes-browser.mjs
- spec/src/current/contracts/api/41-interactive-agent-execution.md
- spec/src/current/contracts/architecture/20-interactive-agent-execution.md
- spec/src/current/contracts/cli/35-interactive-agent-execution.md
- spec/src/current/contracts/collaboration/36-interactive-agent-execution.md
- spec/src/current/contracts/compatibility/13-interactive-agent-execution.md
- spec/src/current/contracts/documentation/08-interactive-agent-execution.md
- spec/src/current/contracts/mutations/22-interactive-agent-execution.md
- spec/src/current/contracts/ui/29-editable-agents-settings.md
- spec/src/current/governance/22-interactive-agent-execution.md
- spec/src/current/progress/M11/history/M11.4/index.md
- spec/src/current/progress/M11/index.md
- spec/src/current/progress/M13/history/M13.4/index.md
- spec/src/current/progress/M13/index.md
- spec/src/current/progress/M15/history/M15.5/index.md
- spec/src/current/progress/M15/index.md
- spec/src/current/progress/M18/history/M18.1/index.md
- spec/src/current/progress/M18/index.md
- spec/src/current/progress/M23/history/M23/index.md
- spec/src/current/progress/M23/index.md
- spec/src/current/progress/M9/history/M9.5/index.md
- spec/src/current/progress/M9/index.md
- spec/src/history/2026/10/06/E-EXEC-008/index.md
- spec/src/history/2026/10/06/E-EXEC-009/index.md
- spec/src/history/2026/10/07/E-EXEC-012/index.md
- spec/src/history/2026/10/07/E-EXEC-013/SUMMARY.nav.md
- spec/src/history/2026/10/07/E-EXEC-013/index.md
- spec/src/history/2026/10/07/E-EXEC-014/SUMMARY.nav.md
- spec/src/history/2026/10/07/E-EXEC-014/index.md
- spec/src/history/2026/10/07/E-EXEC-015/SUMMARY.nav.md
- spec/src/history/2026/10/07/E-EXEC-015/index.md
- spec/src/history/2026/10/07/E-EXEC-016/SUMMARY.nav.md
- spec/src/history/2026/10/07/E-EXEC-016/index.md
- spec/src/history/2026/10/07/E-EXEC-017/SUMMARY.nav.md
- spec/src/history/2026/10/07/E-EXEC-017/index.md
- spec/src/history/2026/10/07/E-EXEC-018/SUMMARY.nav.md
- spec/src/history/2026/10/07/E-EXEC-018/index.md
- spec/src/history/2026/10/07/E-EXEC-019/SUMMARY.nav.md
- spec/src/history/2026/10/07/E-EXEC-019/index.md
- spec/src/history/2026/10/07/E-EXEC-020/SUMMARY.nav.md
- spec/src/history/2026/10/07/E-EXEC-020/index.md
- spec/src/history/2026/10/07/E-EXEC-021/SUMMARY.nav.md
- spec/src/history/2026/10/07/E-EXEC-021/index.md
- spec/src/history/2026/10/07/E-EXEC-022/SUMMARY.nav.md
- spec/src/history/2026/10/07/E-EXEC-022/details-02.md
- spec/src/history/2026/10/07/E-EXEC-022/details-03.md
- spec/src/history/2026/10/07/E-EXEC-022/index.md
- spec/src/history/2026/10/07/E-UI-107/SUMMARY.nav.md
- spec/src/history/2026/10/07/E-UI-107/index.md
- spec/src/history/2026/10/07/SUMMARY.nav.md
- spec/src/history/2026/10/07/index.md
- spec/src/previous-locations.md
- web/src/lib/api/generated.ts
- web/src/lib/controller/agent-execution.svelte.ts
- web/src/lib/controller/agent-runtimes.svelte.ts
- web/src/lib/ui/store-settings/StoreAgentRuntimes.css
- web/src/lib/ui/store-settings/StoreAgentRuntimes.svelte
- web/src/lib/ui/store-settings/StoreAgentSettings.svelte
- web/src/lib/ui/store-settings/StoreSettings.svelte
- web/tests/vite/agent-runtimes.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-server/src/agent_execution/tooling/tests.rs#private_install_verifies_before_selection_and_survives_cold_journal_restart
- crates/jj-store-server/src/agent_execution/tooling/tests.rs#failed_update_and_interrupted_staging_preserve_previous_selection
- crates/jj-store-server/src/agent_execution/tooling/tests.rs#aborted_installer_kills_its_process_group_without_late_side_effects
- crates/jj-store-server/src/agent_execution/tooling/tests.rs#fixed_install_command_never_inherits_npm_credentials_or_executes_a_shell
- crates/jj-store-server/src/agent_execution/tooling/tests.rs#disk_budget_does_not_follow_symlinks_outside_staging
- crates/jj-store-server/src/agent_execution/tooling/tests.rs#real_private_codex_install_verifies_pinned_versions_and_cold_selection
- crates/jj-store-server/src/agent_execution/tooling/paths.rs#managed_discovery_survives_missing_ambient_path_and_checks_executable_bits
- crates/jj-store-server/src/agent_execution/tooling/paths.rs#versions_are_bounded_numeric_tuples
- crates/jj-store-http/src/agent_execution/tests.rs#runtime_management_is_local_admin_only_with_fixed_typed_inputs_and_original_receipts
- crates/jj-store-cli/tests/agent_execution/tooling.rs#runtime_receipts_recover_without_execution_or_automatic_install_replay
- web/tests/vite/agent-runtimes.test.ts#explicit install and original receipt
  polling never dispatch execution
- web/tests/vite/agent-runtimes.test.ts#failed refresh retains rows, marks
  status uncertain and blocks new installs until recovery
- web/tests/vite/agent-runtimes.test.ts#Ready never installs and unknown durable
  receipts remain reserved
- web/tests/vite/agent-runtimes.test.ts#lost install response retains the exact
  request and recovery never resends
- web/tests/vite/agent-runtimes.test.ts#service refusals and foreign receipts
  preserve separate request state
- web/tests/vite/agent-runtimes.test.ts#read adopts another client's durable
  installation and teardown discards late responses
- web/tests/vite/agent-execution-controller.test.ts#stopping an agent targets
  its session without changing another chat or replaying a prompt
<!-- /evidence-field -->

<!-- generated-navigation -->
## Pages

- [E-EXEC-022 details 2](details-02.md)
- [E-EXEC-022 details 3](details-03.md)
