---
schema: 1
id: E-M9-032
---

# E-M9-032

<!-- evidence-field:id markdown -->
## Id

E-M9-032
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M9
- M9.5
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-29
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

collaboration-application-presentation
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- .storybook/vite.config.ts
- catalog/components/store-application/StoreCollaboration.stories.svelte
- catalog/components/store-application/StoreConnection.stories.svelte
- catalog/controller/collaboration-store-controller.ts
- catalog/controller/in-memory-store-collaboration.svelte.ts
- catalog/controller/in-memory-store-controller.svelte.ts
- catalog/controller/in-memory-store-history.ts
- catalog/controller/in-memory-store-options.ts
- catalog/controller/in-memory-store-route.ts
- catalog/controller/in-memory-store-status.ts
- catalog/fixtures/StoreRepositoryApplicationFixture.svelte
- catalog/fixtures/collaboration.ts
- crates/jj-store-http/src/assets.rs
- crates/jj-store-http/tests/assets.rs
- docs/catalog.md
- package.json
- pnpm-lock.yaml
- scripts/local-package-pnpmfile.cjs
- scripts/local-package-pnpmfile.test.mjs
- scripts/web-manifest.mjs
- spec/src/api.md
- spec/src/architecture.md
- spec/src/collaboration.md
- spec/src/milestones.md
- spec/src/spec-governance.md
- spec/src/ui.md
- spec/src/verification.md
- web/src/lib/collaboration/collaboration-canvas.ts
- web/src/lib/collaboration/presentation.ts
- web/src/lib/controller/contracts.ts
- web/src/lib/controller/http-collaboration-data-source.ts
- web/src/lib/controller/http-store-data-source.ts
- web/src/lib/controller/http-store-write-data-source.ts
- web/src/lib/controller/native-data-source.ts
- web/src/lib/controller/production-route-restorer.ts
- web/src/lib/controller/production-store-actions.ts
- web/src/lib/controller/production-store-collaboration.svelte.ts
- web/src/lib/controller/production-store-reader.svelte.ts
- web/src/lib/controller/production-store-state.svelte.ts
- web/src/lib/controller/production-store-view.ts
- web/src/lib/controller/store-route-codec.ts
- web/src/lib/controller/store-route-history.svelte.ts
- web/src/lib/ui/community-navigation/CommunityNavigation.svelte
- web/src/lib/ui/community-navigation/icons.ts
- web/src/lib/ui/community-navigation/model.ts
- web/src/lib/ui/store-application/StoreApplication.svelte
- web/src/lib/ui/store-application/StoreRepositoryApplication.svelte
- web/src/lib/ui/store-collaboration/StoreAgentSessions.svelte
- web/src/lib/ui/store-collaboration/StoreCollaborationSkeleton.svelte
- web/src/lib/ui/store-collaboration/StoreReviewThreads.svelte
- web/src/lib/ui/store-collaboration/store-collaboration.css
- web/tests/store-route-codec.test.ts
- web/tests/vite/production-store-collaboration.test.ts
- web/tests/vite/production-store-data-source.test.ts
- web/tests/vite/production-store-events.test.ts
- web/tests/vite/store-route-history.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/store-route-codec.test.ts#collaboration routes preserve stable
  session and thread selections
- web/tests/vite/store-route-history.test.ts#collaboration selections survive
  reload and browser Back and Forward
- web/tests/vite/production-store-collaboration.test.ts#late collaboration
  details cannot repopulate an invalidated session
- web/tests/vite/production-store-data-source.test.ts#explicit snapshot refresh
  invalidates cached collaboration roots
- web/tests/vite/production-store-events.test.ts#collaboration publication
  events announce freshness without mixing snapshots
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M9-032 details 2](details-02.md)
