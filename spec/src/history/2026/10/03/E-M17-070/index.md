---
schema: 1
id: E-M17-070
---

# E-M17-070

<!-- evidence-field:id markdown -->
## Id

E-M17-070
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M17
- M17.2
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-03
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

installation-browser-capture-management
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change rywrmxqw
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/StoreCaptureSettings.stories.svelte
- catalog/fixtures/CaptureSettingsFixture.svelte
- catalog/fixtures/capture-management.ts
- spec/src/api.md
- spec/src/collaboration.md
- spec/src/ui.md
- spec/src/verification.md
- web/src/lib/controller/capture-errors.ts
- web/src/lib/controller/capture-http.ts
- web/src/lib/controller/capture-management.svelte.ts
- web/src/lib/controller/capture-types.ts
- web/src/lib/controller/capture-validation.ts
- web/src/lib/controller/contracts.ts
- web/src/lib/controller/production-capture-management.ts
- web/src/lib/controller/production-store-controller.svelte.ts
- web/src/lib/controller/production-store-lifecycle.svelte.ts
- web/src/lib/controller/production-store-state.svelte.ts
- web/src/lib/controller/production-store-view.ts
- web/src/lib/ui/store-settings/StoreCaptureDiagnostics.svelte
- web/src/lib/ui/store-settings/StoreCaptureImport.svelte
- web/src/lib/ui/store-settings/StoreCaptureSettings.svelte
- web/src/lib/ui/store-settings/StoreCaptureSources.svelte
- web/src/lib/ui/store-settings/StoreServiceSettings.svelte
- web/tests/browser/local-capture-settings.spec.ts
- web/tests/vite/capture-management.test.ts
- web/tests/vite/production-capture-management.test.ts
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/vite/capture-management.test.ts#capture transport preserves exact
  local policy
- web/tests/vite/capture-management.test.ts#unknown control retains exact
  original intent
- web/tests/vite/capture-management.test.ts#capture source pages reject stale
  continuations
- web/tests/vite/capture-management.test.ts#capture import requires its matching
  original receipt
- web/tests/vite/capture-management.test.ts#capture refuses malformed or
  mismatched acceptance
- web/tests/vite/capture-management.test.ts#capture single flight and teardown
  ignore late replies
- web/tests/vite/capture-management.test.ts#capture responses cannot invent
  history completion
- web/tests/vite/production-capture-management.test.ts#installation capture
  survives repository disconnect
- web/tests/browser/local-capture-settings.spec.ts#cold capture controls
  preserve intent and snapshot
- web/tests/browser/local-capture-settings.spec.ts#cold capture capability gate
  withholds transport
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M17-070 details 2](details-02.md)
- [E-M17-070 details 3](details-03.md)
