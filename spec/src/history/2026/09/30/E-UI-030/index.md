---
schema: 1
id: E-UI-030
---

# E-UI-030

<!-- evidence-field:id markdown -->
## Id

E-UI-030
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4.18
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-30
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

transient-store-settings
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

The profile menu exposed only connection actions. Repository discovery permission was omitted from summaries, while connected capabilities and snapshot authority were not available through the public presentation state and search health had no data-source or settings presentation path.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- .storybook/vite.config.ts
- catalog/components/store-application/StoreApplication.stories.svelte
- catalog/controller/in-memory-store-controller.svelte.ts
- catalog/controller/in-memory-store-options.ts
- catalog/controller/in-memory-store-session.svelte.ts
- catalog/controller/in-memory-store-settings.svelte.ts
- catalog/fixtures/StoreApplicationFixture.svelte
- catalog/fixtures/snapshots.ts
- spec/src/api.md
- spec/src/ui.md
- spec/src/milestones.md
- spec/src/verification.md
- web/src/lib/controller/contracts.ts
- web/src/lib/controller/http-store-data-source.ts
- web/src/lib/controller/native-data-source.ts
- web/src/lib/controller/production-store-actions.ts
- web/src/lib/controller/production-store-controller.svelte.ts
- web/src/lib/controller/production-store-index.ts
- web/src/lib/controller/production-store-lifecycle.svelte.ts
- web/src/lib/controller/production-store-state.svelte.ts
- web/src/lib/controller/production-store-view.ts
- web/src/lib/ui/community-profile-menu/CommunityProfileMenu.svelte
- web/src/lib/ui/store-application/StoreApplication.svelte
- web/src/lib/ui/store-settings/StoreRepositorySettings.svelte
- web/src/lib/ui/store-settings/StoreSearchSettings.svelte
- web/src/lib/ui/store-settings/StoreServiceSettings.svelte
- web/src/lib/ui/store-settings/StoreSettings.css
- web/src/lib/ui/store-settings/StoreSettings.svelte
- web/src/lib/ui/store-settings/StoreSettingsNavigation.css
- web/src/lib/ui/store-settings/StoreSettingsNavigation.svelte
- web/src/lib/ui/store-settings/model.ts
- web/tests/browser/local-bootstrap.spec.ts
- web/tests/vite/production-store-bootstrap.test.ts
- web/tests/vite/production-store-data-source.test.ts
- web/tests/vite/production-store-settings.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/vite/production-store-bootstrap.test.ts#LocalTrusted bootstrap
  auto-opens the sole catalog repository without a token
- web/tests/vite/production-store-data-source.test.ts#explicit search status
  refresh bypasses the protected query cache
- web/tests/vite/production-store-settings.test.ts#settings refresh loads
  advertised search health and clears it on disconnect
- web/tests/vite/production-store-settings.test.ts#settings skip unavailable
  search and prepare the selected operation
- web/tests/vite/production-store-settings.test.ts#settings prefer the operation
  reported by an index-lag response
- web/tests/vite/production-store-settings.test.ts#settings do not prepare an
  index without an exact operation
- web/tests/browser/local-bootstrap.spec.ts#local trusted bootstrap auto-opens
  the sole repository without credentials
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-UI-030 details 2](details-02.md)
- [E-UI-030 details 3](details-03.md)
