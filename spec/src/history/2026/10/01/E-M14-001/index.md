---
schema: 1
id: E-M14-001
---

# E-M14-001

<!-- evidence-field:id markdown -->
## Id

E-M14-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M14
- M14.1
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-01
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

implementation
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- spec/src/ui.md
- spec/src/api.md
- spec/src/verification.md
- spec/src/milestones.md
- web/src/lib/controller/production-store-index.ts
- web/src/lib/ui/store-settings/StoreGraphIndexSettings.svelte
- web/src/lib/ui/store-settings/StoreRepositorySettings.svelte
- web/src/lib/ui/store-settings/StoreSearchSettings.svelte
- web/src/lib/ui/store-settings/StoreSettings.svelte
- web/src/lib/ui/store-application/StoreRepositoryNotices.svelte
- catalog/controller/in-memory-store-settings.svelte.ts
- catalog/components/store-application/StoreApplication.stories.svelte
- web/tests/vite/production-store-settings.test.ts
- web/tests/browser/local-bootstrap.spec.ts
- web/tests/browser/reads.spec.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/vite/production-store-settings.test.ts#graph preparation works
  without text search and preserves search health
- web/tests/vite/production-store-settings.test.ts#graph preparation requires
  the native index-task capability
- web/tests/browser/local-bootstrap.spec.ts#local trusted bootstrap auto-opens
  the sole repository without credentials
- web/tests/browser/reads.spec.ts#index_preparation_retries_the_original_query_after_context_edits
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M14-001 details 2](details-02.md)
- [E-M14-001 details 3](details-03.md)
