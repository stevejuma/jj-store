---
schema: 1
id: E-M8A-003
---

# E-M8A-003

<!-- evidence-field:id markdown -->
## Id

E-M8A-003
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M8a
- M8a.4
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-28
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

local-trusted-access
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- Cargo.lock
- crates/jj-store-service/src/access.rs
- crates/jj-store-service/src/auth.rs
- crates/jj-store-service/src/native.rs
- crates/jj-store-service/src/transport.rs
- crates/jj-store-service/tests/authorization.rs
- crates/jj-store-rpc/src/client.rs
- crates/jj-store-rpc/src/server.rs
- crates/jj-store-rpc/tests/local_trusted.rs
- crates/jj-store-http/src/lib.rs
- crates/jj-store-http/src/dto.rs
- crates/jj-store-http/src/routes.rs
- crates/jj-store-http/src/contract.rs
- crates/jj-store-server/src/config.rs
- crates/jj-store-server/src/config/types.rs
- crates/jj-store-server/src/installation.rs
- crates/jj-store-server/src/process.rs
- crates/jj-store-server/src/search.rs
- crates/jj-store-server/tests/configuration.rs
- crates/jj-store-server/tests/local_installation.rs
- crates/jj-store-cli/src/attachment.rs
- crates/jj-store-cli/src/binding.rs
- crates/jj-store-cli/src/commands.rs
- crates/jj-store-cli/src/initialization.rs
- crates/jj-store-cli/src/server_commands.rs
- crates/jj-store-cli/tests/local_trusted_cli.rs
- crates/jj-store-cli/tests/native_lifecycle/http_reads.rs
- crates/jj-store-cli/tests/native_lifecycle/server_search.rs
- crates/jj-store-cli/tests/server_executable.rs
- docs/cli/server-config.md
- docs/cli/server-commands.md
- schemas/openapi.json
- web/src/lib/api/client.ts
- web/src/lib/api/generated.ts
- web/src/lib/controller/production-store-controller.svelte.ts
- web/src/lib/controller/production-store-state.svelte.ts
- web/src/lib/controller/production-store-view.ts
- web/src/lib/ui/store-application/StoreConnection.svelte
- web/src/lib/ui/store-application/StoreRepositoryApplication.svelte
- web/tests/client.test.ts
- web/tests/vite/production-store-bootstrap.test.ts
- web/tests/browser/local-bootstrap.spec.ts
- catalog/components/store-application/StoreConnection.stories.svelte
- package.json
- spec/src/api.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/compatibility.md
- spec/src/security.md
- spec/src/ui.md
- spec/src/verification.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-service/tests/authorization.rs#local_trusted_access_has_a_stable_principal_and_explicit_permission
- crates/jj-store-rpc/tests/local_trusted.rs#local_unix_rpc_requires_the_owner_peer_and_no_bearer
- crates/jj-store-cli/tests/local_trusted_cli.rs#tokenless_local_cli_initializes_and_attaches_a_physical_workspace
- crates/jj-store-cli/tests/native_lifecycle/http_reads.rs#local_trusted_bootstrap_and_http_admission_fail_closed
- crates/jj-store-cli/tests/native_lifecycle/server_search.rs#server_search_diagnostics_use_authorized_live_status_and_handle_unavailability
- crates/jj-store-server/tests/configuration.rs#older_configuration_formats_are_explicitly_rejected
- crates/jj-store-cli/tests/server_executable.rs#local_init_creates_strict_format_two_configuration
- crates/jj-store-cli/tests/server_executable.rs#local_trusted_executable_bootstraps_registers_and_serves_without_tokens
- crates/jj-store-cli/tests/server_executable.rs#cold_tls_server_serves_only_the_configured_browser_bundle
- crates/jj-store-cli/tests/server_executable.rs#cold_tls_http_reads_support_http1_and_http2
- web/tests/vite/production-store-bootstrap.test.ts#LocalTrusted bootstrap
  auto-opens the sole catalog repository without a token
- web/tests/browser/local-bootstrap.spec.ts#local trusted bootstrap auto-opens
  the sole repository without credentials
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M8A-003 details 2](details-02.md)
- [E-M8A-003 details 3](details-03.md)
