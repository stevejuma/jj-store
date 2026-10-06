---
schema: 1
id: E-M15-025
---

# E-M15-025 details 2

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
jj --no-pager st; scoped source/specification reads; CUA Community forum and current Thread inspection; metadata-bearing Node timeline GET
```

**Notes**

Clean ultsumzo / 5a01c3bd based on main oxunmxvt. Community source pinned at 741b8285e2081f1c9af58afe5a9b72f538fad781. Actual native root edits and distinct reply IDs verified read-only; no user content mutated.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-threads.test.ts (before new regression)
```

**Notes**

17 pass / 1 fail: focused reply lacked its historical ancestor. /tmp/jj-thread-bugs-before-unit.log.
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
pnpm exec playwright test --config /tmp/jj-thread-fix-playwright.config.mjs threads-alignment.spec.ts (before new regression)
```

**Notes**

Inline post composer absent; /tmp/jj-thread-fix-browser-before.log. Existing isolated browser_preview executable reused; no Rust build.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-threads.test.ts (implementation and extended reruns)
```

**Notes**

Initial 18-test rerun passes; then 20 tests and finally 21 including unknown create/body receipt recovery, stable main revisions and historical-parent root edits. /tmp/jj-thread-fix-unit.log, unit2.log, unit3.log.
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
pnpm web:check (initial/second reruns)
```

**Notes**

0 errors and 0 warnings; /tmp/jj-thread-fix-web-check.log, web-check2.log.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:architecture (initial, second, third reruns)
```

**Notes**

Controller exceeds 300-line budget at 317, 305 and 301 counted lines. Extracted bounded-history and post-flow helpers; retained architecture limits. /tmp/jj-thread-fix-architecture.log, architecture2.log, architecture3.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-threads/StoreThreads.stories.svelte catalog/components/store-threads/StoreThreadAlignment.stories.svelte (initial)
```

**Notes**

2 interactions failed: editor accessible name and asynchronous menu pointer cleanup. Actor HoverCard also produced one LinkPreview teardown diagnostic when toggling away the chip. Use the requested anchored Popover primitive instead; do not modify the foundation or suppress diagnostics. /tmp/jj-thread-fix-stories.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-threads (second)
```

**Notes**

7 / 8 stories pass, no teardown diagnostics. Settings scenario must await menu/dialog pointer cleanup. /tmp/jj-thread-fix-stories2.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-threads/StoreThreadAlignment.stories.svelte (three focused followups)
```

**Notes**

Two preceding reruns each failed 1 interaction; final 3 / 3 pass with assertions for actual pointer restoration and menu closure. /tmp/jj-thread-fix-settings-stories.log, settings-stories2.log, settings-stories3.log.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check; pnpm catalog:check (third web / first catalog)
```

**Notes**

Unsupported DropdownMenu modal option and insufficient reaction-command union narrowing; removed unsupported option and narrowed set/clear commands. /tmp/jj-thread-fix-web-check3.log, catalog-check.log.
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

All local component closure/provenance checks pass. /tmp/jj-thread-fix-ownership.log.
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
pnpm web:build (four fresh implementation builds)
```

**Notes**

All fresh production asset builds pass. Existing chunk-size advisory remains; /tmp/jj-thread-fix-build.log through build4.log. No foundation edits, package refresh or Rust rebuild.
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
pnpm exec playwright test --config /tmp/jj-thread-fix-playwright.config.mjs threads-alignment.spec.ts (implementation reruns 1-4)
```

**Notes**

Each run failed one assertion: wrong bubble selector; missing fixture actor prevented edit action; incorrect edit label; broad bubble locator violated strictness. Corrected fixture snapshot actor and exact scoped assertions. Earlier layout/reply geometry assertions passed. /tmp/jj-thread-fix-browser1.log through browser4.log.
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
pnpm web:check; pnpm catalog:check (final type reruns)
```

**Notes**

0 errors/warnings and 121-module catalog boundary pass; /tmp/jj-thread-fix-web-check4.log, catalog-check2.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-threads catalog/components/store-work/StoreWork.stories.svelte catalog/components/store-application/StoreCollaboration.stories.svelte catalog/components/store-application/NativeMessageActions.stories.svelte catalog/components/store-application/NativeReviewDetail.stories.svelte
```

**Notes**

26 / 26 interactions/accessibility stories across 7 directly affected Thread/Agent/Review/WorkStore families; cold Vite process and no unhandled diagnostics. /tmp/jj-thread-fix-direct-stories.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-threads.test.ts web/tests/vite/local-browser-merge.test.ts
```

**Notes**

25 / 25 direct controller/profile integration units; /tmp/jj-thread-fix-direct-units.log.
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
pnpm check:architecture (final)
```

**Notes**

625 modules within unchanged source budgets. /tmp/jj-thread-fix-architecture4.log.
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
pnpm exec prettier --write (changed sources/stories/specifications/provenance, incremental passes)
```

**Notes**

Formatting logs /tmp/jj-thread-fix-format.log through format9.log; only changed production/source ownership surfaces.
<!-- /run -->
<!-- /evidence-field -->

