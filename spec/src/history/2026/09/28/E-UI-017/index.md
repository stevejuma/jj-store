---
schema: 1
id: E-UI-017
---

# E-UI-017

<!-- evidence-field:id markdown -->
## Id

E-UI-017
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4
- M4.3
- M4.15
- M6
- M6.2
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
- README.md
- catalog/components/store-application/StoreConnection.stories.svelte
- catalog/components/store-application/RepositoryReview.stories.svelte
- catalog/components/store-application/StoreRepositorySearch.stories.svelte
- catalog/components/store-search/StoreSearchCommand.stories.svelte
- catalog/controller/in-memory-store-controller.svelte.ts
- catalog/controller/in-memory-store-search.svelte.ts
- catalog/fixtures/StoreSearchCommandFixture.svelte
- catalog/fixtures/search.ts
- docs/browser.md
- docs/catalog.md
- docs/implementation-plan.md
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
- web/src/lib/controller/production-store-controller.svelte.ts
- web/src/lib/controller/production-store-search-controller.ts
- web/src/lib/controller/production-store-search.ts
- web/src/lib/controller/production-store-state.svelte.ts
- web/src/lib/controller/production-store-view.ts
- web/src/lib/controller/store-query-client.ts
- web/src/lib/ui/community-navigation/CommunityNavigation.svelte
- web/src/lib/ui/community-navigation/CommunityNavigationSearch.svelte
- web/src/lib/ui/store-application/StoreApplication.svelte
- web/src/lib/ui/store-search/StoreSearchCommand.svelte
- web/src/lib/ui/store-search/StoreSearchOverlay.svelte
- web/src/lib/ui/store-search/store-search-command.css
- web/tests/browser/native-browser-fixture.ts
- web/tests/browser/search.spec.ts
- web/tests/production-store-search.test.ts
- web/tests/real/native-repository.spec.ts
- web/tests/vite/production-store-data-source.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/production-store-search.test.ts#ignores a late response from an
  older query
- web/tests/production-store-search.test.ts#continues with the opaque cursor and
  deduplicates overlapping pages
- web/tests/production-store-search.test.ts#presents cursor reset failures
  without retaining a refreshing state
- web/tests/vite/production-store-data-source.test.ts#temporary repository busy
  responses are retried
- web/tests/browser/search.spec.ts#command search opens native results and
  routes through stable identities
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-search/StoreSearchCommand.stories.svelte catalog/components/store-application/StoreRepositorySearch.stories.svelte
```

**Notes**

The initial focused run passed three stories and found the unavailable dialog close assertion observing its exit transition. The assertion now verifies the authoritative closed state without changing the component behavior.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:architecture; pnpm web:check; pnpm catalog:check; pnpm exec tsx --test web/tests/production-store-search.test.ts; pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-store-data-source.test.ts
```

**Notes**

The 159-module architecture guard, production/catalog diagnostics, six search controller regressions and seven cached data-source tests pass.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-search/StoreSearchCommand.stories.svelte catalog/components/store-application/StoreRepositorySearch.stories.svelte
```

**Notes**

All four focused Command View interaction and accessibility stories pass, including loading, unavailable, paging, result selection and full-app keyboard navigation.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:test:browser --grep command search opens native results and routes through stable identities
```

**Notes**

The built captured-response browser opens search from the sidebar and Control+K, navigates commit and file results with full native identities, and keeps queries/cursors out of hashes.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm dev:restart; curl browser root; query authenticated search status and search
```

**Notes**

The single pnpm task rebuilt the UI/native binaries, replaced only the labelled RustFS container and recorded server, seeded a fresh repository, restarted with its Tantivy worker, waited for indexed position 11 and returned a non-stale commit/operation/file result set.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec playwright test --config web/playwright.real.config.ts
```

**Notes**

Real native search passed 1/1. The existing refresh case then exposed one transient 503 busy response while Tantivy processed the external publication; the query policy had incorrectly treated every typed API error as permanent.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-store-data-source.test.ts; pnpm dev:restart; pnpm exec playwright test --config web/playwright.real.config.ts
```

**Notes**

The data-source lane passes 7/7 and the fresh built native lane passes 2/2 after retrying only the explicit transient busy code. Authorization and semantic API errors remain non-retrying.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools VITEST_MAX_WORKERS=1 pnpm spec:check
```

**Notes**

The first aggregate reached 60/61 passing Storybook interactions. The existing shell refinement still required the navigation header to be absent, while the reviewed Community search composition intentionally adds a search-only header. The assertion now excludes the removed brand while requiring the search trigger.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/RepositoryReview.stories.svelte
```

**Notes**

All three focused shell refinement stories pass after retaining the removed body branding while accepting the sidebar search trigger.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools VITEST_MAX_WORKERS=1 pnpm spec:check
```

**Notes**

All 18 canonical specification lanes pass, including architecture, generated contracts, production and catalog diagnostics, 61 Storybook interaction/accessibility stories, cold catalog construction, browser acceptance and the complete locked Rust workspace.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check
```

**Notes**

Specification validation, source boundaries, the 159-module architecture budget, progress bookkeeping, rustfmt, generated contracts, production/catalog diagnostics and locked workspace Clippy all pass.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools VITEST_MAX_WORKERS=1 pnpm test
```

**Notes**

The complete non-visual test gate passes: 23 governance, three storage-harness, six contract, 29 production web, 34 catalog unit, 61 Storybook interaction/accessibility and the complete locked Rust workspace tests, plus cold catalog browser acceptance.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build
```

**Notes**

The cold Storybook catalog, production browser bundle and complete locked Rust workspace build successfully. Existing Design Core :global selector and bundle-size diagnostics remain warnings rather than failures.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first; pnpm check:progress; pnpm format:check
```

**Notes**

Final protected-file mapping, milestone bookkeeping, rustfmt and Prettier checks pass after recording the aggregate outcomes.
<!-- /run -->
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-UI-017 details 2](details-02.md)
