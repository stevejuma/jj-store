---
schema: 1
id: E-CI-BROWSER-001
---

# E-CI-BROWSER-001 details 7

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
CI=true JJ_STORE_CI_PREBUILT=1 pnpm exec playwright test --config web/playwright.config.ts threads-alignment.spec.ts --max-failures=1 (/tmp/jj-store-ci-thread-focus-after.log)
```

**Notes**

The original selected-reply focus assertion passed after focusing the shared
navigable item. The later create step still targeted the retired New
thread/Thread name UI. This run is not final acceptance.
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
CI=true JJ_STORE_CI_PREBUILT=1 pnpm exec playwright test --config web/playwright.config.ts threads-alignment.spec.ts --max-failures=1 (/tmp/jj-store-ci-thread-focus-after2.log)
```

**Notes**

Complete cold served Threads scenario passed: exact copied IDs/revision,
embedded details, rename, native references, immutable edit, reply/reaction,
scoped search, selected-reply keyboard focus, reload/independent panel widths,
current installation creation, pointer restoration and accessibility, including
the open creation dialog. The fixture now admits the current installation-owned
creation endpoint rather than inventing an old repository command.
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
CI=true JJ_STORE_CI_PREBUILT=1 pnpm exec playwright test --config web/playwright.config.ts agent-tool-rich.spec.ts --max-failures=1 (/tmp/jj-store-ci-tool-reader-after12.log)
```

**Notes**

Restored captured grammars exposed insufficient application-owned GitHub
syntax-palette contrast on the muted source background. WCAG assertions were
retained.
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
CI=true JJ_STORE_CI_PREBUILT=1 pnpm exec playwright test --config web/playwright.config.ts agent-tool-rich.spec.ts --max-failures=1 (/tmp/jj-store-ci-tool-reader-after13.log)
```

**Notes**

Cold served captured-tool case passed with exact native feedback text parts,
standalone syntax, independent raw/rich state, both edited source versions,
full-width sidebar/inbox presentation, event markers and accessibility. The
owned muted surface uses public Mira token variables mixed toward theme
foreground; immutable text and source maps remain unchanged.
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
CI=true pnpm exec vitest run --config catalog/vitest.config.ts --project storybook AgentReplyIndependence.stories.svelte AgentChatBody.stories.svelte --reporter=verbose (/tmp/jj-store-ci-captured-contrast-final13.log)
```

**Notes**

27 final interaction/accessibility checks passed, covering the final
muted-source syntax palette and colored comment/diff ranges, raw disclosure,
comments and read sidebar.
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
CI=true pnpm exec vitest run --config catalog/vitest.config.ts --project storybook --shard=1/3, --shard=2/3, --shard=3/3 --reporter=verbose (/tmp/jj-store-ci-storybook-shard1-12.log, -shard2-10.log, -shard3-12.log)
```

**Notes**

All 198 files accepted across disjoint shards: 334 + 326 + 309 = 969 tests.
Final captured-reader CSS refinement additionally passed its complete 27-test
interaction family above. Shard 2 precedes the reply-focus leaf; its exact cold
served scenario passed separately. Native ResizeObserver messages and one
existing highlighting-tag warning remain visible in runner output; no filters or
test skips were added to conceal them.
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
pnpm web:check; pnpm catalog:check; pnpm check:architecture; pnpm check:ui-ownership (/tmp/jj-store-ci-*-final12.log)
```

**Notes**

Both type checks report zero errors/warnings; catalog boundaries 442 modules and
architecture 1903 modules. UI ownership passes.
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
pnpm web:test (/tmp/jj-store-ci-web-tests-final12.log)
```

**Notes**

All 180 web Node tests passed.
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
pnpm format:rust:check; node --test scripts/ci.test.mjs scripts/automation-governance.test.mjs scripts/release.test.mjs; actionlint (/tmp/jj-store-ci-rust-format12.log, -tooling-final12.log, -actionlint-final12.log)
```

**Notes**

Rust formatting, all 21 orchestration/release/automation tests and workflow lint
passed.
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
pnpm build-storybook; CI=true JJ_STORE_CI_PREBUILT=1 node --test scripts/catalog-browser.test.mjs (/tmp/jj-store-ci-catalog-build12.log, /tmp/jj-store-ci-catalog-browser-final13.log)
```

**Notes**

Static catalog built; its browser smoke passed one test. Static artifacts
precede the final muted-source CSS refinement, whose fresh web bundle and
focused interaction family are separately qualified.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 14
hasNote: true
noteEncoding: markdown

```

```shell
CI=true JJ_STORE_CI_PREBUILT=1 pnpm exec playwright test --config web/playwright.config.ts --max-failures=0 (/tmp/jj-store-ci-served-final13.log)
```

**Notes**

100 passed. All product history/query/capture/grammar/focus regressions passed.
Remaining failures are retired refresh labels, Settings categories/secondary
tabs, retained hidden-card geometry, old global-dialog/search mocks, completed
Settings return-route assertions, and old message-bubble locators. Each original
target remains selected for focused correction; this is not final complete
served acceptance.
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
CI=true JJ_STORE_CI_PREBUILT=1 JJ_STORE_REAL_TEST=repository-settings.spec.ts JJ_STORE_DEV_PORT=7550 JJ_STORE_DEV_RUSTFS_RUNTIME=<isolated regular runtime> JJ_STORE_ACP_EXECUTABLE=/tmp/jj-acp-0.1.0-aarch64-apple-darwin/jj-acp pnpm web:test:real (/tmp/jj-store-ci-cold-settings-final13.log)
```

**Notes**

Fresh real native server/RustFS with final13 browser/Help assets: four passed,
one specialized unavailable-upstream fixture skip. Cold startup 3324ms; physical
restart 752ms; 278528 KiB resident. Full owned fixture preparation 212202ms.
Settings exact route, historical snapshot, reload/back, global Settings return
and folder drafts passed. The runner stopped its owned server. Interactive
.local/dev-demo was not used.
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
pnpm spec:validate --only governance,summary,fileLimits,verification; pnpm check:progress; pnpm spec:first (/tmp/jj-store-ci-spec-final15.log, -progress-final15.log, -spec-first-final15.log)
```

**Notes**

3847 chapters, 198 requirements, 155 tasks and 807 evidence records; 122
protected files and 19 canonical chapters. This intermediate snapshot precedes
final conversion, workflow and evidence edits; final static checks are recorded
separately.
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
pnpm docs:check; pnpm release:check; pnpm spec:build (/tmp/jj-store-ci-docs-final15.log, -release-final15.log, -spec-book-final16.log)
```

**Notes**

Documentation closure: 143 files and valid links/anchors. All 21
release/orchestration tests and all 20 portable Cargo packages at coordinated
0.1.0 passed. Specification book built with the existing 56.7 MB search-index
warning. These are local checks, not hosted release or publication acceptance.
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
CI=true JJ_STORE_CI_PREBUILT=1 pnpm exec playwright test --config web/playwright.config.ts repository-conversion.spec.ts --max-failures=1 (/tmp/jj-store-ci-conversion-reactivation-before.log)
```

**Notes**

Restoring an unknown conversion receipt issued six reads in 700ms. The observer
construction synchronously tracked its own busy flag, causing repeated teardown
and resubscription. Exact pending request identity and no-replay assertions
remained selected.
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
CI=true JJ_STORE_CI_PREBUILT=1 pnpm exec playwright test --config web/playwright.config.ts repository-conversion.spec.ts --max-failures=1 (/tmp/jj-store-ci-conversion-reactivation-after19.log)
```

**Notes**

All three cold served conversion cases passed against the final19 browser/Help
bundle. Observer construction now uses untrack: one restored receipt read, then
idle. Exact retry, status, verified report and dismissed-error lifecycle
assertions passed.
<!-- /run -->
<!-- /evidence-field -->
