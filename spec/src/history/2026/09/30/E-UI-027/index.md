---
schema: 1
id: E-UI-027
---

# E-UI-027

<!-- evidence-field:id markdown -->
## Id

E-UI-027
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4.8
- M4.16
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-30
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

bookmark-context-and-commit-selector
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/StoreRepositoryBranchContext.stories.svelte
- catalog/components/store-mutations/StoreRepositoryActions.stories.svelte
- spec/src/api.md
- spec/src/milestones.md
- spec/src/ui.md
- web/src/lib/ui/store-application/StoreRepositoryBookmarkDetails.svelte
- web/src/lib/ui/store-application/StoreRepositoryDetails.svelte
- web/src/lib/ui/store-mutations/StoreMutationCommitPicker.svelte
- web/src/lib/ui/store-mutations/StoreMutationCommitTarget.svelte
- web/src/lib/ui/store-mutations/StoreMutationDialog.svelte
- web/src/lib/ui/store-mutations/StoreMutationFields.svelte
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
<!-- /evidence-field -->

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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreRepositoryBranchContext.stories.svelte catalog/components/store-mutations/StoreRepositoryActions.stories.svelte --reporter=dot
```

**Notes**

Before implementation, the branch rail still exposed Selected bookmark and stacked metadata, while Create change and Create bookmark still rendered raw commit text inputs instead of the shared searchable target presentation.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:architecture && pnpm web:check && pnpm catalog:check
```

**Notes**

The first structural pass caught StoreRepositoryDetails at 358 counted lines over the 300-line production limit; bookmark metadata was extracted into its own production component before retrying.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte catalog/components/store-application/StoreRepositoryBranchContext.stories.svelte catalog/components/store-mutations/StoreRepositoryActions.stories.svelte --reporter=dot
```

**Notes**

All 16 focused Storybook interaction and automatic accessibility stories pass. They cover the compact bookmark header and revision summary, one-row status and target counts, multiple conflicted target SHA controls, fixed-row bookmark/workspace chips and searchable commit selection for Create change, Create bookmark and Move bookmark.
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
pnpm check:architecture && pnpm web:check && pnpm catalog:check
```

**Notes**

The extracted source stays within production budgets, both Svelte projects report zero diagnostics and the catalog boundary accepts all 65 modules.
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
source audit for raw commit target inputs under web/src/lib
```

**Notes**

Create change, Create bookmark and Move bookmark all use StoreMutationCommitPicker; Edit description and Abandon use StoreMutationCommitTarget. Only the semantically distinct reconciliation operation ID remains a raw fixed-width input.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check
```

**Notes**

The first aggregate run stopped at check:progress because the executable slice had not yet added this required same-change evidence entry; earlier specification, package-boundary and architecture stages passed.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check
```

**Notes**

The complete repository check lane passes, including formatting, specifications, architecture, Svelte diagnostics, catalog boundaries, Clippy and Rust formatting.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test
```

**Notes**

The complete test lane passes, including governance, storage harnesses, LocalTrusted and contract checks, 54 web tests, 60 catalog unit tests, 87 Storybook interaction/accessibility stories, the cold catalog browser contract and the full Rust workspace suites.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build
```

**Notes**

Storybook, the production web bundle, embedded RustFS and the locked Rust workspace build successfully. Existing Lightning CSS :global(svg) and large-chunk warnings remain informational.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm dev:restart and live in-app browser verification at http://127.0.0.1:7460/
```

**Notes**

The checkout-owned LocalTrusted application restarted against the persisted sample repository. The selected conflict/demo bookmark renders as the bold rail heading with avatar, one-line message, author/time metadata, a status chip, aligned target-count chip, Git SHA controls and branch actions. Create bookmark renders the shared revision row, opens a searchable Command View of avatar/message/Git/JJ/time/bookmark metadata, and updates the closed row to the selected complete native commit ID without submitting a mutation.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check
```

**Notes**

Specification validation and the cold catalog browser pass, but the production browser lane first timed out after 30000ms while starting its disposable Playwright web server; no production browser test ran in this invocation.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 20
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm exec playwright test --config web/playwright.config.ts --timeout=5000
```

**Notes**

A warm-server diagnostic confirms all 20 inherited production browser tests still fail before this presentation slice is reached. Nineteen wait for native-browser-fixture.ts to fill the removed Repository ID/token controls; the LocalTrusted bootstrap test waits for stale profile and branch expectations.
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
pnpm format:check && pnpm spec:first && pnpm check:progress
```

**Notes**

Rust and Prettier formatting pass, all eight protected implementation files map to the API, UI and progress chapters, and the same-change evidence ledger remains valid.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Migrate the inherited production browser fixture from removed Repository
  ID/token controls to LocalTrusted bootstrap and clear its 20 failures.
- Visual baselines remain unapproved.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Node.js 24.10.0; pnpm 10.32.1; Svelte 5.57.1; Design Core ^0.2.0; Community ^0.1.0.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No native object, manifest, WAL, receipt, HTTP, OpenAPI or route format changes. Presentation introduces a shared commit row and a Design Core Command View selector whose bound value remains the complete native commit ID.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


