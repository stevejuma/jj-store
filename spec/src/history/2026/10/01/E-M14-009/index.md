---
schema: 1
id: E-M14-009
---

# E-M14-009

<!-- evidence-field:id markdown -->
## Id

E-M14-009
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M14
- M14.4
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-01
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

editable-search-management
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/StoreApplication.stories.svelte
- catalog/components/store-application/StoreSearchSettings.stories.svelte
- catalog/controller/in-memory-store-controller.svelte.ts
- catalog/controller/in-memory-store-settings.svelte.ts
- catalog/fixtures/SearchSettingsFixture.svelte
- catalog/fixtures/search-settings.ts
- crates/jj-store-adapters/src/runner.rs
- crates/jj-store-cli/src/server_commands.rs
- crates/jj-store-cli/tests/native_lifecycle/http_local_management.rs
- crates/jj-store-cli/tests/native_lifecycle/http_search_management.rs
- crates/jj-store-cli/tests/native_lifecycle/search_settings.rs
- crates/jj-store-cli/tests/native_lifecycle/server_search.rs
- crates/jj-store-http/src/dto.rs
- crates/jj-store-http/src/lib.rs
- crates/jj-store-http/src/routes.rs
- crates/jj-store-http/src/search.rs
- crates/jj-store-http/src/search_management.rs
- crates/jj-store-server/src/config.rs
- crates/jj-store-server/src/lib.rs
- crates/jj-store-server/src/process.rs
- crates/jj-store-server/src/replication.rs
- crates/jj-store-server/src/search.rs
- crates/jj-store-server/src/search_config.rs
- crates/jj-store-server/src/search_management/file.rs
- crates/jj-store-server/src/search_management/mod.rs
- crates/jj-store-server/src/search_worker.rs
- crates/jj-store-service/src/lib.rs
- crates/jj-store-service/src/search_management.rs
- crates/jj-store-tantivy/src/disk.rs
- crates/jj-store-tantivy/src/paging.rs
- crates/jj-store-tantivy/src/projection.rs
- schemas/openapi.json
- scripts/dev-demo-search.mjs
- scripts/dev-demo-search.test.mjs
- scripts/dev-demo.mjs
- spec/src/adapters.md
- spec/src/api.md
- spec/src/cli.md
- spec/src/compatibility.md
- spec/src/milestones.md
- spec/src/mutations.md
- spec/src/replication.md
- spec/src/security.md
- spec/src/spec-governance.md
- spec/src/ui.md
- web/src/lib/api/generated.ts
- web/src/lib/controller/contracts.ts
- web/src/lib/controller/http-repository-transports.ts
- web/src/lib/controller/native-data-source.ts
- web/src/lib/controller/production-search-settings.ts
- web/src/lib/controller/production-store-actions.ts
- web/src/lib/controller/production-store-index.ts
- web/src/lib/controller/production-store-state.svelte.ts
- web/src/lib/controller/production-store-view.ts
- web/src/lib/controller/search-settings.ts
- web/src/lib/ui/store-settings/StoreSearchConfiguration.svelte
- web/src/lib/ui/store-settings/StoreSearchHealth.svelte
- web/src/lib/ui/store-settings/StoreSearchSettings.svelte
- web/src/lib/ui/store-settings/StoreSettings.svelte
- web/tests/browser/local-search-settings.spec.ts
- web/tests/vite/production-search-settings.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/native_lifecycle/search_settings.rs#local_search_save_apply_rebuild_disable_and_restart_preserve_authority_and_pins
- web/tests/browser/local-search-settings.spec.ts#Search tabs preserve drafts
  and separate Save from Apply
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M14-009 details 2](details-02.md)
- [E-M14-009 details 3](details-03.md)
