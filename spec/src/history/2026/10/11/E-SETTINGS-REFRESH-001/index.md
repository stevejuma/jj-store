---
schema: 1
id: E-SETTINGS-REFRESH-001
---

# E-SETTINGS-REFRESH-001

<!-- evidence-field:id markdown -->
## Id

E-SETTINGS-REFRESH-001
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

- JST-UI-012
- JST-UI-016
- JST-API-001
- JST-API-004
- JST-DOC-001
- JST-DOC-002
- JST-DOC-003
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-11
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- .storybook/vite.config.ts
- catalog/components/store-application/StoreRepositorySettings.stories.svelte
- catalog/components/store-application/StoreSearchSettings.stories.svelte
- catalog/components/store-application/StoreServerSettings.stories.svelte
- catalog/components/store-application/StoreSettingsIdentifier.stories.svelte
- catalog/components/store-application/StoreStorageSettings.stories.svelte
- catalog/components/store-application/StoreSyncSettings.stories.svelte
- catalog/fixtures/ReadOnlyRowFixture.svelte
- catalog/fixtures/RepositorySettingsFixture.svelte
- catalog/fixtures/SearchSettingsFixture.svelte
- catalog/fixtures/ServerSettingsFixture.svelte
- catalog/fixtures/StorageSettingsFixture.svelte
- catalog/fixtures/SyncSettingsFixture.svelte
- docs/maintenance/coverage.json
- docs/site/guides/settings.md
- docs/site/reference/configuration.md
- docs/ui-provenance.json
- spec/src/current/contracts/api/30-installation-configuration-editing.md
- spec/src/current/contracts/documentation/01-application-documentation.md
- spec/src/current/contracts/ui/09-transient-store-settings.md
- spec/src/current/contracts/ui/28-complete-supported-configuration-settings.md
- spec/src/current/governance/spec-governance.md
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
- spec/src/history/2026/10/11/E-SETTINGS-REFRESH-001/SUMMARY.nav.md
- spec/src/history/2026/10/11/E-SETTINGS-REFRESH-001/details-02.md
- spec/src/history/2026/10/11/E-SETTINGS-REFRESH-001/details-03.md
- spec/src/history/2026/10/11/E-SETTINGS-REFRESH-001/index.md
- spec/src/history/2026/10/11/SUMMARY.nav.md
- spec/src/history/2026/10/11/index.md
- web/src/lib/ui/store-application/StoreApplicationMain.svelte
- web/src/lib/ui/store-application/StoreViewTabs.svelte
- web/src/lib/ui/store-application/view-tabs.css
- web/src/lib/ui/store-settings/StoreConfigurationEditor.svelte
- web/src/lib/ui/store-settings/StoreGraphIndexSettings.svelte
- web/src/lib/ui/store-settings/StoreLocalFolderBinding.svelte
- web/src/lib/ui/store-settings/StoreLocalFolders.svelte
- web/src/lib/ui/store-settings/StoreReadOnlyRow.svelte
- web/src/lib/ui/store-settings/StoreRepositoryConversion.svelte
- web/src/lib/ui/store-settings/StoreRepositoryIdentity.svelte
- web/src/lib/ui/store-settings/StoreRepositorySettings.svelte
- web/src/lib/ui/store-settings/StoreRepositorySettingsView.svelte
- web/src/lib/ui/store-settings/StoreSearchConfiguration.svelte
- web/src/lib/ui/store-settings/StoreSearchFields.svelte
- web/src/lib/ui/store-settings/StoreSearchHealth.svelte
- web/src/lib/ui/store-settings/StoreSettingsCard.svelte
- web/src/lib/ui/store-settings/StoreSettingsFooter.svelte
- web/src/lib/ui/store-settings/StoreSettingsHeading.svelte
- web/src/lib/ui/store-settings/StoreSettingsIcon.svelte
- web/src/lib/ui/store-settings/StoreSettingsPage.svelte
- web/src/lib/ui/store-settings/StoreStorageOperation.svelte
- web/src/lib/ui/store-settings/StoreStoragePosition.svelte
- web/src/lib/ui/store-settings/StoreStorageSettings.svelte
- web/src/lib/ui/store-settings/StoreSyncConfiguration.svelte
- web/src/lib/ui/store-settings/StoreSyncReconciliation.svelte
- web/src/lib/ui/store-settings/StoreSyncSettings.svelte
- web/src/lib/ui/store-settings/StoreSyncWait.svelte
- web/src/lib/ui/store-settings/StoreWorkspaceSettings.svelte
- web/src/lib/ui/store-settings/repository-settings.css
- web/src/lib/ui/store-settings/settings-presentation.ts
- web/src/lib/ui/store-settings/settings-row-icons.ts
- web/tests/real/repository-settings-presentation.spec.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/real/repository-settings.spec.ts#repository settings restores exact
  Stacks context and Details state
- web/tests/real/repository-settings.spec.ts#repository Settings deep links
  reload and browser history restore subtabs
- web/tests/real/repository-settings.spec.ts#global Settings contains
  installation configuration and returns to repository Settings
- web/tests/real/repository-settings.spec.ts#native folder drafts survive
  category changes and Back with accessible mobile navigation
- web/tests/real/repository-settings.spec.ts#Sync observation settles without
  resubscribing or flickering
- web/tests/repository-settings-navigation.test.ts#category/subtab navigation
  preserves the exact file entry and operation
- web/tests/repository-settings-navigation.test.ts#Back restores focused Stacks
  history without changing its mode or comparison
- web/tests/repository-settings-navigation.test.ts#direct and reloaded settings
  links fall back to this repository Overview
- web/tests/repository-settings-navigation.test.ts#return routes cannot leak
  into a different repository
- web/tests/repository-settings-navigation.test.ts#malformed, duplicate and
  incompatible settings fields fail closed
- web/tests/repository-settings-navigation.test.ts#local and authenticated
  profiles expose only supported categories
- web/tests/repository-settings-navigation.test.ts#capability-aware defaults
  select Health for readers and General for unsupported categories
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

- [E-SETTINGS-REFRESH-001 details 2](details-02.md)
- [E-SETTINGS-REFRESH-001 details 3](details-03.md)
