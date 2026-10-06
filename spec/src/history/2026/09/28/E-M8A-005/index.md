---
schema: 1
id: E-M8A-005
---

# E-M8A-005

<!-- evidence-field:id markdown -->
## Id

E-M8A-005
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M8a
- M8a.8
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-28
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

implementation
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- AGENTS.md
- Cargo.lock
- catalog/components/store-application/StoreConnection.stories.svelte
- crates/jj-store-cli/src/commands.rs
- crates/jj-store-cli/src/initialization.rs
- crates/jj-store-cli/src/lib.rs
- crates/jj-store-cli/src/server_commands.rs
- crates/jj-store-cli/src/store_commands.rs
- crates/jj-store-cli/src/token_commands.rs
- crates/jj-store-cli/src/token_journal.rs
- crates/jj-store-cli/tests/administration_rpc.rs
- crates/jj-store-cli/tests/local_trusted_cli.rs
- crates/jj-store-cli/tests/native_lifecycle/http_reads.rs
- crates/jj-store-cli/tests/server_executable.rs
- crates/jj-store-http/Cargo.toml
- crates/jj-store-http/src/contract/tests.rs
- crates/jj-store-http/src/dto.rs
- crates/jj-store-http/src/lib.rs
- crates/jj-store-http/src/repositories.rs
- crates/jj-store-http/src/routes.rs
- crates/jj-store-rpc/proto/native.proto
- crates/jj-store-rpc/src/client.rs
- crates/jj-store-rpc/src/server.rs
- crates/jj-store-server/src/config/format-2-example.toml
- crates/jj-store-server/src/config/types.rs
- crates/jj-store-server/src/config.rs
- crates/jj-store-server/src/installation.rs
- crates/jj-store-server/src/lib.rs
- crates/jj-store-server/src/process.rs
- crates/jj-store-server/src/registry.rs
- crates/jj-store-server/src/registry_catalog.rs
- crates/jj-store-server/src/registry_persistence.rs
- crates/jj-store-server/tests/configuration.rs
- crates/jj-store-server/tests/local_installation.rs
- crates/jj-store-service/src/lib.rs
- crates/jj-store-service/src/registry.rs
- docs/cli/commands.md
- docs/cli/server-config.md
- docs/cli/server-operator.md
- docs/cli/tokens.md
- schemas/openapi.json
- spec/src/api.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/compatibility.md
- spec/src/jj-integration.md
- spec/src/milestones.md
- spec/src/mutations.md
- spec/src/security.md
- spec/src/spec-governance.md
- spec/src/ui.md
- spec/src/verification.md
- web/src/App.svelte
- web/src/lib/api/generated.ts
- web/src/lib/api/repositories.ts
- web/src/lib/controller/production-store-actions.ts
- web/src/lib/controller/production-store-controller.svelte.ts
- web/src/lib/controller/production-store-lifecycle.svelte.ts
- web/src/lib/ui/store-application/StoreConnection.svelte
- web/tests/client.test.ts
- web/tests/vite/production-store-bootstrap.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-server/tests/local_installation.rs#registry_indexes_token_candidates_without_conferring_access
- crates/jj-store-server/tests/local_installation.rs#registry_reserves_aliases_before_exposing_published_repositories
- crates/jj-store-cli/tests/administration_rpc.rs#rpc_created_repositories_attach_with_real_cli_and_survive_host_restart
- crates/jj-store-cli/tests/server_executable.rs#token_generate_prints_one_valid_manual_credential
- crates/jj-store-cli/tests/server_executable.rs#registry_init_creates_one_private_authenticated_catalog
- crates/jj-store-cli/tests/native_lifecycle/http_reads.rs#authenticated_repository_discovery_reauthorizes_one_token_per_repository
- crates/jj-store-http/src/repositories.rs#repository_cursor_resets_after_registry_generation_changes
- web/tests/vite/production-store-bootstrap.test.ts#authenticated bootstrap
  accepts one token and exposes every authorized repository
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M8A-005 details 2](details-02.md)
