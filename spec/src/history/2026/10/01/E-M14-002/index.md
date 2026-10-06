---
schema: 1
id: E-M14-002
---

# E-M14-002

<!-- evidence-field:id markdown -->
## Id

E-M14-002
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M14
- M14.2
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-01
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

implementation
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/src/lib.rs
- crates/jj-store-cli/src/server_commands.rs
- crates/jj-store-cli/src/workspace_management.rs
- crates/jj-store-cli/tests/native_lifecycle/http_local_management.rs
- crates/jj-store-cli/tests/native_lifecycle/http_reads.rs
- crates/jj-store-cli/tests/server_executable.rs
- crates/jj-store-cli/tests/workspace_management.rs
- crates/jj-store-http/src/dto.rs
- crates/jj-store-http/src/lib.rs
- crates/jj-store-http/src/local_management.rs
- crates/jj-store-http/src/routes.rs
- crates/jj-store-server/src/lib.rs
- crates/jj-store-server/src/process.rs
- crates/jj-store-server/src/workspaces.rs
- crates/jj-store-server/tests/workspace_links.rs
- crates/jj-store-service/src/lib.rs
- crates/jj-store-service/src/local_management.rs
- schemas/openapi.json
- scripts/contracts.mjs
- spec/src/api.md
- spec/src/cli.md
- spec/src/compatibility.md
- spec/src/milestones.md
- spec/src/mutations.md
- spec/src/security.md
- spec/src/spec-governance.md
- spec/src/verification.md
- web/src/lib/api/generated.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-server/tests/workspace_links.rs#links_are_read_only_until_explicitly_added_and_survive_restart
- crates/jj-store-server/tests/workspace_links.rs#revisions_repository_scope_and_limits_fail_closed
- crates/jj-store-server/tests/workspace_links.rs#foreign_corrupt_and_symlinked_registries_are_rejected
- crates/jj-store-cli/tests/workspace_management.rs#discovery_is_nonmutating_and_binds_the_exact_installation
- crates/jj-store-cli/tests/workspace_management.rs#unqualified_git_corrupt_and_symlink_metadata_fail_closed
- crates/jj-store-cli/tests/native_lifecycle/http_local_management.rs#local_workspace_management_requires_local_admin_and_captures_contracts
- crates/jj-store-cli/tests/server_executable.rs#local_trusted_executable_bootstraps_registers_and_serves_without_tokens
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M14-002 details 2](details-02.md)
- [E-M14-002 details 3](details-03.md)
