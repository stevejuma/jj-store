---
schema: 1
id: E-UI-011
---

# E-UI-011 details 2

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
temporary pnpm add /tmp/jj-store-design-core-a11y/lapismd-design-core-0.1.2.tgz; pnpm web:build; focused mobile Playwright; full Playwright; restore manifest/lockfile
```

**Notes**

Packed Design Core source at fa2d34e5 passes the exact mobile accessibility regression 1/1 and all eight production browser scenarios. package.json, pnpm-lock.yaml and registry @lapismd/design-core@0.2.0 were restored afterward; no local dependency remains.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-application/RepositoryReview.stories.svelte
```

**Notes**

After switching shell bodies to independent regions, two story assertions still expected the removed body ScrollArea. Product behavior was retained and the assertions were updated to verify fixed body overflow and column-owned scrolling.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-application/RepositoryReview.stories.svelte; pnpm check:architecture
```

**Notes**

All eight focused interactions passed; architecture then found growth in two frozen hotspots. The shared assertion and hidden-title style were extracted without raising budgets.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-application/RepositoryReview.stories.svelte; pnpm check:architecture
```

**Notes**

All eight shell/repository interaction and accessibility stories pass. Body titles remain accessibility-only, Branches/JJ Log/Op Log have no body scroller, and the 124-module architecture budget passes.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:test
```

**Notes**

The first focused unit run exposed that Node tsx cannot import TanStack Svelte Query .svelte exports. Loader/presentation tests passed; Query-backed tests moved to the existing Svelte-capable Vitest lane.
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
pnpm web:test; pnpm catalog:test
```

**Notes**

All 13 Node web tests passed. Three Query regressions passed, while the cancellation assertion observed TanStack CancelledError and the file used node:test under Vitest. The test now asserts cancellation semantically with Vitest.
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
pnpm catalog:test; pnpm catalog:check; pnpm check:architecture
```

**Notes**

All 27 Vite unit tests pass, the catalog stays transport-free with zero Svelte warnings, and the 124-module architecture scan passes.
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
pnpm web:test:browser
```

**Notes**

Seven of eight production browser scenarios pass against registry @lapismd/design-core@0.2.0. The mobile scenario fails only Axe region on the Design Core mobile-stage role=group landmark. The source-owned fix is committed at Design Core fa2d34e5 and passed 8/8 through the temporary package run recorded above; jj-store remains on the released semver dependency.
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
VITEST_MAX_WORKERS=1 pnpm test
```

**Notes**

The first serial aggregate reached Rust assets and exposed cleanup ordering for the new WOFF2 fixture. crates/jj-store-http/tests/assets.rs now removes font-123.woff2 before replacing the fixture directory.
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
cargo test -p jj-store-http --test assets browser_asset_symlinks_are_rejected -- --exact; cargo test -p jj-store-http --test assets
```

**Notes**

The exact Rust asset regression passes 1/1 and the complete asset suite passes 3/3 after the fixture cleanup repair.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
VITEST_MAX_WORKERS=1 pnpm test
```

**Notes**

The second serial aggregate reached Storybook and exposed an asynchronous ProfileLoginFooter sign-in assertion: the menu applies the action after its close transition, so the immediate assertion still observed Guest. The exact Maya Chen expectation was retained and synchronized with close completion.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreRepositoryApplication.stories.svelte -t ProfileLoginFooter; pnpm test:storybook
```

**Notes**

The exact footer regression passes 1/1 and the complete Storybook interaction/accessibility lane passes 46/46 after waiting for sign-in close completion.
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
VITEST_MAX_WORKERS=1 pnpm test
```

**Notes**

The serial aggregate passes governance 21/21, storage-harness 3/3, contracts 6/6, web 13/13, catalog unit 27/27, Storybook 46/46, cold catalog browser 1/1, and the complete Rust workspace with only its declared environment-dependent ignored tests.
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
pnpm build; pnpm check
```

**Notes**

Cold Storybook and production web builds, the locked Rust workspace build, specification validation, package boundaries, 124-module architecture budget, progress governance, generated contracts, Svelte checks and locked workspace Clippy all pass. Existing non-fatal CSS :global and bundle-size warnings remain visible in Vite output.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm format:check; pnpm spec:check
```

**Notes**

Rust and Prettier formatting pass. The canonical spec run passes validation, cold catalog, catalog checks, 27/27 catalog units, 46/46 Storybook interactions/accessibility checks, web checks and 13/13 web units, then stops at the known registry Design Core mobile-stage Axe region violation with production browser acceptance at 7/8.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Registry-only production browser acceptance remains 7/8 until the separately
  approved Design Core mobile-stage fix is released and consumed.
- M4.12 typed hash restoration, M4.13 persistent virtualized layouts and M4.14
  cold real RustFS acceptance remain.
- Consume the mobile-stage fix only after a separately approved Design Core
  registry release.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Registry Design Core ^0.2.0; temporary source acceptance fa2d34e5; @tanstack/svelte-query 6.3.0; Svelte 5.57.1; Storybook 10.5.7; Vitest 4.1.11; Playwright 1.63.x. Enclosing Jujutsu change is the revision reference.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Memory-only repository credentials; session/repository/snapshot/query-scoped cache keys; explicit SSE freshness and refresh. Static bundle adds hashed WOFF2 font assets under the existing self-only CSP. No native object, WAL, storage, credential persistence or HTTP schema changes. Body page titles remain accessibility-only; Column Canvas bodies own vertical scrolling. Stories remain visual-pending and visual tests were skipped at user request.
<!-- /evidence-field -->

