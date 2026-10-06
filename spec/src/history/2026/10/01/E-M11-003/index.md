---
schema: 1
id: E-M11-003
---

# E-M11-003

<!-- evidence-field:id markdown -->
## Id

E-M11-003
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M11
- M11.1
- M11.3
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-01
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

workstore-browser-real-rustfs
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; jj-cli/jj-lib 0.45.1; Node 26.9.0; pnpm 10.32.1; Svelte 5.57.1; Vitest 4.1.11; Playwright 1.63.0; pinned embedded RustFS runtime
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Manifest 3; WorkRootV1/WorkObjectV1; owned HTTP Work V1 contract; independent pinned paged collections and validated project/task hash fields; retained exact request bytes for unknown command outcomes. Development runtime format 2 records full fixture/search preparation separately from a fresh physical restart opening the persisted fixture; resource gates unchanged.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- AGENTS.md
- catalog/components/store-work/StoreWork.stories.svelte
- catalog/fixtures/StoreWorkFixture.svelte
- crates/jj-store-cli/tests/work_rustfs.rs
- scripts/dev-demo.mjs
- scripts/real-browser-acceptance.mjs
- spec/src/api.md
- spec/src/cli.md
- spec/src/compatibility.md
- spec/src/milestones.md
- spec/src/projects.md
- spec/src/spec-governance.md
- spec/src/ui.md
- spec/src/verification.md
- web/src/lib/controller/contracts.ts
- web/src/lib/controller/http-store-data-source.ts
- web/src/lib/controller/http-work-data-source.ts
- web/src/lib/controller/native-data-source.ts
- web/src/lib/controller/production-store-controller.svelte.ts
- web/src/lib/controller/production-store-lifecycle.svelte.ts
- web/src/lib/controller/production-store-view.ts
- web/src/lib/controller/production-work-integration.ts
- web/src/lib/controller/production-work.svelte.ts
- web/src/lib/controller/store-error.ts
- web/src/lib/controller/store-route-codec.ts
- web/src/lib/controller/store-route-history.svelte.ts
- web/src/lib/controller/work-types.ts
- web/src/lib/ui/community-navigation/model.ts
- web/src/lib/ui/store-application/StoreRepositoryApplication.svelte
- web/src/lib/ui/store-application/repository-sections.ts
- web/src/lib/ui/store-work/StoreWork.svelte
- web/src/lib/ui/store-work/StoreWorkComments.svelte
- web/src/lib/ui/store-work/StoreWorkProjectForm.svelte
- web/src/lib/ui/store-work/StoreWorkTaskForm.svelte
- web/src/lib/ui/store-work/store-work.css
- web/src/lib/ui/store-work/work-hierarchy.ts
- web/tests/real/native-repository.spec.ts
- web/tests/real/work-store.spec.ts
- web/tests/store-route-codec.test.ts
- web/tests/vite/production-work.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/vite/production-work.test.ts#work keeps unknown request intent
  across refresh and retries exactly
- web/tests/vite/production-work.test.ts#late reads cannot replace a new
  selection or another session
- web/tests/vite/production-work.test.ts#explicit model rejection releases the
  draft for correction
- web/tests/vite/production-work.test.ts#loaded hierarchy orders children after
  parents and retains page fragments
- web/tests/real/work-store.spec.ts#native projects tasks and discussion persist
  on RustFS and refresh explicitly
- crates/jj-store-cli/tests/work_rustfs.rs#workstore_real_rustfs_restart_checkpoint_copy_and_historical_recovery
- web/tests/real/native-repository.spec.ts#demo history exposes one native
  divergent change
- web/tests/real/native-repository.spec.ts#external publication waits for
  explicit refresh and preserves selection
- web/tests/real/native-repository.spec.ts#browser mutation previews and
  publishes one native request
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M11-003 details 2](details-02.md)
- [E-M11-003 details 3](details-03.md)
- [E-M11-003 details 4](details-04.md)
