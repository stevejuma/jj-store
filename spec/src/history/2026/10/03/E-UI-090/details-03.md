---
schema: 1
id: E-UI-090
---

# E-UI-090 details 3

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
cargo check --locked -p jj-store-jj -p jj-store-service -p jj-store-http
```

**Notes**

Owning backend boundary compiled. /tmp/rich-search-rust-check2.log
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
cargo test --locked -p jj-store-jj query::language
```

**Notes**

Three language/catalog/parser cases. /tmp/rich-search-language-tests2.log
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
cargo test --locked -p jj-store-cli --test native_lifecycle history_queries_filter_sparse_ancestry_before_paging_and_never_publish
```

**Notes**

Exact native sparse pagination, real parent gaps, context, limits, cursor mismatch and zero-publication proof. /tmp/rich-search-native-tests.log
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo test --locked -p jj-store-cli --test local_browser history_query
```

**Notes**

Fixture iterations caught missing Debug, TOML quoted-key, escaped-newline and operation-description mistakes; fixed without weakening product assertions. /tmp/rich-search-local-tests*.log
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
cargo test --locked -p jj-store-cli --test local_browser history_query (final)
```

**Notes**

Pinned actual CLI oracle for six aliases, poisoned repo alias isolation, sparse operations and unchanged dirty files/heads. /tmp/rich-search-local-tests5.log
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
cargo test --locked -p jj-store-cli --test native_lifecycle http_browsing_windows_are_authorized_pinned_cursor_bound_and_contract_captured
```

**Notes**

Exact metadata auth/pin, filtered operations and HTTP400 diagnostic. /tmp/rich-search-http-tests.log
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
cargo test --locked -p jj-store-cli --test native_lifecycle graph_acceptance (shared cache attempts)
```

**Notes**

Our runners waited behind simultaneous other-workspace builds. Stopped only our queued work and cloned ignored artifacts to isolated target using cp -cpR; no other service modified.
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
cargo test --locked -p jj-store-cli --test native_lifecycle graph_acceptance (isolated before assertion updates)
```

**Notes**

15 passed /3 failed: generic Invalid assertions now receive intentional typed query diagnostics. /tmp/rich-search-isolated-graphs.log
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
cargo test --locked -p jj-store-cli --test native_lifecycle graph_acceptance (isolated final)
```

**Notes**

All18 graph/cache/cancellation/resource/history cases pass; assertions now require exact diagnostic kinds. /tmp/rich-search-isolated-graphs2.log
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
pnpm contracts:generate
```

**Notes**

Additive OpenAPI and TypeScript contracts generated. /tmp/rich-search-contract-generate.log
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
pnpm web:check (initial)
```

**Notes**

One native snapshot field typo corrected to operation_id. /tmp/rich-search-web-check.log
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
pnpm web:check (subsequent and final)
```

**Notes**

Zero errors/warnings. /tmp/rich-search-web-check2.log, check3.log, check4.log
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
pnpm catalog:check (new fixture)
```

**Notes**

Fixture-only StoreDataSource cast fixed through explicit unknown conversion. /tmp/rich-search-catalog-check2.log
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
pnpm catalog:check (final)
```

**Notes**

Zero errors/warnings;131 catalog modules. /tmp/rich-search-catalog-check3.log
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite
```

**Notes**

Existing direct controller surface31 files /196 tests. /tmp/rich-search-web-existing-unit.log
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
pnpm exec vitest run --config web/vitest.config.ts web/tests/vite/history-query.test.ts web/tests/vite/history-language.test.ts
```

**Notes**

Mistyped nonexistent config path; corrected to catalog config. No test ran. /tmp/rich-search-web-query-final.log
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/history-query.test.ts web/tests/vite/history-language.test.ts
```

**Notes**

Six focused language and controller cases. /tmp/rich-search-web-query-final2.log
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/history-query-transport.test.ts
```

**Notes**

Three regressions prove fresh identical native/local requests after cancellation and metadata operation mismatch rejection. /tmp/rich-search-transport-tests.log
<!-- /run -->
<!-- /evidence-field -->

