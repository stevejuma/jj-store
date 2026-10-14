---
schema: 1
id: E-PUBLIC-RELEASE-001
---

# E-PUBLIC-RELEASE-001

<!-- evidence-field:id markdown -->
## Id

E-PUBLIC-RELEASE-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M30
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-ARCH-005
- JST-CLI-002
- JST-CLI-004
- JST-DOC-001
- JST-DOC-002
- JST-SEC-004
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-14
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

uuoxqsoy
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- .changeset/README.md
- .changeset/config.json
- .config/nextest.toml
- .dockerignore
- .github/actions/setup/action.yml
- .github/dependabot.yml
- .github/workflows/ci.yml
- .github/workflows/publish-release.yml
- .github/workflows/publish-spec-pages.yml
- .github/workflows/release.yml
- .gitignore
- CHANGELOG.md
- Cargo.lock
- Cargo.toml
- Dockerfile
- LICENSE.md
- README.md
- SECURITY.md
- crates/jj-agent/Cargo.toml
- crates/jj-agent/LICENSE.md
- crates/jj-agent/NOTICE.md
- crates/jj-agent/README.md
- crates/jj-collab/Cargo.toml
- crates/jj-collab/LICENSE.md
- crates/jj-collab/NOTICE.md
- crates/jj-collab/README.md
- crates/jj-review/Cargo.toml
- crates/jj-review/LICENSE.md
- crates/jj-review/NOTICE.md
- crates/jj-review/README.md
- crates/jj-store-adapters/Cargo.toml
- crates/jj-store-adapters/LICENSE.md
- crates/jj-store-adapters/NOTICE.md
- crates/jj-store-adapters/README.md
- crates/jj-store-adapters/src/context.rs
- crates/jj-store-capture/Cargo.toml
- crates/jj-store-capture/LICENSE.md
- crates/jj-store-capture/NOTICE.md
- crates/jj-store-capture/README.md
- crates/jj-store-capture/src/agent_sanitization.rs
- crates/jj-store-capture/src/transcript_sanitization.rs
- crates/jj-store-cli/Cargo.toml
- crates/jj-store-cli/LICENSE.md
- crates/jj-store-cli/NOTICE.md
- crates/jj-store-cli/README.md
- crates/jj-store-cli/help/agent.md
- crates/jj-store-cli/help/attach.md
- crates/jj-store-cli/help/capabilities.md
- crates/jj-store-cli/help/commands.md
- crates/jj-store-cli/help/credentials.md
- crates/jj-store-cli/help/doctor.md
- crates/jj-store-cli/help/init.md
- crates/jj-store-cli/help/migrate-local.md
- crates/jj-store-cli/help/pagination.md
- crates/jj-store-cli/help/review.md
- crates/jj-store-cli/help/serve-local.md
- crates/jj-store-cli/help/server-commands.md
- crates/jj-store-cli/help/server-config.md
- crates/jj-store-cli/help/server-doctor.md
- crates/jj-store-cli/help/server-operator.md
- crates/jj-store-cli/help/server-run.md
- crates/jj-store-cli/help/server.md
- crates/jj-store-cli/help/stacks.md
- crates/jj-store-cli/help/status.md
- crates/jj-store-cli/help/store.md
- crates/jj-store-cli/help/sync.md
- crates/jj-store-cli/help/tokens.md
- crates/jj-store-cli/help/version.md
- crates/jj-store-cli/help/work.md
- crates/jj-store-cli/help/workflows.md
- crates/jj-store-cli/src/commands.rs
- crates/jj-store-cli/src/server_commands.rs
- crates/jj-store-cli/src/stack_commands/args.rs
- crates/jj-store-cli/src/store_commands.rs
- crates/jj-store-cli/src/token_commands.rs
- crates/jj-store-cli/src/workflow_host/discovery.rs
- crates/jj-store-cli/tests/administration_rpc.rs
- crates/jj-store-cli/tests/agent_execution/context_delivery.rs
- crates/jj-store-cli/tests/agent_execution/conversations.rs
- crates/jj-store-cli/tests/agent_execution.rs
- crates/jj-store-cli/tests/attachment_recovery.rs
- crates/jj-store-cli/tests/capture_cli/fixture.rs
- crates/jj-store-cli/tests/executable_native.rs
- crates/jj-store-cli/tests/local_agent.rs
- crates/jj-store-cli/tests/local_agent_provenance/fixture.rs
- crates/jj-store-cli/tests/local_agent_provenance_cli.rs
- crates/jj-store-cli/tests/local_browser/index_tasks.rs
- crates/jj-store-cli/tests/local_browser/preparation.rs
- crates/jj-store-cli/tests/local_browser.rs
- crates/jj-store-cli/tests/local_collab_cli.rs
- crates/jj-store-cli/tests/local_review_files.rs
- crates/jj-store-cli/tests/local_work.rs
- crates/jj-store-cli/tests/native_collab_cli.rs
- crates/jj-store-cli/tests/native_lifecycle/content.rs
- crates/jj-store-cli/tests/server_executable/configured_capture.rs
- crates/jj-store-cli/tests/server_executable/git_bridge.rs
- crates/jj-store-cli/tests/server_executable/workflows.rs
- crates/jj-store-cli/tests/server_executable.rs
- crates/jj-store-cli/tests/workspace_migration.rs
- crates/jj-store-http/Cargo.toml
- crates/jj-store-http/LICENSE.md
- crates/jj-store-http/NOTICE.md
- crates/jj-store-http/README.md
- crates/jj-store-http/src/local_browser/binding.rs
- crates/jj-store-jj/Cargo.toml
- crates/jj-store-jj/LICENSE.md
- crates/jj-store-jj/NOTICE.md
- crates/jj-store-jj/README.md
- crates/jj-store-jj/src/agent_workspaces/inspection.rs
- crates/jj-store-jj/src/commands/access.rs
- crates/jj-store-jj/src/local_browser/budget.rs
- crates/jj-store-jj/src/query/access.rs
- crates/jj-store-jj/src/query/cache/resident.rs
- crates/jj-store-jj/src/query/memory.rs
- crates/jj-store-jj/src/query/prepared/filters.rs
- crates/jj-store-jj/src/query/prepared/operations/history/tests.rs
- crates/jj-store-model/Cargo.toml
- crates/jj-store-model/LICENSE.md
- crates/jj-store-model/NOTICE.md
- crates/jj-store-model/README.md
- crates/jj-store-object/Cargo.toml
- crates/jj-store-object/LICENSE.md
- crates/jj-store-object/NOTICE.md
- crates/jj-store-object/README.md
- crates/jj-store-read-index/Cargo.toml
- crates/jj-store-read-index/LICENSE.md
- crates/jj-store-read-index/NOTICE.md
- crates/jj-store-read-index/README.md
- crates/jj-store-replication/Cargo.toml
- crates/jj-store-replication/LICENSE.md
- crates/jj-store-replication/NOTICE.md
- crates/jj-store-replication/README.md
- crates/jj-store-rpc/Cargo.toml
- crates/jj-store-rpc/LICENSE.md
- crates/jj-store-rpc/NOTICE.md
- crates/jj-store-rpc/README.md
- crates/jj-store-server/Cargo.toml
- crates/jj-store-server/LICENSE.md
- crates/jj-store-server/NOTICE.md
- crates/jj-store-server/README.md
- crates/jj-store-server/src/agent_execution/compute/tests.rs
- crates/jj-store-server/src/agent_execution/configuration/tests.rs
- crates/jj-store-server/src/agent_execution/local_agent.rs
- crates/jj-store-server/src/agent_execution/workspace_turns.rs
- crates/jj-store-server/src/directory/tests.rs
- crates/jj-store-server/src/process/preparation.rs
- crates/jj-store-server/src/remote_agents/execution_tests.rs
- crates/jj-store-server/tests/capture_supervisor/raw_handoff.rs
- crates/jj-store-server/tests/configuration.rs
- crates/jj-store-server/tests/fixtures/agent_control.py
- crates/jj-store-server/tests/fixtures/configuration.md
- crates/jj-store-server/tests/fixtures/server-config.md
- crates/jj-store-server/tests/remote_agent_configuration.rs
- crates/jj-store-service/Cargo.toml
- crates/jj-store-service/LICENSE.md
- crates/jj-store-service/NOTICE.md
- crates/jj-store-service/README.md
- crates/jj-store-service/src/local_repository/work_cache/memory.rs
- crates/jj-store-service/src/native_inventory/cache/memory.rs
- crates/jj-store-service/src/read_indexes.rs
- crates/jj-store-service/src/runtime_policy.rs
- crates/jj-store-service/tests/workflow_store.rs
- crates/jj-store-tantivy/Cargo.toml
- crates/jj-store-tantivy/LICENSE.md
- crates/jj-store-tantivy/NOTICE.md
- crates/jj-store-tantivy/README.md
- crates/jj-store-wal/Cargo.toml
- crates/jj-store-wal/LICENSE.md
- crates/jj-store-wal/NOTICE.md
- crates/jj-store-wal/README.md
- crates/jj-store-wal/src/catalog/additions.rs
- crates/jj-store-wal/src/inventory.rs
- crates/jj-work/Cargo.toml
- crates/jj-work/LICENSE.md
- crates/jj-work/NOTICE.md
- crates/jj-work/README.md
- crates/jj-workflow/Cargo.toml
- crates/jj-workflow/LICENSE.md
- crates/jj-workflow/NOTICE.md
- crates/jj-workflow/README.md
- desktop/Cargo.toml
- desktop/src/installation.rs
- docs/licenses/NOTICE.md
- docs/releases.md
- docs/upstream-lock.md
- package.json
- pnpm-lock.yaml
- runtime/embedded-rustfs/Cargo.toml
- runtime/embedded-rustfs/LICENSE.md
- runtime/embedded-rustfs/NOTICE.md
- runtime/embedded-rustfs/README.md
- rust-toolchain.toml
- scripts/browser-fixtures.mjs
- scripts/changeset-check.mjs
- scripts/check-boundaries.mjs
- scripts/ci-browser.mjs
- scripts/ci-build.mjs
- scripts/ci-plan.mjs
- scripts/ci-range.mjs
- scripts/ci-required.mjs
- scripts/ci-storage.mjs
- scripts/ci-tests.mjs
- scripts/ci.test.mjs
- scripts/container-acceptance.py
- scripts/contracts.mjs
- scripts/desktop-resources.mjs
- scripts/desktop.mjs
- scripts/dev-agent-runtime.mjs
- scripts/dev-agent-runtime.test.mjs
- scripts/dev-demo-agents.mjs
- scripts/dev-demo-profiles.acceptance.mjs
- scripts/dev-demo-profiles.mjs
- scripts/dev-demo-repository.mjs
- scripts/dev-demo.mjs
- scripts/dev-ui-native-fixture.mjs
- scripts/docs/references.mjs
- scripts/docs.mjs
- scripts/governance.test.mjs
- scripts/install-cli.mjs
- scripts/managed-cargo.mjs
- scripts/package-check.mjs
- scripts/profile-acceptance.mjs
- scripts/release-archive.mjs
- scripts/release-assets.mjs
- scripts/release-evidence.mjs
- scripts/release-finalize.mjs
- scripts/release-metadata.mjs
- scripts/release-orchestrate.mjs
- scripts/release-registry.mjs
- scripts/release-version.mjs
- scripts/release.test.mjs
- scripts/rustfs-contract.py
- scripts/spec/book-references.mjs
- scripts/spec/revision-reader.mjs
- scripts/storage-browser-acceptance.mjs
- spec/src/current/contracts/api/01-read-apis-and-browser.md
- spec/src/current/contracts/architecture/03-requirements.md
- spec/src/current/contracts/architecture/23-public-distribution.md
- spec/src/current/contracts/architecture/SUMMARY.nav.md
- spec/src/current/contracts/architecture/index.md
- spec/src/current/contracts/cli/40-public-installation.md
- spec/src/current/contracts/cli/SUMMARY.nav.md
- spec/src/current/contracts/cli/index.md
- spec/src/current/contracts/collaboration/01-agent-activity-repository-reviews-threads-and-reactions.md
- spec/src/current/contracts/compatibility/01-compatibility-and-release.md
- spec/src/current/contracts/documentation/01-application-documentation.md
- spec/src/current/contracts/local-jj/01-local-git-backed-jj-collaboration.md
- spec/src/current/contracts/publication/01-publication-and-recovery.md
- spec/src/current/contracts/security/02-requirements.md
- spec/src/current/governance/spec-governance.md
- spec/src/current/progress/M30/SUMMARY.nav.md
- spec/src/current/progress/M30/index.md
- spec/src/current/progress/SUMMARY.nav.md
- spec/src/current/progress/index.md
- spec/src/current/verification/SUMMARY.nav.md
- spec/src/current/verification/index.md
- spec/src/current/verification/public-distribution.md
- spec-validator.config.mjs
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- scripts/release.test.mjs#all public package versions have an acyclic coordinated graph
- scripts/ci.test.mjs#docs-only impact stays light; shared and service changes select heavy lanes
<!-- /evidence-field -->


<!-- generated-navigation -->

## Pages

- [Release preparation evidence](details-02.md)
