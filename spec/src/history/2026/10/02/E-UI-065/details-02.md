---
schema: 1
id: E-UI-065
---

# E-UI-065 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
jj --no-pager st; jj describe -m "Use repository card width and preserve branches while loading scoped history"
```

**Notes**

Clean konqzwkq above lwxwqzum/709855c2 in reused Appearance workspace. Inspected canonical UI/API/projects and milestone scope. Community repositoryListRow and CommunityProjects.css inspected first; Design Core public Card and ColumnCanvas inspected second. Before: native status chip lives in access row; intrinsic-width grid target and listitem wrappers leave descriptions narrow and card heights unequal. Production branch actions already avoid reference reads; shared graph/bookmark loading and graph-empty skeleton unmount the loaded first column.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/ProjectsDirectory.stories.svelte catalog/components/store-application/StoreRepositoryBranchContext.stories.svelte -t 'LinkedRepositoryCardsShareHeightAndUseTheirWidth|BranchSelectionRetainsTheLoadedFirstColumn'
```

**Notes**

Before production edits: delayed graph read removes the retained bookmark DOM node. Card fixture first attempt races initial project restore; corrected to wait for Edit project. Log /tmp/jj-repository-layout-before.log.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/ProjectsDirectory.stories.svelte -t LinkedRepositoryCardsShareHeightAndUseTheirWidth
```

**Notes**

Three fixture/before runs: nonexistent project heading, duplicate Retired label in sidebar, then corrected linked-list scope reaches geometric assertion. Before chip/name vertical center difference69.375px exceeds3px. Logs /tmp/jj-repository-cards-before{,-final,-geometry}.log.
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
pnpm exec prettier --write (14 changed supported source/spec files)
```

**Notes**

Formatted changed sources and canonical chapters. /tmp/jj-repository-layout-format.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/ProjectsDirectory.stories.svelte catalog/components/store-application/StoreRepositoryBranchContext.stories.svelte -t 'LinkedRepositoryCardsShareHeightAndUseTheirWidth|BranchSelectionRetainsTheLoadedFirstColumn'
```

**Notes**

2 regressions pass;4 stories skipped. Wide/narrow linked-card widths980/620/300 and retained bookmark DOM during deferred graph read. /tmp/jj-repository-layout-after.log.
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
pnpm web:check
```

**Notes**

0 errors/warnings. /tmp/jj-repository-layout-web-check.log.
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
pnpm catalog:check
```

**Notes**

Catalog delayed-graph fixture widens loading literal to string; fixed with const assertion. Production check is clean. /tmp/jj-repository-layout-catalog-check.log.
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
pnpm catalog:check
```

**Notes**

After fixture type repair:0 errors/warnings;129 catalog modules. /tmp/jj-repository-layout-catalog-check-final.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/store-workspaces.test.ts web/tests/vite/production-automatic-refresh.test.ts web/tests/vite/store-route-history.test.ts
```

**Notes**

22/22. Branch/workspace selections retain reference array identities, exact operation and next-page cursor, only scoped graph/detail reads. Existing refresh/new-operation and external history regressions retained. /tmp/jj-repository-layout-unit.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/ProjectsDirectory.stories.svelte catalog/components/store-work/ProjectsHub.stories.svelte catalog/components/store-work/StoreWork.stories.svelte catalog/components/store-application/StoreRepositoryBranchContext.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte
```

**Notes**

40/40 in6 direct consumer families; fresh browser. Includes project directory, linked cards, work forms, branch actions, graph/diff and connected application. /tmp/jj-repository-layout-stories.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/ProjectsDirectory.stories.svelte catalog/components/store-application/StoreRepositoryBranchContext.stories.svelte catalog/components/store-application/RepositoryLoading.stories.svelte catalog/components/store-application/RepositoryApplication.stories.svelte catalog/components/store-application/RepositoryLayout.stories.svelte catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.stories.svelte
```

**Notes**

22/22 in6 direct families after strengthened regressions: bookmark and working-copy graph loading retains original DOM and typed query; card descriptions remain two visible lines. Cold pending reference skeletons, repository layout and branch filters remain supported. /tmp/jj-repository-layout-stories-final.log.
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
pnpm check:architecture
```

**Notes**

737 modules within source limits. /tmp/jj-repository-layout-architecture.log.
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
cua_repl: navigate linked project repositories, select Grid, open local repository Branches and main; create fresh hidden branch tab, await scoped graph, close temporary tab
```

**Notes**

Live HMR card measurements: both heights166.515625px, description/content widths460.90625px, Retired chip right gap0px. Local branch selection preserves first column and opens scoped JJ Log; fresh tab restores exact operation and bookmark main. Read-only UI checks; no fixture publications, resets or backend restart. Screenshot /tmp/jj-repository-cards-final.jpg.
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
pnpm spec:first; pnpm spec:validate; pnpm check:progress; pnpm check:ui-ownership; pnpm check:architecture; pnpm catalog:check
```

**Notes**

Final changed-source checks:11 protected files/4 canonical chapters;5 validators/21 chapters/147 requirements;120 tasks;ownership1 validator;737 architecture and129 catalog modules;0 type errors/warnings. Logs /tmp/jj-repository-layout-{spec-first,spec-validate,progress,ownership,architecture-final,catalog-check-post-format}.log. No root aggregate lanes.
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
pnpm exec prettier --check catalog/components/store-application/StoreRepositoryBranchContext.stories.svelte catalog/components/store-work/ProjectsDirectory.stories.svelte catalog/controller/loading-store-controller.ts catalog/fixtures/ProjectsHubFixture.svelte catalog/fixtures/RepositoryFixture.svelte catalog/fixtures/StoreRepositoryApplicationFixture.svelte spec/src/api.md spec/src/milestones.md spec/src/projects.md spec/src/ui.md web/src/lib/ui/community-projects/CommunityProjectsRepositoryLog.svelte web/src/lib/ui/store-application/StoreRepositoryApplication.svelte web/src/lib/ui/store-work/StoreRepositoryDirectoryRow.svelte web/src/lib/ui/store-work/community-work-directory.css web/tests/vite/store-workspaces.test.ts; pnpm check:progress
```

**Notes**

All15 changed files formatted; updated ledger passes120-task bookkeeping. Logs /tmp/jj-repository-layout-{format-check,progress-final}.log. Final evidence-only formatting/check repeats use pnpm exec prettier --write spec/src/milestones.md and pnpm check:progress.
<!-- /run -->
<!-- /evidence-field -->

