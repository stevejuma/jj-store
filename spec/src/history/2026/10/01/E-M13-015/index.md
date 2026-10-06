---
schema: 1
id: E-M13-015
---

# E-M13-015

<!-- evidence-field:id markdown -->
## Id

E-M13-015
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M13.4
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-01
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

local-browser-filesystem-runtime
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- Cargo.lock
- crates/jj-store-cli/src/server_commands.rs
- crates/jj-store-cli/tests/local_browser.rs
- crates/jj-store-http/Cargo.toml
- crates/jj-store-http/src/contract/tests.rs
- crates/jj-store-http/src/contract.rs
- crates/jj-store-http/src/dto.rs
- crates/jj-store-http/src/lib.rs
- crates/jj-store-http/src/local_browser.rs
- crates/jj-store-http/src/repositories.rs
- crates/jj-store-jj/src/local_browser/mod.rs
- crates/jj-store-server/src/config/types.rs
- crates/jj-store-server/src/config.rs
- crates/jj-store-server/src/installation.rs
- crates/jj-store-server/src/local_repositories.rs
- crates/jj-store-server/src/process.rs
- crates/jj-store-server/src/storage.rs
- crates/jj-store-server/tests/local_installation.rs
- crates/jj-store-service/src/local_browser.rs
- schemas/openapi.json
- scripts/contracts.mjs
- spec/src/api.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/compatibility.md
- spec/src/jj-integration.md
- spec/src/mutations.md
- spec/src/spec-governance.md
- web/src/lib/api/generated.ts
- spec/src/local-jj.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/local_browser.rs#filesystem_runtime_registers_and_reads_without_native_storage
- crates/jj-store-http/src/contract/tests.rs#local_contracts_preserve_separate_profiles_and_strict_pinned_queries
- crates/jj-store-server/tests/local_installation.rs#filesystem_initialization_is_private_and_rejects_nonlocal_authority
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M13-015 details 2](details-02.md)
- [E-M13-015 details 3](details-03.md)
