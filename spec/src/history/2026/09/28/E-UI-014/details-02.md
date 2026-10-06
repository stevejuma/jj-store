---
schema: 1
id: E-UI-014
---

# E-UI-014 details 2

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
pnpm install --force --offline --frozen-lockfile
```

**Notes**

The guarded local Design Core refresh was rejected with ERR_PNPM_LOCKFILE_CONFIG_MISMATCH because the validation-only file dependency differs from the registry-resolved lock configuration. The release manifest and lockfile were not changed.
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
pnpm install --force --offline --lockfile=false
```

**Notes**

The guarded pnpm hook installed the sibling Design Core source for local acceptance without reading or rewriting the release lockfile.
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
pnpm checks
```

**Notes**

Design Core source acceptance passes specification, formatting and strict type checks, 715 unit tests, the cold static catalog, 638 interaction/accessibility tests, 14 workspace browser tests, 11 shell browser tests, 3 AI browser tests and 35 shadcn browser tests with one declared WebKit-only skip. The public FileListingDirectory barrel correction is committed at 6f8ee457 on top of lazy-directory support 4725befb and cold-docs repair c7937794.
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
pnpm catalog:test web/tests/vite/production-store-data-source.test.ts catalog/components/community-projects/repository-browser.test.ts; pnpm web:test
```

**Notes**

The focused controller and browser-presentation boundary passes 10/10 and the Node browser unit lane passes 21/21, including query forwarding, explicit directories and isolated tree versus recursive pages.
<!-- /run -->

<!-- run -->
### Run 5

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

Production and catalog types passed after the upstream public export refresh. Architecture then found the in-memory and production controllers at 303 and 304 counted lines; existing actions were simplified without raising the 300-line budget.
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
pnpm check:architecture; pnpm web:check; pnpm catalog:check
```

**Notes**

The 144-module architecture scan and both Svelte diagnostics pass with zero warnings after the composition roots returned below their fixed budgets.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm web:test:browser --grep repository directories load lazily|graph_and_directory_pages
```

**Notes**

The snapshot-bound pagination regression passed. The new lazy-directory scenario proved expansion but initially queried the Design Core view toggle as a button instead of its accessible radio role; the product behavior was unchanged and the test was corrected.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm exec playwright test --config web/playwright.config.ts --grep repository directories load lazily
```

**Notes**

The exact built-browser regression passes 1/1: root paths are nonrecursive, src/lib.rs is absent until src disclosure loads it, and list mode requests a recursive root window.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first
```

**Notes**

Spec-first validation rejected the first evidence draft because the changed canonical API and UI chapters were not yet mapped in the implementation entry. Both authored chapters are now recorded in this slice.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/RepositoryReview.stories.svelte
```

**Notes**

All three focused repository-review interaction and accessibility stories pass against the shared lazy-directory controller contract.
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
pnpm spec:check
```

**Notes**

All 18 canonical lanes pass, including governance, cold Storybook browser acceptance, interactions/accessibility, production unit/browser tests, generated contracts, architecture, complete Rust workspace and spec-first validation.
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
pnpm check
```

**Notes**

Specification, package, 144-module architecture and progress boundaries, rustfmt, generated contracts, production/catalog Svelte diagnostics and all-target Clippy pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools VITEST_MAX_WORKERS=1 pnpm test
```

**Notes**

The serialized aggregate passes 23 governance tests, 3 storage-harness self-tests, 6 contract tests, 21 production browser units, 32 catalog units, 56 Storybook interaction/accessibility tests, the cold catalog browser contract and the full Rust workspace. Declared external RustFS and OS-credential cases remain ignored in the normal lane.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build
```

**Notes**

Cold Storybook, production browser and complete Rust workspace builds pass. Existing upstream Design Core :global minifier and bundle-size warnings remain non-failing.
<!-- /run -->

<!-- run -->
### Run 15

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

Rustfmt and Prettier pass across the complete authored tree.
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
pnpm dev:restart; curl -fsS http://127.0.0.1:7460/
```

**Notes**

The single restart task rebuilt the production UI and native binaries, replaced only the labelled checkout-owned pinned RustFS container, started the native server, seeded a fresh repository and printed its URL, repository ID and one-time token. The browser root returned HTTP 200 and the credential remains outside the checkout, URL and browser storage.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Continue commit changed-path pagination and operation change-group and
  per-group changed-path pagination.
- Consume the Design Core extension from a separately approved registry release;
  package.json and pnpm-lock.yaml remain registry-resolved.
- Run M4.14 external CLI publication, SSE notification and explicit-refresh
  acceptance against the isolated native repository and pinned RustFS.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Registry Design Core ^0.2.0 with guarded local source validation at c7937794, 4725befb and 6f8ee457; @tanstack/svelte-query 6.3.0; Svelte 5.57.1; Storybook 10.5.7; Vitest 4.1.11; Playwright 1.63.x. Enclosing Jujutsu change is the revision reference.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Tree mode keeps opaque cursor-paged nonrecursive pages per directory and loads children only on disclosure or directory navigation. List and package modes request a separate recursive root window. Directory rows and file terms preserve native IDs and the selected operation snapshot. The in-memory catalog controller uses the same controlled view/disclosure contract without transport. Registry manifests and the lockfile are unchanged. Stories remain visual-pending and visual tests were skipped at the user request.
<!-- /evidence-field -->

