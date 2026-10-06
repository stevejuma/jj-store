---
schema: 1
id: E-UI-019
---

# E-UI-019

<!-- evidence-field:id markdown -->
## Id

E-UI-019
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4
- M4.3
- M4.17
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

- catalog/components/store-application/RepositoryApplication.stories.svelte
- catalog/components/store-mutations/StoreConflictEditor.stories.svelte
- catalog/components/store-search/StoreSearchCommand.stories.svelte
- catalog/controller/in-memory-store-conflicts.ts
- catalog/controller/in-memory-store-controller.svelte.ts
- catalog/controller/in-memory-store-mutations.svelte.ts
- catalog/fixtures/StoreConflictEditorFixture.svelte
- catalog/fixtures/StoreSearchCommandFixture.svelte
- catalog/fixtures/conflicts.ts
- docs/browser.md
- docs/implementation-plan.md
- scripts/dev-demo-repository.mjs
- scripts/dev-demo.mjs
- spec/src/api.md
- spec/src/architecture.md
- spec/src/milestones.md
- spec/src/spec-governance.md
- spec/src/ui.md
- spec/src/verification.md
- web/src/lib/controller/contracts.ts
- web/src/lib/controller/http-store-data-source.ts
- web/src/lib/controller/native-data-source.ts
- web/src/lib/controller/production-store-conflicts.ts
- web/src/lib/controller/production-store-controller.svelte.ts
- web/src/lib/controller/production-store-index.ts
- web/src/lib/controller/production-store-loaders.ts
- web/src/lib/controller/production-store-reader.svelte.ts
- web/src/lib/controller/production-store-state.svelte.ts
- web/src/lib/controller/production-store-view.ts
- web/src/lib/ui/store-conflicts/StoreConflictEditor.svelte
- web/src/lib/ui/store-conflicts/StoreConflictPath.svelte
- web/src/lib/ui/store-conflicts/model.ts
- web/src/lib/ui/store-conflicts/store-conflicts.css
- web/src/lib/ui/store-mutations/StoreMutationPreview.svelte
- web/src/lib/ui/store-mutations/StoreRepositoryActions.svelte
- web/tests/browser/conflicts.spec.ts
- web/tests/production-store-conflicts.test.ts
- web/tests/real/native-repository.spec.ts
- web/tests/store-conflict-model.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/store-conflict-model.test.ts#atomic conflict commands retain every
  path and native fingerprint
- web/tests/store-conflict-model.test.ts#atomic conflict commands reject
  omissions and foreign term indexes
- web/tests/store-conflict-model.test.ts#uploaded conflict resolution keeps the
  scoped staged handle
- web/tests/production-store-conflicts.test.ts#conflict uploads bind bytes to
  the captured head commit path and fingerprint
- web/tests/production-store-conflicts.test.ts#conflict uploads reject oversized
  bytes before transport
- web/tests/browser/conflicts.spec.ts#native conflict editor stages and previews
  one complete atomic selection
- web/tests/real/native-repository.spec.ts#native conflict selection previews
  and publishes one atomic resolution
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-UI-019 details 2](details-02.md)
- [E-UI-019 details 3](details-03.md)
