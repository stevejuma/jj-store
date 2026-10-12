---
schema: 1
id: E-GLOBAL-SETTINGS-NAV-001
---

# E-GLOBAL-SETTINGS-NAV-001

<!-- evidence-field:id markdown -->
## Id

E-GLOBAL-SETTINGS-NAV-001
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-12
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4.18
- M14.2
- M14.6
- M16
- M19
- M25
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-005
- JST-UI-012
- JST-UI-016
- JST-UI-018
- JST-API-005
- JST-API-008
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

- catalog/components/store-application/GlobalSettingsLoading.stories.svelte
- catalog/components/store-application/SettingsNavigation.stories.svelte
- catalog/controller/in-memory-store-navigation.ts
- catalog/fixtures/GlobalSettingsLoadingFixture.svelte
- docs/maintenance/coverage.json
- docs/site/guides/settings.md
- spec/src/current/contracts/api/02-read-apis-and-browser-continued-2.md
- spec/src/current/contracts/api/05-bootstrap-and-browser-admission.md
- spec/src/current/contracts/documentation/01-application-documentation.md
- spec/src/current/contracts/ui/03-requirements.md
- spec/src/current/contracts/ui/09-transient-store-settings.md
- spec/src/current/contracts/ui/10-local-management-delivery.md
- spec/src/current/contracts/ui/28-complete-supported-configuration-settings.md
- spec/src/current/contracts/ui/SUMMARY.nav.md
- spec/src/current/contracts/ui/index.md
- spec/src/current/progress/M14/history/M14.2/index.md
- spec/src/current/progress/M14/history/M14.6/index.md
- spec/src/current/progress/M14/index.md
- spec/src/current/progress/M16/history/M16/index.md
- spec/src/current/progress/M16/index.md
- spec/src/current/progress/M19/history/M19/index.md
- spec/src/current/progress/M19/index.md
- spec/src/current/progress/M25/history/M25/index.md
- spec/src/current/progress/M25/index.md
- spec/src/current/progress/M4/history/M4.18/index.md
- spec/src/current/progress/M4/index.md
- spec/src/history/2026/10/12/SUMMARY.nav.md
- spec/src/history/2026/10/12/index.md
- spec/src/history/2026/10/SUMMARY.nav.md
- spec/src/history/2026/10/index.md
- web/src/lib/controller/controller-actions.ts
- web/src/lib/controller/global-settings-navigation.ts
- web/src/lib/controller/global-settings-route-codec.ts
- web/src/lib/controller/production-profile-route.ts
- web/src/lib/controller/production-store-connect.ts
- web/src/lib/controller/production-store-lifecycle.svelte.ts
- web/src/lib/controller/production-store-refresh.ts
- web/src/lib/controller/repository-settings-actions.ts
- web/src/lib/controller/store-route-browser.ts
- web/src/lib/controller/store-route-codec.ts
- web/src/lib/controller/store-route-history.svelte.ts
- web/src/lib/controller/store-route-types.ts
- web/src/lib/controller/store-route-writer.ts
- web/src/lib/controller/store-stacks-navigation.ts
- web/src/lib/ui/AGENTS.md
- web/src/lib/ui/store-appearance/StoreAppearance.svelte
- web/src/lib/ui/store-application/StoreApplication.svelte
- web/src/lib/ui/store-application/StoreApplicationMain.svelte
- web/src/lib/ui/store-settings/StoreAgentDefaults.svelte
- web/src/lib/ui/store-settings/StoreAgentRuntimes.svelte
- web/src/lib/ui/store-settings/StoreAgentSettings.svelte
- web/src/lib/ui/store-settings/StoreCaptureConfiguration.svelte
- web/src/lib/ui/store-settings/StoreCaptureLoading.svelte
- web/src/lib/ui/store-settings/StoreCaptureSettings.svelte
- web/src/lib/ui/store-settings/StoreCaptureSources.svelte
- web/src/lib/ui/store-settings/StoreComputeSettings.svelte
- web/src/lib/ui/store-settings/StoreConfigurationEditor.svelte
- web/src/lib/ui/store-settings/StoreDeploymentConfiguration.svelte
- web/src/lib/ui/store-settings/StoreGlobalSettingsLoading.svelte
- web/src/lib/ui/store-settings/StoreServiceSettings.svelte
- web/src/lib/ui/store-settings/StoreSettings.svelte
- web/src/lib/ui/store-settings/StoreSettingsSkeleton.svelte
- web/src/lib/ui/store-settings/global-settings-skeleton.ts
- web/src/lib/ui/store-settings/model.ts
- web/src/lib/ui/store-settings/settings-skeleton.css
- web/src/lib/ui/store-settings/settings-skeleton.ts
- web/tests/global-settings-navigation.test.ts
- web/tests/vite/global-settings-history.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/vite/global-settings-history.test.ts#global settings history
  restores tabs and exact returns across repositories
- web/tests/vite/global-settings-history.test.ts#global settings keep navigation
  guards and fresh links return to Projects
- web/tests/vite/global-settings-history.test.ts#restoring global settings does
  not invalidate reads owners or drafts
- web/tests/vite/global-settings-history.test.ts#global settings startup skips
  repository and project reads
- web/tests/vite/global-settings-history.test.ts#identical global URLs retain
  repository navigation guards
- web/tests/global-settings-navigation.test.ts#root startup resumes only
  validated section and tab while explicit routes win
- web/tests/global-settings-navigation.test.ts#global loading geometry follows
  every selected subtab
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-GLOBAL-SETTINGS-NAV-001 details 2](details-02.md)
- [E-GLOBAL-SETTINGS-NAV-001 details 3](details-03.md)
- [E-GLOBAL-SETTINGS-NAV-001 details 4](details-04.md)
