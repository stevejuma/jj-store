---
schema: 1
id: E-UI-020
---

# E-UI-020

<!-- evidence-field:id markdown -->
## Id

E-UI-020
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4
- M4.3
- M4.11
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

- .storybook/preview.css
- catalog/components/community-projects/CommunityProjectsColumnAlert.stories.svelte
- catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte
- catalog/components/store-application/RepositoryLoading.stories.svelte
- catalog/fixtures/RepositoryFixture.svelte
- catalog/projects/repository-log-lane-stability.test.ts
- scripts/dev-demo-repository.mjs
- spec/src/milestones.md
- spec/src/ui.md
- spec/src/verification.md
- web/src/lib/projects/repository-log-graph.ts
- web/src/lib/projects/repository-log-lanes.ts
- web/src/lib/ui/community-projects/CommunityProjectsColumnAlert.svelte
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryFiles.svelte
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryLog.svelte
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryLogFiles.svelte
- web/src/lib/ui/community-projects/CommunityProjectsRepositorySkeleton.svelte
- web/src/lib/ui/community-projects/CommunityProjectsRevisionGraph.svelte
- web/src/style.css
- web/tests/real/native-repository.spec.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- catalog/projects/repository-log-lane-stability.test.ts#keeps an unrelated
  outer lane connected while an inner fork converges
- web/tests/real/native-repository.spec.ts#demo history exposes one native
  divergent change
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit catalog/projects/repository-log-graph.test.ts catalog/projects/repository-log-lane-stability.test.ts
```

**Notes**

The focused regression proved that convergence removed an inner lane by compacting the unrelated outer lane between adjacent rows.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 6
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte catalog/components/store-application/RepositoryLoading.stories.svelte
```

**Notes**

The focused stories reproduced the SVG paint order, three unstyled column banners and both detail-loading cards being replaced by generic skeleton headers.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit catalog/projects/repository-log-graph.test.ts catalog/projects/repository-log-lane-stability.test.ts
```

**Notes**

All eight graph units pass with stable vacant lane slots, safe reuse and convergence paths.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte catalog/components/store-application/RepositoryLoading.stories.svelte catalog/components/community-projects/CommunityProjectsColumnAlert.stories.svelte
```

**Notes**

The first implementation run found two blocking alert color-contrast failures plus one transient module reload while Vite optimized the newly imported icon. Alert text now uses the accessible theme foreground while semantic color remains in the icon and tint.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte catalog/components/store-application/RepositoryLoading.stories.svelte catalog/components/community-projects/CommunityProjectsColumnAlert.stories.svelte
```

**Notes**

All 18 focused interaction and accessibility stories pass for flat semantic banners, graph paint order and header-preserving changed-file skeletons.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm dev:restart
```

**Notes**

The checkout-owned LocalTrusted app and embedded RustFS restarted with a fresh native repository containing the real file conflict and divergent same-change pair; the stack remains available on loopback for review.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm exec playwright test --config web/playwright.real.config.ts --grep 'demo history exposes one native divergent change'
```

**Notes**

The one focused cold-browser case proves both named divergent rows render with different Git commit IDs and the same full JJ change ID.
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
target/debug/jj -R <demo-repository> file show -r conflict/demo 'root:conflict.txt'
```

**Notes**

The native file view reports one unresolved conflict with distinct Conflict base, Conflict left and Conflict right terms and realistic policy content.
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
pnpm web:check; pnpm catalog:check; pnpm check:architecture
```

**Notes**

Production and catalog Svelte diagnostics report no warnings or errors, catalog boundaries pass for 53 modules and all 192 governed modules remain within architecture budgets.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:first; pnpm check:progress
```

**Notes**

Spec-first required the changed demo seed and real-browser test to be stated in their mapped API and governance chapters. Those chapters now define the native conflict/divergence fixture boundary and focused identity acceptance.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 7
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:first; pnpm check:progress
```

**Notes**

Spec-first passed for 17 protected files across four chapters. Progress rejected Storybook names as named executable tests and the new graph case still used Vitest's it alias; the ledger now lists only parser-verifiable tests and the graph test uses test().
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:first; pnpm check:progress
```

**Notes**

Spec-first maps 17 protected files to four canonical chapters and all 53 tasks pass evidence bookkeeping with the focused native and catalog tests named directly.
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

Community repository graph source fd44ccb98bd9b3113714d9feb76c47db0496ea0a; @lapismd/design-core 0.1.2 Alert and Skeleton source at 0f7d6e10e276; Svelte 5.57.1; Storybook 10.5.7; Vitest 4.1.11; Playwright 1.63.x.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Graph lanes retain null internal slots until safe reuse and trim only trailing vacancies. Column banners compose the public Design Core Alert with application warning/destructive tones. Changed-file loading keeps the production header and virtual-list shell. Demo divergence preserves one native JJ change ID across two distinct commit IDs, and the conflict remains a native unresolved merge. No API, native identity, object, WAL, credential, route or persistent-browser format changes. Stories remain visual-pending and visual baselines were skipped by user direction.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


