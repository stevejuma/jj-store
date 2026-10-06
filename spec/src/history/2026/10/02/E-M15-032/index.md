---
schema: 1
id: E-M15-032
---

# E-M15-032

<!-- evidence-field:id markdown -->
## Id

E-M15-032
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M15
- M15.5
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-02
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

parallel-thread-loading-and-targeted-reaction-refresh
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change wtqyykvt
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Backend E-M15-031 is committed in main. Existing UI serializes directory before conversation, reloads the whole conversation/library after confirmed reactions and admits12 concurrent reaction reads. Live baseline before both slices: messages~2.3s, reactions~3.5s and explicit refresh~3.2s. Scope is existing Community-owned presentation with preserved request/receipt identities.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- spec/src/api.md
- spec/src/collaboration.md
- spec/src/milestones.md
- spec/src/ui.md
- web/src/lib/collaboration/thread-presentation.ts
- web/src/lib/controller/bounded-reads.ts
- web/src/lib/controller/collab-actions.svelte.ts
- web/src/lib/controller/http-query.ts
- web/src/lib/controller/http-store-data-source.ts
- web/src/lib/controller/http-threads-data-source.ts
- web/src/lib/controller/message-reactions.svelte.ts
- web/src/lib/controller/production-repository-connect.ts
- web/src/lib/controller/production-threads.svelte.ts
- web/src/lib/controller/reaction-library.svelte.ts
- web/src/lib/controller/thread-commands.ts
- web/src/lib/controller/thread-committed-refresh.ts
- web/src/lib/controller/thread-types.ts
- web/src/lib/ui/store-threads/StoreThreadReplies.svelte
- web/src/lib/ui/store-threads/thread-view.ts
- web/tests/browser/thread-performance.spec.ts
- web/tests/vite/reaction-library.test.ts
- web/tests/vite/thread-indexes.test.ts
- web/tests/vite/thread-performance.test.ts
- web/tests/vite/thread-query-cache.test.ts
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/thread-performance.test.ts
```

**Notes**

Before implementation: directory-first waterfall, full reaction-triggered reload and12 simultaneous target reads. /tmp/jj-thread-perf-ui-before.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/thread-performance.test.ts -t 'parallel directory failure'
```

**Notes**

Additional concurrency regression: successful conversation overwrote the directory failure. Separate directory/conversation loading and error state now preserves it. /tmp/jj-thread-perf-parallel-error-before.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/thread-performance.test.ts web/tests/vite/production-threads.test.ts web/tests/vite/reaction-feedback.test.ts
```

**Notes**

First implementation run:27 pass/1 failure; an extra queue microtask deferred preview admission and broke the existing disconnect test. Admitted reads now start synchronously. Exact three-file rerun28/28 passes; /tmp/jj-thread-perf-ui-{after,after2}.log. Assertions unchanged.
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
pnpm check:architecture
```

**Notes**

Initial helper integration exceeded300 counted lines: HTTP store305 and Threads controller309. Extracted owned query/command/refresh helpers. /tmp/jj-thread-perf-architecture.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/reaction-library.test.ts
```

**Notes**

Parallel preview cleanup regression exposed forEach passing extra arguments to revokeObjectURL; explicit lambda now calls it with the URL only. Initial1 pass/1 fail, followed by33/33 library/direct-controller qualification. /tmp/jj-thread-perf-library-{regression,qualified}.log. All settled previews finish before newly created URLs are revoked.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/reaction-library.test.ts web/tests/vite/thread-performance.test.ts web/tests/vite/thread-query-cache.test.ts web/tests/vite/thread-indexes.test.ts web/tests/vite/production-threads.test.ts web/tests/vite/reaction-feedback.test.ts web/tests/vite/production-store-*.test.ts web/tests/vite/http-workspace-links.test.ts web/tests/vite/store-route-history.test.ts
```

**Notes**

Final100/100 across15 files. Covers exact-target refresh and retained event/other-reaction identities, emoji URL reuse/cleanup, four-read bound, duplicate main/reply coalescing, obsolete queued reads, cancellation without cache admission, session isolation, parallel error preservation,1200 replies and5000-deep/cyclic descendant fixtures. /tmp/jj-thread-perf-final-unit2.log. Earlier incremental complete-impact runs28/63/64/58/33/99 pass at their stages; logs ui-after2/ui-final/query-consumers/ui-qualified/library-qualified/final-unit, not additional final tests.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-threads catalog/components/store-application/StoreCollaboration.stories.svelte catalog/components/store-application/NativeReviewDetail.stories.svelte catalog/components/store-application/NativeMessageActions.stories.svelte
```

**Notes**

24/25 pass; unchanged NativeCopyReplyAndOwnAuthorEdit cannot find Copy comment ID submenu, parent-proven in E-M15-030. No story assertion/source changed. /tmp/jj-thread-perf-stories.log. This broader lane remains incomplete.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-threads
```

**Notes**

Final Threads-only interaction/axe lane12/12 across3 files after directory-error and preview-cleanup fixes. /tmp/jj-thread-perf-final-thread-stories.log. Visual-pending, no baseline approval.
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
pnpm web:check; pnpm catalog:check; pnpm check:architecture; pnpm check:ui-ownership
```

**Notes**

Final source: zero Svelte errors/warnings;129 catalog modules and724 architecture modules; owned Community independence passes. /tmp/jj-thread-perf-final-{web-check,catalog-check,architecture,ownership}.log. Earlier incremental checks also passed after extraction.
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
pnpm web:build
```

**Notes**

Fresh main production bundle and asset manifest pass after final UI changes. Rebuilt at initial, revoke-cleanup, final and last stages; /tmp/jj-thread-perf-build*.log. Existing large-bundle/minifier advisories retained.
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
pnpm exec playwright test --config /tmp/jj-thread-main-slices-playwright.config.mjs thread-performance.spec.ts reaction-metadata.spec.ts threads-alignment.spec.ts
```

**Notes**

Initial cold run3 pass/1 failure: test used the wrong custom reaction accessible label. Inspected the actual label and corrected to1 reaction with Lapis; fixture preview lookup corrected to its actual definition. /tmp/jj-thread-perf-cold.log. No production behavior or assertion requirement weakened.
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
pnpm exec playwright test --config /tmp/jj-thread-main-slices-playwright.config.mjs thread-performance.spec.ts reaction-metadata.spec.ts threads-alignment.spec.ts
```

**Notes**

Final cold built-bundle4/4: held directory does not block conversation, one confirmed reply reaction rereads exactly one target, no thread/library/asset reload or JJ refs/log fetch, main/reply unknown-action metadata retains original identity, settings/search/reply geometry and accessibility pass. /tmp/jj-thread-perf-cold-{final,last}.log. Isolated port7458 uses production static router and deterministic API fixtures; browser/service cleaned up. This is not live native timing.
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
pnpm exec prettier --write spec/src/api.md spec/src/collaboration.md spec/src/milestones.md spec/src/ui.md web/src/lib/collaboration/thread-presentation.ts web/src/lib/controller/bounded-reads.ts web/src/lib/controller/collab-actions.svelte.ts web/src/lib/controller/http-query.ts web/src/lib/controller/http-store-data-source.ts web/src/lib/controller/http-threads-data-source.ts web/src/lib/controller/message-reactions.svelte.ts web/src/lib/controller/production-repository-connect.ts web/src/lib/controller/production-threads.svelte.ts web/src/lib/controller/reaction-library.svelte.ts web/src/lib/controller/thread-commands.ts web/src/lib/controller/thread-committed-refresh.ts web/src/lib/controller/thread-types.ts web/src/lib/ui/store-threads/StoreThreadReplies.svelte web/src/lib/ui/store-threads/thread-view.ts web/tests/browser/thread-performance.spec.ts web/tests/vite/reaction-library.test.ts web/tests/vite/thread-indexes.test.ts web/tests/vite/thread-performance.test.ts web/tests/vite/thread-query-cache.test.ts
```

**Notes**

Final targeted24-file formatting passes: /tmp/jj-thread-perf-final-format.log. Earlier targeted formatting at each extraction/regression repair is recorded in /tmp/jj-thread-perf-{ui,ui-helper,ui-helper2,tests,extra,browser,library,revoke,error}-format.log. No whole-repository formatting.
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
JJ_THREAD_SLICE_EVIDENCE_PREFIX=/tmp/jj-thread-perf-final-main node /tmp/jj-thread-perf-restart.mjs
```

**Notes**

Runtime guard stopped before stopping any process or changing config: another run redirected7460 to jj-store-appearance/web/dist. /tmp/jj-thread-perf-restart-final.log. Asked user whether to restore main on7460 or preserve Appearance and verify separately; answer pending. Later read-only inspection still shows Appearance web_root and live PID13961. Shared runtime preserved.
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
pnpm spec:first; pnpm spec:validate; pnpm check:progress; pnpm exec prettier --check spec/src/milestones.md spec/src/api.md spec/src/ui.md spec/src/collaboration.md
```

**Notes**

Canonical/protected-file mapping, five validators/21 chapters/146 requirements and120-task evidence checks pass. /tmp/jj-thread-perf-final-{spec-first,spec-validate,progress}.log. Final evidence annotation is consumed again with targeted formatting before commit; product lanes are not repeated.
<!-- /run -->
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M15-032 details 2](details-02.md)
