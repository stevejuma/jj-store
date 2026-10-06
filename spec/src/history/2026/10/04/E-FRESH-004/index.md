---
schema: 1
id: E-FRESH-004
---

# E-FRESH-004

<!-- evidence-field:id markdown -->
## Id

E-FRESH-004
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Scoped browser event consumers and hidden projection dirtiness
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Node 26.9.0; pnpm 10.32.1; Vitest 4.1.11; TanStack Svelte Query 6.3.0
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Application SSE V1; browser session-only dirty journals and independent query leases; historical pins and durable command/receipt formats unchanged.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- docs/browser.md
- docs/implementation-plan.md
- docs/maintenance/coverage.json
- docs/site/guides/browsing.md
- docs/site/guides/capture.md
- docs/site/guides/settings.md
- docs/site/reference/api/installation.md
- docs/site/reference/api/types.md
- docs/site/reference/transport.md
- docs/site/technical/architecture.md
- spec/src/ui.md
- web/src/lib/api/application-events.ts
- web/src/lib/controller/application-freshness.ts
- web/src/lib/controller/automatic-refresh.ts
- web/src/lib/controller/capture-configuration-observer.ts
- web/src/lib/controller/capture-management.svelte.ts
- web/src/lib/controller/capture-observations.ts
- web/src/lib/controller/capture-refresh.ts
- web/src/lib/controller/contracts.ts
- web/src/lib/controller/freshness-journal.ts
- web/src/lib/controller/production-automatic-refresh.ts
- web/src/lib/controller/production-capture-management.ts
- web/src/lib/controller/production-index-preparation.ts
- web/src/lib/controller/production-local-configuration.ts
- web/src/lib/controller/production-projects-hub.svelte.ts
- web/src/lib/controller/production-projects-integration.ts
- web/src/lib/controller/production-read-services.ts
- web/src/lib/controller/production-search-settings.ts
- web/src/lib/controller/production-session-events.ts
- web/src/lib/controller/production-store-connect.ts
- web/src/lib/controller/production-store-controller.svelte.ts
- web/src/lib/controller/production-store-index.ts
- web/src/lib/controller/production-store-lifecycle.svelte.ts
- web/src/lib/controller/production-store-reader.svelte.ts
- web/src/lib/controller/production-store-refresh.ts
- web/src/lib/controller/production-store-view.ts
- web/src/lib/controller/projects-activity.svelte.ts
- web/src/lib/controller/projects-agent-scope.ts
- web/src/lib/controller/projects-agents.svelte.ts
- web/src/lib/controller/projects-auxiliary-restore.ts
- web/src/lib/controller/projects-command-state.ts
- web/src/lib/controller/projects-dirtiness.ts
- web/src/lib/controller/projects-hub-types.ts
- web/src/lib/controller/projects-navigation.svelte.ts
- web/src/lib/controller/projects-refresh-scope.ts
- web/src/lib/controller/projects-related-views.ts
- web/src/lib/controller/projects-repositories.svelte.ts
- web/src/lib/controller/projects-reviews.svelte.ts
- web/src/lib/controller/repository-thread-navigation.svelte.ts
- web/src/lib/controller/settings-refresh.ts
- web/src/lib/ui/store-application/StoreApplication.svelte
- web/src/lib/ui/store-settings/StoreLocalFolders.svelte
- web/src/lib/ui/store-settings/StoreRepositoryConversion.svelte
- web/src/lib/ui/store-settings/StoreSearchSettings.svelte
- web/src/lib/ui/store-settings/StoreSettings.svelte
- web/src/lib/ui/store-settings/StoreSettingsRepository.svelte
- web/src/lib/ui/store-threads/thread-navigation-binding.svelte.ts
- web/tests/real/event-freshness.spec.ts
- web/tests/vite/application-freshness.test.ts
- web/tests/vite/index-freshness.test.ts
- web/tests/vite/projects-freshness.test.ts
- web/tests/vite/settings-freshness.test.ts
- spec/src/documentation.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/vite/application-freshness.test.ts#healthy idle and unchanged
  five-minute renewal perform zero consumer reads
- web/tests/vite/projects-freshness.test.ts#hidden Agents projections refresh
  only their dirty repository on return
- web/tests/vite/index-freshness.test.ts#completion before start response is
  observed at the exact retained operation
- web/tests/vite/settings-freshness.test.ts#storage events read storage without
  refetching installation configuration or other sections
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-FRESH-004 details 2](details-02.md)
