---
schema: 1
id: E-READ-021
---

# E-READ-021

<!-- evidence-field:id markdown -->
## Id

E-READ-021
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4.1
- M13.5
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-04
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change szzuwrwr
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Cancelable browser graph preparation and scoped polling lifetime
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Node26.9.0; pnpm10.32.1; Chromium via Playwright1.63.0; inherited temporary Design Core source validation.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Wire/persistence formats unchanged. Controller adds task status and cancellation; native/local adapters forward observation signals and bodyless cancel.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/controller/in-memory-store-controller.svelte.ts
- catalog/controller/in-memory-store-settings.svelte.ts
- spec/src/api.md
- spec/src/ui.md
- web/src/lib/controller/contracts.ts
- web/src/lib/controller/http-local-index-data-source.ts
- web/src/lib/controller/http-store-data-source.ts
- web/src/lib/controller/http-store-write-data-source.ts
- web/src/lib/controller/native-data-source.ts
- web/src/lib/controller/production-automatic-refresh.ts
- web/src/lib/controller/production-index-preparation.ts
- web/src/lib/controller/production-store-actions.ts
- web/src/lib/controller/production-store-controller.svelte.ts
- web/src/lib/controller/production-store-index.ts
- web/src/lib/controller/production-store-lifecycle.svelte.ts
- web/src/lib/controller/production-store-state.svelte.ts
- web/src/lib/controller/production-store-view.ts
- web/src/lib/ui/store-application/StoreRepositoryNotices.svelte
- web/src/lib/ui/store-settings/StoreGraphIndexSettings.svelte
- web/tests/production-store-loaders.test.ts
- web/tests/real/local-index-cancel.spec.ts
- web/tests/vite/production-automatic-refresh.test.ts
- web/tests/vite/production-store-settings.test.ts
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-012
- JST-UI-005
- JST-API-001
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/vite/production-store-settings.test.ts#preparation stops before
  polling after navigation during its delay
- web/tests/vite/production-store-settings.test.ts#preparation cancels only its
  original task and waits for terminal status
- web/tests/vite/production-store-settings.test.ts#clearing preparation aborts a
  pending observation and releases its timer
- web/tests/vite/production-store-settings.test.ts#cancel transport failure
  remains unconfirmed and polls without replaying cancel
- web/tests/vite/production-automatic-refresh.test.ts#graph preparation retains
  its read scope until terminal status
- web/tests/real/local-index-cancel.spec.ts#local preparation recovery and
  settings cancel the captured task
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-store-settings.test.ts
```

**Notes**

6pass/2fail before: obsolete status request after navigation delay; no cancel action. index-cancel-before.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-store-settings.test.ts web/tests/vite/local-browser.test.ts
```

**Notes**

Intermediate26pass then29pass after expanded abort/unconfirmed-cancel/bodyless-native coverage. index-cancel-tests.log/index-cancel-tests-final.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-automatic-refresh.test.ts
```

**Notes**

5pass/1fail: automatic refresh invalidated a live preparation scope. Added guard without suppressing explicit navigation. index-cancel-refresh-before.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-store-settings.test.ts web/tests/vite/local-browser.test.ts web/tests/vite/production-automatic-refresh.test.ts
```

**Notes**

35passed across3direct controller/transport/lifecycle consumers. index-cancel-units-final.log.
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
pnpm web:check; pnpm catalog:check; pnpm web:build
```

**Notes**

First web check caught LocalIndexTask snapshot lacks local-head fields needed by snapshotOperation; used task's own operation field. Catalog passed; build passed with existing chunk advisory. index-cancel-check.log.
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
pnpm web:check; pnpm catalog:check; pnpm web:build
```

**Notes**

Final web0errors/0warnings; catalog0errors/0warnings and182module boundaries; built bundle. Rebuilt after automatic-refresh/lifecycle fixes. index-cancel-check-sealed.log/index-cancel-catalog.log/index-cancel-build-final.log.
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
pnpm spec:first; pnpm spec:validate; pnpm check:architecture; pnpm check:ui-ownership
```

**Notes**

First spec-first found missing mapped api.md; next architecture found301counted controller lines. Added owning API policy and moved route-error initialization to existing lifecycle initializer. index-cancel-governance.log/index-cancel-governance-final.log.
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
pnpm spec:first; pnpm spec:validate; pnpm check:architecture; pnpm check:ui-ownership
```

**Notes**

21protected source files/2chapters,167requirements,932modules and ownership check pass. index-cancel-governance-sealed.log.
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
JJ_STORE_REAL_BASE_URL=http://127.0.0.1:18778/ JJ_STORE_LOCAL_BROWSER_FIXTURE=<owned fixture>/browser-fixture.json pnpm exec playwright test --config web/playwright.real.config.ts local-index-cancel.spec.ts local-browser.spec.ts
```

**Notes**

Two cold runs each5pass/1new-test timeout. First test assumed generation1 after reconnect/removal; second looked on General instead of Advanced. Corrected captured generation and actual control destination. All5existing local cold tests pass against final rebuilt bundle. index-cancel-browser.log/index-cancel-browser-final.log.
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
JJ_STORE_REAL_BASE_URL=http://127.0.0.1:18778/ JJ_STORE_LOCAL_BROWSER_FIXTURE=<owned fixture>/browser-fixture.json pnpm exec playwright test --config web/playwright.real.config.ts local-index-cancel.spec.ts
```

**Notes**

1passed/5.3s on fresh browser against restarted owned server/final bundle. Recovery and Settings cancellation, keyboard activation, pending cancel, final canceled and axe0violations. Mocked task responses qualify UI; existing real service cancellation evidence is separate. index-cancel-browser-sealed.log.
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
pnpm exec prettier --write <changed TS/Svelte and spec/src/api.md/spec/src/ui.md>
```

**Notes**

Focused formatter runs recorded in index-cancel-format*.log; ledger appended without rewriting historical evidence.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Status errors and unknown cancellation remain explicit; browser abort does not
  claim worker termination. Native/local task services retain separate prior
  cancellation evidence. Completed recovery keeps exact operation; no credential
  or persistence change.
- Linux full preparation still running on its original pre-E019/E020 binary; no
  large-scale pass claimed. Remaining detailed local reads,
  graph/prefix/copy/search/RPC, collaboration projections/cards and
  scale/fault/real-RustFS gates open.
- Root aggregate/visual baseline/Rust/contract-generation lanes out of scope for
  this browser-only wire-preserving slice. Inherited registry and full-contract
  gaps remain. Original dev services untouched; only owned18778fixture server
  used.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Commit cancellation slice; convert detailed local readers to verified prepared generations and complete Linux/large-scale qualification.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


