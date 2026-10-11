---
schema: 1
id: E-NUMERIC-SETTINGS-001
---

# E-NUMERIC-SETTINGS-001

<!-- evidence-field:id markdown -->
## Id

E-NUMERIC-SETTINGS-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M14.4
- M14.6
- M19
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

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

- catalog/components/store-application/StoreNumericSetting.stories.svelte
- catalog/components/store-application/StoreSearchSettings.stories.svelte
- catalog/components/store-application/StoreServerSettings.stories.svelte
- catalog/fixtures/NumericSettingFixture.svelte
- docs/maintenance/coverage.json
- docs/site/guides/settings.md
- spec/src/current/contracts/api/30-installation-configuration-editing.md
- spec/src/current/contracts/documentation/01-application-documentation.md
- spec/src/current/contracts/ui/28-complete-supported-configuration-settings.md
- spec/src/current/progress/M14/history/M14.4/index.md
- spec/src/current/progress/M14/history/M14.6/index.md
- spec/src/current/progress/M14/index.md
- spec/src/current/progress/M19/history/M19/index.md
- spec/src/current/progress/M19/index.md
- spec/src/history/2026/10/11/E-NUMERIC-SETTINGS-001/SUMMARY.nav.md
- spec/src/history/2026/10/11/E-NUMERIC-SETTINGS-001/details-02.md
- spec/src/history/2026/10/11/E-NUMERIC-SETTINGS-001/details-03.md
- spec/src/history/2026/10/11/E-NUMERIC-SETTINGS-001/index.md
- spec/src/history/2026/10/11/SUMMARY.nav.md
- spec/src/history/2026/10/11/index.md
- web/src/lib/ui/AGENTS.md
- web/src/lib/ui/store-settings/StoreNumericSettingNumberRow.svelte
- web/src/lib/ui/store-settings/StoreNumericSettingRow.svelte
- web/src/lib/ui/store-settings/StoreSearchAdvanced.svelte
- web/src/lib/ui/store-settings/StoreSearchFields.svelte
- web/src/lib/ui/store-settings/StoreSyncConfiguration.svelte
- web/src/lib/ui/store-settings/numeric-setting.css
- web/src/lib/ui/store-settings/numeric-setting.ts
- web/src/lib/ui/store-settings/search-numeric-fields.ts
- web/src/lib/ui/store-settings/sync-numeric-fields.ts
- web/tests/numeric-setting.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/numeric-setting.test.ts#numeric units preserve exact time and byte
  values
- web/tests/numeric-setting.test.ts#numeric validation rejects empty,
  fractional, unsafe and out-of-range values without rounding
- web/tests/vite/production-search-settings.test.ts#search configuration keeps
  explicit scope identities and validates duplicates and budgets
- web/tests/vite/production-search-settings.test.ts#save preserves draft and its
  revision through stale failure and refresh
- web/tests/vite/production-search-settings.test.ts#pending edits block apply
  and late replies cannot affect another repository
- web/tests/vite/production-search-settings.test.ts#automatic status reads
  retain the revision that started a dirty search draft
- web/tests/vite/production-search-settings.test.ts#cancelling a search draft
  adopts the refreshed configuration revision
- web/tests/vite/production-search-settings.test.ts#save and apply use
  independent revision checked protected requests
- web/tests/vite/production-search-settings.test.ts#invalid search drafts stay
  inline across refresh and clear on cancellation
- web/tests/vite/production-search-settings.test.ts#advanced Search limits
  roundtrip and reject invalid worker bounds
- web/tests/vite/server-settings.test.ts#typed configuration saves only its
  section and retains independent drafts
- web/tests/vite/server-settings.test.ts#refresh never rebases a dirty section
  after external or Search edits
- web/tests/vite/server-settings.test.ts#unknown save retains original intent
  until fresh matching observation
- web/tests/vite/server-settings.test.ts#mismatching uncertain draft may be
  discarded only after observation
- web/tests/vite/server-settings.test.ts#old GET cannot replace a save
  acknowledgement and observations coalesce
- web/tests/vite/server-settings.test.ts#unavailable connections perform no
  configuration I/O
- web/tests/vite/server-settings.test.ts#validation prevents credential values,
  unsafe upstreams and unbounded budgets
<!-- /evidence-field -->


<!-- generated-navigation -->

## Pages

- [E-NUMERIC-SETTINGS-001 details 2](details-02.md)
- [E-NUMERIC-SETTINGS-001 details 3](details-03.md)
