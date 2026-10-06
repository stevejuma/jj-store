---
schema: 1
id: E-M14-013
---

# E-M14-013 details 2

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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-sync-settings.test.ts (before)
```

**Notes**

3 pass/1 fail: missing wait action; /tmp/jj-local-management-sync-before.log.
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
pnpm exec tsx --test web/tests/production-store-mutations.test.ts (before reconciliation)
```

**Notes**

3 pass/1 fail: missing capture action; /tmp/jj-local-management-reconcile-before.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-sync-settings.test.ts web/tests/vite/production-store-settings.test.ts
```

**Notes**

10/10 tests; /tmp/jj-local-management-sync-unit-final.log.
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
pnpm exec tsx --test web/tests/production-store-mutations.test.ts
```

**Notes**

4/4 tests including exact fresh head selection and unknown request retention; /tmp/jj-local-management-reconcile-after.log.
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
pnpm contracts:generate
```

**Notes**

Generated OpenAPI and TypeScript from registered handlers; /tmp/jj-local-management-contracts.log.
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
cargo test --locked -p jj-store-cli --test native_lifecycle local_workspace_management_requires_local_admin_and_captures_contracts
```

**Notes**

1/1 actual HTTP capture fixture; timeout/reached, invalid request and admin admission; /tmp/jj-local-management-sync-http.log. Initial unused Json warning fixed.
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
cargo check --locked -p jj-store-service -p jj-store-http -p jj-store-server
```

**Notes**

Affected Rust boundary check; /tmp/jj-local-management-rust-check.log.
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
cargo test --locked -p jj-store-server --lib replication::tests
```

**Notes**

1/1 outage/control test; /tmp/jj-local-management-sync-worker.log.
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
pnpm test:storybook catalog/components/store-application/StoreSyncSettings.stories.svelte
```

**Notes**

5/5 interaction/accessibility cases; /tmp/jj-local-management-sync-stories.log.
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
pnpm test:storybook StoreSyncSettings.stories.svelte StoreSyncReconciliation.stories.svelte StoreRepositoryActions.stories.svelte (initial)
```

**Notes**

10 pass/1 fail; new test expected combobox but public Select exposes button with listbox popup. Corrected semantic query without changing product or relaxing behavior; /tmp/jj-local-management-reconcile-stories.log.
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
pnpm web:check; pnpm catalog:check; pnpm check:architecture; pnpm check:ui-ownership; pnpm spec:validate
```

**Notes**

Web has 0 errors/warnings; catalog/ownership and architecture focused checks. Final checks recorded below.
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
pnpm web:build; node scripts/browser-fixtures.mjs; pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/local-sync.spec.ts -g "captured sync targets|Sync reconciliation"
```

**Notes**

Fresh built bundle and server:2/2 browser tests including historical route retention, exact request recovery and mobile axe; /tmp/jj-local-management-sync-browser-final.log. Bundle emits existing large-chunk warning.
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
pnpm exec prettier --write changed frontend/spec files; rustfmt --edition 2024 changed Rust files
```

**Notes**

Targeted formatting only.
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
pnpm contracts:check
```

**Notes**

156 actual HTTP/SSE response captures and 7 contract tests pass.
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
pnpm catalog:check (initial reconciliation fixture)
```

**Notes**

Literal widening then catalog boundary rejected a direct API type import. Return type now derives from owned controller contracts; final type and boundary checks pass.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/mutations.spec.ts (first)
```

**Notes**

4 pass/1 fail: pending-command guard tracked mutation status inside a UI effect and reset file preview. Guard now reads untracked.
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
pnpm web:build && pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/mutations.spec.ts web/tests/browser/local-sync.spec.ts -g "text file edit|repository action|repository command|unknown mutation|captured sync targets|Sync reconciliation"
```

**Notes**

7/7 cold built-bundle tests pass.
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
pnpm test:storybook catalog/components/store-mutations/StoreRepositoryActions.stories.svelte catalog/components/store-application/StoreSyncReconciliation.stories.svelte
```

**Notes**

6/6 pass; fixed reconciliation exact story also 1/1; Sync family 5/5 passed.
<!-- /run -->
<!-- /evidence-field -->

