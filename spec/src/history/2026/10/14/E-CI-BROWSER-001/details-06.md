---
schema: 1
id: E-CI-BROWSER-001
---

# E-CI-BROWSER-001 details 6

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
CI=true JJ_STORE_CI_PREBUILT=1 pnpm exec playwright test --config web/playwright.config.ts <focused affected conversation/capture files> --max-failures=0 (/tmp/jj-store-ci-served-focused9.log)
```

**Notes**

4 passed. Current footer scopes, collapsed harness state and healthy event
transport still required fixture alignment. Test infrastructure was still under
repair; final complete runs remain required.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
CI=true JJ_STORE_CI_PREBUILT=1 pnpm exec playwright test --config web/playwright.config.ts <focused affected conversation/capture files> --max-failures=0 (/tmp/jj-store-ci-served-focused10.log)
```

**Notes**

4 passed. Legacy tool-group and Threads geometry selectors remained. Source
snippets were edited during diagnostic execution, so this run is not final
acceptance. Test infrastructure was still under repair; final complete runs
remain required.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
CI=true JJ_STORE_CI_PREBUILT=1 pnpm exec playwright test --config web/playwright.config.ts <focused affected conversation/capture files> --max-failures=0 (/tmp/jj-store-ci-served-focused11.log)
```

**Notes**

5 passed. Footer chat and source-control cases passed. Capture reactivation,
selected tool evidence and thread presentation remained selected. Test
infrastructure was still under repair; final complete runs remain required.
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
CI=true JJ_STORE_CI_PREBUILT=1 pnpm exec playwright test --config web/playwright.config.ts <focused affected conversation/capture files> --max-failures=0 (/tmp/jj-store-ci-served-focused12.log)
```

**Notes**

4 passed. Configuration draft, numeric, saved-state and accessibility case
passed. Reopening Capture still failed on the older final8 bundle. Test
infrastructure was still under repair; final complete runs remain required.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
CI=true JJ_STORE_CI_PREBUILT=1 pnpm exec playwright test --config web/playwright.config.ts <focused affected conversation/capture files> --max-failures=0 (/tmp/jj-store-ci-served-focused13.log)
```

**Notes**

4 passed. Captured syntax, old transient rescan disablement and retired
scroll-shell selector exposed. Capture reactivation passed on rebuilt final9.
Test infrastructure was still under repair; final complete runs remain required.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
CI=true JJ_STORE_CI_PREBUILT=1 pnpm exec playwright test --config web/playwright.config.ts <focused affected conversation/capture files> --max-failures=0 (/tmp/jj-store-ci-served-focused14.log)
```

**Notes**

4 passed. Remaining captured-code highlighting and retired import/scroll
selectors were diagnosed; a later fixture edit changed the failure snippet after
this import. Test infrastructure was still under repair; final complete runs
remain required.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
CI=true JJ_STORE_CI_PREBUILT=1 pnpm exec playwright test --config web/playwright.config.ts <focused affected conversation/capture files> --max-failures=0 (/tmp/jj-store-ci-served-focused15.log)
```

**Notes**

5 passed. All five Capture cases passed, including open-stream event delivery,
closure/reopening and exact unknown-command recovery. Remaining captured-file
path and reply-focus regressions were retained. Test infrastructure was still
under repair; final complete runs remain required.
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
pnpm add -D @lapismd/design-core@0.3.2 --save-prefix=^ --ignore-pnpmfile (/tmp/jj-store-ci-design-core-refresh.log)
```

**Notes**

Consumed the existing registry release with mobile shell-owner teleport cleanup;
public npm range and integrity-locked artifact remain portable. No sibling
source or publication changed. Inherited package build-policy notices were not
approved or bypassed.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit catalog/fixtures/delta-chat/captured-code.test.ts (/tmp/jj-store-ci-captured-grammar-before.log)
```

**Notes**

Cold dependency resolution found an uncompiled published Svelte export. Explicit
catalog-unit inlining of the released Design Core package repaired this tooling
boundary; no dependency source patch or replacement was used. No grammar
assertion ran in this attempt.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit catalog/fixtures/delta-chat/captured-code.test.ts (/tmp/jj-store-ci-captured-grammar-before2.log)
```

**Notes**

Five passed and the new TypeScript regression failed with a null grammar.
Captured readers had depended on grammar supplied only by editable file hosts.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit catalog/fixtures/delta-chat/captured-code.test.ts catalog/fixtures/delta-chat/adapter.test.ts (/tmp/jj-store-ci-captured-grammar-after.log)
```

**Notes**

15 passed. Captured readers reuse the existing standalone file grammar
selection; numbered source, Unicode/UTF-16 offsets, exact copy and unknown
plain-text formats remain intact.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit --maxWorkers=2 (/tmp/jj-store-ci-units-final10.log)
```

**Notes**

178 complete affected unit files: 1065 passed, one specialized fixture skip.
Includes capture reactivation and captured-reader grammar. Later reply-focus
leaf and CSS changes are qualified by their exact browser/interaction
boundaries.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
CI=true pnpm exec vitest run --config catalog/vitest.config.ts --project storybook --shard=<1,2,3>/3 --reporter=verbose (/tmp/jj-store-ci-storybook-shard*-9.log)
```

**Notes**

Diagnostic shards were terminated after native ResizeObserver messages. Those
messages alone were not demonstrated to be assertion failures: later exact story
runs exit zero while retaining the messages. No warning filter, diagnostic
suppression or baseline change was introduced. These terminated shards establish
no complete acceptance.
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
CI=true pnpm exec vitest run --config catalog/vitest.config.ts --project storybook WorkspaceFilePresentation.stories.svelte ApplicationMenu.stories.svelte --reporter=verbose (/tmp/jj-store-ci-shell-released10.log)
```

**Notes**

13 interactions/accessibility checks passed on the released shell dependency.
Native browser ResizeObserver messages remain emitted during viewport
relocation; they are recorded separately and are not a claimed repair. The
completed runner reports no failed assertions or unhandled-error gate.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
CI=true pnpm exec vitest run --config catalog/vitest.config.ts --project storybook --shard=1/3 and --shard=3/3 --reporter=verbose (/tmp/jj-store-ci-storybook-shard1-10.log and -shard3-10.log)
```

**Notes**

Each stopped at its first accessibility failure. Newly restored syntax colors
lacked contrast over selected/comment and diff ranges; the remaining files were
not accepted. Existing WCAG thresholds remained unchanged.
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
CI=true pnpm exec vitest run --config catalog/vitest.config.ts --project storybook AgentReplyIndependence.stories.svelte AgentChatBody.stories.svelte --reporter=verbose (/tmp/jj-store-ci-captured-contrast-after.log)
```

**Notes**

27 interaction/accessibility checks passed after owned tinted ranges retained
the theme foreground for nested syntax. This precedes the final muted-surface
palette refinement.
<!-- /run -->
<!-- /evidence-field -->
