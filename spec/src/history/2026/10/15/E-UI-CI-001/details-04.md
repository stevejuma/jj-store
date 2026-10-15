---
schema: 1
id: E-UI-CI-001
---

# E-UI-CI-001 details 4

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
JJ_STORE_CI_REPORTS=1 pnpm test:storybook --shard=3/3
```

**Notes**

Complete diagnostic third shard: 308 passed / 1 failed (NativeCopyReplyAndOwnAuthorEdit); all disclosure and review-preference scenarios passed. Reply targeted a hidden toolbar before the existing 180ms hover delay. Added the same data-actions-visible wait already used by the story menu helper. Temporary diagnostic error formatting removed. /tmp/jj-ui-story-shard3-diagnostic.log.
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
pnpm test:storybook catalog/components/store-application/NativeMessageActions.stories.svelte
```

**Notes**

Three native-message scenarios passed before and after the explicit hover-readiness assertion on fresh runs (10.50s and 10.43s). The broader failure is retained; complete third-shard rerun still required. /tmp/jj-ui-native-actions-exact.log and /tmp/jj-ui-native-actions-after.log.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress; pnpm exec prettier --check <changed files>
```

**Notes**

Intermediate ledger check identified temporary diagnostic setup and two newly changed files not yet listed in evidence. Changed-file formatting identified generated M30 history. Temporary setup was restored and final file inventory/history formatting will be refreshed; no product failure from these bookkeeping checks.
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
CI=true JJ_STORE_CI_PREBUILT=1 pnpm exec playwright test --config web/playwright.config.ts agent-prompt-markup.spec.ts local-capture-settings.spec.ts -g "cold capture controls preserve intent and snapshot|prompt"
```

**Notes**

All six focused browser cases passed on final built application assets, including classic/document desktop and touch prompt disclosures and exactly one import command through automatic receipt recovery. 16.6s; /tmp/jj-ui-prompt-capture-final.log.
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
JJ_STORE_CI_REPORTS=1 pnpm test:storybook --shard=3/3
```

**Notes**

Second complete third shard: 308 passed / 1 failed in 297.73s. Native message hover, disclosure persistence and review preference cases passed. FocusAndSplitPreserveToolDiscussionDrafts synchronously queried a virtualized tool row after the sidebar shell appeared. The failure DOM contained the mounted but still-empty conversation list. /tmp/jj-ui-story-shard3-final.log.
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
pnpm test:storybook catalog/components/store-application/AppearanceDiscussions.stories.svelte
```

**Notes**

The exact Focus/Split story passed after awaiting the virtualized tool row and its editor. Draft equality, surface count, cancellation and existing-message assertions remain intact. 14.20s; /tmp/jj-ui-appearance-discussion-after.log.
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
pnpm catalog:check; pnpm check:progress; pnpm spec:first; pnpm spec:validate --only governance,summary,fileLimits,verification; JJ_STORE_DOC_REFERENCE=$PWD/.local/build-targets/public-release/debug/examples/docs_reference pnpm docs:check
```

**Notes**

Catalog: zero errors/warnings, 442 modules. Progress: 155 tasks and 818 records. Spec-first: 33 protected files/eight canonical chapters. Four validators: 3895 chapters/198 requirements. Help: nine diagrams and 143 files with links/anchors valid. Logs /tmp/jj-ui-*-final.log. Final evidence inventory will be revalidated after adding the last story helper.
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
pnpm build-storybook && node --test scripts/catalog-browser.test.mjs
```

**Notes**

Final static catalog build includes both added story readiness assertions. Complete cold_catalog_contract passed in 16.51s with no skipped checks. /tmp/jj-ui-catalog-build-verified.log and /tmp/jj-ui-static-verified.log.
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
CI=true JJ_STORE_CI_PREBUILT=1 JJ_STORE_CI_REPORTS=1 PLAYWRIGHT_JUNIT_OUTPUT_FILE=/tmp/jj-ui-browser-verified.xml pnpm exec playwright test --config web/playwright.config.ts
```

**Notes**

Complete served-browser run: 112 passed / two failed (comparison-parent route assertion and workspace pagination) in 4.3m. The former sampled the URL before the selection read committed; changed to await the exact parent URL. Trace of the latter shows workspace cursor request aborted by automatic freshness while pagination was active. /tmp/jj-ui-browser-verified.log. These were missed by intermediate tail-only progress updates; the complete result is authoritative.
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
CI=true JJ_STORE_CI_PREBUILT=1 pnpm exec playwright test --config web/playwright.config.ts comparison-parents.spec.ts workspaces.spec.ts
```

**Notes**

Four original navigation cases (including local-workspaces matched by the filter) passed on a focused pre-fix rerun in 15.4s, confirming timing-dependent symptoms. /tmp/jj-ui-navigation-exact.log.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-automatic-refresh.test.ts
```

**Notes**

Added deterministic tests reproduce premature automatic refresh during workspace and bookmark pagination: two failed, eight passed against the previous controller. /tmp/jj-ui-reference-refresh-before.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-automatic-refresh.test.ts web/tests/vite/production-reference-reader.test.ts web/tests/vite/production-store-workspaces.test.ts web/tests/vite/automatic-refresh.test.ts
```

**Notes**

33 focused tests passed after deferring automatic refresh while explicit reference pagination is refreshing. Scheduler retries are retained; Projects/Threads do not wait on stale code-view statuses. /tmp/jj-ui-reference-refresh-after.log.
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
JJ_STORE_CI_REPORTS=1 pnpm test:storybook --shard=3/3
```

**Notes**

Complete final third shard: all 309 tests across 66 files passed in 352.27s. Combined disjoint Storybook inventory: 334 + 326 + 309 = 969 tests. Existing ResizeObserver delivery warnings remain visible; Vitest exited zero. /tmp/jj-ui-story-shard3-verified.log.
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
pnpm web:check; pnpm web:test; pnpm catalog:test
```

**Notes**

After the reference freshness guard: web type check zero errors/warnings; all 180 web unit tests passed; 1068 catalog tests passed across 178 files, retaining the one existing skip. /tmp/jj-ui-web-check-refresh.log, /tmp/jj-ui-web-unit-refresh.log, /tmp/jj-ui-catalog-unit-refresh.log.
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
JJ_STORE_DOC_REFERENCE=$PWD/.local/build-targets/public-release/debug/examples/docs_reference pnpm web:build
```

**Notes**

Fresh application and Help assets built after the reference-refresh guard. Nine Mermaid diagrams rendered; 143 Help files with links/anchors valid. /tmp/jj-ui-web-build-refresh.log.
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
CI=true JJ_STORE_CI_PREBUILT=1 pnpm exec playwright test --config web/playwright.config.ts comparison-parents.spec.ts workspaces.spec.ts automatic-refresh.spec.ts
```

**Notes**

All six original navigation and automatic-refresh browser cases passed on rebuilt assets in 24.6s. Historical/current selection and exact workspace/parent identities retained. /tmp/jj-ui-navigation-refresh-after.log.
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
CI=true JJ_STORE_CI_PREBUILT=1 JJ_STORE_CI_REPORTS=1 PLAYWRIGHT_JUNIT_OUTPUT_FILE=/tmp/jj-ui-browser-final-proof.xml pnpm exec playwright test --config web/playwright.config.ts
```

**Notes**

All 114 served-browser tests passed on the final rebuilt application in 3.7m, with no retries or skipped tests. /tmp/jj-ui-browser-final-proof.log and XML. Complete local core UI inventory is 969 Storybook + 114 served browser + 20 Help + one static catalog = 1104 passing checks across separate commands.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:architecture; pnpm check:ui-ownership; pnpm check:progress; pnpm spec:first
```

**Notes**

Final production architecture: 1921 modules. UI ownership passed. Evidence ledger: 155 tasks/818 records. Spec-first: 37 protected files/eight canonical chapters. Main remains aeb3e95e68d3; administrator enforcement and strict required check remain enabled.
<!-- /run -->
<!-- /evidence-field -->

