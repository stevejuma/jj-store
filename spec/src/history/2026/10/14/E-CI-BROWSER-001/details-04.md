---
schema: 1
id: E-CI-BROWSER-001
---

# E-CI-BROWSER-001 details 4

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
node --test scripts/ci.test.mjs (/tmp/jj-store-ci-tooling-final.log)
```

**Notes**

13 real config/child-process CI tests passed, including disjoint cold
directories, split lane forwarding, same-run resources and failed predecessor
gates.
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
pnpm web:check; pnpm catalog:check; pnpm check:architecture (final prompt/menu production source)
```

**Notes**

Both type checks reported zero errors and warnings; catalog boundaries 442
modules and architecture 1903 modules. No source limit or architectural baseline
increased.
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
JJ_STORE_DOC_REFERENCE=$PWD/.local/build-targets/public-release/debug/examples/docs_reference pnpm web:build (/tmp/jj-store-ci-web-final4.log)
```

**Notes**

Asset build completed but Help staging rejected the stale application
documentation fingerprint, as required.
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
pnpm docs:review application <specific unchanged-doc explanation>; JJ_STORE_DOC_REFERENCE=$PWD/.local/build-targets/public-release/debug/examples/docs_reference pnpm web:build (/tmp/jj-store-ci-web-final5.log and final6.log)
```

**Notes**

Rebuilt final prompt and legacy-context projection, recorded raw-copy menu and
task-scroll source. Help closure 143 files, 26066173 bytes. Existing released
dependency CSS minifier warnings remain; optimized JavaScript/CSS warnings were
not suppressed.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
CI=true JJ_STORE_CI_PREBUILT=1 pnpm exec playwright test agent-prompt-markup.spec.ts --config web/playwright.config.ts (/tmp/jj-store-ci-prompt-exact14.log and exact15.log)
```

**Notes**

One failure in each diagnostic: retired virtual-row locator, then missing
recorded menu. Replaced locator with actual immutable message row and restored
existing menu integration. No folding, remeasurement, exact copy, safety or
accessibility assertions removed.
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
CI=true JJ_STORE_CI_PREBUILT=1 pnpm exec playwright test agent-prompt-markup.spec.ts --config web/playwright.config.ts (/tmp/jj-store-ci-prompt-exact16.log)
```

**Notes**

Five cold served tests passed in 11.1s. Classic and Document, desktop and touch
mobile, activity and legacy chunks retain nested Markdown, disclosure sizing,
raw copy and accessible safe inert tags.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
CI=true pnpm test:storybook --shard=N/3 (initial local parallel diagnostics; /tmp/jj-store-ci-storybook-final-{1,2,3}.log)
```

**Notes**

Shard 1: 92 passed then old immediate clipboard-spy assertion failed. Shard 2:
old root-navigation/editor assertion failed. Shard 3 was interrupted after a
ResizeObserver diagnostic. Separate hosted runners avoid shared local Vite
caches; interrupted work is not acceptance.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
CI=true pnpm test:storybook catalog/components/store-application/StoreFileEditor.stories.svelte catalog/components/store-application/NativeAgentMenus.stories.svelte (/tmp/jj-store-ci-storybook-editor-menu-exact1.log through exact5.log)
```

**Notes**

Five exact diagnostic attempts retained the original assertions. Canonical draft
guards intentionally block user navigation; tests now verify draft retention and
explicitly cancel. Committed mutation adoption was incorrectly blocked only in
the catalog fixture and now uses its internal successful-publication path, like
production. Removed a temporary effect-order experiment and all diagnostic
source logging.
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
CI=true pnpm test:storybook catalog/components/store-application/StoreFileEditor.stories.svelte catalog/components/store-application/NativeAgentMenus.stories.svelte (/tmp/jj-store-ci-storybook-editor-menu-exact6.log)
```

**Notes**

Three exact stories passed in 15.38s: original edit/rename publication and
immutable rendering checks, draft navigation guards and native exact clipboard
identifiers.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit catalog/controller/store-controller.test.ts web/tests/vite/conversation-projection.test.ts (/tmp/jj-store-ci-controller-projection-final.log)
```

**Notes**

19 tests passed, including guarded committed destination adoption and exact
non-commentable legacy prompt source.
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
CI=true JJ_STORE_CI_PREBUILT=1 pnpm exec playwright test appearance.spec.ts --config web/playwright.config.ts (/tmp/jj-store-ci-appearance-exact1.log)
```

**Notes**

Current Appearance now uses Theme/Preferences tabs; the mobile test expected the
removed navigation heading. Geometry, persisted preferences, multi-tab
propagation, CSP and accessibility assertions retained.
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
CI=true JJ_STORE_CI_PREBUILT=1 pnpm exec playwright test appearance.spec.ts --config web/playwright.config.ts (/tmp/jj-store-ci-appearance-exact2.log)
```

**Notes**

Original appearance acceptance passed in 5.5s through current tabs and
accessible section navigation.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 7
hasNote: true
noteEncoding: markdown

```

```shell
CI=true JJ_STORE_CI_PREBUILT=1 pnpm exec playwright test automatic-refresh.spec.ts --config web/playwright.config.ts (/tmp/jj-store-ci-automatic-exact1.log through exact6.log and diagnostic.log)
```

**Notes**

Diagnostics identified fixture transport mismatch: production uses admitted
application events, while the test emitted only legacy repository frames. Socket
interception was inapplicable to the injected HTTP seam. The corrected test uses
admitted application SSE witnesses; historical views observe the changed head
without redundant immutable reference reads. One setup attempt found an owned
orphan preview listener after a diagnostic interrupt; it was stopped before
retry.
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
CI=true JJ_STORE_CI_PREBUILT=1 pnpm exec playwright test automatic-refresh.spec.ts --config web/playwright.config.ts (/tmp/jj-store-ci-automatic-exact7.log)
```

**Notes**

Both exact current and historical scenarios passed in 15.3s. Current selection
follows the new head; historical selection stays exact and cached.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
CI=true JJ_STORE_CI_PREBUILT=1 pnpm exec playwright test --config web/playwright.config.ts (/tmp/jj-store-ci-served-final.log, final2.log and final3.log)
```

**Notes**

Sequential complete-suite diagnostics stopped at retired Appearance controls and
the obsolete automatic-refresh fixture. A third repeat was interrupted before
completion and does not qualify. Final complete diagnostics continue separately.
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
CI=true JJ_STORE_CI_PREBUILT=1 JJ_STORE_REAL_SHARD=2/4 JJ_STORE_DEV_PORT=7530 JJ_STORE_DEV_RUSTFS_RUNTIME=<owned same-source runtime> JJ_STORE_ACP_EXECUTABLE=<published 0.1.0> JJ_STORE_AGENT_JJ_EXECUTABLE=<owned custom client> pnpm web:test:real (/tmp/jj-store-ci-cold-final-2.log)
```

**Notes**

Seven passed, one specialized-fixture skip, 58.0s browser execution. Fresh
isolated RustFS/storage and native current tools; owned fixture stopped and
removed after execution.
<!-- /run -->
<!-- /evidence-field -->
