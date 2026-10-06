---
schema: 1
id: E-M9-032
---

# E-M9-032 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

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

The first architecture gate identified the 305-line catalog controller and 330-line HTTP data source. Collaboration, write transport, status and history projection responsibilities were extracted rather than exempted.
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
pnpm exec vitest run --config web/vite.config.ts web/tests/vite/production-store-collaboration.test.ts
```

**Notes**

All four focused collaboration controller cases pass, including the regression that invalidated late AgentStore and ReviewStore detail responses cannot repopulate state.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreCollaboration.stories.svelte
```

**Notes**

The first focused browser run exposed assumptions about prettified JSON text and the Column Canvas public separator role; production behavior was sound and the interactions were corrected to assert accessible semantics.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreCollaboration.stories.svelte
```

**Notes**

All six focused Chromium interaction and automatic accessibility stories pass: agent transcript, keyboard resizing, review reply/resolve, representative loading, empty and failed states. Visual baselines remained skipped and visual-pending.
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
pnpm exec vitest run --config web/vite.config.ts web/tests/vite/store-route-history.test.ts web/tests/vite/production-store-data-source.test.ts web/tests/vite/production-store-events.test.ts
```

**Notes**

All 15 focused route, query and freshness cases pass. Session/thread routes survive reconstruction and simulated history; SSE changes only mark freshness; explicit snapshot refresh invalidates cached collaboration roots.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check; pnpm catalog:check; pnpm check:architecture; node --test scripts/local-package-pnpmfile.test.mjs
```

**Notes**

Web and catalog Svelte diagnostics are clean, 217 governed modules satisfy size/layer/cycle boundaries, catalog boundaries cover 62 modules and all three local-package-hook cases pass.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:validate; pnpm spec:first; pnpm check:progress
```

**Notes**

The first governed documentation run validated all 89 requirements, then required API, architecture, UI and spec-governance coverage for the new package, controller and catalog boundaries. Those canonical chapters were updated rather than suppressing the mapping.
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
pnpm web:build
```

**Notes**

The first production build reached Community's aggregate component export and failed because its unrelated Turso WASM graph uses top-level await that cannot be emitted into Vite's IIFE worker format. Community added focused public message and composer subpaths instead of jj-store importing package internals or carrying a consumer alias.
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
pnpm web:build
```

**Notes**

After consuming the focused Community exports, all 4,072 Vite transforms completed and the static-manifest gate rejected the newly reachable WOFF and TTF fallbacks. The bounded asset contract was extended with explicit media types and a 256-file cap under the existing self-only CSP.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-http --test assets --locked
```

**Notes**

The focused router regression initially rejected WOFF and TTF as unknown browser media while preserving its path, symlink and size denials.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-http --test assets --locked; pnpm web:build
```

**Notes**

All three browser-asset tests pass with explicit WOFF2, WOFF and TTF media, and the complete production Vite bundle plus bounded manifest now builds. Existing Design Core :global selector and bundle-size diagnostics remain warnings.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreCollaboration.stories.svelte
```

**Notes**

The first cold run after refreshing the local Community package exposed Vite eagerly optimizing Community's declared but unused Turso WASM dependency. The catalog adopted Community's matching dependency-optimization exclusion rather than transforming or aliasing the package.
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
node --import tsx --test web/tests/store-route-codec.test.ts; pnpm exec vitest run --config web/vite.config.ts web/tests/vite/production-store-collaboration.test.ts web/tests/vite/store-route-history.test.ts web/tests/vite/production-store-data-source.test.ts web/tests/vite/production-store-events.test.ts; pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreCollaboration.stories.svelte
```

**Notes**

All four route cases, 19 collaboration/query/history/freshness cases and six cold Chromium interaction/accessibility stories pass. Visual tests remain skipped and every changed story remains visual-pending.
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
pnpm web:check; pnpm catalog:check; pnpm check:architecture; pnpm check:boundaries; node --test scripts/local-package-pnpmfile.test.mjs; pnpm spec:validate; pnpm spec:first; pnpm check:progress
```

**Notes**

Web and catalog diagnostics remain clean; 217 governed web modules, 62 catalog modules and 13 packages satisfy boundaries; all three local package-hook cases, 89 requirements, 49 protected-file mappings and 61 progress tasks pass.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

[]
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Node.js 26.9.0; Svelte 5.57.1; TanStack Query 6.3.0; TanStack Virtual 3.13.39; Community presentation inspected at 5691eebd61d2ae387a4cf2fd70af5a525c2bc1e3 with focused public exports accepted at 0df681141dcd6c70c31c1ba796323a7934c3488e through @lapismd/lapis-community@^0.1.0; Design Core source 5596d04ee7b2b6da9b0f3f066c5c0c2db7037f4d through @lapismd/design-core@^0.2.0.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Adds agents and reviews hash routes with full 16-byte stable selection IDs, cursor-paged collaboration HTTP reads, existing ReviewCommand v1 writes, versioned local layout keys and no persisted credentials or server cursors. Production consumes focused Community public component subpaths; catalog fixtures remain isolated. The bounded browser manifest permits at most 256 hashed JS, CSS, WOFF2, WOFF and TTF assets with explicit media types under the existing self-only CSP. No native object or wire format changes.
<!-- /evidence-field -->

