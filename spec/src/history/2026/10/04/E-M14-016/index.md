---
schema: 1
id: E-M14-016
---

# E-M14-016

<!-- evidence-field:id markdown -->
## Id

E-M14-016
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M14
- M14.4
- M14.6
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-012
- JST-UI-016
- JST-UI-019
- JST-API-001
- JST-API-006
- JST-CLI-005
- JST-REPL-011
- JST-MUT-003
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-04
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Complete supported operational configuration editors
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change ylxwsusl
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Clean main ylxwsusl/9f7571b1 above qnyktnvp/1367c5d4; clean codex-preview workspace above the same parent. Only Agents and basic Search are editable. Resource budgets, installation protection, replication configuration and seven Search worker/extraction fields have no controls. Remaining launch/storage configuration is not visible. Narrow acceptance: section-safe persistence, retained drafts, explicit running values, every supported operational field, read-only infrastructure coverage and saved selection on next startup. Final review found generic notices inherited the route scope; align them to a stable installation scope so a dismissed unchanged condition cannot replay on repository navigation.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ0.45.1; Node24.10.0; pnpm10.32.1; pinned Rust and locked Svelte5/Storybook10.5.7/Vitest4.1.11/Playwright1.63.0. Dependency versions unchanged.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Format2 configuration and all native object/WAL/manifest/capture/credential formats unchanged. Additive actor-free version1 GET/POST /v1/local/settings and capability. Search DTOs expose seven existing bounded worker/extraction fields with established defaults. Exact SHA256 file revisions and common settings lock; normalized TOML values and synchronized same-directory replacement. Credential references only; no secret values or arbitrary TOML editing.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/StoreSearchSettings.stories.svelte
- catalog/components/store-application/StoreServerSettings.stories.svelte
- catalog/fixtures/SearchSettingsFixture.svelte
- catalog/fixtures/ServerSettingsFixture.svelte
- catalog/fixtures/search-settings.ts
- catalog/fixtures/server-settings.ts
- crates/jj-store-cli/tests/native_lifecycle/search_settings.rs
- crates/jj-store-http/src/contract.rs
- crates/jj-store-http/src/dto.rs
- crates/jj-store-http/src/lib.rs
- crates/jj-store-http/src/server_settings.rs
- crates/jj-store-server/src/capture_management/authority.rs
- crates/jj-store-server/src/capture_management.rs
- crates/jj-store-server/src/lib.rs
- crates/jj-store-server/src/process/preparation.rs
- crates/jj-store-server/src/search_management/file.rs
- crates/jj-store-server/src/server_settings/inventory.rs
- crates/jj-store-server/src/server_settings.rs
- crates/jj-store-server/tests/capture_management/server_settings.rs
- crates/jj-store-server/tests/capture_management.rs
- crates/jj-store-service/src/lib.rs
- crates/jj-store-service/src/search_management.rs
- crates/jj-store-service/src/server_settings.rs
- schemas/openapi.json
- scripts/contracts.mjs
- spec/src/api.md
- spec/src/cli.md
- spec/src/compatibility.md
- spec/src/milestones.md
- spec/src/mutations.md
- spec/src/replication.md
- spec/src/spec-governance.md
- spec/src/ui.md
- web/src/lib/api/generated.ts
- web/src/lib/controller/contracts.ts
- web/src/lib/controller/production-local-configuration.ts
- web/src/lib/controller/production-store-controller.svelte.ts
- web/src/lib/controller/production-store-lifecycle.svelte.ts
- web/src/lib/controller/production-store-state.svelte.ts
- web/src/lib/controller/production-store-view.ts
- web/src/lib/controller/search-advanced.ts
- web/src/lib/controller/search-settings.ts
- web/src/lib/controller/server-configuration.ts
- web/src/lib/controller/server-settings.svelte.ts
- web/src/lib/ui/store-settings/StoreConfigurationEditor.svelte
- web/src/lib/ui/store-settings/StoreConfigurationFields.svelte
- web/src/lib/ui/store-settings/StoreDeploymentConfiguration.svelte
- web/src/lib/ui/store-settings/StoreRepositorySettings.svelte
- web/src/lib/ui/store-settings/StoreSearchAdvanced.svelte
- web/src/lib/ui/store-settings/StoreSearchConfiguration.svelte
- web/src/lib/ui/store-settings/StoreSearchSettings.svelte
- web/src/lib/ui/store-settings/StoreServiceSettings.svelte
- web/src/lib/ui/store-settings/StoreSettings.svelte
- web/src/lib/ui/store-settings/StoreStorageSettings.svelte
- web/src/lib/ui/store-settings/StoreSyncConfiguration.svelte
- web/src/lib/ui/store-settings/StoreSyncSettings.svelte
- web/tests/browser/local-server-settings.spec.ts
- web/tests/vite/production-search-settings.test.ts
- web/tests/vite/server-settings.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-server/tests/capture_management/server_settings.rs#saves_preserve_other_sections_and_apply_only_at_restart
- crates/jj-store-server/tests/capture_management/server_settings.rs#settings_revalidate_authority_budgets_locks_and_regular_files
- crates/jj-store-server/tests/capture_management/server_settings.rs#http_settings_contracts_are_actor_free_conditional_and_local_admin_only
- crates/jj-store-cli/tests/native_lifecycle/search_settings.rs#local_search_save_apply_rebuild_disable_and_restart_preserve_authority_and_pins
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M14-016 details 2](details-02.md)
- [E-M14-016 details 3](details-03.md)
