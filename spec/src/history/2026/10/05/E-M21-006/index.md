---
schema: 1
id: E-M21-006
---

# E-M21-006

<!-- evidence-field:id markdown -->
## Id

E-M21-006
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M21
- M21.2
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-05
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

utwnrkmk
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Production operation-pinned local Stacks browsing and application lifecycle injection
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Custom JJ 0.45.1; pnpm 10.32.1; Vitest 4.1.11; parent 1f7ff787; isolated cold browser server 7489; source Storybook 6057
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No storage, companion, generated HTTP or checkout format changes. Frontend opaque read context binds repository/profile/operation/binding generation; bookmark and history cursors remain process-local reads. Presentation stack/hunk IDs grant no mutation authority.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-stack-workspace/StoreStackWorkspaceProduction.stories.svelte
- catalog/fixtures/StackWorkspaceProductionFixture.svelte
- catalog/fixtures/stack-workspace/production-http.ts
- docs/implementation-plan.md
- docs/maintenance/coverage.json
- docs/site/technical/architecture.md
- scripts/stacks-browser-acceptance.mjs
- spec/src/api.md
- spec/src/architecture.md
- spec/src/documentation.md
- spec/src/spec-governance.md
- spec/src/ui.md
- spec/src/verification.md
- web/src/App.svelte
- web/src/lib/controller/contracts.ts
- web/src/lib/controller/controller-actions.ts
- web/src/lib/controller/production-command-services.ts
- web/src/lib/controller/production-route-restorer.ts
- web/src/lib/controller/production-stacks.svelte.ts
- web/src/lib/controller/production-store-controller.svelte.ts
- web/src/lib/controller/production-store-lifecycle.svelte.ts
- web/src/lib/controller/production-store-reader.svelte.ts
- web/src/lib/controller/production-store-view.ts
- web/src/lib/controller/stack-workspace/branches.svelte.ts
- web/src/lib/controller/stack-workspace/controller-files.ts
- web/src/lib/controller/stack-workspace/controller-snapshot.ts
- web/src/lib/controller/stack-workspace/controller.svelte.ts
- web/src/lib/controller/stack-workspace/issued-history.ts
- web/src/lib/controller/stack-workspace/lazy-details.svelte.ts
- web/src/lib/controller/stack-workspace/loaded-ranges.ts
- web/src/lib/controller/stack-workspace/local-references.ts
- web/src/lib/controller/stack-workspace/presentation.ts
- web/src/lib/controller/stack-workspace/production-adapter.ts
- web/src/lib/controller/stack-workspace/production-snapshot.ts
- web/src/lib/controller/stack-workspace/read-context.ts
- web/src/lib/controller/stack-workspace/read-session.ts
- web/src/lib/controller/stack-workspace/reference-pages.ts
- web/src/lib/controller/stack-workspace/source-range.ts
- web/src/lib/controller/stack-workspace/stack-history.svelte.ts
- web/src/lib/controller/stack-workspace/target-history.svelte.ts
- web/src/lib/controller/stack-workspace/types.ts
- web/src/lib/ui/store-application/StoreApplicationMain.svelte
- web/src/lib/ui/store-application/StoreRepositoryPicker.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackBookmarkCard.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackLane.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceCanvas.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceSetup.svelte
- web/tests/real/stacks-browser.spec.ts
- web/tests/vite/production-stack-adapter.test.ts
- web/tests/vite/production-stacks-application.test.ts
- web/tests/vite/production-stacks-lifecycle.test.ts
- web/tests/vite/stack-workspace-stack-history.test.ts
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Catalog adapter only. Production injection absent; bookmark continuation, ancestor retention, scoped lifecycle and cold real-folder read acceptance unqualified. Narrow missing-module/history/pagination regressions recorded before implementation.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Production-owned read adapter, bounded references and exact history/files/diffs, App injection, registered-folder eligibility, existing index notices, staged read-owner refresh, shared-transport cancellation, direct consumer tests and manual/spec coverage.
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/vite/production-stack-adapter.test.ts#production metadata is bounded
  and never initializes target, membership, files or commands
- web/tests/vite/production-stack-adapter.test.ts#branch continuation reaches
  later target choices and rejects replayed cursors
- web/tests/vite/production-stack-adapter.test.ts#refresh retains verified
  loaded ancestors at an unchanged head and base
- web/tests/vite/production-stack-adapter.test.ts#operation refresh revalidates
  loaded target ancestry and rejects the old read scope
- web/tests/vite/production-stacks-application.test.ts#real bootstrap injects a
  verified Stacks controller and its route never reads the ordinary graph
- web/tests/vite/production-stacks-application.test.ts#explicit index
  preparation restores Stacks at its exact operation and performs no code
  command
- web/tests/vite/production-stacks-lifecycle.test.ts#clearing during discovery
  aborts it and late results cannot expose a controller
- web/tests/real/stacks-browser.spec.ts#cold_registered_stacks_browse_real_ancestors_files_and_diffs_without_writes
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M21-006 details 2](details-02.md)
- [E-M21-006 details 3](details-03.md)
- [E-M21-006 details 4](details-04.md)
