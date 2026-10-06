---
schema: 1
id: E-M15-030
---

# E-M15-030 details 2

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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-threads/StoreThreads.stories.svelte -t 'Unknown reaction stays|Unknown reply reaction stays|Rejected reaction stays'
```

**Notes**

Setup filter skipped 7/7; not acceptance. /tmp/jj-reaction-row-before.log.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-threads/StoreThreads.stories.svelte -t 'Unknown.*[Rr]eaction|Rejected.*[Rr]eaction'
```

**Notes**

Before2: 3 exact regression failures. After: 2 failures plus one Bits derived_inert diagnostic; fixture reply flag was corrected to the intended main story and error tooltip tested with its actual content selector. After2: 1 tooltip assertion failure, 2 passed. After3: 3/3 pass. After4 failed parsing one missing closing token during simplification; corrected. Before/after logs retained.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-threads/StoreThreads.stories.svelte -t 'Unknown.*[Rr]eaction|Rejected.*[Rr]eaction'
```

**Notes**

Final after5: 3/3 pass, 4 skipped; no runtime diagnostics. Main and reply pending feedback, timestamp/reply ordering, icon-only retries, destructive failure status and accessible error details. /tmp/jj-reaction-row-after5.log.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-threads catalog/components/store-application/StoreCollaboration.stories.svelte catalog/components/store-application/NativeReviewDetail.stories.svelte catalog/components/store-application/NativeMessageActions.stories.svelte catalog/components/store-application/AppearanceDiscussions.stories.svelte catalog/components/store-work/StoreWork.stories.svelte
```

**Notes**

Intermediate 31/31 assertions but two transient recovery-tooltip teardown diagnostics. Replaced the disappearing recovery tooltips with ordinary titled icon buttons, retaining shadcn reaction/error tooltips. Next run: 30 passed/1 failed, no diagnostics; AppearanceDiscussions missed Appearance synchronously after switching settings. /tmp/jj-reaction-row-consumers{,-final}.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AppearanceDiscussions.stories.svelte
```

**Notes**

Exact rerun of combined Appearance failure: 1/1 passes without changes; /tmp/jj-reaction-row-appearance-retry.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/reaction-feedback.test.ts web/tests/vite/production-threads.test.ts
```

**Notes**

25/25, including retained uncertain request/retry identity. /tmp/jj-reaction-row-controller.log.
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
pnpm web:check; pnpm catalog:check
```

**Notes**

Final checks zero errors/warnings. Intermediate web check had three noninteractive-tabindex warnings; changed statuses to semantic tooltip triggers. Catalog boundaries 129 modules. /tmp/jj-reaction-row-{web-check-final,catalog-check-final}.log.
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
pnpm web:build
```

**Notes**

Initial build pass. Intermediate final failed one missing closing token during icon-button simplification; final2 rebuilt successfully after correction. Existing large-chunk advisory retained. /tmp/jj-reaction-row-build{,-final,-final2}.log.
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
pnpm exec playwright test --config /tmp/jj-thread-main-slices-playwright.config.mjs reaction-metadata.spec.ts
```

**Notes**

Cold final built bundle: 2/2. Held command verifies actual in-flight spinner, then unknown-outcome spinner with icon-only status/retry controls. Timestamp, reply avatars/count and feedback centers align within 2px, status last. Original request status and message body unchanged. /tmp/jj-reaction-row-cold{,-final}.log.
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
pnpm check:architecture; pnpm check:ui-ownership
```

**Notes**

Owned-source budgets and no Community dependencies pass; no new copied files or provenance removals. /tmp/jj-reaction-row-{architecture-final,ownership}.log.
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
JJ_THREAD_SLICE_EVIDENCE_PREFIX=/tmp/jj-thread-reaction-row-main node /tmp/jj-thread-main-slice-restart.mjs
```

**Notes**

Reuses inspected owned restart helpers and explicit native binary. Main PID46157-&gt;52603/storage52604,1333ms. Repository snapshots, selected thread/timeline, directory, fixture and installation unchanged; search ready/HTTP200. Served index matches rebuilt SHA256d348a8c941d04924bfcde2e43d4208fd70f1b9d5ec713875bfae950a3469f37d. /tmp/jj-reaction-row-restart.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-threads catalog/components/store-application/StoreCollaboration.stories.svelte catalog/components/store-application/NativeReviewDetail.stories.svelte catalog/components/store-application/NativeMessageActions.stories.svelte catalog/components/store-application/AppearanceDiscussions.stories.svelte catalog/components/store-work/StoreWork.stories.svelte
```

**Notes**

Second combined run 30/31 assertions, no diagnostics; NativeCopyReplyAndOwnAuthorEdit could not find its Copy comment ID submenu. /tmp/jj-reaction-row-consumers-qualified.log. Exact isolated retry also fails, proving it is not solely a combined-run timing issue.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/NativeMessageActions.stories.svelte -t NativeCopyReplyAndOwnAuthorEdit
```

**Notes**

Current isolated story fails opening Copy comment ID. Read-only parent-source comparison temporarily selected all changed web source bytes from @-, ran identical target and then restored every current source byte in finally, checking for concurrent edits before restore. Unchanged parent also fails at Copy comment ID. /tmp/jj-reaction-row-native-{actions-retry,parent}.log. Owning existing NativeMessageActions submenu acceptance remains separate; no test weakened.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-threads/StoreThreads.stories.svelte catalog/components/store-application/NativeMessageActions.stories.svelte -t 'Unknown.*[Rr]eaction|Rejected.*[Rr]eaction|NativeCopyReplyAndOwnAuthorEdit'
```

**Notes**

Dedicated metadata-slot implementation: all three exact reaction regressions pass; the existing copy-submenu failure remains. This motivated parent-source qualification above rather than another unqualified rerun. /tmp/jj-reaction-row-slots-exact.log.
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
pnpm spec:first; pnpm spec:validate; pnpm check:progress
```

**Notes**

Initial spec-first requires the mapped api.md chapter for protected web source; updated its UI/request semantics explicitly. Spec validate and progress pass. Final document checks follow.
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
pnpm web:check; pnpm catalog:check; pnpm web:build; pnpm check:architecture; pnpm check:ui-ownership
```

**Notes**

Final dedicated-slot source: both Svelte checks report zero errors/warnings, catalog boundaries pass, built assets/manifest pass, architecture budgets and owned Community-source independence pass. Existing build chunk advisory retained. /tmp/jj-reaction-row-slots-{web-check,catalog-check,build,architecture,ownership}.log.
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
pnpm exec playwright test --config /tmp/jj-thread-main-slices-playwright.config.mjs reaction-metadata.spec.ts
```

**Notes**

Final dedicated-slot built bundle: 2/2 cold main/reply tests, including actual in-flight spinner and subsequent uncertain-outcome recovery, same-row geometry and original request identity/body preservation. /tmp/jj-reaction-row-slots-cold.log. Isolated preview process cleaned up.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-threads catalog/components/store-application/StoreCollaboration.stories.svelte catalog/components/store-application/NativeReviewDetail.stories.svelte catalog/components/store-application/NativeMessageActions.stories.svelte catalog/components/store-application/AppearanceDiscussions.stories.svelte catalog/components/store-work/StoreWork.stories.svelte
```

**Notes**

Final dedicated-slot direct consumers: 29/31 pass, no runtime diagnostics. NativeCopyReplyAndOwnAuthorEdit retains the parent-proven Copy comment ID failure. FocusAndSplitPreserveToolDiscussionDrafts again misses its synchronous Appearance query after switching Settings; its prior isolated rerun passed without source changes. These existing qualification gaps remain explicit; no assertions or shared helpers changed. /tmp/jj-reaction-row-slots-consumers.log.
<!-- /run -->
<!-- /evidence-field -->

