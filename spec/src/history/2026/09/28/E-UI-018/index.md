---
schema: 1
id: E-UI-018
---

# E-UI-018

<!-- evidence-field:id markdown -->
## Id

E-UI-018
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4
- M4.3
- M4.16
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-28
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

implementation
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- .storybook/vite.config.ts
- catalog/components/store-mutations/StoreRepositoryActions.stories.svelte
- catalog/controller/in-memory-store-controller.svelte.ts
- catalog/controller/in-memory-store-member.ts
- catalog/controller/in-memory-store-mutation-commit.ts
- catalog/controller/in-memory-store-mutations.svelte.ts
- catalog/controller/in-memory-store-repository.svelte.ts
- catalog/controller/in-memory-store-route.ts
- catalog/fixtures/StoreRepositoryActionsFixture.svelte
- catalog/fixtures/repository.ts
- catalog/fixtures/snapshots.ts
- crates/jj-store-cli/tests/native_lifecycle/abandon.rs
- crates/jj-store-cli/tests/native_lifecycle/http_mutations.rs
- crates/jj-store-cli/tests/native_lifecycle/reconciliation.rs
- crates/jj-store-cli/tests/native_lifecycle/resolve.rs
- crates/jj-store-http/src/contract.rs
- crates/jj-store-http/src/error.rs
- crates/jj-store-service/src/mutations.rs
- docs/browser.md
- docs/implementation-plan.md
- schemas/openapi.json
- scripts/browser-fixtures.mjs
- spec/src/api.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/compatibility.md
- spec/src/milestones.md
- spec/src/mutations.md
- spec/src/spec-governance.md
- spec/src/ui.md
- spec/src/verification.md
- web/src/lib/api/generated.ts
- web/src/lib/controller/contracts.ts
- web/src/lib/controller/http-store-data-source.ts
- web/src/lib/controller/native-data-source.ts
- web/src/lib/controller/production-store-controller.svelte.ts
- web/src/lib/controller/production-store-mutations.ts
- web/src/lib/controller/production-store-state.svelte.ts
- web/src/lib/controller/production-store-view.ts
- web/src/lib/controller/store-mutation-types.ts
- web/src/lib/ui/store-application/StoreApplication.svelte
- web/src/lib/ui/store-application/StoreApplicationHeaderActions.svelte
- web/src/lib/ui/store-mutations/StoreMutationDialog.svelte
- web/src/lib/ui/store-mutations/StoreMutationFields.svelte
- web/src/lib/ui/store-mutations/StoreMutationPreview.svelte
- web/src/lib/ui/store-mutations/StoreRepositoryActions.svelte
- web/src/lib/ui/store-mutations/model.ts
- web/src/lib/ui/store-mutations/store-mutations.css
- web/tests/browser/mutations.spec.ts
- web/tests/browser/native-browser-fixture.ts
- web/tests/production-store-mutations.test.ts
- web/tests/real/native-repository.spec.ts
- web/tests/store-mutation-model.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/production-store-mutations.test.ts#preview and apply retain one
  exact request through commitment
- web/tests/production-store-mutations.test.ts#unknown outcomes keep the request
  identity for status recovery
- web/tests/production-store-mutations.test.ts#late preview responses cannot
  repopulate a departed session
- web/tests/store-mutation-model.test.ts#native mutation inputs reject
  abbreviated object identities
- web/tests/store-mutation-model.test.ts#bookmark commands preserve exact
  expected target sets
- web/tests/store-mutation-model.test.ts#reconcile and abandon retain their
  native command controls
- web/tests/browser/mutations.spec.ts#repository action menu unlocks the page
- web/tests/browser/mutations.spec.ts#repository command dialog cancel unlocks
  the page
- web/tests/browser/mutations.spec.ts#repository command preview and apply
  retain the captured head and request
- web/tests/browser/mutations.spec.ts#unknown mutation outcome is recovered
  through durable status
- web/tests/real/native-repository.spec.ts#browser mutation previews and
  publishes one native request
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-UI-018 details 2](details-02.md)
