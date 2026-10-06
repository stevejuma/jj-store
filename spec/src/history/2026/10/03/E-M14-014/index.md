---
schema: 1
id: E-M14-014
---

# E-M14-014

<!-- evidence-field:id markdown -->
## Id

E-M14-014
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M14
- M14.5
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-03
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

native-storage-management
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change lmurxoml
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

At Sync commit 2c5ab73d the Storage section, maintenance HTTP API and durable exact-target operations were absent. New WAL target failed compilation with 10 missing-API errors; new controller test failed module import.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ 0.45.1; Rust 1.98.1; Svelte 5; pnpm 10.32.1; existing Design Core source-validation installation retained; no dependency change.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive optional native LocalTrusted Storage capability and versioned overview/operation contracts. Private device maintenance journal V1 outside checkout content; 128 identities/4 MiB per repository. Existing native object, manifest, checkpoint, WAL and receipt encodings unchanged.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/StoreStorageSettings.stories.svelte
- catalog/controller/in-memory-store-settings.svelte.ts
- catalog/fixtures/StorageSettingsFixture.svelte
- crates/jj-store-cli/tests/native_lifecycle/http_local_management.rs
- crates/jj-store-cli/tests/native_lifecycle/http_storage_management.rs
- crates/jj-store-http/src/dto.rs
- crates/jj-store-http/src/lib.rs
- crates/jj-store-http/src/routes.rs
- crates/jj-store-http/src/storage_management.rs
- crates/jj-store-server/src/lib.rs
- crates/jj-store-server/src/process.rs
- crates/jj-store-server/src/storage_management/journal.rs
- crates/jj-store-server/src/storage_management/mod.rs
- crates/jj-store-service/src/lib.rs
- crates/jj-store-service/src/storage_backend.rs
- crates/jj-store-service/src/storage_management.rs
- crates/jj-store-wal/src/publication/maintenance.rs
- crates/jj-store-wal/src/publication.rs
- crates/jj-store-wal/tests/publication.rs
- schemas/openapi.json
- scripts/storage-browser-acceptance.mjs
- spec/src/api.md
- spec/src/cli.md
- spec/src/compatibility.md
- spec/src/milestones.md
- spec/src/mutations.md
- spec/src/publication.md
- spec/src/spec-governance.md
- spec/src/ui.md
- web/src/lib/api/generated.ts
- web/src/lib/controller/contracts.ts
- web/src/lib/controller/http-local-store-data-source.ts
- web/src/lib/controller/http-repository-transports.ts
- web/src/lib/controller/native-data-source.ts
- web/src/lib/controller/production-storage-settings.ts
- web/src/lib/controller/production-store-actions.ts
- web/src/lib/controller/production-store-controller.svelte.ts
- web/src/lib/controller/production-store-index.ts
- web/src/lib/controller/production-store-state.svelte.ts
- web/src/lib/controller/production-store-view.ts
- web/src/lib/controller/repository-profile.ts
- web/src/lib/controller/storage-settings.ts
- web/src/lib/ui/store-settings/StoreSettings.svelte
- web/src/lib/ui/store-settings/StoreSettingsNavigation.svelte
- web/src/lib/ui/store-settings/StoreSettingsPage.svelte
- web/src/lib/ui/store-settings/StoreStoragePosition.svelte
- web/src/lib/ui/store-settings/StoreStorageSettings.svelte
- web/src/lib/ui/store-settings/model.ts
- web/tests/browser/local-storage.spec.ts
- web/tests/real/storage-maintenance.spec.ts
- web/tests/vite/local-browser.test.ts
- web/tests/vite/production-storage-settings.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-wal/tests/publication.rs#maintenance_reuses_captured_target_and_reports_covering_checkpoint
- crates/jj-store-cli/tests/native_lifecycle/http_local_management.rs#local_workspace_management_requires_local_admin_and_captures_contracts
- web/tests/vite/production-storage-settings.test.ts#storage retains an
  uncertain command identity and never recaptures its checkpoint
- web/tests/real/storage-maintenance.spec.ts#native Storage keeps exact
  checkpoint targets through cold restart and verifies captured data
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M14-014 details 2](details-02.md)
- [E-M14-014 details 3](details-03.md)
- [E-M14-014 details 4](details-04.md)
