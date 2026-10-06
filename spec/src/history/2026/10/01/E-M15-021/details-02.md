---
schema: 1
id: E-M15-021
---

# E-M15-021 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 226
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check (iterative)
```

**Notes**

Initial type failures progressed 226 to 90 to 27 to 17 to 4 to 6 to 0; extraction and owned interfaces fixed. No diagnostic suppression.
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
pnpm catalog:check (iterative)
```

**Notes**

Final new story used unsupported exact option in Testing Library ByRole; removed the unsupported option while retaining exact UI assertions.
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
pnpm web:check; pnpm catalog:check
```

**Notes**

Final passes: 0 errors/warnings and 121 catalog modules; /tmp/jj-alignment-web-check-final2.log, /tmp/jj-alignment-catalog-check-final2.log.
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
pnpm catalog:test
```

**Notes**

Full directly impacted catalog/controller suite: 180 tests across 31 files pass; /tmp/jj-alignment-catalog-unit-final.log. Earlier 178-test pass preceded two revocation/removed-source regressions.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/global-search.test.ts
```

**Notes**

All 6 scoped fanout/cursor/cancellation/revocation/source-removal regressions pass; /tmp/jj-alignment-global-search-final.log.
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
pnpm exec tsx --test web/tests/native-entity-links.test.ts web/tests/store-route-codec.test.ts
```

**Notes**

13 tests pass; /tmp/jj-alignment-routes-final.log. Initial optional comment/null route drift corrected; legacy channel normalization does not change Review identities.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook (full runs)
```

**Notes**

Full run 1: 153 passes, 1 stale document-indicator assertion and 1 browser connection error; full run 2: 143 passes, no functional failure and 1 browser connection error. Three fresh shards retain the full suite without lowering tests. Copied indicator contract is 3px, not stale 2px.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook --shard 1/3
```

**Notes**

55 tests across 17 files pass; /tmp/jj-alignment-storybook-shard1.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook --shard 2/3
```

**Notes**

54 passes and one stale Search repository dialog assertion; workspace dialog/input are Search workspace while the sidebar entry stays Search repository.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreRepositorySearch.stories.svelte
```

**Notes**

Exact repaired search story passes; /tmp/jj-alignment-search-recovery.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook --shard 2/3
```

**Notes**

55 tests across 16 files pass; /tmp/jj-alignment-storybook-shard2-final.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook --shard 3/3
```

**Notes**

69 tests across 16 files pass; /tmp/jj-alignment-storybook-shard3.log. Complete shard coverage: 179 interactions across 49 files.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-threads/StoreThreads.stories.svelte catalog/components/store-threads/StoreThreadAlignment.stories.svelte catalog/components/store-search/StoreRecentSearch.stories.svelte
```

**Notes**

7 final directly affected stories pass, including rename/archive/restore/removal cancellation, custom reply reactions, native chip removal/Markdown, readonly conversations and recent/actions. /tmp/jj-alignment-thread-stories-final2.log. Earlier new-story progression was 3 failures to 2 to 1 to 4 passes.
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
pnpm web:build
```

**Notes**

Fresh production assets pass; /tmp/jj-alignment-web-build7.log. Bundle warning retained: main JS about 5.45 MiB, gzip about 1.49 MiB.
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
CARGO_INCREMENTAL=0 pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/threads-alignment.spec.ts (cold runs 1-9)
```

**Notes**

Each failed run had one failing test. Progression: forum row locator; cargo-lock startup timeout; two shell geometry fixture assumptions; strict-CSP body pointer lock after dialog close (dedupe alone insufficient); scope # prefix; scope changes dialog accessible title; missing level-one heading. Public preventScroll=false matches the fixed viewport; strict CSP remains unchanged. Native relative Markdown repaired through Mira supported remark hooks, readonly actions/aria-disabled fixed.
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
CARGO_INCREMENTAL=0 pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/threads-alignment.spec.ts
```

**Notes**

Cold run 10 passes in 8.6s against fresh production assets and a new LocalTrusted HTTP preview. Forum directory; absent repository tabs; full main width and shell far-end panel geometry; rename/pointer restoration; slash chip/rendered message link; reply creation/custom reaction/actor dialog; recent scoped search with exact server filters; child reply focus and reload; bottom action creates a Thread; full-page axe has zero violations. /tmp/jj-alignment-cold-browser10.log. Thread commands use catalog transport and captured native APIs; this is not real-RustFS full-flow evidence.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm build-storybook; node --test scripts/catalog-browser.test.mjs
```

**Notes**

Fresh static build passes. Inherited cold catalog fixture looks for JJ Log in the left navigation although repository views moved to the right details rail. Repaired only the control used to open Repository views; no product assertion removed. /tmp/jj-alignment-storybook-build-final2.log and /tmp/jj-alignment-catalog-cold-final.log.
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

Second cold run reached Markdown: safe renderer omits untrusted raw script markup rather than rendering it as preview text. Moved the full original-byte assertion to Source and retained preview no-script assertion. /tmp/jj-alignment-catalog-cold-final2.log.
<!-- /run -->
<!-- /evidence-field -->

