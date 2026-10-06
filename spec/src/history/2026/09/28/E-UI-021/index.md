---
schema: 1
id: E-UI-021
---

# E-UI-021

<!-- evidence-field:id markdown -->
## Id

E-UI-021
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4
- M4.3
- M4.11
- M4.12
- M4.14
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

- catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte
- catalog/components/community-projects/CommunityProjectsRevisionGraph.stories.svelte
- catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.stories.svelte
- catalog/controller/in-memory-store-branch-actions.ts
- catalog/controller/in-memory-store-controller.svelte.ts
- catalog/controller/in-memory-store-mutation-commit.ts
- catalog/controller/store-controller.test.ts
- catalog/fixtures/RepositoryFixture.svelte
- catalog/fixtures/large-repository.ts
- catalog/fixtures/repository.ts
- catalog/projects/repository-log-graph.test.ts
- catalog/projects/repository-log-lane-stability.test.ts
- crates/jj-store-cli/tests/native_lifecycle/graphs.rs
- crates/jj-store-jj/src/query/graphs.rs
- crates/jj-store-jj/src/query/mod.rs
- schemas/openapi.json
- spec/src/api.md
- spec/src/cli.md
- spec/src/compatibility.md
- spec/src/jj-integration.md
- spec/src/milestones.md
- spec/src/ui.md
- web/src/lib/api/generated.ts
- web/src/lib/controller/contracts.ts
- web/src/lib/controller/native-presentation.ts
- web/src/lib/controller/production-branch-actions.ts
- web/src/lib/controller/production-route-restorer.ts
- web/src/lib/controller/production-store-controller.svelte.ts
- web/src/lib/controller/production-store-loaders.ts
- web/src/lib/controller/production-store-reader.svelte.ts
- web/src/lib/controller/production-store-state.svelte.ts
- web/src/lib/controller/production-store-view.ts
- web/src/lib/projects/repository-log.ts
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryLog.svelte
- web/src/lib/ui/community-projects/CommunityProjectsRevisionGraphCopy.svelte
- web/src/lib/ui/community-repository-branch-command/CommunityRepositoryBranchCommand.svelte
- web/src/lib/ui/store-application/StoreRepositoryApplication.svelte
- web/tests/browser/reads.spec.ts
- web/tests/production-store-loaders.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/native_lifecycle/graphs.rs#graph_query_three_histories_filtered_edges_and_snapshot_pagination
- web/tests/production-store-loaders.test.ts#branch graph requests bind exact
  target ancestry
- catalog/controller/store-controller.test.ts#working_copy_card_scopes_branch_history_to_its_native_commit
- web/tests/browser/reads.spec.ts#branch targets scope native history and expose
  repository decorations
- web/tests/browser/reads.spec.ts#graph_and_directory_pages_retain_exact_snapshots_and_query_context
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
pnpm exec tsx --test web/tests/production-store-loaders.test.ts
```

**Notes**

The focused loader regression proved that native graph bookmark, workspace, conflict and divergence decorations were discarded before reaching production presentation.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte catalog/components/community-projects/CommunityProjectsRevisionGraph.stories.svelte catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.stories.svelte
```

**Notes**

Focused interaction coverage reproduced the missing native revision chips, absent working-copy card and branch selection retaining the repository-wide graph.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle graph_acceptance::graph_query_three_histories_filtered_edges_and_snapshot_pagination --locked -- --exact
```

**Notes**

The native graph regression initially failed to compile because GraphBookmark did not preserve a remote bookmark identity.
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
pnpm exec tsx --test web/tests/production-store-loaders.test.ts
```

**Notes**

All seven focused presentation and paging tests pass, including exact single-target, conflicted-target-union and deleted-bookmark graph revsets.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit catalog/controller/store-controller.test.ts catalog/projects/repository-log-graph.test.ts catalog/projects/repository-log-lane-stability.test.ts
```

**Notes**

All 12 focused controller and graph units pass; working-copy selection records the full commit/change route and scopes its adjacent history.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte catalog/components/community-projects/CommunityProjectsRevisionGraph.stories.svelte catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.stories.svelte
```

**Notes**

All 14 focused interaction and accessibility stories pass for native graph chips, branch cards and the working-copy card. Visual baselines were skipped by user direction.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle graph_acceptance::graph_query_three_histories_filtered_edges_and_snapshot_pagination --locked -- --exact
```

**Notes**

The exact native lifecycle case passes and preserves separate local main and remote main@origin decorations through graph filtering and paging.
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
pnpm test:contracts
```

**Notes**

All six generated-contract tests and the TypeScript contract compile pass with the required nullable remote bookmark field.
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
pnpm web:check; pnpm catalog:check; pnpm check:architecture; pnpm web:build
```

**Notes**

Production and catalog Svelte diagnostics pass, all governed modules stay within their counted-line and dependency budgets, and the production browser bundle builds with only the existing Design Core global-selector and bundle-size warnings.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/reads.spec.ts --grep 'branch targets scope native history and expose repository decorations|graph_and_directory_pages_retain_exact_snapshots_and_query_context'
```

**Notes**

Both focused cold built-browser cases pass, proving bookmark and working-copy target scoping, JJ/Op Log decorations and scoped graph pagination.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm dev:restart
```

**Notes**

The checkout-owned LocalTrusted browser, server and embedded RustFS restarted against a freshly seeded native repository. The review stack remains available on 127.0.0.1:7460 without credentials.
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
live in-app browser verification at http://127.0.0.1:7460/
```

**Notes**

The live Branches column shows the demo working-copy card above its filters; main opens a three-row exact ancestry while the working copy opens its five-row ancestry and full commit/change route. JJ Log shows local bookmark, working-copy, divergent and conflicted chips. Op Log exposes working-copy, snapshot and concurrent-head chips.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:first && pnpm check:progress && DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm format:check
```

**Notes**

Same-change governance maps all protected files to the six updated canonical chapters, validates evidence bookkeeping and confirms Rust plus Prettier formatting without running the aggregate test suite.
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

jj-lib 0.45.1; OpenAPI 3.1; TypeScript generated contracts; @lapismd/design-core 0.2.0; Svelte 5.57.1; Storybook 10.5.7; Vitest 4.1.11; Playwright 1.63.x.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

GraphBookmark adds one required nullable remote name in OpenAPI and generated TypeScript. Native commit graph DTOs now retain local and remote bookmarks separately plus existing workspace, conflict and divergence metadata; operation rows retain snapshot/workspace/head metadata. Branch selection sends exact target-bound ancestry revsets and working-copy selection stores full commit/change identities in the existing hash route. Native objects, manifest, WAL, receipt, credential and browser layout formats are unchanged. Stories remain visual-pending and visual baselines were skipped by user direction.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


