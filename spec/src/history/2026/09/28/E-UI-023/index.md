---
schema: 1
id: E-UI-023
---

# E-UI-023

<!-- evidence-field:id markdown -->
## Id

E-UI-023
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4
- M4.8
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

- catalog/components/store-application/StoreRepositoryApplication.stories.svelte
- catalog/controller/in-memory-store-controller.svelte.ts
- catalog/controller/store-controller.test.ts
- spec/src/api.md
- spec/src/milestones.md
- spec/src/ui.md
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryFiles.svelte
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- catalog/controller/store-controller.test.ts#snapshot_reference_selection_keeps_reference_and_commit_identity
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreRepositoryApplication.stories.svelte -t RepositoryFiles --reporter=dot
```

**Notes**

The focused interaction reproduced the live regression after the in-memory controller matched production reference resolution: selecting feature/native-objects displayed its resolved change/commit label instead of the bookmark name.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 43
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreRepositoryApplication.stories.svelte -t RepositoryFiles --reporter=dot
```

**Notes**

An intermediate fixture correction keyed divergent commits by their shared change ID and triggered duplicate virtual-row keys. The fixture retained its unique commit-backed keys while the selector learned to resolve either production change keys or an exact target commit.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreRepositoryApplication.stories.svelte -t RepositoryFiles --reporter=dot
```

**Notes**

The exact RepositoryFiles interaction and accessibility story passes, including persistent bookmark and change labels after each selection. Visual baselines were skipped by user direction.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit catalog/controller/store-controller.test.ts --reporter=dot
```

**Notes**

The new unit assertion initially assumed the change ID was derived from the commit ID; the fixture correctly models both divergent commits with one independent JJ change ID.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit catalog/controller/store-controller.test.ts --reporter=dot
```

**Notes**

All five focused controller tests pass and preserve separate bookmark, resolved commit and divergent change identities.
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
pnpm web:check; pnpm catalog:check; pnpm check:architecture
```

**Notes**

Production and catalog Svelte diagnostics report no warnings, catalog boundaries pass and all 196 governed modules remain within their architecture budgets.
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
pnpm web:build
```

**Notes**

The production browser bundle builds successfully with only the existing Design Core global-selector and bundle-size warnings.
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
pnpm dev:restart
```

**Notes**

The first restart was blocked during Rust linking because the active full-Xcode developer directory has an unaccepted license; no product test failed.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm dev:restart
```

**Notes**

The checkout-owned LocalTrusted demo restarted through the installed Command Line Tools with repository 6b767a3b23c9600bed1b4dcb0635514a, no credentials and persistent embedded RustFS.
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
live in-app browser verification at http://127.0.0.1:7460/
```

**Notes**

The restored experiment/layout deep link resolves and appends its full commit while the repository selector continues to display experiment/layout.
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
pnpm spec:first; pnpm check:progress; pnpm format:check
```

**Notes**

The first governance pass required the mapped API chapter because the selector correction preserves the controller boundary between selected reference and resolved commit identities.
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
pnpm spec:first; pnpm check:progress; pnpm format:check
```

**Notes**

Spec-first passed after the API update; the progress guard cannot resolve Svelte Story declarations as named tests, so the exact RepositoryFiles story remains recorded by its focused command rather than the named-test field.
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
pnpm spec:first; pnpm check:progress; pnpm format:check
```

**Notes**

Same-change governance maps the protected application and fixture files to the updated UI contract, validates evidence bookkeeping and confirms formatting without running unrelated aggregate suites.
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

@lapismd/design-core 0.2.0; Svelte 5.57.1; Storybook 10.5.7; Vitest 4.1.11.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

The selected reference remains distinct from its resolved commit in the existing typed hash route. Native IDs, OpenAPI, object, manifest, WAL, receipt and browser-layout formats are unchanged. Stories remain visual-pending and visual baselines were skipped by user direction.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


