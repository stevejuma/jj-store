---
schema: 1
id: E-UI-091
---

# E-UI-091 details 2

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
jj --no-pager st; jj --no-pager workspace list; jj --no-pager log; jj --no-pager diff --summary (main and temporary app/Design Core checkouts)
```

**Notes**

Both main working copies and both rich-search temporary working copies are clean; committed overlap inspected and current canonical chapters read. Unrelated registered workspaces and processes retained.
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
jj new @- ookknysq -m "Integrate custom search languages into main" (Design Core); jj new @- lsmrmlpp -m "Integrate rich history search into main" (app)
```

**Notes**

Design Core merge has no conflicts. App has seven conflict paths: six chapters/ledger and one import. Retained newer main prose, all original main JSON evidence entries, rich-search chapters and both controller imports. Renamed only rich-search evidence to E-UI-090 and combined verification/task links. Both original implementation commits remain ancestors.
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
pnpm install --ignore-scripts --lockfile=false
```

**Notes**

Existing ignored source hook resolves Design Core from ../design-core main. Main manifest, portable registry lock and installed dependency graph contain no temporary checkout reference. /tmp/rich-search-main-install.log
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/history-query.test.ts web/tests/vite/history-language.test.ts web/tests/vite/history-query-transport.test.ts web/tests/vite/automatic-refresh.test.ts web/tests/vite/production-automatic-refresh.test.ts web/tests/vite/store-route-history.test.ts
```

**Notes**

Six selected files and29 tests pass before the integration regression. One supplied optional filename has no matching file; actual discovered surface is recorded. /tmp/rich-search-main-query-before.log
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
pnpm test:storybook catalog/components/store-history-query/StoreHistoryQuery.stories.svelte (first main regression)
```

**Notes**

Three scenarios: two pass, new scenario fails during initial accepted-query setup. Isolated fixture history from the Storybook URL and reused the proven keyboard completion setup. /tmp/rich-search-main-refresh-before.log
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
pnpm test:storybook catalog/components/store-history-query/StoreHistoryQuery.stories.svelte (isolated before)
```

**Notes**

Two cases pass; exact new regression proves current-head refresh replaces invalid bad( draft with accepted mutable(). Preserved draft/diagnostic across operation-only changes while retaining cancellation and accepted-route synchronization. /tmp/rich-search-main-refresh-before2.log
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
pnpm test:storybook catalog/components/store-history-query/StoreHistoryQuery.stories.svelte (after)
```

**Notes**

All three keyboard/axe scenarios pass, including draft plus accepted-query preservation on a new displayed operation. /tmp/rich-search-main-refresh-after.log
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:architecture (initial and first extraction)
```

**Notes**

Initial combined contracts303/controller309 counted lines exceed300. After route-type and shared reader/query construction extraction, controller301 remains; moved operation binding into the shared factory. No contract or runtime behavior weakened. /tmp/rich-search-main-architecture.log and architecture-after.log
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
pnpm check:architecture (final)
```

**Notes**

818 modules pass source budget and catalog/production boundaries. /tmp/rich-search-main-architecture-final.log
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite (before, extraction and final)
```

**Notes**

All46 files /278 tests pass on each run; final follows the shared constructor binding change. Complete direct controller consumer lane rerun because merged reader construction, route contracts and automatic refresh share lifecycle consumers. /tmp/rich-search-main-controller-unit.log, controller-unit-after.log and controller-unit-final.log
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
pnpm exec tsx --test web/tests/store-route-codec.test.ts web/tests/production-store-loaders.test.ts
```

**Notes**

21 route, snapshot and presentation regressions pass. /tmp/rich-search-main-route-unit.log
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
pnpm web:check; pnpm catalog:check (initial and final)
```

**Notes**

Zero errors/warnings on both checks;155 catalog modules. Final checks follow the route extraction and draft preservation. /tmp/rich-search-main-web-check-final.log and catalog-check-final.log
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
pnpm check:boundaries; pnpm check:ui-ownership; pnpm test:governance
```

**Notes**

16 package boundaries, ownership validator and all43 governance tests pass. Full governance lane qualifies the merged shared audit and contract consumers. /tmp/rich-search-main-boundaries.log, ownership.log and governance.log
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store-rich-search/target pnpm contracts:generate; CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store-rich-search/target pnpm contracts:check
```

**Notes**

Regeneration from combined executable routes matches merged artifacts. All153 actual HTTP/SSE responses conform; seven contract tests and TypeScript consumers pass. /tmp/rich-search-main-contract-generate.log and contract-check.log
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store-rich-search/target cargo test --locked -p jj-store-cli --test native_lifecycle graph_acceptance
```

**Notes**

18 native graph/cache/query cases pass, including sparse filtered operations, traversal accounting and no publication. /tmp/rich-search-main-native-graphs.log
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store-rich-search/target cargo test --locked -p jj-store-cli --test local_browser
```

**Notes**

All7 filesystem registration, reader, alias and preservation cases pass on integrated source. /tmp/rich-search-main-local-browser.log
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store-rich-search/target cargo test --locked -p jj-store-jj --lib query::language
```

**Notes**

Three pinned catalog/context and bounded grammar regressions pass. /tmp/rich-search-main-language.log
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store-rich-search/target node scripts/browser-fixtures.mjs; CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store-rich-search/target cargo build --locked -p jj-store-http --example browser_preview; pnpm web:build
```

**Notes**

Recaptured native HTTP/SSE/mutation fixtures and rebuilt isolated production static host and UI from main. Existing CSS/chunk/plugin timing advisories only. /tmp/rich-search-main-fixtures.log, preview-build.log and web-build.log
<!-- /run -->
<!-- /evidence-field -->

