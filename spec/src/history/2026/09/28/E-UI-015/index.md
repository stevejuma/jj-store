---
schema: 1
id: E-UI-015
---

# E-UI-015

<!-- evidence-field:id markdown -->
## Id

E-UI-015
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4
- M4.3
- M4.11
- M4.13
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

- catalog/components/community-projects/CommunityProjectsRepositoryLogFiles.stories.svelte
- catalog/controller/in-memory-store-controller.svelte.ts
- spec/src/api.md
- spec/src/milestones.md
- spec/src/ui.md
- spec/src/verification.md
- web/src/lib/controller/contracts.ts
- web/src/lib/controller/production-inspection-pages.ts
- web/src/lib/controller/production-snapshot.ts
- web/src/lib/controller/production-store-controller.svelte.ts
- web/src/lib/controller/production-store-loaders.ts
- web/src/lib/controller/production-store-reader.svelte.ts
- web/src/lib/controller/production-store-state.svelte.ts
- web/src/lib/controller/production-store-view.ts
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryLog.svelte
- web/src/lib/ui/store-application/StoreRepositoryApplication.svelte
- web/tests/browser/native-browser-fixture.ts
- web/tests/browser/reads.spec.ts
- web/tests/production-inspection-pages.test.ts
- web/tests/production-store-loaders.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/production-inspection-pages.test.ts#commit inspection continues
  opaque changed-path pages
- web/tests/production-inspection-pages.test.ts#operation inspection pages
  groups and every exact comparison
- web/tests/browser/reads.spec.ts#changed-file collections consume commit, group
  and path cursors
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
pnpm spec:first
```

**Notes**

Spec-first validation required the existing operation-change and diff-path cursor contract to be mapped explicitly in api.md. The canonical API chapter now states the independent full-identity and snapshot bindings consumed by the browser.
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
pnpm exec tsx --test web/tests/production-inspection-pages.test.ts web/tests/production-store-loaders.test.ts
```

**Notes**

All eight focused loader tests pass, including opaque commit path continuation, operation-group continuation, every exact group's path continuation, comparison mapping and snapshot mismatch rejection.
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
pnpm web:check; pnpm catalog:check; pnpm check:architecture
```

**Notes**

Production and catalog diagnostics passed. Architecture found the production and in-memory controller composition roots at 306 and 301 counted lines after adding detail continuation state. Production state projection was extracted and fixture imports consolidated without raising the 300-line budget.
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
pnpm web:test; pnpm check:architecture
```

**Notes**

All 23 production browser units pass and the 147-module architecture boundary passes after extracting the production state view.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 6
hasNote: true
noteEncoding: markdown

```

```shell
pnpm format:check
```

**Notes**

Rustfmt passed; Prettier identified six newly authored or changed TypeScript/Svelte files. They were formatted without changing behavior.
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
pnpm web:build; DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm exec playwright test --config web/playwright.config.ts --grep changed-file collections consume
```

**Notes**

Commit continuation passed. The operation assertion reused the cached commit comparison because the fixture supplied the same full from/to identities, then expected a second network-only filename. The fixture now uses distinct native comparisons for its second operation group and retains the cache behavior.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm exec playwright test --config web/playwright.config.ts --grep changed-file collections consume
```

**Notes**

The cold built-browser regression passes 1/1: virtual changed-file rows request the commit path cursor, the operation change-group cursor and the exact second group's changed-path comparison, then render every returned file.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/community-projects/CommunityProjectsRepositoryLogFiles.stories.svelte
```

**Notes**

All four focused changed-file stories pass interaction and accessibility checks, including the virtual end-of-list continuation callback.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools VITEST_MAX_WORKERS=1 pnpm spec:check
```

**Notes**

All 18 canonical lanes pass, including 23 browser units, 32 catalog units, 57 Storybook interaction/accessibility cases, the cold catalog contract, 12 cold built-browser acceptance cases, the locked Rust workspace and mdBook. Declared external RustFS and OS-credential cases remain ignored in the normal lane.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check
```

**Notes**

Specification validation, package boundaries, the 147-module architecture budget, progress bookkeeping, rustfmt, 51 captured HTTP/SSE responses, six contract tests, production and catalog Svelte diagnostics, catalog boundaries and locked-workspace clippy all pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools VITEST_MAX_WORKERS=1 pnpm test
```

**Notes**

The aggregate passes 23 governance tests, three storage-harness self-tests, six contract tests, 23 browser units, 32 catalog units, 57 Storybook interaction/accessibility cases, the cold catalog contract and the complete locked Rust workspace. Declared external-service cases remain ignored.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build
```

**Notes**

The cold Storybook catalog, production browser bundle and complete locked Rust workspace build successfully. Existing Design Core :global selector and bundle-size diagnostics remain warnings rather than failures.
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
pnpm format:check
```

**Notes**

Rustfmt and Prettier pass after the final evidence update.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

The progress guard rejected a pass evidence state while Design Core registry integration, M4.14 real acceptance and the deferred controls remain listed. The verified slice keeps an explicit pending evidence state until those gaps close.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Consume the Design Core lazy listing extension from a separately approved
  registry release; package.json and pnpm-lock.yaml remain registry-resolved.
- Run M4.14 external CLI publication, SSE notification and explicit-refresh
  acceptance against the isolated native repository and pinned RustFS.
- Continue the separately deferred mutation, conflict-editing and repository
  search controls after browsing acceptance.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Registry Design Core ^0.2.0 with guarded local source validation at c7937794, 4725befb and 6f8ee457; @tanstack/svelte-query 6.3.0; Svelte 5.57.1; Storybook 10.5.7; Vitest 4.1.11; Playwright 1.63.x. Enclosing Jujutsu change is the revision reference.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-UI-015 details 2](details-02.md)
