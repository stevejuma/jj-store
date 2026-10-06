---
schema: 1
id: E-UI-095
---

# E-UI-095

<!-- evidence-field:id markdown -->
## Id

E-UI-095
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M14
- M15.5
- M4.10
- M13.4
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-04
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Reusable Settings copy rows, thread form alignment and collapsible history search
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change uxoswqxv
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Main continues from zulyxxpy/90d119e0. Thread rename uses a plain form; Thread ID and Revision have different copy presentation; folder control is capped. History search is always visible with text Run and a details context. Exact baseline counts and subsequent failures are recorded below.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Node 24.10.0; pnpm 10.32.1; JJ 0.45.1; locked Svelte 5, Storybook 10.5.7, Vitest 4.1.11 and Playwright 1.63.0. Existing E-UI-090/E-UI-092/E-UI-093 ignored foundation source-validation hook remains; package manifests retain normal semver ranges. No dependency edits or publication.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Presentation only, existing authorized route/clipboard/controller interfaces. No HTTP, WAL, manifest, receipt, native identity, companion metadata or persistence format changes.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/StoreSettingsIdentifier.stories.svelte
- catalog/components/store-application/StoreSyncSettings.stories.svelte
- catalog/components/store-application/StoreWorkspaceSettings.stories.svelte
- catalog/components/store-history-query/StoreHistoryQuery.stories.svelte
- catalog/components/store-threads/StoreThreadAlignment.stories.svelte
- catalog/fixtures/HistoryQueryFixture.svelte
- catalog/fixtures/ReadOnlyRowFixture.svelte
- spec/src/api.md
- spec/src/milestones.md
- spec/src/ui.md
- web/src/lib/ui/community-app-shell/right-sidebar.css
- web/src/lib/ui/community-channel-settings/CommunityChannelSettings.css
- web/src/lib/ui/community-channel-settings/CommunityChannelSettings.svelte
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryLog.svelte
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryLogHeader.svelte
- web/src/lib/ui/store-application/StoreApplication.svelte
- web/src/lib/ui/store-application/StoreWorkspacePicker.svelte
- web/src/lib/ui/store-history-query/StoreHistoryQuery.svelte
- web/src/lib/ui/store-history-query/StoreHistoryQueryContext.svelte
- web/src/lib/ui/store-history-query/StoreHistoryQueryHeader.svelte
- web/src/lib/ui/store-history-query/StoreHistoryQueryToggle.svelte
- web/src/lib/ui/store-history-query/history-query.css
- web/src/lib/ui/store-settings/StoreReadOnlyRow.svelte
- web/src/lib/ui/store-settings/StoreRepositorySettings.svelte
- web/src/lib/ui/store-settings/StoreSearchHealth.svelte
- web/src/lib/ui/store-settings/StoreServiceSettings.svelte
- web/src/lib/ui/store-settings/StoreSettings.css
- web/src/lib/ui/store-settings/StoreSettings.svelte
- web/src/lib/ui/store-settings/StoreSettingsRepository.svelte
- web/src/lib/ui/store-settings/StoreStorageOperation.svelte
- web/src/lib/ui/store-settings/StoreStoragePosition.svelte
- web/src/lib/ui/store-settings/StoreStorageSettings.svelte
- web/src/lib/ui/store-settings/StoreSyncSettings.svelte
- web/src/lib/ui/store-settings/StoreSyncWait.svelte
- web/src/lib/ui/store-settings/StoreWorkspaceSettings.svelte
- web/src/lib/ui/store-settings/settings-navigation.ts
- web/src/lib/ui/store-threads/StoreThreadSettings.svelte
- web/tests/browser/history-query.spec.ts
- web/tests/browser/local-workspaces.spec.ts
- web/tests/browser/threads-alignment.spec.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/browser/history-query.spec.ts#cold native history search highlights,
  completes and retains invalid drafts under CSP
- web/tests/browser/history-query.spec.ts#cold local-folder history uses pinned
  custom grammars and local query transport
- web/tests/browser/local-workspaces.spec.ts#local folder settings preserve
  drafts and link without changing repository context
- web/tests/browser/threads-alignment.spec.ts#cold LocalTrusted Threads shell
  settings references scoped search and reply focus
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-UI-095 details 2](details-02.md)
- [E-UI-095 details 3](details-03.md)
- [E-UI-095 details 4](details-04.md)
