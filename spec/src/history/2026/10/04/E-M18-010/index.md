---
schema: 1
id: E-M18-010
---

# E-M18-010

<!-- evidence-field:id markdown -->
## Id

E-M18-010
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M18.8
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-04
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change truyzyxr
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Usable capture loading, grouped Codex settings and harness-filtered source table
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Node24.10.0; pnpm10.32.1; JJ0.45.1; Svelte5.57.1; Storybook10.5.7; Vitest4.1.11; released Design Core Skeleton, Collapsible, Button, Table, Select and Badge; existing browser_preview binary.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

UI and ordinary-read policy only. Existing CaptureSettings, CaptureStatus, CaptureSourcePage and receipt contracts remain unchanged. GET capture_control_busy retries at most five attempts with150/300/450/600ms delays, cancellable on closure. POST commands retain exact identity and are never automatically retried. Harness filtering is local presentation over loaded sources; paths/native keys remain absent from source rows.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/StoreCaptureSettings.stories.svelte
- catalog/components/store-application/capture-harness-play.ts
- catalog/components/store-application/capture-loading-play.ts
- catalog/fixtures/CaptureSettingsFixture.svelte
- catalog/fixtures/capture-management.ts
- spec/src/api.md
- spec/src/collaboration.md
- spec/src/ui.md
- spec/src/milestones.md
- web/src/lib/controller/capture-errors.ts
- web/src/lib/controller/capture-http.ts
- web/src/lib/controller/capture-observations.ts
- web/src/lib/controller/capture-settings-management.ts
- web/src/lib/ui/store-settings/StoreCaptureConfiguration.svelte
- web/src/lib/ui/store-settings/StoreCaptureHarnessConfiguration.svelte
- web/src/lib/ui/store-settings/StoreCaptureLoading.svelte
- web/src/lib/ui/store-settings/StoreCaptureSettings.svelte
- web/src/lib/ui/store-settings/StoreCaptureSources.svelte
- web/tests/browser/local-capture-loading.spec.ts
- web/tests/browser/local-capture-settings.spec.ts
- web/tests/vite/capture-freshness.test.ts
- web/tests/vite/capture-observation-http.test.ts
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M18-010 details 2](details-02.md)
- [E-M18-010 details 3](details-03.md)
