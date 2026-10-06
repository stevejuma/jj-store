---
schema: 1
id: E-M14-006
---

# E-M14-006

<!-- evidence-field:id markdown -->
## Id

E-M14-006
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

- catalog/components/store-application/StoreWorkspaceSettings.stories.svelte
- catalog/controller/in-memory-store-controller.svelte.ts
- catalog/fixtures/WorkspaceSettingsFixture.svelte
- crates/jj-store-cli/src/folder_picker.rs
- crates/jj-store-cli/src/folder_picker_tests.rs
- crates/jj-store-cli/src/lib.rs
- crates/jj-store-cli/src/server_commands.rs
- crates/jj-store-cli/tests/native_lifecycle/http_local_management.rs
- crates/jj-store-http/src/dto.rs
- crates/jj-store-http/src/lib.rs
- crates/jj-store-http/src/local_management.rs
- crates/jj-store-http/src/routes.rs
- crates/jj-store-server/src/process.rs
- crates/jj-store-service/src/local_management.rs
- schemas/openapi.json
- spec/src/api.md
- spec/src/cli.md
- spec/src/compatibility.md
- spec/src/mutations.md
- spec/src/security.md
- spec/src/ui.md
- web/src/lib/api/generated.ts
- web/src/lib/controller/contracts.ts
- web/src/lib/controller/http-workspace-links.ts
- web/src/lib/controller/production-store-actions.ts
- web/src/lib/controller/production-store-view.ts
- web/src/lib/controller/production-workspace-links.ts
- web/src/lib/controller/workspace-link-types.ts
- web/src/lib/ui/store-settings/StoreRepositorySettings.svelte
- web/src/lib/ui/store-settings/StoreSettings.css
- web/src/lib/ui/store-settings/StoreWorkspaceSettings.svelte
- web/tests/browser/local-workspaces.spec.ts
- web/tests/vite/http-workspace-links.test.ts
- web/tests/vite/production-store-workspaces.test.ts
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/src/folder_picker_tests.rs#selection_cancellation_and_unavailable_preserve_path_bytes
- crates/jj-store-cli/src/folder_picker_tests.rs#busy_timeout_and_cancellation_hold_ownership_until_reaped
- crates/jj-store-cli/src/folder_picker_tests.rs#output_is_bounded_before_process_exit
- crates/jj-store-cli/tests/native_lifecycle/http_local_management.rs#local_workspace_management_requires_local_admin_and_captures_contracts
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M14-006 details 2](details-02.md)
- [E-M14-006 details 3](details-03.md)
