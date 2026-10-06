---
schema: 1
id: E-UI-013
---

# E-UI-013 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 6
hasNote: true
noteEncoding: markdown

```

```shell
pnpm install --lockfile=false --force; pnpm exec vitest run --config catalog/vitest.config.ts --project storybook <affected stories>
```

**Notes**

The first local-source run found six suites importing newly added Design Core FileListing virtualization files from a stale pnpm package snapshot. Refreshing through the guarded pnpm hook restored the complete sibling package without changing package.json or pnpm-lock.yaml.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check; pnpm catalog:check; pnpm exec vitest run --config catalog/vitest.config.ts --project storybook <affected stories>
```

**Notes**

The first compiled runtime exposed a null virtual-row host and graph loading announcements that were not valid children of role=list. The host and listitem/status composition were corrected without weakening accessibility.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreRepositoryApplication.stories.svelte -t LargeVirtualizedRepository
```

**Notes**

The first large fixture selected a generated revision absent from the in-memory detail map. The fixture controller now resolves every generated revision to deterministic native-width detail data.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreRepositoryApplication.stories.svelte -t LargeVirtualizedRepository
```

**Notes**

The 600-row file tree mounted every row because FileListing observed the shell Scroll Area before its runtime overflow style settled. A source-owned Design Core regression was added and fixed rather than retaining a consumer overflow workaround.
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
pnpm exec vitest run --project storybook src/shared/diff/file-listing/FileListing.stories.svelte -t Virtualizes; pnpm checks
```

**Notes**

Design Core focused FileListing acceptance passes 1/1. The complete nonvisual source gate passes specification/format/type checks, 714 unit tests, 637 Storybook interactions/accessibility checks, static build, 14 workspace browser tests, 11 shell tests, 3 AI tests and 35 passing shadcn browser tests with one declared WebKit-only skip. The fix is committed at fddad817 on top of a9820137.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreRepositoryApplication.stories.svelte -t LargeVirtualizedRepository
```

**Notes**

After file rows were bounded, accessibility rejected a nested self-scroll wrapper inside the reference-picker listbox. Virtual rows now bind to CommandView public ScrollArea instead of introducing a second scroll region.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte catalog/components/store-application/RepositoryReview.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte
```

**Notes**

All 17 affected interaction/accessibility stories pass, including the 600-item repository across bookmark cards, graphs, changed files, repository files, directory rows and reference choices.
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
pnpm web:check; pnpm catalog:check; pnpm exec tsx --test web/tests/layout-storage.test.ts; node --test scripts/local-package-pnpmfile.test.mjs
```

**Notes**

Production and catalog Svelte checks report zero errors/warnings, the catalog boundary scans 33 modules, layout persistence passes 4/4 and guarded local package resolution passes 2/2.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/RepositoryReview.stories.svelte -t ShellAndGraphAlignment
```

**Notes**

Before the breadcrumb correction, the repository breadcrumb did not occupy the available header-center width or preserve the final page segment under constrained width.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/RepositoryReview.stories.svelte -t ShellAndGraphAlignment
```

**Notes**

The focused shell regression passes with a centered, single-line, full-width breadcrumb that truncates earlier identity segments before the final page label.
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
node --test scripts/catalog-browser.test.mjs
```

**Notes**

The first cold catalog run compared virtual rows across independent columns and nested Design Core lists, reporting false overlap.
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
node --test scripts/catalog-browser.test.mjs
```

**Notes**

The cold catalog contract passes 1/1 after measuring only direct row-container children within each virtual-row host in rendered order.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/RepositoryLoading.stories.svelte
```

**Notes**

All seven representative page and detail loading stories pass their interaction and accessibility assertions.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/RepositoryLoading.stories.svelte catalog/components/store-application/RepositoryReview.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte
```

**Notes**

All 24 affected Storybook interactions and accessibility audits pass across loading, shell geometry, persistence, virtualization and graph compositions.
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
pnpm spec:first; pnpm check:architecture; pnpm web:check; pnpm catalog:check; pnpm format:check; pnpm web:build
```

**Notes**

Spec-first mapping, the 142-module architecture boundary, production/catalog type checks, Rust/Prettier formatting and the production browser build pass after skeleton and stylesheet extraction.
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
pnpm test:storybook
```

**Notes**

All 56 Storybook interaction and accessibility tests pass across 13 files.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:test:browser
```

**Notes**

The cold static Storybook build and loopback browser contract pass 1/1. Existing Design Core :global minifier and chunk-size warnings remain non-failing; no visual baseline was run or changed.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm dev:restart
```

**Notes**

The new local-stack command first treated Docker's lowercase missing-container diagnostic as fatal, then mounted RustFS temporary data without write permission. Case-insensitive absence handling and an explicit writable tmpfs mode corrected both startup defects.
<!-- /run -->
<!-- /evidence-field -->

