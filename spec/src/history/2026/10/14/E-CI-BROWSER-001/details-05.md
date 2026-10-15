---
schema: 1
id: E-CI-BROWSER-001
---

# E-CI-BROWSER-001 details 5

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
CI=true JJ_STORE_CI_PREBUILT=1 JJ_STORE_REAL_SHARD=4/4 JJ_STORE_DEV_PORT=7560 JJ_STORE_DEV_RUSTFS_RUNTIME=<owned same-source runtime> JJ_STORE_ACP_EXECUTABLE=<published 0.1.0> JJ_STORE_AGENT_JJ_EXECUTABLE=<owned custom client> pnpm web:test:real (/tmp/jj-store-ci-cold-final-4.log)
```

**Notes**

Four applicable tests passed, five fixture-specific skips, 58.6s browser
execution. Task reaction cross-client freshness, WorkStore and additive samples
remain asserted; original separate parents retain their own qualification.
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
CI=true JJ_STORE_CI_PREBUILT=1 JJ_STORE_REAL_SHARD=3/4 JJ_STORE_DEV_PORT=7550 pnpm web:test:real (/tmp/jj-store-ci-cold-final-3.log)
```

**Notes**

The project-polish case observed Loading project details after filtering and
exhausted the ordinary 5s display assertion during concurrent fixture load. It
now waits for the existing bounded 30s read completion before asserting
activity, filters, exact navigation and accessibility; full fresh shard rerun
remains required.
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
CI cold shards 1/4, 2/4, 3/4, 4/4 with final7 assets (logs /tmp/jj-store-ci-cold-final-1.log, -2.log, /tmp/jj-store-ci-cold-final3-3.log, /tmp/jj-store-ci-cold-final-4.log)
```

**Notes**

27 passed, 16 specialized fixture skips across four complete native shards,
including the actual five-minute idle/renewal. Profile restart and Git-cycle
remain separate parent fixtures. These assets precede guarded browser history,
retained index query context and capture reactivation fixes; this proves the
earlier native source boundary, not final rebuilt acceptance. Owned fixtures
were stopped.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook <ten failing files> (/tmp/jj-store-ci-storybook-failures-exact2.log)
```

**Notes**

44 tests in ten files passed, including prompt projection, loading labels,
initial project loading and tab resize interaction. This exact rerun is not full
Storybook qualification.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 42
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec playwright test --config web/playwright.config.ts --max-failures=0 (diagnostic /tmp/jj-store-ci-served-diagnostic-all.log)
```

**Notes**

82 passed, 42 failed. Retired Settings, connection-storage, footer, prompt,
Threads and layout assumptions were identified alongside genuine history/query
recovery defects. One final diagnostic case overlapped final7 staging, so this
run is diagnostic only.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/store-route-history.test.ts web/tests/vite/projects-code-navigation.test.ts (/tmp/jj-store-ci-browser-back-after.log)
```

**Notes**

26 tests passed. A rejected identified browser traversal returns to its original
entry without overwriting the destination; duplicate traversal notifications
retain draft guards. Before state reproduced corruption of the root entry.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit <five index/route files> (/tmp/jj-store-ci-index-context-after.log)
```

**Notes**

43 tests passed. Settings index preparation retains graph time and history email
for the original query; ordinary navigation continues clearing its context.
Before state observed null time and empty email.
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
pnpm docs:review application <guarded history, index context and inbox domain review>; JJ_STORE_DOC_REFERENCE=.local/build-targets/public-release/debug/examples/docs_reference pnpm web:build (/tmp/jj-store-ci-web-final8.log)
```

**Notes**

Fresh complete browser and Help assets include history/query recovery and
domain-specific inbox loading. Inherited released dependency CSS warnings
remain. The subsequent capture reactivation repair requires another fresh build.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
CI=true JJ_STORE_CI_PREBUILT=1 pnpm exec playwright test --config web/playwright.config.ts file-editor-navigation.spec.ts reads.spec.ts local-capture-settings.spec.ts threads-alignment.spec.ts agent-fleet.spec.ts agent-tool-rich.spec.ts --max-failures=0 (/tmp/jj-store-ci-served-focused8.log)
```

**Notes**

18 passed. Both file-editor return paths and all reads, including
timestamp-preserving index retry and unavailable optional metadata, passed on
the rebuilt bundle. Five remaining old-fixture/activation cases retained for
diagnosis.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentInboxTree.stories.svelte (/tmp/jj-store-ci-inbox-label8.log)
```

**Notes**

Eight interaction/accessibility stories passed, including the new
domain-specific Threads loading announcement and existing Session label.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit (/tmp/jj-store-ci-units-final8.log)
```

**Notes**

178 files; 1064 passed, one specialized fixture skip. This precedes capture
reactivation, which has an exact before/after regression and requires final
affected-unit requalification.
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
pnpm web:check; pnpm catalog:check (/tmp/jj-store-ci-web-check-final8.log and /tmp/jj-store-ci-catalog-check-final8.log)
```

**Notes**

Both checks: zero errors and warnings; catalog boundaries 442 modules.
Subsequent fixture/activation edits remain selected for final checks.
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
node --test scripts/ci.test.mjs scripts/automation-governance.test.mjs scripts/release.test.mjs; actionlint (/tmp/jj-store-ci-orchestration-final8.log and /tmp/jj-store-ci-actionlint-final8.log)
```

**Notes**

21 orchestration/release/automation tests passed; actionlint passed. Disposable
Git fixtures disable inherited commit signing only for their own Git
invocations. Release/repository signing is unchanged.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook (/tmp/jj-store-ci-storybook-final8.log; terminated after observer failure)
```

**Notes**

Full diagnostic run emitted an uncaught ResizeObserver-loop error and was
terminated at that failure. Remaining tests are not accepted; final
qualification uses the same three disjoint shards as CI with immediate failure
reporting.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/capture-event-scope.test.ts (/tmp/jj-store-ci-capture-reopen-before.log)
```

**Notes**

A healthy application feed missed the status/source observation after Capture
closed, received a notice, and reopened. The previous empty interests survived
subscription replacement.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/capture-event-scope.test.ts web/tests/vite/capture-freshness.test.ts (/tmp/jj-store-ci-capture-reopen-after.log)
```

**Notes**

11 tests passed after resetting scoped interests for each newly active
subscription. Healthy unchanged open views remain idle; closed owners make no
reads and exact commands/drafts retain installation ownership.
<!-- /run -->
<!-- /evidence-field -->
