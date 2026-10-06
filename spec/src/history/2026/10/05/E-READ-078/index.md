---
schema: 1
id: E-READ-078
---

# E-READ-078

<!-- evidence-field:id markdown -->
## Id

E-READ-078
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M13.4
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-05
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

xzlqzrvo
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Exact browser recovery and controls for bounded native kind preparation
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ/jj-lib0.45.1; Svelte5.57.1; Vitest4.1.11; Storybook10.5.7; pnpm10.32.1; Node26.9.0; macOS ARM64. Existing local Design Core source-validation dependency refreshed offline; registry-resolved release qualification remains deferred.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Consumes existing KindPreparationTask HTTP/OpenAPI/generated client contracts. No HTTP/RPC schema, persisted identity/object/manifest/WAL/receipt/cache-format change. Preparation UI state is session-only.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Browser preparation owner, native HTTP data source, Settings controls and direct graph/local consumers. Exact 17-test kind boundary followed by all seven direct controller unit files, web/catalog types, four cold production-component keyboard/interaction/accessibility stories and ownership/architecture/canonical checks. No Rust production change, builds or Rust tests for this UI slice. Complete scale/RSS/latency/fault/cold full-flow/RustFS/installed-runtime/release/Help qualification and visual baselines remain deferred. No milestone promotion.
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-API-001
- JST-UI-001
- JST-UI-005
- JST-UI-019
- JST-GOV-002
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/StoreKindPreparation.stories.svelte
- catalog/fixtures/KindPreparationFixture.svelte
- docs/implementation-plan.md
- spec/src/api.md
- spec/src/architecture.md
- spec/src/spec-governance.md
- spec/src/ui.md
- web/src/lib/controller/contracts.ts
- web/src/lib/controller/http-store-data-source.ts
- web/src/lib/controller/http-store-write-data-source.ts
- web/src/lib/controller/index-preparation-labels.ts
- web/src/lib/controller/native-data-source.ts
- web/src/lib/controller/production-index-preparation.ts
- web/src/lib/controller/production-store-actions.ts
- web/src/lib/controller/production-store-index.ts
- web/src/lib/controller/production-store-state.svelte.ts
- web/src/lib/controller/production-store-view.ts
- web/src/lib/controller/store-settings-types.ts
- web/src/lib/ui/store-application/StoreRepositoryNotices.svelte
- web/src/lib/ui/store-settings/StoreGraphIndexSettings.svelte
- web/tests/vite/kind-preparation-http.test.ts
- web/tests/vite/kind-preparation.test.ts
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/vite/kind-preparation.test.ts#native kind preparation retains the
  original operation and restores only after complete progress
- web/tests/vite/kind-preparation.test.ts#copy preparation uses only copy task
  routes and cannot overlap graph preparation
- web/tests/vite/kind-preparation.test.ts#kind status observes bounded progress
  even with healthy settlement events
- web/tests/vite/kind-preparation.test.ts#kind cancellation remains retryable
  when confirmation fails
- web/tests/vite/kind-preparation.test.ts#one in-flight kind status read
  suppresses later requests and stale output after navigation
- web/tests/vite/kind-preparation.test.ts#native kind actions do no work for
  local or unsupported sources
- web/tests/vite/kind-preparation-http.test.ts#native kind transport retains the
  selected operation and bodyless exact cancellation
- web/tests/vite/kind-preparation-http.test.ts#native kind task reset remains an
  explicit error without graph or inventory fallback
- web/tests/vite/production-store-settings.test.ts#cancel transport failure
  remains unconfirmed and polls without replaying cancel
- web/tests/vite/index-freshness.test.ts#healthy preparation ignores foreign
  tasks and performs no periodic status reads
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-READ-078 details 2](details-02.md)
- [E-READ-078 details 3](details-03.md)
