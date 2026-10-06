---
schema: 1
id: E-M14-015
---

# E-M14-015

<!-- evidence-field:id markdown -->
## Id

E-M14-015
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M14
- M14.7
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-03
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

settings-repository-context-and-presentation
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change soytvzqt
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Main workspace, coordinated with native chrome task; its protected files committed separately as vssplolt/bdf88069. New navigation regression: 3 failing stories for missing repository context/toolbar refresh and Appearance introduction. Native Local folders row assertion: 1 failed, 5 passed. Browser comments showed oversized top spacing, horizontal action groups, plain guidance, borderless/narrow folder path and undersized opacity reset glyph.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Svelte 5, Storybook 10.5.7, Svelte CSF 5.1.2, Vitest browser 4.1.11, Playwright Chromium; released Design Core ^0.2.0 public components. Existing native executable and .local/dev-demo fixture reused; no dependency changes.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Presentation-only. Existing repository-open and Settings refresh actions, draft/pending guards and native identities retained. No API, native object, WAL, manifest, receipt, credential or storage format changes.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-appearance/StoreAppearance.stories.svelte
- catalog/components/store-application/SettingsNavigation.stories.svelte
- catalog/components/store-application/StoreLocalFolderDialog.stories.svelte
- catalog/components/store-application/StoreStorageSettings.stories.svelte
- catalog/components/store-application/StoreWorkspaceSettings.stories.svelte
- catalog/fixtures/SearchSettingsFixture.svelte
- catalog/fixtures/WorkspaceSettingsFixture.svelte
- spec/src/api.md
- spec/src/milestones.md
- spec/src/ui.md
- web/src/lib/ui/store-appearance/AppearanceThemes.svelte
- web/src/lib/ui/store-appearance/StoreAppearance.svelte
- web/src/lib/ui/store-appearance/appearance.css
- web/src/lib/ui/store-application/StoreApplication.svelte
- web/src/lib/ui/store-application/StoreRepositoryPicker.svelte
- web/src/lib/ui/store-collaboration/StoreReviewThreads.svelte
- web/src/lib/ui/store-settings/StoreGraphIndexSettings.svelte
- web/src/lib/ui/store-settings/StoreLocalFolderDialog.svelte
- web/src/lib/ui/store-settings/StoreLocalFolders.svelte
- web/src/lib/ui/store-settings/StoreRepositorySettings.svelte
- web/src/lib/ui/store-settings/StoreSearchConfiguration.svelte
- web/src/lib/ui/store-settings/StoreSearchHealth.svelte
- web/src/lib/ui/store-settings/StoreSearchSettings.svelte
- web/src/lib/ui/store-settings/StoreSettings.css
- web/src/lib/ui/store-settings/StoreSettingsNotice.svelte
- web/src/lib/ui/store-settings/StoreSettingsPage.svelte
- web/src/lib/ui/store-settings/StoreSettingsRepository.svelte
- web/src/lib/ui/store-settings/StoreStorageOperation.svelte
- web/src/lib/ui/store-settings/StoreStorageSettings.svelte
- web/src/lib/ui/store-settings/StoreSyncReconciliation.svelte
- web/src/lib/ui/store-settings/StoreSyncSettings.svelte
- web/src/lib/ui/store-settings/StoreSyncWait.svelte
- web/src/lib/ui/store-settings/StoreWorkspaceSettings.svelte
- web/src/lib/ui/store-work/StoreWorkField.svelte
- web/src/lib/ui/store-work/community-work-forms.css
- web/tests/browser/local-search-settings.spec.ts
- web/tests/browser/local-storage.spec.ts
- web/tests/browser/local-workspaces.spec.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/browser/local-search-settings.spec.ts#Search tabs preserve drafts
  and separate Save from Apply
- web/tests/browser/local-storage.spec.ts#Storage recovers a lost response
  without replacing the captured target
- web/tests/browser/local-workspaces.spec.ts#local folder settings preserve
  drafts and link without changing repository context
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M14-015 details 2](details-02.md)
