---
schema: 1
id: E-M18-003
---

# E-M18-003

<!-- evidence-field:id markdown -->
## Id

E-M18-003
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M18
- M18.8
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-04
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

capture-settings
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing JJ change kyvxzyzu
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/StoreCaptureSettings.stories.svelte
- catalog/fixtures/capture-management.ts
- crates/jj-store-http/src/contract/capture_tests.rs
- crates/jj-store-server/src/capture_management/report.rs
- crates/jj-store-server/src/capture_management.rs
- crates/jj-store-server/src/capture_runtime/control.rs
- crates/jj-store-server/src/capture_runtime/sources.rs
- crates/jj-store-server/src/capture_runtime.rs
- crates/jj-store-server/src/capture_supervisor/tests/observations.rs
- crates/jj-store-server/src/capture_supervisor.rs
- crates/jj-store-server/tests/capture_management/coverage.rs
- crates/jj-store-server/tests/capture_management/http.rs
- crates/jj-store-server/tests/capture_management/imports/inspection.rs
- crates/jj-store-server/tests/capture_management/imports.rs
- crates/jj-store-server/tests/capture_management/rescan.rs
- crates/jj-store-service/src/capture_management.rs
- docs/capture-preview.md
- schemas/openapi.json
- scripts/contracts.mjs
- scripts/dev-capture.mjs
- spec/src/api.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/compatibility.md
- spec/src/mutations.md
- spec/src/spec-governance.md
- spec/src/ui.md
- web/src/lib/api/generated.ts
- web/src/lib/controller/capture-root-validation.ts
- web/src/lib/controller/capture-validation.ts
- web/src/lib/ui/store-settings/StoreAgentSettings.svelte
- web/src/lib/ui/store-settings/StoreCaptureImport.svelte
- web/src/lib/ui/store-settings/StoreCaptureRoots.svelte
- web/src/lib/ui/store-settings/StoreCaptureSettings.svelte
- web/src/lib/ui/store-settings/StoreCaptureSources.svelte
- web/tests/browser/local-capture-settings.spec.ts
- web/tests/vite/capture-management.test.ts
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-server/tests/capture_management/coverage.rs#harness_coverage_is_bounded_cached_and_reports_configured_roots
- crates/jj-store-server/src/capture_supervisor/tests/observations.rs#configured_roots_retain_default_home_override_and_explicit_selection
- crates/jj-store-server/tests/capture_management/imports/inspection.rs#missing_reader_link_and_unqualified_publication_withhold_completion_without_repair
- crates/jj-store-http/src/contract/capture_tests.rs#capture_management_contract_is_installation_local_and_actor_free
- web/tests/vite/capture-management.test.ts#configured harness roots are bounded
  absolute paths with honest counts
- web/tests/browser/local-capture-settings.spec.ts#cold capture controls
  preserve intent and snapshot
- web/tests/browser/local-capture-settings.spec.ts#visible capture follows
  observations and stops when settings closes
<!-- /evidence-field -->

<!-- evidence-field:removed_tests list -->
## Removed tests

- crates/jj-store-server/tests/capture_management/coverage.rs#harness_coverage_is_bounded_cached_and_contains_no_roots
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M18-003 details 2](details-02.md)
- [E-M18-003 details 3](details-03.md)
