---
schema: 1
id: E-UI-008
---

# E-UI-008

<!-- evidence-field:id markdown -->
## Id

E-UI-008
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4
- M4.3
- M4.9
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

- .storybook/preview.css
- catalog/controller/in-memory-store-controller.svelte.ts
- catalog/fixtures/StoreApplicationFixture.svelte
- catalog/fixtures/StoreRepositoryApplicationFixture.svelte
- catalog/index.ts
- docs/catalog.md
- scripts/check-catalog.mjs
- scripts/web-architecture-baseline.json
- spec/src/architecture.md
- spec/src/ui.md
- spec/src/milestones.md
- web/src/lib/controller/contracts.ts
- web/src/lib/projects/repository-log.ts
- web/src/lib/ui/store-application/StoreApplication.svelte
- web/src/lib/ui/store-application/StoreRepositoryApplication.svelte
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- catalog/controller/store-controller.test.ts#controller_restores_snapshot_history
- catalog/controller/store-controller.test.ts#controller_isolates_instances_and_conflicts
- scripts/web-architecture.test.mjs#allows stories to consume production
  presentation modules
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 11
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook
```

**Notes**

The first cold interaction import exposed that the moved top-level typography stylesheet had not moved with component subdirectories; no story executed.
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
pnpm catalog:test
```

**Notes**

All 20 catalog unit tests pass against production-owned presentation modules and the catalog-owned in-memory controller.
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
pnpm test:storybook
```

**Notes**

All 37 Chromium interaction and accessibility tests pass after moving the shared typography asset.
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
pnpm web:check && pnpm catalog:check && pnpm check:architecture
```

**Notes**

Production/catalog type checks, the transport-free catalog audit and the 106-module architecture scan pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check
```

**Notes**

Specification validation, package and web architecture boundaries, progress, rustfmt, generated contracts, production/catalog types and locked workspace clippy pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test
```

**Notes**

Governance, storage harness, contract, 8 web, 20 catalog, 37 Storybook, cold catalog and full Rust workspace lanes pass; external OS credential/RustFS tests remain intentionally ignored.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build
```

**Notes**

Cold Storybook, served web bundle and locked Rust workspace builds pass. Existing Lightning CSS :global(svg) and large Storybook chunk warnings remain non-fatal.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm format:check
```

**Notes**

rustfmt and Prettier checks pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check
```

**Notes**

The first aggregate canonical run exposed missing mapped api.md and spec-governance.md updates after promoting modules into web ownership; all functional lanes passed before spec-first stopped the run.
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
pnpm spec:first
```

**Notes**

The focused spec-first regression passes for 144 protected files and five mapped canonical chapters.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check
```

**Notes**

All 18 canonical validation lanes pass after recording the web ownership and catalog boundary in api.md and spec-governance.md.
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

Community source d859f33a484a9238bb11f974f4ead8e9be729ec6; Design Core ^0.2.0; jj-store parent bce792e4.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Presentation components, models and styles moved from catalog ownership to web/src/lib without changing their rendered contract. The shared asynchronous controller route now separates snapshot operation, inspected operation, commit, change, comparison-parent and comparison-group identities. Catalog fixtures remain transport-free. No HTTP, storage, credential or browser persistence format changes.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


