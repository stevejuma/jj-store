---
schema: 1
id: E-REUSABLE-AGENTS-008
---

# E-REUSABLE-AGENTS-008 details 2

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
CARGO_INCREMENTAL=0 pnpm dev:cargo -- check -p jj-store-jj --no-default-features --features local-history --locked
```

**Notes**

Before: profile absent. First implementation exposed 48 native-coupling compile errors; next two passes compiled with 28 then 3 warnings. Native-only unused methods/imports were gated explicitly, without diagnostic suppression.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_INCREMENTAL=0 pnpm dev:cargo -- test -p jj-store-jj --no-default-features --features local-history --lib --locked
```

**Notes**

First unit compile revealed shared generation-lock tests needed Pin under cfg(test), even when production native is disabled. Fixed the test configuration without dropping shared coverage.
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
CARGO_INCREMENTAL=0 pnpm dev:cargo -- test -p jj-store-jj --no-default-features --features local-history --lib --locked
```

**Notes**

59 passed / 3 existing ignored, no warnings.
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
JJ_STORE_TEST_STOCK_JJ=/opt/homebrew/bin/jj CARGO_INCREMENTAL=0 pnpm dev:cargo -- test -p jj-store-jj --no-default-features --features local-history --test local_history --locked
```

**Notes**

Real stock JJ 0.45.1 fixture: cancellation, private index, pinned log/text diff, byte-exact source/dirty-file preservation, external writer and old pin, newer-commit rejection, registration generation and completion authorization. One deprecated atomic API warning subsequently corrected.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 24
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_INCREMENTAL=0 pnpm dev:cargo -- clippy -p jj-store-jj --no-default-features --features local-history --all-targets --locked -- -D warnings
```

**Notes**

Native qualification examples were still auto-selected and referenced unavailable native adapters: 11 query-example and 13 content-example compile errors. Added required native features, preserving defaults.
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
CARGO_INCREMENTAL=0 pnpm dev:cargo -- clippy -p jj-store-jj --no-default-features --features local-history --all-targets --locked -- -D warnings
```

**Notes**

Standalone all-target lint passes with warnings denied.
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
JJ_STORE_TEST_STOCK_JJ=/opt/homebrew/bin/jj CARGO_INCREMENTAL=0 pnpm dev:cargo -- test -p jj-store-jj --no-default-features --features local-history --locked
```

**Notes**

80 runnable tests passed:59 unit,6 source evidence,1 real history and14 local metadata;3 existing ignored; zero doc tests. Final real-source test is warning-free.
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
CARGO_INCREMENTAL=0 pnpm dev:cargo -- test -p jj-store-jj --lib --locked
```

**Notes**

Final native unit suite:96 passed /3 existing ignored in6.98s; no cache-lock failure.
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
CARGO_INCREMENTAL=0 pnpm dev:cargo -- clippy -p jj-store-jj --all-targets --locked -- -D warnings
```

**Notes**

Default native library, all integration and qualification-example targets compile and lint with warnings denied.
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
node --test scripts/governance.test.mjs scripts/local-history-features.test.mjs scripts/conversations-features.test.mjs scripts/capture-features.test.mjs
```

**Notes**

34 tests pass. Guards prove transitive normal dependency closure excludes native storage/services/Forge/Tauri/execution runtimes and every native-only test/example stays gated and enabled by defaults.
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
node scripts/check-boundaries.mjs
```

**Notes**

24 package boundaries pass.
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
node scripts/package-check.mjs
```

**Notes**

25 Cargo packages retain registry dependency metadata and contained compilation inputs.
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
pnpm docs:review local 'Reviewed focused local-history dependency closure, retained physical admission and operation pins, private standard-index construction, native-only execution gates and explicit unqualified Tauri integration.'
```

**Notes**

Owning source review refreshed.
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
node scripts/docs.mjs render
```

**Notes**

144 files/26,682,695 bytes; strict manual links/anchors valid;9 offline diagrams in both themes. Render-only does not claim full generated-reference comparison.
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
CARGO_INCREMENTAL=0 pnpm dev:cargo -- check -p jj-store-server --lib --locked
```

**Notes**

Direct server consumer compiles in40.52s.
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
CARGO_INCREMENTAL=0 pnpm dev:cargo -- check -p jj-store-jj --no-default-features --features companion --locked
```

**Notes**

Original smaller metadata-only profile still compiles in1.56s.
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
CARGO_INCREMENTAL=0 pnpm --silent dev:cargo -- run --quiet --locked -p jj-store-server --bin openapi
```

**Notes**

Validation command selected the wrong crate/target; no server binary named openapi. Correct owning example is in jj-store-http.
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
pnpm spec:first
```

**Notes**

Fresh evidence needed exact named-test anchors, not a bare script path. Corrected only E008 references; earlier evidence remains immutable.
<!-- /run -->
<!-- /evidence-field -->

