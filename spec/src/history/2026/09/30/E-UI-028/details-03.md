---
schema: 1
id: E-UI-028
---

# E-UI-028 details 3

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:build
```

**Notes**

The production web bundle rebuilt with deferred search selection and the same preventScroll=false dialog policy used by the other application modal surfaces; only existing Lightning CSS :global and bundle-size warnings were emitted.
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
pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/mutations.spec.ts web/tests/browser/reads.spec.ts web/tests/browser/search.spec.ts
```

**Notes**

All 17 focused browser cases pass against the rebuilt bundle, covering body unlock and working clicks after search selection, exact mutation capture, renamed-file refresh and commit history, and route-retained multiple-head details.
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
pnpm web:test:browser
```

**Notes**

The complete cold lane rebuilt web/dist, recaptured native HTTP/SSE and mutation fixtures, and passed all 21 Chromium browser tests.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
PATH=<scratch-cmake> CMAKE=<scratch-cmake> BOOST_ROOT=<scratch-boost> CC=<xcode-wrapper> CXX=<xcode-wrapper> pnpm spec:check
```

**Notes**

Specification validation, the cold catalog browser and the 181-test native lifecycle binary passed before the Rust aggregate contract lane found one stale mutation-union cardinality assertion: edit_file increased the advertised typed commands from 8 to 9.
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
PATH=<scratch-cmake> CMAKE=<scratch-cmake> BOOST_ROOT=<scratch-boost> CC=<xcode-wrapper> CXX=<xcode-wrapper> cargo test --locked -p jj-store-http contract::tests::mutation_contracts_require_typed_intent_and_write_permission -- --exact
```

**Notes**

The exact OpenAPI regression passes with all 9 typed mutation command variants present.
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
PATH=<scratch-cmake> CMAKE=<scratch-cmake> BOOST_ROOT=<scratch-boost> CC=<xcode-wrapper> CXX=<xcode-wrapper> pnpm check
```

**Notes**

All preceding gates passed before the architecture budget found RepositoryReview.stories.svelte at 514 counted lines against its established 513-line limit. Compacting one multiline story assignment restored the existing budget without changing behavior.
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
pnpm check:architecture
```

**Notes**

All 250 production and catalog modules pass ownership, import and line-budget checks after the story compaction.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
PATH=<scratch-cmake> CMAKE=<scratch-cmake> BOOST_ROOT=<scratch-boost> CC=<xcode-wrapper> CXX=<xcode-wrapper> pnpm check
```

**Notes**

The complete required check lane passes specification validation, boundaries, architecture, progress, formatting, embedded RustFS clippy, 70 generated HTTP/SSE captures, both Svelte projects and workspace clippy.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
PATH=<scratch-cmake> CMAKE=<scratch-cmake> BOOST_ROOT=<scratch-boost> CC=<xcode-wrapper> CXX=<xcode-wrapper> pnpm test
```

**Notes**

The aggregate reached test:embedded-rustfs before rustfs-ecstore compilation failed with No space left on device (os error 28). Only this workspace's ignored embedded RustFS target and incremental cache were removed before retrying; the active original workspace was not changed.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:architecture
```

**Notes**

The Files container and selected-file preview exceeded the 300-line production budget after the requested breadcrumb and header controls were added. The breadcrumb and metadata header were extracted into focused production components.
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
pnpm web:check && pnpm catalog:check && pnpm check:architecture
```

**Notes**

Both Svelte projects, catalog boundaries and all 255 production architecture modules pass after the breadcrumb and preview metadata header extractions.
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
node --test scripts/dev-demo-repository.test.mjs
```

**Notes**

The isolated deterministic seed regression creates nonempty README, CODE_OF_CONDUCT, CONTRIBUTING, LICENSE and SECURITY root documents.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/community-projects/CommunityProjectsRepositoryDirectory.stories.svelte catalog/components/store-application/RepositoryReview.stories.svelte catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-application/StoreFileEditor.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte catalog/components/store-application/StoreRepositorySearch.stories.svelte
```

**Notes**

Sixteen of 17 focused interaction/accessibility stories passed. The remaining directory story still expected the file-tree heading to align with the breadcrumb that now begins beyond the left sidebar.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/RepositoryReview.stories.svelte --testNamePattern RepositoryDirectoryNavigation && pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/community-projects/CommunityProjectsRepositoryDirectory.stories.svelte catalog/components/store-application/RepositoryReview.stories.svelte catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-application/StoreFileEditor.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte catalog/components/store-application/StoreRepositorySearch.stories.svelte
```

**Notes**

The exact geometry regression and all 17 focused Chromium interaction/accessibility stories pass. Coverage includes the Files-only borderless breadcrumb, no breadcrumb on Activity, Workspaces, Review or Commits, the far-left tree toggle, icon-only edit control and breadcrumb filename input. Stories remain visual-pending.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 6
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:build && pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/mutations.spec.ts web/tests/browser/navigation.spec.ts web/tests/browser/reads.spec.ts web/tests/browser/search.spec.ts
```

**Notes**

The bundle built and 13 of 19 browser cases passed. Six cases retained navigation through the removed application-header repository link and timed out before reaching their assertions.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/mutations.spec.ts web/tests/browser/navigation.spec.ts web/tests/browser/reads.spec.ts web/tests/browser/search.spec.ts
```

**Notes**

All 19 focused production-browser cases pass after entering the file browser through the Files tab, including native edit/rename, route restoration, repository reads and search-result interactivity.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first
```

**Notes**

The protected package script and production component extraction required the mapped architecture chapter to record the isolated seed-test and browser module boundaries.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first && pnpm check:progress
```

**Notes**

All 50 protected files map to nine current canonical chapters, and the evidence ledger covers the complete change.
<!-- /run -->
<!-- /evidence-field -->

