---
schema: 1
id: E-UI-052
---

# E-UI-052 details 2

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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/appearance.test.ts web/tests/vite/settings.test.ts web/tests/vite/threads.test.ts web/tests/vite/mixed-profile.test.ts
```

**Notes**

Focused Appearance and Settings/Threads/mixed-profile acceptance: 33 tests across four files, following a 26-test baseline.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit
```

**Notes**

204 tests across 34 files, including preference validation, complete theme pairs, session drafts and local URL cards.
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
pnpm web:test
```

**Notes**

70 tests pass; no transport contract changed.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-appearance/StoreAppearance.stories.svelte catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-threads/StoreThreads.stories.svelte
```

**Notes**

Initial new draft flow used the wrong member label; 18/19 pass. Later label correction exposed and resolved Dialog pointer teardown timing and the Bits UI deferred scrollbar warning through the source-owned native host strategy.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 12
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-threads/StoreThreads.stories.svelte catalog/components/store-application/StoreCollaboration.stories.svelte
```

**Notes**

An extracted draft initially mutated unbound props, fixed with owner callbacks. Native viewport regions then duplicated landmarks, fixed in Design Core. Final pass left one active-chip contrast failure (4.37:1); app-owned success text mix adjusted.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreCollaboration.stories.svelte
```

**Notes**

Nine interactions and axe pass, including agent response replies and Focus/Split draft preservation. The new scenario is subsequently extracted into AppearanceDiscussions with a separate play helper to preserve source budgets.
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

Production assets and manifest built. Fonts remain local files under unchanged font-src through assetsInlineLimit=0.
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
pnpm exec playwright test -c .scratch/appearance.playwright.config.ts
```

**Notes**

Initial strict CSP run exposed inlined font assets and existing static toolbar style, corrected in production source. Remaining Bits Command template violations belong to inherited Overview bookmarks; the cold Appearance lane enters Threads to isolate the owned setting surface. No CSP weakened. A peer using Overview also requested an inherited Gravatar; both fixture tabs now enter Threads. Narrow viewport assertion was scoped to the page heading to disambiguate the shell heading.
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
pnpm exec playwright test -c .scratch/appearance.playwright.config.ts
```

**Notes**

Cold built UI: startup/reload persistence, cross-tab propagation, OS System pairs, dark/light selections, geometry/zoom, reduced-motion desktop and 390px mobile axe, all appearance controls, zero CSP violations and zero external requests in this flow.
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
pnpm web:check
```

**Notes**

Zero errors and warnings.
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
pnpm catalog:check
```

**Notes**

Zero errors/warnings and catalog boundaries passed before final regression corrections; a final check follows below.
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
pnpm check:architecture
```

**Notes**

Initial oversized palette and owner modules failed the source budget; generated palette sheets and extracted ownership/story helpers resolve the final 707-module check.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 8
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook
```

**Notes**

First full UI run: 7 failed/39 passed files of 51 reached; 8 failed/167 passed tests, 7 unhandled errors. Related selection, link-name, task Activity and Split review regressions were corrected; a browser closed before the complete suite finished. Final full run is recorded separately.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/community-projects/CommunityProjectsRevisionGraph.stories.svelte catalog/components/community-projects/RepositoryDocuments.stories.svelte catalog/components/store-application/NativeReviewDetail.stories.svelte
```

**Notes**

Intermediate related regressions: graph/document acceptance passed; one detail-header test still selected the retained Split context viewport. After correction, NativeReviewDetail passes 2/2 in the next layout regression run. Logs /tmp/jj-appearance-regressions.log and /tmp/jj-appearance-layout-regressions2.log.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/ProjectsHub.stories.svelte catalog/components/store-application/NativeReviewDetail.stories.svelte
```

**Notes**

Intermediate 10 passed/1 failed: both review scenarios passed; moving Activity outside task metadata exposed selector ancestry. Owning disclosure selectors and fixture context were corrected.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/ProjectsHub.stories.svelte
```

**Notes**

Final task layout, discussion, response and accessibility acceptance: 9/9, /tmp/jj-appearance-task-final.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-appearance/StoreAppearance.stories.svelte catalog/components/store-threads/StoreThreads.stories.svelte catalog/components/store-application/AppearanceDiscussions.stories.svelte
```

**Notes**

Final controls, keyboard, dark mode, native replies and agent Focus/Split draft-retention acceptance: 8/8 across three files, including axe; /tmp/jj-appearance-feature-final.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-mutations/StoreMutationCommitPicker.stories.svelte catalog/components/store-application/StoreComparisonParentPicker.stories.svelte
```

**Notes**

Read-only untouched-main baseline: 2 failed/2 passed with the same aria-controls violations; /tmp/jj-appearance-baseline-pickers.log. Existing accessibility was not suppressed.
<!-- /run -->
<!-- /evidence-field -->

