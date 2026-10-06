---
schema: 1
id: E-UI-097
---

# E-UI-097

<!-- evidence-field:id markdown -->
## Id

E-UI-097
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4.31
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-04
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

notification-main-integration-and-workspace-retirement
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

mompnpzq (enclosing Jujutsu merge change)
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Both main workspaces and both notifications workspaces were clean. jj-store main had advanced from c8b7ea72 to d4a70441 with sidebar actions, reusable copy rows and collapsible history search. Merging kkoupnws produced eight conflicted paths; Design Core main could advance directly from 04b1ec1b to lqkpxqtl/95ef5207.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; Node24.10.0, pnpm10.32.1, JJ0.45.1, Svelte5.57.1, Vitest4.1.11, Playwright1.63.0. Main Design Core source95ef5207922a remains package0.1.2; consumer manifest remains portable ^0.2.0. Existing local source-validation hook is used; no registry release acceptance is claimed.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No API wire, native identity, manifest, WAL, storage or credential formats changed. Existing main navigation/copy/search behavior and notification lifecycle contracts are preserved. Source E-UI-094 was renumbered to E-UI-096 because newer main already owns E-UI-094 and E-UI-095; every existing main evidence object is retained verbatim.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- AGENTS.md
- spec/src/api.md
- spec/src/ui.md
- spec/src/milestones.md
- web/src/lib/ui/store-application/StoreApplication.svelte
- web/src/lib/ui/store-settings/StoreSearchHealth.svelte
- web/src/lib/ui/store-settings/StoreSettingsRepository.svelte
- web/src/lib/ui/store-settings/StoreStorageSettings.svelte
- web/src/lib/ui/store-settings/StoreSyncSettings.svelte
- web/src/lib/ui/store-threads/StoreThreadSettings.svelte
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/browser/notifications.spec.ts#operational toasts deduplicate, retain
  dismissal and clear on recovery
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-UI-097 details 2](details-02.md)
