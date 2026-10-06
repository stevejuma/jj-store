---
schema: 1
id: E-M14-009
---

# E-M14-009 details 2

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
cargo check --locked -p jj-store-server -p jj-store-cli
```

**Notes**

Initial additive capability construction duplicated five fields; corrected, then check passed.
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
cargo test --locked -p jj-store-cli --test native_lifecycle local_search_save_apply_rebuild_disable_and_restart_preserve_authority_and_pins
```

**Notes**

Actual native storage with server manager: stale revisions, preserved unrelated values, Save without Apply, exclusions, failed replacement rollback, Retry/Rebuild, disable, pinned old and temporary readers, cleanup, disabled and enabled cold owner reopen, unchanged manifest. Initial fixture referenced unavailable toml crate; replaced with public Config::load. First cursor assertion omitted original freshness parameters; restored identical query binding.
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
cargo test --locked -p jj-store-cli --test native_lifecycle content_acceptance::http_acceptance::local_management::local_workspace_management_requires_local_admin_and_captures_contracts -- --exact
```

**Notes**

LocalTrusted Admin capability, read/save/control, strict version/unknown intent/extra field denial, bearer/read-only/origin denial and response capture. Mock manager tests transport admission; separate native test qualifies actual lifecycle.
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
cargo test --locked -p jj-store-cli --test native_lifecycle search
```

**Notes**

24 native search identity, authorization, paging, HTTP and server lifecycle tests pass.
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
cargo test --locked -p jj-store-adapters -p jj-store-tantivy
```

**Notes**

34 package tests pass: 22 adapter and 12 Tantivy tests; doc targets have no tests.
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
cargo test --locked -p jj-store-server --test configuration search; cargo test --locked -p jj-store-server --lib replication
```

**Notes**

One explicit/bounded configuration test and one configured sync outage/control test pass. Replaced search health receiver preserves independent sync ownership.
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
cargo test --locked -p jj-store-cli --test native_lifecycle server_search
```

**Notes**

6 pass after final lifecycle review: process cancellation cascades to repository tokens, drained join failures are removed safely, temporary cursor owners survive disable and release roots, saved enablement/scopes survive restart.
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
cargo clippy --locked -p jj-store-service -p jj-store-adapters -p jj-store-tantivy -p jj-store-http -p jj-store-server --lib -- -D warnings; cargo clippy --locked -p jj-store-server --lib -- -D warnings
```

**Notes**

Affected library boundaries pass; final server-specific rerun after lifetime review.
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
pnpm contracts:generate; pnpm contracts:check
```

**Notes**

98 captured actual HTTP/SSE responses and 6 contract tests pass. Initial generator panic was a request-body path missing /v1; corrected.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-search-settings.test.ts web/tests/vite/production-store-settings.test.ts
```

**Notes**

10 tests pass for scope parsing, independent Save/Apply, stale-save drafts, explicit refresh, single flight and late replies; final exact Search rerun has 4 tests.
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
pnpm web:check; pnpm catalog:check
```

**Notes**

Zero errors/warnings; 107 catalog modules. Initial web run raced contract generation, then exposed bigint budget conversion and missing optional data-source property; corrected.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreSearchSettings.stories.svelte catalog/components/store-application/StoreApplication.stories.svelte
```

**Notes**

15 interaction/axe stories pass. Initial 2 failures found an invalid status role on dd; placed status in a span. Final shell-only rerun covers active-tab-only forms.
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
pnpm web:build; pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/local-search-settings.spec.ts web/tests/browser/local-workspaces.spec.ts web/tests/browser/local-sync.spec.ts
```

**Notes**

3 cold browser tests pass including mobile overflow and axe. Final fresh bundle and exact Search rerun passes after adding File scopes. Initial three-tab test exposed duplicate form IDs in retained hidden panels; render only the active form while keeping drafts in controller state.
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
node --test scripts/dev-demo-search.test.mjs
```

**Notes**

2 tests: explicitly disabled Search bypasses index wait; configured Search still requires a fresh ready index.
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
cargo fmt -p jj-store-service -p jj-store-server -p jj-store-http -p jj-store-cli -p jj-store-adapters -p jj-store-tantivy; pnpm exec prettier --write <changed TS/Svelte/Markdown/MJS>; pnpm check:architecture; pnpm check:boundaries
```

**Notes**

439 web modules, 15 packages. Extracted unavailable catalog management actions after the controller reached 301 counted lines.
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
pnpm dev:restart
```

**Notes**

Main-owned demo rebuilt with persisted repository/identity/data retained at http://127.0.0.1:7460/. Initial live review led to a separate File scopes tab; final restart includes that layout.
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
pnpm spec:first; pnpm spec:validate; pnpm check:progress
```

**Notes**

Initial pending ledger failed missing actual runs; populated exact evidence before final gate. 132 requirements and 111 tasks.
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
cua_repl: reload owned demo tab; profile Settings; Search; getAXState and screenshot
```

**Notes**

Final live main demo shows Configuration/File scopes/Health tabs, 512 MiB current budget, enabled desired/effective state, Save/Apply/Retry/Rebuild controls. No live settings changed. Tab retained for review. Final shell-only story rerun passes 12 stories.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

