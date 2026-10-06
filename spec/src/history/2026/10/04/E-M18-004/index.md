---
schema: 1
id: E-M18-004
---

# E-M18-004

<!-- evidence-field:id markdown -->
## Id

E-M18-004
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M18
- M18.8
- M4.31
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-04
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

capture-notification-rebase
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing JJ change krnstxms
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/StoreCaptureSettings.stories.svelte
- catalog/fixtures/CaptureSettingsFixture.svelte
- spec/src/api.md
- spec/src/collaboration.md
- spec/src/spec-governance.md
- spec/src/ui.md
- spec/src/verification.md
- web/src/lib/controller/capture-management.svelte.ts
- web/src/lib/controller/capture-observations.ts
- web/src/lib/notifications/notification-recovery.svelte.ts
- web/src/lib/ui/store-notifications/StoreNotice.svelte
- web/src/lib/ui/store-settings/StoreCaptureImport.svelte
- web/src/lib/ui/store-settings/StoreCaptureSettings.svelte
- web/tests/browser/local-capture-settings.spec.ts
- web/tests/vite/capture-freshness.test.ts
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/vite/capture-freshness.test.ts#capture conditions recover only after
  complete successful observations
- web/tests/browser/local-capture-settings.spec.ts#capture notifications retain
  dismissal and drafts until observed recovery
- web/tests/browser/local-capture-settings.spec.ts#cold capture controls
  preserve intent and snapshot
- web/tests/browser/notifications.spec.ts#operational toasts deduplicate, retain
  dismissal and clear on recovery
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M18-004 details 2](details-02.md)
- [E-M18-004 details 3](details-03.md)
