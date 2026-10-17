---
schema: 1
id: E-REUSABLE-AGENTS-007
---

# E-REUSABLE-AGENTS-007 details 2

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
pnpm dev:disk
```

**Notes**

Before Rust work:40.7GiB free,24.6GiB primary target; no cleanup, server stop or fixture change required.
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
CARGO_INCREMENTAL=0 pnpm dev:cargo -- test -p jj-store-jj --lib query::workers::tests --locked
```

**Notes**

Before extraction:2original worker/live-limit tests pass.
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
CARGO_INCREMENTAL=0 pnpm dev:cargo -- test -p jj-store-jj --lib query::workers::tests --locked
```

**Notes**

Initial move exposed one private Cancellation tuple-field consumer. Reused its existing shared token accessor rather than broadening field visibility.
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
CARGO_INCREMENTAL=0 pnpm dev:cargo -- test -p jj-store-jj --lib query::workers::tests --locked
```

**Notes**

After extraction:2original tests pass, including real shared permit observation.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_INCREMENTAL=0 pnpm dev:cargo -- test -p jj-store-jj --lib --locked
```

**Notes**

Initial normal parallel run:94pass/2Busy cache-lock failures/3existingignored. Both failures occurred opening a cold cache immediately after fixture construction. No test assertion, locking or budget was changed.
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
CARGO_INCREMENTAL=0 pnpm dev:cargo -- test -p jj-store-jj --lib --locked
```

**Notes**

Exact parent Rust sources restored temporarily with all12owned edits saved and restored byte-exactly afterwards:96pass/3existingignored in6.74seconds. Initial lock failures did not reproduce; no inherited-failure claim is made.
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
CARGO_INCREMENTAL=0 pnpm dev:cargo -- test -p jj-store-jj --lib query::cache::proof_tests::cold_proof_budget_and_cancellation_do_not_become_rebuild_success --locked
```

**Notes**

Exact first cold-cache failure passes in isolation on extracted sources.
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
CARGO_INCREMENTAL=0 pnpm dev:cargo -- test -p jj-store-jj --lib query::cache::proof_tests::failed_cold_recovery_admission_preserves_existing_files_and_receipt --locked
```

**Notes**

Exact second cold-cache failure passes in isolation on extracted sources.
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
CARGO_INCREMENTAL=0 pnpm dev:cargo -- test -p jj-store-jj --lib --locked
```

**Notes**

Final normal parallel owning-crate run:96pass/3existingignored in6.81seconds. Initial nonreproduced lock failures remain recorded; no timing, locking or assertion was weakened.
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
CARGO_INCREMENTAL=0 pnpm dev:cargo -- clippy -p jj-store-jj --lib --locked -- -D warnings
```

**Notes**

Strict owning library lint passes.
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
CARGO_INCREMENTAL=0 pnpm dev:cargo -- check -p jj-store-server --lib --locked
```

**Notes**

Direct consumer compiles, including JJ service/RPC/HTTP/Tantivy/capture boundaries;40.06seconds.
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
CARGO_INCREMENTAL=0 pnpm dev:cargo -- run --quiet --locked -p jj-store-http --example openapi
```

**Notes**

Initial validation helper captured pnpm script banners with JSON, so JSON decoding failed. The generated program itself succeeded; suppressed wrapper banners for the final comparison.
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
CARGO_INCREMENTAL=0 pnpm --silent dev:cargo -- run --quiet --locked -p jj-store-http --example openapi
```

**Notes**

Complete newly generated public OpenAPI deep-equals checked-in schemas/openapi.json. No DTO, schema or generated TypeScript changes required.
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
CARGO_INCREMENTAL=0 pnpm dev:cargo -- check -p jj-store-jj --no-default-features --features companion --locked
```

**Notes**

Existing focused companion consumer still compiles in13.27seconds; no feature/dependency edge added.
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
CARGO_INCREMENTAL=0 pnpm dev:cargo -- fmt -p jj-store-jj -- --check
```

**Notes**

Affected Rust formatting passes.
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
node scripts/check-boundaries.mjs; node scripts/package-check.mjs
```

**Notes**

24package boundary/25Cargo package input and registry metadata checks pass.
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
pnpm docs:review local "Reviewed shared owned read budgets, diagnostics, cancellation and worker admission; native/local public aliases and original allowances remain exact, while the companion-only profile still excludes independent history reads."
```

**Notes**

Refreshed only owning local-source review after source freeze.
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
node scripts/docs.mjs render
```

**Notes**

Strict render-only manual:144files/26680227bytes; links/anchors valid,9diagrams offline in both themes. Rust reference comparison, real-RustFS and UI lanes are outside this internal Rust refactor scope.
<!-- /run -->
<!-- /evidence-field -->

