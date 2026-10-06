---
schema: 1
id: E-READ-049
---

# E-READ-049 details 2

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
jj --no-pager st (main and bounded-reads); jj new otplssvv pqoyvznw -m "Integrate bounded performance delivery with current main"
```

**Notes**

Both workspaces were clean before the authorized merge. Main parent ac401d2e/otplssvv, verified performance parent a7da51b1/pqoyvznw. Nine two-sided conflicts: seven canonical/ledger documents and two code files.
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
python3 .local/bounded-reads/integration/resolve.py; jj --no-pager st
```

**Notes**

All nine conflicts resolved. Combines main directory cache with bounded taskActivity, initial route-count arithmetic 126/51 pending runtime verification (subsequently corrected against the generated parent union), current freshness requirements and prepared/native/local performance requirements. Strict JSON equality preserves all 472 main and 482 feature evidence records in 489-record union; no actual parent record disagreement. Task evidence references combine without milestone promotion.
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
cargo test -p jj-store-http --lib --locked contract:: -- --nocapture
```

**Notes**

test result: FAILED. 16 passed; 1 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.43s
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
pnpm web:check
```

**Notes**

svelte-check found 0 errors and 0 warnings
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
pnpm catalog:check
```

**Notes**

svelte-check found 0 errors and 0 warnings; catalog boundaries ok (190 modules)
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/projects-repositories.test.ts web/tests/vite/projects-repository-visibility.test.ts web/tests/vite/projects-directories.test.ts web/tests/vite/projects-freshness.test.ts web/tests/vite/local-browser.test.ts web/tests/vite/read-query.test.ts web/tests/vite/http-query.test.ts web/tests/vite/repository-sources.test.ts web/tests/vite/production-store-data-source.test.ts web/tests/vite/application-freshness.test.ts
```

**Notes**

⎯⎯⎯⎯⎯⎯⎯ Failed Tests 2 ⎯⎯⎯⎯⎯⎯⎯; Test Files  1 failed | 9 passed (10); Tests  2 failed | 64 passed (66)
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
cargo test -p jj-store-http --lib --locked -- --nocapture
```

**Notes**

test result: ok. 21 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.59s
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
cargo test -p jj-store-jj --lib --locked -- --nocapture
```

**Notes**

test result: ok. 55 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 6.01s
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
cargo test -p jj-store-service --lib --locked -- --nocapture
```

**Notes**

test result: ok. 35 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.95s
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
cargo test -p jj-store-cli --test native_lifecycle --locked -- --nocapture
```

**Notes**

test result: ok. 223 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 73.93s
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
cargo test -p jj-store-cli --test local_browser --locked -- --nocapture
```

**Notes**

test result: ok. 33 passed; 0 failed; 2 ignored; 0 measured; 0 filtered out; finished in 48.32s
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
cargo check -p jj-store-jj -p jj-work -p jj-store-service -p jj-store-http -p jj-store-rpc -p jj-store-cli --locked
```

**Notes**

Exit 0; see packages-check-049.log.
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
cargo fmt -p jj-store-jj -p jj-work -p jj-store-service -p jj-store-http -p jj-store-rpc -p jj-store-cli -- --check
```

**Notes**

Exit 0; see rust-format-049.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/projects-repositories.test.ts web/tests/vite/projects-repository-visibility.test.ts web/tests/vite/projects-directories.test.ts web/tests/vite/projects-freshness.test.ts web/tests/vite/local-browser.test.ts web/tests/vite/read-query.test.ts web/tests/vite/http-query.test.ts web/tests/vite/repository-sources.test.ts web/tests/vite/production-store-data-source.test.ts web/tests/vite/application-freshness.test.ts
```

**Notes**

Test Files  10 passed (10); Tests  68 passed (68)
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
pnpm check:boundaries
```

**Notes**

package boundaries ok (17 packages)
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
pnpm check:architecture
```

**Notes**

web architecture ok (988 modules)
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
pnpm check:ui-ownership
```

**Notes**

Specification validated: 1 validators, 23 chapters, 172 requirements.
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
pnpm contracts:generate
```

**Notes**

Regenerated OpenAPI and TypeScript successfully. Exact path keys equal the complete union of both parent artifacts:119 total,42 local,77 nonlocal.
<!-- /run -->
<!-- /evidence-field -->

