---
schema: 1
id: E-M21-044
---

# E-M21-044

<!-- evidence-field:id markdown -->
## Id

E-M21-044
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M21
- M21.1
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-05
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

mvqnsormqpwutlrmqvxnzuqpwtkttmsx
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Focused repository Stacks navigation and contextual right rail
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ/jj-lib 0.45.1 and foundational package ranges unchanged; Node 26.9.0, pnpm 10.32.1, Vitest 4.1.11, Playwright 1.63.0.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Frontend-only context=stacks and stack_tab=branches|workspace route fields; session return/history selection state. No local TOML v1, JJ command, OpenAPI/generated type or persisted column-layout format change.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/StoreRepositoryPickerPending.stories.svelte
- catalog/components/store-stack-workspace/StoreStackWorkspace.stories.svelte
- catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte
- catalog/components/store-stack-workspace/StoreStacksNavigation.stories.svelte
- catalog/controller/in-memory-store-navigation.ts
- catalog/fixtures/RepositoryPickerPendingFixture.svelte
- catalog/fixtures/StackWorkspaceApplicationFixture.svelte
- catalog/fixtures/StackWorkspaceSetupApplicationFixture.svelte
- catalog/fixtures/stack-workspace/production-http.ts
- docs/maintenance/coverage.json
- docs/site/guides/browsing.md
- scripts/stacks-browser-acceptance.mjs
- spec/src/api.md
- spec/src/documentation.md
- spec/src/milestones.md
- spec/src/spec-governance.md
- spec/src/ui.md
- web/src/lib/controller/controller-actions.ts
- web/src/lib/controller/production-route-restorer.ts
- web/src/lib/controller/production-stacks.svelte.ts
- web/src/lib/controller/production-store-actions.ts
- web/src/lib/controller/production-store-controller.svelte.ts
- web/src/lib/controller/production-store-reader.svelte.ts
- web/src/lib/controller/store-route-codec.ts
- web/src/lib/controller/store-route-fields.ts
- web/src/lib/controller/store-route-history.svelte.ts
- web/src/lib/controller/store-route-types.ts
- web/src/lib/controller/store-stacks-navigation.ts
- web/src/lib/controller/store-stacks-route-codec.ts
- web/src/lib/ui/store-application/StoreApplication.svelte
- web/src/lib/ui/store-application/StoreApplicationHeaderActions.svelte
- web/src/lib/ui/store-application/StoreApplicationMain.svelte
- web/src/lib/ui/store-application/StoreApplicationNavigation.svelte
- web/src/lib/ui/store-application/StoreApplicationRightPanels.svelte
- web/src/lib/ui/store-application/StoreRepositoryPicker.svelte
- web/src/lib/ui/store-application/StoreRepositoryTabs.svelte
- web/src/lib/ui/store-application/StoreViewTabs.svelte
- web/src/lib/ui/store-application/repository-sections.ts
- web/src/lib/ui/store-application/store-navigation.ts
- web/src/lib/ui/store-stack-workspace/StoreStackWorkspace.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceDetails.svelte
- web/src/lib/ui/store-stack-workspace/StoreStacksApplicationView.svelte
- web/src/lib/ui/store-stack-workspace/StoreStacksNavigation.svelte
- web/src/lib/ui/store-stack-workspace/stack-workspace-layout.css
- web/src/lib/ui/store-work/StoreProjectsRepositories.svelte
- web/tests/real/stacks-browser.spec.ts
- web/tests/vite/production-stacks-application.test.ts
- web/tests/vite/production-stacks-lifecycle.test.ts
- web/tests/vite/store-route-history.test.ts
- web/tests/vite/store-stacks-navigation.test.ts
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-API-005
- JST-DOC-002
- JST-GOV-002
- JST-GOV-003
- JST-UI-005
- JST-UI-014
- JST-UI-020
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Stacks used a separate sidebar item and canvas-only tabs, ordinary repository chrome on history, and the generic right rail. Back request refined to transparent at rest with a circular hover surface. Fresh route reads could replace a pending target choice or make a branch click unavailable during canvas-tab switching.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Reuse owned repository body-tab overflow, Button/Tooltip, Settings read-only rows and existing repository-wide history renderers/readers. Stacks tab precedes Branches, current repository stays highlighted, focused Back/Branches/Workspace/Commits/Op Log hides ordinary repository/Terminal actions but retains Details, workspace picker and actions. Session-only return route, validated route fields, mounted canvas/controller, exact pending state and bounded owner revalidation. Far-right rail follows workspace, stack/dependent branch, file, commit and operation selections. Full refresh retains its data reads while canvas switching avoids redundant workspace reads. Cold verification also corrects the shared owned picker to keep open choices disabled during pending reads rather than silently discarding their clicks; direct picker, connection, Settings, WorkStore and Stacks consumers are rechecked. The direct Projects consumer check also exposed an initial directory skeleton predicate missing viewport-owned metadata loading. Extended that predicate only; observation leases, bounded reads and loaded-content retention remain unchanged.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M21-044 details 2](details-02.md)
- [E-M21-044 details 3](details-03.md)
- [E-M21-044 details 4](details-04.md)
