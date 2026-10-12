---
schema: 1
id: E-SETTINGS-SKELETON-001
---

# E-SETTINGS-SKELETON-001

<!-- evidence-field:id markdown -->
## Id

E-SETTINGS-SKELETON-001
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-11
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4.18
- M14.1
- M14.2
- M14.4
- M14.5
- M14.6
- M19
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-005
- JST-UI-010
- JST-UI-012
- JST-UI-016
- JST-API-001
- JST-API-004
- JST-DOC-001
- JST-DOC-002
- JST-DOC-003
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/RepositorySettingsLoading.stories.svelte
- catalog/components/store-application/StoreRepositorySettings.stories.svelte
- catalog/components/store-application/StoreSearchSettings.stories.svelte
- catalog/components/store-application/StoreServerSettings.stories.svelte
- catalog/components/store-application/StoreStorageSettings.stories.svelte
- catalog/components/store-application/StoreSyncSettings.stories.svelte
- catalog/components/store-application/StoreWorkspaceSettings.stories.svelte
- catalog/fixtures/RepositorySettingsFixture.svelte
- catalog/fixtures/RepositorySettingsLoadingFixture.svelte
- catalog/fixtures/SearchSettingsFixture.svelte
- catalog/fixtures/ServerSettingsFixture.svelte
- catalog/fixtures/StorageSettingsFixture.svelte
- catalog/fixtures/SyncSettingsFixture.svelte
- catalog/fixtures/WorkspaceSettingsFixture.svelte
- docs/maintenance/coverage.json
- docs/site/guides/settings.md
- spec/src/current/contracts/documentation/01-application-documentation.md
- spec/src/current/contracts/ui/07-application-startup.md
- spec/src/current/contracts/ui/09-transient-store-settings.md
- spec/src/current/progress/M14/history/M14.1/index.md
- spec/src/current/progress/M14/history/M14.2/index.md
- spec/src/current/progress/M14/history/M14.4/index.md
- spec/src/current/progress/M14/history/M14.5/index.md
- spec/src/current/progress/M14/history/M14.6/index.md
- spec/src/current/progress/M14/index.md
- spec/src/current/progress/M19/history/M19/index.md
- spec/src/current/progress/M19/index.md
- spec/src/current/progress/M4/history/M4.18/index.md
- spec/src/current/progress/M4/index.md
- spec/src/history/2026/10/11/E-SETTINGS-SKELETON-001/SUMMARY.nav.md
- spec/src/history/2026/10/11/E-SETTINGS-SKELETON-001/details-02.md
- spec/src/history/2026/10/11/E-SETTINGS-SKELETON-001/details-03.md
- spec/src/history/2026/10/11/E-SETTINGS-SKELETON-001/index.md
- spec/src/history/2026/10/11/SUMMARY.nav.md
- spec/src/history/2026/10/11/index.md
- web/src/lib/ui/store-application/StoreApplicationMain.svelte
- web/src/lib/ui/store-settings/StoreConfigurationEditor.svelte
- web/src/lib/ui/store-settings/StoreLocalFolders.svelte
- web/src/lib/ui/store-settings/StoreRepositorySettingsLoading.svelte
- web/src/lib/ui/store-settings/StoreRepositorySettingsView.svelte
- web/src/lib/ui/store-settings/StoreSearchConfiguration.svelte
- web/src/lib/ui/store-settings/StoreSearchHealth.svelte
- web/src/lib/ui/store-settings/StoreSettingsSkeleton.svelte
- web/src/lib/ui/store-settings/StoreStorageSettings.svelte
- web/src/lib/ui/store-settings/StoreSyncSettings.svelte
- web/src/lib/ui/store-settings/StoreWorkspaceSettings.svelte
- web/src/lib/ui/store-settings/settings-skeleton.css
- web/src/lib/ui/store-settings/settings-skeleton.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/vite/repository-settings-ownership.test.ts#pending Settings commands
  settle across category history and repository return
- web/tests/vite/repository-settings-ownership.test.ts#a late Settings response
  remains rejected after repository owner replacement
- web/tests/vite/repository-settings-ownership.test.ts#exact index preparation
  retains its task across code-read cancellation
- web/tests/vite/repository-settings-restoration.test.ts#settings URL
  restoration preserves drafts and code views without ordinary reads
- web/tests/vite/repository-settings-restoration.test.ts#guarded repository
  changes leave the active owner and drafts intact before connection teardown
- web/tests/vite/settings-freshness.test.ts#storage events read storage without
  refetching installation configuration or other sections
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-SETTINGS-SKELETON-001 details 2](details-02.md)
- [E-SETTINGS-SKELETON-001 details 3](details-03.md)
