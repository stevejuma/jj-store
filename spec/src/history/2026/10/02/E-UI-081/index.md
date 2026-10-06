---
schema: 1
id: E-UI-081
---

# E-UI-081

<!-- evidence-field:id markdown -->
## Id

E-UI-081
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M15.5
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-02
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Repository-owned sidebar threads and contextual breadcrumbs
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

Enclosing Jujutsu change rsnmqnln
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-002
- JST-UI-005
- JST-UI-006
- JST-UI-017
- JST-API-006
- JST-GOV-002
- JST-GOV-003
- JST-GOV-007
- JST-GOV-008
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Visual baselines remain visual-pending. No milestone status promotion. Rust
  and root aggregate lanes, built production CSP qualification, backend
  rebuild/reset/restart, main landing and publication are out of scope for this
  UI-only slice.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Reuse the appearance workspace and existing Vite HMR. Community navigation and Projects breadcrumbs inspected first, Design Core Collapsible/Command View/Breadcrumb second. Remove the standalone Threads section; native repositories disclose indented Browse threads and exact thread children. Browse threads label omits the redundant repository name. Group icon-only action and compact Actions entry use the connected native owner or authorized native fallback. Independent memory-only bounded list readers load up to two repositories concurrently, retain loaded windows on refresh, validate repository/epoch/thread root and clear protected data on session teardown. Selection binds the explicit owner, guards late connections and omits unsupported local folders. Repository/Threads/selected-thread breadcrumbs reuse Projects header/mobile placement and preserve available code context. Owned picker/action/lifecycle extraction and shared controller callbacks keep existing architecture budgets. Directly affected thread/navigation/project persistence stories and production bootstrap/refresh tests are the complete focused impact surface; root aggregates do not apply.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-threads/StoreThreadNavigation.stories.svelte
- catalog/components/store-threads/StoreThreads.stories.svelte
- catalog/fixtures/StoreThreadsFixture.svelte
- docs/ui-provenance.json
- spec/src/api.md
- spec/src/milestones.md
- spec/src/spec-governance.md
- spec/src/ui.md
- web/src/lib/controller/contracts.ts
- web/src/lib/controller/production-store-controller.svelte.ts
- web/src/lib/controller/production-store-lifecycle.svelte.ts
- web/src/lib/controller/production-store-refresh.ts
- web/src/lib/controller/production-store-view.ts
- web/src/lib/controller/production-threads-integration.ts
- web/src/lib/controller/repository-thread-navigation.svelte.ts
- web/src/lib/ui/community-navigation/CommunityNavigation.svelte
- web/src/lib/ui/community-navigation/CommunityNavigationAction.svelte
- web/src/lib/ui/community-navigation/CommunityNavigationSectionPicker.svelte
- web/src/lib/ui/community-navigation/CommunityNavigationTree.css
- web/src/lib/ui/community-navigation/icons.ts
- web/src/lib/ui/community-navigation/model.ts
- web/src/lib/ui/store-application/StoreApplication.svelte
- web/src/lib/ui/store-application/store-navigation.ts
- web/src/lib/ui/store-threads/StoreThreadsBreadcrumb.svelte
- web/src/lib/ui/store-threads/thread-navigation-binding.svelte.ts
- web/src/lib/ui/store-threads/thread-navigation.ts
- web/tests/vite/automatic-store-refresh.test.ts
- web/tests/vite/repository-thread-navigation.test.ts
- web/tests/vite/thread-navigation-integration.test.ts
- web/tests/vite/thread-navigation.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Node26.9.0; pnpm10.32.1; Svelte5.57.1; Vitest4.1.11; Storybook10.5.7; existing Design Core ^0.2.0 and owned Community provenance retained. No dependency changes.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Existing bounded Threads HTTP reads, native thread IDs, route/hash, code operation pins, native object/manifest/WAL/receipt and OpenAPI formats unchanged. Optional controller navigation and repository owner fields are presentation-only. Disclosure flags retain existing versioned browser layout storage; sidebar thread data and credentials stay memory-only.
<!-- /evidence-field -->

<!-- evidence-field:artifacts markdown -->
## Artifacts

/tmp/jj-thread-navigation-*.log; /tmp/jj-thread-navigation-live.jpg
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Retain visual-pending; use the verified committed workspace for further UI changes. Main landing and publication require a separate request.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-UI-081 details 2](details-02.md)
