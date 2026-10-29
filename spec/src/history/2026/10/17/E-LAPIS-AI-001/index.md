---
schema: 1
id: E-LAPIS-AI-001
---

# E-LAPIS-AI-001

<!-- evidence-field:id markdown -->
## Id

E-LAPIS-AI-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M13
- M23
- M24
- M25
- M30
- M32
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-ARCH-001
- JST-ARCH-005
- JST-ARCH-006
- JST-COLLAB-031
- JST-COLLAB-032
- JST-COLLAB-033
- JST-COLLAB-034
- JST-COLLAB-035
- JST-COLLAB-036
- JST-LOCAL-001
- JST-LOCAL-002
- JST-CLI-004
- JST-DESK-001
- JST-DOC-001
- JST-DOC-002
- JST-GOV-009
- JST-GOV-011
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-17
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- .github/actions/setup/action.yml
- .gitignore
- .prettierignore
- AGENTS.md
- Cargo.lock
- Cargo.toml
- README.md
- crates/jj-store-cli/help/server-config.md
- crates/jj-store-cli/tests/agent_execution/fleet.rs
- crates/jj-store-cli/tests/agent_execution/metadata.rs
- crates/jj-store-cli/tests/agent_execution/metadata_recovery.rs
- crates/jj-store-cli/tests/agent_execution/providers.rs
- crates/jj-store-cli/tests/agent_execution/tooling.rs
- crates/jj-store-cli/tests/fixtures/agent_control.py
- crates/jj-store-git/src/repository/collab_commands.rs
- crates/jj-store-server/Cargo.toml
- crates/jj-store-server/README.md
- crates/jj-store-server/src/agent_execution/catalog.rs
- crates/jj-store-server/src/agent_execution/commands.rs
- crates/jj-store-server/src/agent_execution/compute/mod.rs
- crates/jj-store-server/src/agent_execution/compute/process.rs
- crates/jj-store-server/src/agent_execution/fleet.rs
- crates/jj-store-server/src/agent_execution/local_agent.rs
- crates/jj-store-server/src/agent_execution/metadata_runtime.rs
- crates/jj-store-server/src/agent_execution/mod.rs
- crates/jj-store-server/src/agent_execution/native_runtime.rs
- crates/jj-store-server/src/agent_execution/prompt_context.rs
- crates/jj-store-server/src/agent_execution/provider_catalog.rs
- crates/jj-store-server/src/agent_execution/publication.rs
- crates/jj-store-server/src/agent_execution/runtime.rs
- crates/jj-store-server/src/agent_execution/tooling/mod.rs
- crates/jj-store-server/src/agent_execution/workspace_deferred.rs
- crates/jj-store-server/src/process/preparation.rs
- crates/jj-store-server/tests/fixtures/agent_control.py
- crates/jj-store-server/tests/fixtures/server-config.md
- desktop/Cargo.lock
- desktop/Cargo.toml
- desktop/README.md
- desktop/src/agent_resources.rs
- desktop/src/installation.rs
- docs/agent-execution.md
- docs/cli/server-config.md
- docs/implementation-plan.md
- docs/local-compute.md
- docs/maintenance/coverage.json
- docs/site/cli/index.md
- docs/site/cli/jj-store.md
- docs/site/guides/agent-execution.md
- docs/site/technical/architecture.md
- docs/site/technical/lifecycle.md
- package.json
- runtime/THIRD_PARTY.md
- runtime/compute/Cargo.lock
- runtime/compute/Cargo.toml
- runtime/compute/README.md
- runtime/compute/src/engine.rs
- runtime/compute/src/gateway.rs
- runtime/compute/src/main.rs
- runtime/compute/src/models.rs
- runtime/compute/src/peers.rs
- runtime/compute/src/progress.rs
- runtime/compute/src/tests.rs
- runtime/local-agent/Cargo.lock
- runtime/local-agent/Cargo.toml
- runtime/local-agent/README.md
- runtime/local-agent/UPSTREAM.md
- runtime/local-agent/scripts/model-capabilities.json
- runtime/local-agent/scripts/normative-corpus.json
- runtime/local-agent/src/chatgpt_token.rs
- runtime/local-agent/src/main.rs
- runtime/local-agent/src/metadata.rs
- runtime/local-agent/src/provider_catalog.rs
- runtime/local-agent/tests/metadata.rs
- runtime/local-agent/tests/provider_catalog.rs
- runtime/local-agent/tools/Cargo.toml
- runtime/local-agent/tools/README.md
- runtime/local-agent/tools/src/files.rs
- runtime/local-agent/tools/src/main.rs
- runtime/local-agent/tools/src/shell.rs
- runtime/local-agent/vendor/buzz-agent/Cargo.toml
- runtime/local-agent/vendor/buzz-agent/DATABRICKS_REUSE.md
- runtime/local-agent/vendor/buzz-agent/LICENSE
- runtime/local-agent/vendor/buzz-agent/README.md
- runtime/local-agent/vendor/buzz-agent/src/agent.rs
- runtime/local-agent/vendor/buzz-agent/src/auth.rs
- runtime/local-agent/vendor/buzz-agent/src/auth_http.rs
- runtime/local-agent/vendor/buzz-agent/src/builtin.rs
- runtime/local-agent/vendor/buzz-agent/src/catalog.rs
- runtime/local-agent/vendor/buzz-agent/src/config.rs
- runtime/local-agent/vendor/buzz-agent/src/databricks.rs
- runtime/local-agent/vendor/buzz-agent/src/databricks_test_support.rs
- runtime/local-agent/vendor/buzz-agent/src/databricks_tests.rs
- runtime/local-agent/vendor/buzz-agent/src/handoff.rs
- runtime/local-agent/vendor/buzz-agent/src/hints.rs
- runtime/local-agent/vendor/buzz-agent/src/lib.rs
- runtime/local-agent/vendor/buzz-agent/src/llm/chatgpt_plan/tests.rs
- runtime/local-agent/vendor/buzz-agent/src/llm/chatgpt_plan.rs
- runtime/local-agent/vendor/buzz-agent/src/llm.rs
- runtime/local-agent/vendor/buzz-agent/src/llm_fqn_tests.rs
- runtime/local-agent/vendor/buzz-agent/src/local_catalog.rs
- runtime/local-agent/vendor/buzz-agent/src/main.rs
- runtime/local-agent/vendor/buzz-agent/src/mcp.rs
- runtime/local-agent/vendor/buzz-agent/src/model_capabilities.rs
- runtime/local-agent/vendor/buzz-agent/src/permission.rs
- runtime/local-agent/vendor/buzz-agent/src/types.rs
- runtime/local-agent/vendor/buzz-agent/src/wire.rs
- runtime/local-agent/vendor/buzz-agent/tests/bin/auth_worker.rs
- runtime/local-agent/vendor/buzz-agent/tests/bin/fake_mcp.rs
- runtime/local-agent/vendor/buzz-agent/tests/bin/lock_holder.rs
- runtime/local-agent/vendor/buzz-agent/tests/common/mod.rs
- runtime/local-agent/vendor/buzz-agent/tests/databricks_auth_coordinator.rs
- runtime/local-agent/vendor/buzz-agent/tests/databricks_cli_auth.rs
- runtime/local-agent/vendor/buzz-agent/tests/databricks_oauth.rs
- runtime/local-agent/vendor/buzz-agent/tests/fake_llm.rs
- runtime/local-agent/vendor/buzz-agent/tests/fqn_capabilities.rs
- runtime/local-agent/vendor/buzz-agent/tests/golden_transcripts.rs
- runtime/local-agent/vendor/buzz-agent/tests/hints_integration.rs
- runtime/local-agent/vendor/buzz-agent/tests/openai_auto_upgrade.rs
- runtime/local-agent/vendor/buzz-agent/tests/permission_boundary.rs
- runtime/local-agent/vendor/buzz-agent/tests/regressions.rs
- scripts/agent-runtime-resources.mjs
- scripts/build-storage-inventory.mjs
- scripts/check-boundaries.mjs
- scripts/ci-lapis-source.mjs
- scripts/ci-packages.mjs
- scripts/ci-selection.test.mjs
- scripts/ci-setup-acp.mjs
- scripts/ci.test.mjs
- scripts/conversation-acceptance.py
- scripts/desktop-resources.mjs
- scripts/desktop.mjs
- scripts/dev-agent-runtime.mjs
- scripts/dev-agent-runtime.test.mjs
- scripts/dev-demo.mjs
- scripts/dev-ui-services.mjs
- scripts/dev-ui.test.mjs
- scripts/lapis-ai-acceptance.py
- scripts/lapis-ai-browser.mjs
- scripts/lapis-ai-source.json
- scripts/lapis-ai-test-fixture.mjs
- scripts/lapis-ai.mjs
- spec/src/current/contracts/architecture/20-interactive-agent-execution.md
- spec/src/current/contracts/architecture/21-local-compute-resources.md
- spec/src/current/contracts/architecture/23-public-distribution.md
- spec/src/current/contracts/architecture/25-injectable-collaboration-stores.md
- spec/src/current/contracts/cli/39-development-build-storage.md
- spec/src/current/contracts/collaboration/36-interactive-agent-execution.md
- spec/src/current/contracts/collaboration/37-reusable-agent-conversations.md
- spec/src/current/contracts/collaboration/42-conversation-metadata.md
- spec/src/current/contracts/compatibility/13-interactive-agent-execution.md
- spec/src/current/contracts/desktop/08-interactive-agent-execution.md
- spec/src/current/contracts/documentation/01-application-documentation.md
- spec/src/current/contracts/local-jj/01-local-git-backed-jj-collaboration.md
- spec/src/current/governance/06-verification-scope.md
- spec/src/current/governance/23-shared-ai-clients.md
- spec/src/current/governance/SUMMARY.nav.md
- spec/src/current/governance/index.md
- spec/src/current/progress/M13/index.md
- spec/src/current/progress/M23/index.md
- spec/src/current/progress/M24/SUMMARY.nav.md
- spec/src/current/progress/M24/index.md
- spec/src/current/progress/M24/lapis-ai-review.md
- spec/src/current/progress/M25/index.md
- spec/src/current/progress/M30/index.md
- spec/src/current/progress/M32/index.md
- spec/src/history/2026/10/17/SUMMARY.nav.md
- spec/src/history/2026/10/17/index.md
<!-- /evidence-field -->

<!-- evidence-field:removed_files list -->
## Removed files

- runtime/compute/Cargo.lock
- runtime/compute/Cargo.toml
- runtime/compute/README.md
- runtime/compute/src/engine.rs
- runtime/compute/src/gateway.rs
- runtime/compute/src/main.rs
- runtime/compute/src/models.rs
- runtime/compute/src/peers.rs
- runtime/compute/src/progress.rs
- runtime/compute/src/tests.rs
- runtime/local-agent/Cargo.lock
- runtime/local-agent/Cargo.toml
- runtime/local-agent/README.md
- runtime/local-agent/UPSTREAM.md
- runtime/local-agent/scripts/model-capabilities.json
- runtime/local-agent/scripts/normative-corpus.json
- runtime/local-agent/src/chatgpt_token.rs
- runtime/local-agent/src/main.rs
- runtime/local-agent/src/metadata.rs
- runtime/local-agent/src/provider_catalog.rs
- runtime/local-agent/tests/metadata.rs
- runtime/local-agent/tests/provider_catalog.rs
- runtime/local-agent/tools/Cargo.toml
- runtime/local-agent/tools/README.md
- runtime/local-agent/tools/src/files.rs
- runtime/local-agent/tools/src/main.rs
- runtime/local-agent/tools/src/shell.rs
- runtime/local-agent/vendor/buzz-agent/Cargo.toml
- runtime/local-agent/vendor/buzz-agent/DATABRICKS_REUSE.md
- runtime/local-agent/vendor/buzz-agent/LICENSE
- runtime/local-agent/vendor/buzz-agent/README.md
- runtime/local-agent/vendor/buzz-agent/src/agent.rs
- runtime/local-agent/vendor/buzz-agent/src/auth.rs
- runtime/local-agent/vendor/buzz-agent/src/auth_http.rs
- runtime/local-agent/vendor/buzz-agent/src/builtin.rs
- runtime/local-agent/vendor/buzz-agent/src/catalog.rs
- runtime/local-agent/vendor/buzz-agent/src/config.rs
- runtime/local-agent/vendor/buzz-agent/src/databricks.rs
- runtime/local-agent/vendor/buzz-agent/src/databricks_test_support.rs
- runtime/local-agent/vendor/buzz-agent/src/databricks_tests.rs
- runtime/local-agent/vendor/buzz-agent/src/handoff.rs
- runtime/local-agent/vendor/buzz-agent/src/hints.rs
- runtime/local-agent/vendor/buzz-agent/src/lib.rs
- runtime/local-agent/vendor/buzz-agent/src/llm/chatgpt_plan/tests.rs
- runtime/local-agent/vendor/buzz-agent/src/llm/chatgpt_plan.rs
- runtime/local-agent/vendor/buzz-agent/src/llm.rs
- runtime/local-agent/vendor/buzz-agent/src/llm_fqn_tests.rs
- runtime/local-agent/vendor/buzz-agent/src/local_catalog.rs
- runtime/local-agent/vendor/buzz-agent/src/main.rs
- runtime/local-agent/vendor/buzz-agent/src/mcp.rs
- runtime/local-agent/vendor/buzz-agent/src/model_capabilities.rs
- runtime/local-agent/vendor/buzz-agent/src/permission.rs
- runtime/local-agent/vendor/buzz-agent/src/types.rs
- runtime/local-agent/vendor/buzz-agent/src/wire.rs
- runtime/local-agent/vendor/buzz-agent/tests/bin/auth_worker.rs
- runtime/local-agent/vendor/buzz-agent/tests/bin/fake_mcp.rs
- runtime/local-agent/vendor/buzz-agent/tests/bin/lock_holder.rs
- runtime/local-agent/vendor/buzz-agent/tests/common/mod.rs
- runtime/local-agent/vendor/buzz-agent/tests/databricks_auth_coordinator.rs
- runtime/local-agent/vendor/buzz-agent/tests/databricks_cli_auth.rs
- runtime/local-agent/vendor/buzz-agent/tests/databricks_oauth.rs
- runtime/local-agent/vendor/buzz-agent/tests/fake_llm.rs
- runtime/local-agent/vendor/buzz-agent/tests/fqn_capabilities.rs
- runtime/local-agent/vendor/buzz-agent/tests/golden_transcripts.rs
- runtime/local-agent/vendor/buzz-agent/tests/hints_integration.rs
- runtime/local-agent/vendor/buzz-agent/tests/openai_auto_upgrade.rs
- runtime/local-agent/vendor/buzz-agent/tests/permission_boundary.rs
- runtime/local-agent/vendor/buzz-agent/tests/regressions.rs
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/agent_execution/metadata_recovery.rs#metadata_stale_generation_and_unknown_publication_recover_without_new_inference
- crates/jj-store-cli/tests/agent_execution/tooling.rs#runtime_receipts_recover_without_execution_or_automatic_install_replay
- crates/jj-store-cli/tests/agent_execution/fleet.rs#model_discovery_before_any_agent_or_chat_is_cached_and_never_prompts
- crates/jj-store-git/tests/factories.rs#staged_metadata_is_not_a_selected_domain_read_capability
- scripts/dev-agent-runtime.test.mjs#tampering with a non-ACP helper fails
  before replacing a service
- scripts/dev-ui.test.mjs#an occupied backend port leaves the current service
  and runtime marker intact
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-LAPIS-AI-001 details 2](details-02.md)
- [E-LAPIS-AI-001 details 3](details-03.md)
- [E-LAPIS-AI-001 details 4](details-04.md)
