---
schema: 1
id: E-UI-CI-001
---

# E-UI-CI-001 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 6
hasNote: true
noteEncoding: markdown

```

```shell
gh run view 38737694885 --repo stevejuma/jj-store; downloaded UI JUnit reports
```

**Notes**

Hosted baseline at source 2d9cdf2063e5a64c6a8b81df7b8cfdf87032a87e and producer 38730859426/1: 1098 passed and 6 failed out of 1104 checks. Failures: MobileProjectRepositoryActions, EditAndRename, prompt disclosure, history completion, Search settings geometry and compact native IDs. Prior record E-CI-SUITES-MEASURE-001 remains unchanged.
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
CI=true JJ_STORE_CI_PREBUILT=1 pnpm exec playwright test --config web/playwright.config.ts history-query.spec.ts
```

**Notes**

The initial narrow test passed, but waiting for the debounced mut query and settled loading deterministically exposed completion reset. That strengthened regression failed before stabilization of the bookmark vocabulary, then passed on macOS and Linux Chromium. Before/after logs: /tmp/jj-ui-history-regression-before.log and /tmp/jj-ui-history-regression-after.log.
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
PW_TEST_CONNECT_WS_ENDPOINT=ws://127.0.0.1:3300/ PW_TEST_CONNECT_EXPOSE_NETWORK='<loopback>' CI=true JJ_STORE_CI_PREBUILT=1 pnpm exec playwright test --config web/playwright.config.ts agent-prompt-markup.spec.ts
```

**Notes**

Linux Chromium reproduced the toolbar intercept. Invisible actions no longer take pointer events, and the context toolbar sits above the first disclosure. The final focused test passed with its existing click and sanitization assertions. /tmp/jj-ui-prompt-linux-final.log.
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
pnpm test:storybook catalog/components/store-application/StoreFileEditor.stories.svelte -t EditAndRename
```

**Notes**

Linux focused EditAndRename passed after measuring the real scroll client viewport, excluding the 15px platform scrollbar while retaining the 16px padding limit.
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
pnpm test:storybook catalog/components/store-work/ProjectsRepositoryActions.stories.svelte -t MobileProjectRepositoryActions
```

**Notes**

Linux focused mobile project actions passed after waiting for keyboard focus before Enter; no interaction assertion removed. /tmp/jj-ui-mobile-story.log.
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

Zero errors and warnings.
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
pnpm catalog:check
```

**Notes**

Zero errors and warnings; 442 catalog modules.
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
pnpm web:test
```

**Notes**

180 tests passed.
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
pnpm catalog:test
```

**Notes**

1065 passed; one existing skipped test across 178 files.
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
pnpm check:architecture
```

**Notes**

1921 modules passed.
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
pnpm check:ui-ownership
```

**Notes**

Owned production UI boundary passed.
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
pnpm test:release
```

**Notes**

53 orchestration, provenance, selection and release-caller regressions passed. Tests reject missing or failed UI producers, wrong source/attempt, incomplete receipts and skipped required UI. Complete release gate remains mandatory. /tmp/jj-ui-tooling-final.log.
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
pnpm exec actionlint
```

**Notes**

Workflow lint passed.
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
CI=true JJ_STORE_CI_PREBUILT=1 JJ_STORE_CI_REPORTS=1 PLAYWRIGHT_JUNIT_OUTPUT_FILE=/tmp/jj-ui-browser-full.xml pnpm exec playwright test --config web/playwright.config.ts
```

**Notes**

First full served-browser run: 113 passed and 1 failed in 8.8m. The details rail was already restored open, and the test toggled it closed after reload. Repaired test waits for connection-scoped layout persistence and asserts the restored expanded state and commands. /tmp/jj-ui-browser-full.log.
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
CI=true JJ_STORE_CI_PREBUILT=1 pnpm exec playwright test --config web/playwright.config.ts context-actions.spec.ts
```

**Notes**

Final focused context-action tests: 4 passed in 9.0s. Initial test repair used the unscoped storage key and failed; diagnostic inspection established the correct connection-scoped key. Original mutation, selection and navigation assertions remain.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_CI_REPORTS=1 pnpm test:storybook
```

**Notes**

Interrupted the unsharded local run (exit 130) after memory contention with static compilation. No pass claimed. Replaced with the complete three CI shards executed sequentially locally.
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
JJ_STORE_CI_REPORTS=1 pnpm test:storybook --shard=1/3
```

**Notes**

334 passed across 66 files in 383.66s. Vite emitted a ResizeObserver delivery diagnostic but Vitest exited successfully with no failed tests. The final compact-ID gap was separately proved on both platforms and is checked against final static assets.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
node --test scripts/catalog-browser.test.mjs
```

**Notes**

Intermediate compact width change met the width bound but exposed a 0.125px clipping limit failure on macOS. Reduced internal label-to-ID gap; preserve original maximum width, clipping, padding, hover, touch and exact clipboard assertions.
<!-- /run -->
<!-- /evidence-field -->

