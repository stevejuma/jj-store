---
schema: 1
id: E-UI-091
---

# E-UI-091

<!-- evidence-field:id markdown -->
## Id

E-UI-091
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4.10
- M13.4
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-03
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

rich-history-search-main-integration-and-workspace-retirement
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change suvkzynx
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Both main and temporary checkouts are clean. App main wpmoyoqz/50363aec includes newer Appearance, Projects Agents and automatic refresh; verified rich-search lsmrmlpp/979d0b03 forks yyknxszk/36c92787. Design Core main quoystmy/bb40d39d includes tooltip restoration, while custom language ookknysq/b0a8c96e forks zukowoxl/4922feea. Rich-search E-UI-056 collides with existing main Appearance evidence. User authorizes main commits and removal of only the two rich-search workspaces; Design Core visual tests stay excluded.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-history-query/StoreHistoryQuery.stories.svelte
- catalog/fixtures/HistoryQueryFixture.svelte
- crates/jj-store-cli/tests/local_browser/history_query.rs
- crates/jj-store-cli/tests/local_browser.rs
- crates/jj-store-cli/tests/native_lifecycle/graph_cache.rs
- crates/jj-store-cli/tests/native_lifecycle/graphs.rs
- crates/jj-store-cli/tests/native_lifecycle/history_query.rs
- crates/jj-store-cli/tests/native_lifecycle/http_browsing.rs
- crates/jj-store-cli/tests/native_lifecycle/http_reads.rs
- crates/jj-store-http/src/dto.rs
- crates/jj-store-http/src/error.rs
- crates/jj-store-http/src/routes.rs
- crates/jj-store-jj/src/local_browser/graph_types.rs
- crates/jj-store-jj/src/local_browser/graphs.rs
- crates/jj-store-jj/src/local_browser/mod.rs
- crates/jj-store-jj/src/local_browser/native.rs
- crates/jj-store-jj/src/query/graphs.rs
- crates/jj-store-jj/src/query/language/aliases.rs
- crates/jj-store-jj/src/query/language/callables.json
- crates/jj-store-jj/src/query/language/mod.rs
- crates/jj-store-jj/src/query/language/operations.rs
- crates/jj-store-jj/src/query/mod.rs
- package.json
- pnpm-lock.yaml
- schemas/openapi.json
- scripts/check-boundaries.mjs
- scripts/contracts.types.ts
- scripts/governance.test.mjs
- spec/src/api.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/compatibility.md
- spec/src/jj-integration.md
- spec/src/local-jj.md
- spec/src/milestones.md
- spec/src/spec-governance.md
- spec/src/ui.md
- spec/src/verification.md
- web/src/lib/api/api-error.ts
- web/src/lib/api/client.ts
- web/src/lib/api/generated.ts
- web/src/lib/api/local-client.ts
- web/src/lib/controller/contracts.ts
- web/src/lib/controller/history-query-route.ts
- web/src/lib/controller/history-query-types.ts
- web/src/lib/controller/history-query.svelte.ts
- web/src/lib/controller/http-local-code-data-source.ts
- web/src/lib/controller/http-repository-transports.ts
- web/src/lib/controller/http-store-data-source.ts
- web/src/lib/controller/native-data-source.ts
- web/src/lib/controller/production-store-controller.svelte.ts
- web/src/lib/controller/production-store-lifecycle.svelte.ts
- web/src/lib/controller/production-store-loaders.ts
- web/src/lib/controller/production-store-reader.svelte.ts
- web/src/lib/controller/production-store-state.svelte.ts
- web/src/lib/controller/production-store-view.ts
- web/src/lib/controller/store-error.ts
- web/src/lib/controller/store-route-codec.ts
- web/src/lib/controller/store-route-fields.ts
- web/src/lib/controller/store-route-history.svelte.ts
- web/src/lib/controller/store-route-types.ts
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryLog.svelte
- web/src/lib/ui/store-history-query/StoreHistoryQuery.svelte
- web/src/lib/ui/store-history-query/history-language.ts
- web/src/lib/ui/store-history-query/history-query.css
- web/tests/browser/history-query.spec.ts
- web/tests/store-route-codec.test.ts
- web/tests/vite/history-language.test.ts
- web/tests/vite/history-query-transport.test.ts
- web/tests/vite/history-query.test.ts
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-UI-091 details 2](details-02.md)
- [E-UI-091 details 3](details-03.md)
