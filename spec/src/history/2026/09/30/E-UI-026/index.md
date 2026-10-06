---
schema: 1
id: E-UI-026
---

# E-UI-026

<!-- evidence-field:id markdown -->
## Id

E-UI-026
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

contextual-branch-rail-and-mutation-composer
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.stories.svelte
- catalog/components/store-application/StoreApplication.stories.svelte
- catalog/components/store-application/StoreRepositoryBranchContext.stories.svelte
- catalog/components/store-mutations/StoreRepositoryActions.stories.svelte
- spec/src/api.md
- spec/src/milestones.md
- spec/src/ui.md
- web/src/lib/ui/community-repository-branch-command/CommunityRepositoryBranchCommand.svelte
- web/src/lib/ui/store-application/StoreApplication.svelte
- web/src/lib/ui/store-application/StoreRepositoryDetails.svelte
- web/src/lib/ui/store-mutations/StoreMutationCommitTarget.svelte
- web/src/lib/ui/store-mutations/StoreMutationDialog.svelte
- web/src/lib/ui/store-mutations/StoreMutationFields.svelte
- web/src/lib/ui/store-mutations/StoreRepositoryActions.svelte
- web/src/lib/ui/store-mutations/StoreRepositoryActionsPanel.svelte
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
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.stories.svelte catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte catalog/components/store-mutations/StoreRepositoryActions.stories.svelte --reporter=dot
```

**Notes**

Before implementation, unselected branch hover used the accent color, bookmark selection left the rail in generic repository context, the rail retained its redundant Details close header, and Edit description rendered a textarea plus disabled raw commit input.
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
source audit of Community project overview headings, repository create actions, form composers and Design Core Command View selection tokens
```

**Notes**

The implementation follows Community's compact body heading and trailing create control, consumes the released Community form composer and uses Design Core's selected-row token while preserving jj-store route and mutation ownership.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.stories.svelte catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-application/StoreRepositoryBranchContext.stories.svelte catalog/components/store-mutations/StoreRepositoryActions.stories.svelte --reporter=dot
```

**Notes**

All 13 focused Storybook interaction and automatic accessibility stories pass. They cover neutral branch hover with selected accent retention, contextual rail heading without a duplicate close control, bookmark metadata/actions and revision replacement, create/move dialog wiring, Community composer input and compact copyable commit/change context.
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

The source split remains within production and story budgets, both Svelte projects report zero diagnostics and the catalog boundary accepts all 65 modules.
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

The complete repository check lane passes, including formatting, specifications, architecture, Svelte diagnostics, catalog boundaries, Clippy and Rust formatting.
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

The complete test lane passes, including governance, storage harnesses, LocalTrusted and contract checks, 54 web tests, 60 catalog unit tests, 87 Storybook interaction/accessibility stories, the cold catalog browser contract and the full Rust workspace suites.
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

Storybook, the production web bundle, embedded RustFS and the locked Rust workspace build successfully. Existing Lightning CSS :global(svg) and large-chunk warnings remain informational.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm dev:restart and live in-app browser verification at http://127.0.0.1:7460/
```

**Notes**

The checkout-owned LocalTrusted application restarted against the persisted sample repository. The rebuilt UI removes the duplicate Details close header; shows Branches/Bookmarks with Add bookmark; exposes selected bookmark metadata and move/delete actions; replaces bookmark context with Selected change after revision selection; and renders Edit description with the Community composer plus one avatar, description, Git/JJ copy controls and relative-time row.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 20
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:check
```

**Notes**

Specification validation, the cold catalog browser and the catalog browser pass. The inherited production browser lane still has 20 failures because native-browser-fixture.ts waits for removed Repository ID and Repository token controls instead of using LocalTrusted bootstrap; none reaches this presentation slice.
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

No native object, manifest, WAL, receipt, HTTP, OpenAPI or route format changes. Presentation adds bookmark selection context, page-specific rail headings, existing typed mutation triggers, Community composer prose and a compact commit/change identity row.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


