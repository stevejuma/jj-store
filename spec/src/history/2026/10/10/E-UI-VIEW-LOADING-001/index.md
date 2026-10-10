---
schema: 1
id: E-UI-VIEW-LOADING-001
---

# E-UI-VIEW-LOADING-001

<!-- evidence-field:id markdown -->
## Id

E-UI-VIEW-LOADING-001
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-10
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M24
- M4.9
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-ARCH-004
- JST-UI-005
- JST-UI-010
- JST-API-001
- JST-DOC-003
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- .storybook/vite.config.ts
- catalog/components/store-application/AgentToolGroups.stories.svelte
- catalog/components/store-application/LazyViews.stories.svelte
- catalog/components/store-application/RepositoryLayout.stories.svelte
- catalog/components/store-application/RepositoryReview.stories.svelte
- catalog/components/store-application/agent-comment-replies-play.ts
- catalog/components/store-application/agent-tool-duration-play.ts
- catalog/components/store-application/column-canvas-story-assertions.ts
- catalog/components/store-application/repository-directory-story-assertions.ts
- catalog/components/store-stack-workspace/conflict-story-actions.ts
- catalog/fixtures/LazyViewContent.svelte
- catalog/fixtures/LazyViewFixture.svelte
- docs/maintenance/coverage.json
- docs/site/getting-started/install.md
- docs/site/guides/browsing.md
- docs/ui-provenance.json
- pnpm-lock.yaml
- scripts/local-package-pnpmfile.cjs
- scripts/local-package-pnpmfile.test.mjs
- scripts/view-loading-assets.test.mjs
- scripts/view-module-urls.mjs
- scripts/view-module-urls.test.mjs
- scripts/web-manifest.mjs
- scripts/web-manifest.test.mjs
- spec/src/current/contracts/api/26-browser-appearance-state.md
- spec/src/current/contracts/architecture/06-browser-presentation-ownership.md
- spec/src/current/contracts/documentation/03-delivery-and-validation.md
- spec/src/current/contracts/ui/07-application-startup.md
- spec/src/current/governance/spec-governance.md
- spec/src/current/progress/M24/SUMMARY.index-index/002/index.md
- spec/src/current/progress/M24/SUMMARY.nav.md
- spec/src/current/progress/M24/history/M24/index.md
- spec/src/current/progress/M24/index.md
- spec/src/current/progress/M24/view-loading-review.md
- spec/src/current/progress/M4/history/M4.9/index.md
- spec/src/current/progress/M4/index.md
- spec/src/history/2026/10/10/E-UI-VIEW-LOADING-001/SUMMARY.nav.md
- spec/src/history/2026/10/10/E-UI-VIEW-LOADING-001/details-02.md
- spec/src/history/2026/10/10/E-UI-VIEW-LOADING-001/details-03.md
- spec/src/history/2026/10/10/E-UI-VIEW-LOADING-001/details-04.md
- spec/src/history/2026/10/10/E-UI-VIEW-LOADING-001/index.md
- spec/src/history/2026/10/10/SUMMARY.nav.md
- spec/src/history/2026/10/10/index.md
- web/src/lib/ui/store-application/StoreApplication.svelte
- web/src/lib/ui/store-application/StoreApplicationAgents.svelte
- web/src/lib/ui/store-application/StoreApplicationInspectionPanels.svelte
- web/src/lib/ui/store-application/StoreApplicationMain.svelte
- web/src/lib/ui/store-application/StoreApplicationOverlays.svelte
- web/src/lib/ui/store-application/StoreApplicationRightPanels.svelte
- web/src/lib/ui/store-application/StoreLazyView.svelte
- web/src/lib/ui/store-application/StoreRepositoryApplication.svelte
- web/src/lib/ui/store-application/application-views.ts
- web/src/lib/ui/store-application/lazy-view.ts
- web/tests/browser/lazy-views.spec.ts
- web/tests/lazy-view.test.ts
- web/vite.config.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/lazy-view.test.ts#view imports wait for demand and share completion
  across consumers
- web/tests/lazy-view.test.ts#a failed view import rejects all consumers and
  allows a fresh explicit retry
- scripts/view-module-urls.test.mjs#development prepares multiline view URLs
  with trailing commas and HMR queries
- scripts/view-module-urls.test.mjs#every application view receives a compiler
  URL before development evaluation
- scripts/view-module-urls.test.mjs#catalog retry URLs use the catalog serving
  root and the same owned component
- scripts/view-module-urls.test.mjs#built retry URLs use emitted entries and
  resolve relative to their importing chunk
- scripts/view-module-urls.test.mjs#view URL preparation rejects paths outside
  the application or non-component entries
- scripts/view-loading-assets.test.mjs#built browser defers page views and
  admits their complete dynamic asset closure
- scripts/web-manifest.test.mjs#browser manifest includes Vite's unsplit
  application stylesheet
- web/tests/browser/lazy-views.spec.ts#a failed view chunk offers explicit retry
  without reloading or replaying commands
- web/tests/browser/lazy-views.spec.ts#cold built shell loads selected views on
  demand and reuses completed imports
- web/tests/browser/lazy-views.spec.ts#navigation away from a delayed view
  cannot render its obsolete completion
- scripts/local-package-pnpmfile.test.mjs#local package hook uses the packed
  Design Core artifact without rewriting peers
- scripts/local-package-pnpmfile.test.mjs#local package hook retains registry
  range when only sibling source exists
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-UI-VIEW-LOADING-001 details 2](details-02.md)
- [E-UI-VIEW-LOADING-001 details 3](details-03.md)
- [E-UI-VIEW-LOADING-001 details 4](details-04.md)
