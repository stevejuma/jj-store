---
schema: 1
id: E-M14-013
---

# E-M14-013

<!-- evidence-field:id markdown -->
## Id

E-M14-013
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M14
- M14.6
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-03
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

captured-sync-waits-and-settings-reconciliation
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change xnzxyowv
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Fresh local-management Jujutsu workspace from qyrqmyzz/0ba87fbb; main clean and services preserved. New wait regression fails 1/4 because waitForSync is absent; reconciliation capture regression fails 1/4. Existing Sync only exposes pause/resume/retry and textual reconciliation guidance.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ 0.45.1; Rust 1.98.1; Svelte 5; existing Design Core source-validation dependencies cloned into an isolated ignored node_modules; registry manifests unchanged.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive protected SyncBarrier and LocalSyncWait HTTP contracts. No native object, manifest, WAL or receipt format change. Captured reconciliation uses existing command and exact current heads; uncertain requests retained across presentation closure.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/StoreSyncReconciliation.stories.svelte
- catalog/components/store-application/StoreSyncSettings.stories.svelte
- catalog/controller/in-memory-store-mutations.svelte.ts
- catalog/controller/in-memory-store-settings.svelte.ts
- catalog/fixtures/SyncReconciliationFixture.svelte
- catalog/fixtures/SyncSettingsFixture.svelte
- crates/jj-store-cli/tests/native_lifecycle/http_sync_management.rs
- crates/jj-store-http/src/routes.rs
- crates/jj-store-http/src/sync_management.rs
- crates/jj-store-server/src/replication.rs
- crates/jj-store-server/src/sync_management.rs
- crates/jj-store-service/src/sync_management.rs
- schemas/openapi.json
- spec/src/api.md
- spec/src/cli.md
- spec/src/compatibility.md
- spec/src/mutations.md
- spec/src/replication.md
- spec/src/ui.md
- web/src/lib/api/generated.ts
- web/src/lib/controller/contracts.ts
- web/src/lib/controller/production-mutation-commit.ts
- web/src/lib/controller/production-store-actions.ts
- web/src/lib/controller/production-store-mutations.ts
- web/src/lib/controller/production-sync-settings.ts
- web/src/lib/controller/production-sync-wait.ts
- web/src/lib/controller/sync-settings.ts
- web/src/lib/ui/store-mutations/StoreMutationDialog.svelte
- web/src/lib/ui/store-mutations/StoreMutationFields.svelte
- web/src/lib/ui/store-settings/StoreSettings.svelte
- web/src/lib/ui/store-settings/StoreSyncReconciliation.svelte
- web/src/lib/ui/store-settings/StoreSyncSettings.svelte
- web/src/lib/ui/store-settings/StoreSyncWait.svelte
- web/tests/browser/local-sync.spec.ts
- web/tests/production-store-mutations.test.ts
- web/tests/vite/production-sync-settings.test.ts
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/vite/production-sync-settings.test.ts#sync waits retain a fixed
  barrier across timeout and ignore a stopped response
- web/tests/production-store-mutations.test.ts#reconciliation captures fresh
  heads without changing browsing and retains unknown intent
- crates/jj-store-cli/tests/native_lifecycle/http_local_management.rs#local_workspace_management_requires_local_admin_and_captures_contracts
- web/tests/browser/local-sync.spec.ts#captured sync targets survive Settings
  navigation and timeout
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M14-013 details 2](details-02.md)
- [E-M14-013 details 3](details-03.md)
