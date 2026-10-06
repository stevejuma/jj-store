---
schema: 1
id: E-UI-092
---

# E-UI-092 details 2

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
CUA read-only DOM styles on user main7460 tab; inspect Community, Design Core and jjbutler source
```

**Notes**

Token span colors equal ordinary foreground; focused pill3px ring. jjbutler uses single-line search input. No reference source modified.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/history-language.test.ts; pnpm exec tsx --test web/tests/store-route-codec.test.ts (before)
```

**Notes**

One multiline transaction failure and one multiline URL failure. /tmp/history-language-before.log and /tmp/history-route-before.log.
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
pnpm test:storybook src/shared/filter/search-filter-bar (Design Core)
```

**Notes**

3files/10stories pass. New test initially misplaced the cursor by userEvent.type refocusing contenteditable; keyboard continuation fixes test interaction without changing assertions. /tmp/history-dc-focused.log.
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
cargo test --locked -p jj-store-jj --lib query::language; cargo check --locked -p jj-store-jj -p jj-store-http
```

**Notes**

5language/identity cases pass. Repaired initial Backend API compile error; fallback fixture initially used invalid Git single-quote syntax and now uses valid Git config. /tmp/history-rust-focused.log and /tmp/history-rust-check.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/history-query.test.ts web/tests/vite/history-language.test.ts web/tests/vite/history-query-transport.test.ts; pnpm exec tsx --test web/tests/store-route-codec.test.ts
```

**Notes**

11focused cases and12route cases pass. Initial route fixture used incompatible operation-comparison/log state; preserved its valid Op Log state. /tmp/history-app-unit.log and /tmp/history-route-after.log.
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
cargo test --locked -p jj-store-cli --test local_browser; cargo test --locked -p jj-store-cli --test native_lifecycle graph_acceptance
```

**Notes**

7local cases and18native graph cases pass. /tmp/history-local-browser.log and /tmp/history-native-graph.log.
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
pnpm install --ignore-scripts --lockfile=false; pnpm contracts:generate
```

**Notes**

Source-validation dependency refreshed using existing ignored hook; registry manifest/lock preserved. Contracts regenerated after Rust API repair. Early type checks before generation correctly rejected missing metadata field; reruns follow generation.
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
cargo test --locked -p jj-store-jj --lib query::language (final)
```

**Notes**

6 cases including read-only secure-config binding and foreign-path rejection. /tmp/history-rust-focused.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit; pnpm test:storybook catalog/components/store-history-query/StoreHistoryQuery.stories.svelte
```

**Notes**

55files/314controller tests and4history interaction/accessibility stories pass. Initial/default, delayed manual override, explicit empty, retry and disposed metadata qualified. /tmp/history-app-full-controller.log and /tmp/history-app-stories.log.
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
pnpm web:check; pnpm catalog:check; pnpm check:architecture; pnpm check:ui-ownership; pnpm check:boundaries; pnpm spec:first; pnpm spec:validate; pnpm check:progress; changed-file Prettier check
```

**Notes**

0errors/warnings;156catalog modules,823architecture modules,16packages,125tasks,22chapters/152requirements. Repaired bookkeeping key commands-&gt;runs and mapped owning architecture/CLI chapters after initial gates. /tmp/history-{web-check,catalog-check,architecture,ownership,boundaries,spec-first,spec-validate,progress,format-final}.log.
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
pnpm contracts:generate; pnpm contracts:check
```

**Notes**

Generated contract matches;153actual HTTP/SSE responses conform;7contract tests pass. Native authenticated metadata asserts no host identity hint. /tmp/history-contracts.log and /tmp/history-contracts-check.log.
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
pnpm web:build; node scripts/browser-fixtures.mjs; cargo build --locked -p jj-store-http --example browser_preview; pnpm exec playwright test --config .scratch/history-query.playwright.config.ts web/tests/browser/history-query.spec.ts
```

**Notes**

2cold cases under strict production CSP pass. Ignored config changes only startup to the freshly built identical production browser_preview binary, avoiding Cargo lock waiting. Original run failed1case because Playwright fill converted indentation to NBSP; browser exercises unindented newline while unit tests retain literal tabs/spaces. Next original start timed out30s awaiting Cargo rebuild. Final starts a fresh isolated7458host and proves visible colors, multiline/URL, diagnostics, manual metadata and restored mine() with explicit default. /tmp/history-cold-browser-prebuilt.log.
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
Design Core pnpm checks; pnpm check:types; focused search family; STORYBOOK_PORT=9370 pnpm test:ai-chat-browser; SHADCN_STORYBOOK_PORT=9370 pnpm test:shadcn:pointer
```

**Notes**

715unit,3Turbo,645story interaction/accessibility,14workspace pointer,11shell pointer,3AI cold and35shadcn cold cases pass with existing1skip. Original aggregate stopped on2AI dev-host story-readiness timeouts; cold static rerun resolves both. Specs/format/types/static build/audit pass. Visual comparisons explicitly excluded; baselines unchanged. Shared public source justifies complete nonvisual source lanes. /tmp/history-dc-*.log.
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
pnpm dev:backend:build; pnpm dev:ui; CUA live main7460 verification
```

**Notes**

Latest main backend rebuilt; backend63916 and Vite63917 running from main with .local/dev-demo preserved. Live optional email default present (value not logged). Existing browser held old same-version module cache; Network.enable and ignoreCache reload loaded current component, then cache defaults restored. Two-line valid query succeeds, invalid draft retains accepted URL. Focused input has transparent border/no shadow, pink error tint and8px diagnostic margin; resting secondary Search background visible. Screenshot .scratch/history-feedback/main-query-error.jpg. No reset/reseed or unrelated runtime restart.
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
Design Core pnpm spec:check; changed-file Prettier check; pnpm pack --pack-destination .local/packages; app cargo fmt --check --package jj-store-jj --package jj-store-http; final ledger checks
```

**Notes**

Source spec, formatting and pack pass. Ledger initially rejected pass with recorded before-state failures/deferred registry release; pending bookkeeping accurately retains the separate package gate. No requested functional work remains.
<!-- /run -->
<!-- /evidence-field -->

