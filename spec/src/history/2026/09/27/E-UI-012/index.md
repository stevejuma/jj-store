---
schema: 1
id: E-UI-012
---

# E-UI-012

<!-- evidence-field:id markdown -->
## Id

E-UI-012
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4
- M4.3
- M4.12
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-27
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

implementation
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- .pnpmfile.cjs
- catalog/components/store-application/StoreConnection.stories.svelte
- catalog/controller/in-memory-store-controller.svelte.ts
- package.json
- scripts/local-package-pnpmfile.cjs
- scripts/local-package-pnpmfile.test.mjs
- spec/src/api.md
- spec/src/architecture.md
- spec/src/milestones.md
- spec/src/spec-governance.md
- spec/src/ui.md
- spec/src/verification.md
- web/src/App.svelte
- web/src/lib/controller/contracts.ts
- web/src/lib/controller/production-route-restorer.ts
- web/src/lib/controller/production-store-controller.svelte.ts
- web/src/lib/controller/production-store-events.ts
- web/src/lib/controller/production-store-reader.svelte.ts
- web/src/lib/controller/store-route-codec.ts
- web/src/lib/controller/store-route-history.svelte.ts
- web/src/lib/ui/community-app-shell/CommunityAppShell.svelte
- web/src/lib/ui/store-application/StoreConnection.svelte
- web/tests/browser/native-browser-fixture.ts
- web/tests/browser/navigation.spec.ts
- web/tests/browser/reads.spec.ts
- web/tests/store-route-codec.test.ts
- web/tests/vite/store-route-history.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- scripts/local-package-pnpmfile.test.mjs#local package hook uses available
  Design Core source
- scripts/local-package-pnpmfile.test.mjs#local package hook retains registry
  range when source is absent
- web/tests/store-route-codec.test.ts#native operation route hash round-trips
  every stable selection
- web/tests/store-route-codec.test.ts#repository paths and bookmark names remain
  safely encoded
- web/tests/store-route-codec.test.ts#malformed, duplicate, foreign and unsafe
  route data fails closed
- web/tests/vite/store-route-history.test.ts#route history restores, pushes and
  replaces native browser hashes
- web/tests/vite/store-route-history.test.ts#external browser navigation
  restores known and newly entered routes
- web/tests/vite/store-route-history.test.ts#invalid owned hashes remain
  explicit and never change selection
- web/tests/browser/navigation.spec.ts#hash route restores snapshot and file
  through controls reload and browser history
- web/tests/browser/navigation.spec.ts#malformed owned route is explicit before
  authentication
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
pnpm exec tsx --test web/tests/store-route-codec.test.ts
```

**Notes**

The new snapshot-bound selection regression failed before the parser change because a bookmark without at_op was accepted. The exact assertion was retained.
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
pnpm exec tsx --test web/tests/store-route-codec.test.ts; pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/store-route-history.test.ts
```

**Notes**

Codec acceptance passes 3/3 and browser-history acceptance passes 3/3 after enforcing snapshot-bound and page-compatible selections.
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
pnpm web:build && node scripts/browser-fixtures.mjs && pnpm exec playwright test --config web/playwright.config.ts navigation.spec.ts
```

**Notes**

Both cold built navigation scenarios pass: Back/Forward, reload and reconnect restore the selected source file; credentials stay out of the URL/storage; explicit SSE refresh preserves the exact deep hash; malformed routes remain explicit.
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
pnpm web:test:browser
```

**Notes**

Nine of ten production browser scenarios pass. The sole failure is the previously recorded Axe region violation in registry Design Core 0.2.0 mobile-stage role=group. The source-owned fix remains committed at Design Core fa2d34e5 and no local package substitution is retained.
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
node --test scripts/local-package-pnpmfile.test.mjs
```

**Notes**

The exact hook regression failed before implementation because the local package hook module did not exist. The local and fallback assertions were retained.
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
node --test scripts/local-package-pnpmfile.test.mjs; pnpm install --lockfile=false --force
```

**Notes**

Both hook regressions pass. The install resolves Design Core to ../design-core when present, preserves the package manifest range and peer ranges, and leaves the registry pnpm-lock.yaml byte-for-byte unchanged.
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
pnpm web:test:browser
```

**Notes**

All ten cold built production browser scenarios pass against local Design Core source fa2d34e5, including the mobile accessibility regression and both typed hash navigation scenarios.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:check
```

**Notes**

Every executable canonical lane passed, then spec-first required the new package/install hook boundary in api.md, architecture.md and spec-governance.md. The implementation and tests were retained and all three mapped chapters were updated.
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
pnpm test:governance; pnpm test:storybook
```

**Notes**

Governance passes 23/23, including both local package hook branches, and Storybook interaction/accessibility passes 47/47 against the local Design Core source.
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
VITEST_MAX_WORKERS=1 pnpm test
```

**Notes**

The serial aggregate passes governance 23/23, storage harness 3/3, contracts 6/6, web 16/16, catalog unit 30/30, Storybook 47/47, cold catalog browser 1/1 and the complete Rust workspace with only its declared environment-dependent ignored tests.
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
pnpm build; pnpm check
```

**Notes**

Cold Storybook, production browser and locked Rust builds pass. Specification validation, boundaries, 128-module architecture, progress, Rust formatting, generated contracts, web/catalog Svelte checks and locked workspace Clippy also pass. Existing non-fatal CSS :global and bundle-size warnings remain visible.
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
pnpm spec:first; pnpm spec:check
```

**Notes**

Spec-first passes for 20 protected files across five canonical chapters, and all 18 canonical lanes pass against local Design Core source, including cold catalog, interaction/accessibility, production browser and Rust workspace acceptance.
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
pnpm format:check; pnpm check:progress; pnpm spec:validate; pnpm spec:first
```

**Notes**

Rust and Prettier formatting, 43-task progress governance, all 65 canonical requirements and the five-chapter spec-first mapping pass. Visual tests and baseline updates were skipped at the user's request.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Registry-only production browser acceptance remains 9/10 until the separately
  approved Design Core mobile-stage fix is released and consumed.
- M4.13 persistent virtualized layouts and M4.14 cold real RustFS acceptance
  remain.
- Consume the mobile-stage fix only after a separately approved Design Core
  registry release.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Registry Design Core ^0.2.0 with guarded local source resolution at fa2d34e5 when ../design-core is present; @tanstack/svelte-query 6.3.0; Svelte 5.57.1; Storybook 10.5.7; Vitest 4.1.11; Playwright 1.63.x. Enclosing Jujutsu change is the revision reference.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-UI-012 details 2](details-02.md)
