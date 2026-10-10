---
schema: 1
id: E-UI-SKELETONS-001
---

# Validation runs

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
pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/lazy-views.spec.ts -g 'cold Threads import'
```

**Notes**

Before: 0 pass, 1 fail. Generic module fallback has no Thread conversations region. Exact target rerun after repair: 1 pass, 0 fail. Cold preview uses the built bundle.
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
pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/lazy-views.spec.ts
```

**Notes**

Progressive cold runs: 19/19, then 26/26, then 27/27 passed. Final run adds mobile geometry, selected-file initial reads, saved tree visibility and saved inbox width. No failed imports, retry replay or stale completion regressions. Logs: .local/skeleton-browser.log, .local/skeleton-browser-final.log and .local/skeleton-browser-verified.log.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown
```

```shell
pnpm test:storybook catalog/components/store-work/ProjectsActivity.stories.svelte catalog/components/store-work/ProjectsAgents.stories.svelte catalog/components/store-application/RepositoryLoading.stories.svelte catalog/components/store-application/AgentSessions.stories.svelte
```

**Notes**

First focused run: 35 pass, 5 fail across 40 stories. Four review detail placeholders lacked the writable footer during module import; Projects queried a tab before the module mounted. Fixed the fallback footer and explicit view readiness without removing loading assertions. Log: .local/skeleton-stories.log.
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
pnpm test:storybook catalog/components/store-work/ProjectsActivity.stories.svelte catalog/components/store-application/RepositoryLoading.stories.svelte catalog/components/store-application/AgentSessions.stories.svelte
```

**Notes**

Rerun: 35 pass, 1 fail across 36 stories. Review and Projects repairs passed. New gallery alignment assertion exposed a transient fixture rather than the intended persistent SkeletonFixture. Log: .local/skeleton-stories-rerun.log.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown
```

```shell
pnpm test:storybook catalog/components/store-application/AgentSessions.stories.svelte -t AgentsSkeletonMatchesFixedCardDimensions
```

**Notes**

Initial exact rerun failed when the transient gallery disappeared. Story asChild now renders the named persistent fixture. Exact rerun after that correction passed 1 test, with 10 unrelated stories excluded. Logs: .local/skeleton-gallery.log and .local/skeleton-gallery-final.log.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown
```

```shell
pnpm test:storybook catalog/components/store-work/ProjectsActivity.stories.svelte catalog/components/store-work/ProjectsAgents.stories.svelte catalog/components/store-application/RepositoryLoading.stories.svelte catalog/components/store-application/AgentSessions.stories.svelte
```

**Notes**

Intermediate run: 39 pass, 2 fail across 41 stories. The new grid test queried button instead of radio. File detail had a cold readiness timeout under concurrent compilation; its isolated cold rerun passed unchanged. The detail test now awaits its actual breadcrumb before checking the unchanged blob skeleton. Log: .local/skeleton-stories-final.log.
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
pnpm test:storybook catalog/components/store-work/ProjectsActivity.stories.svelte -t GridSkeletonRetainsProjectColumnSizing
```

**Notes**

Exact grid trial confirmed the wrong role. Corrected to the public single-selection radio contract, retaining physical column and row-height assertions. Log: .local/skeleton-grid.log.
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
pnpm test:storybook catalog/components/store-application/RepositoryLoading.stories.svelte -t FileDetail
```

**Notes**

Isolated cold FileDetail rerun passed 1 test before the readiness adjustment; 17 other stories excluded. Log: .local/skeleton-file-detail.log.
<!-- /run -->

<!-- /evidence-field -->
