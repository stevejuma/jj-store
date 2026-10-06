---
schema: 1
id: E-READ-051
---

# E-READ-051 details 2

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
cargo test -p jj-store-cli --test local_browser --locked reference_embedded::local_embedded_references_prepare_exact_source_and_preserve_outer_cursor_authority -- --exact --nocapture (before implementation)
```

**Notes**

Cold mixed request returned200 instead of409 rebuild required;0 passed,1 failed,35 filtered out. local-embedded-before-051.log.
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
cargo test -p jj-store-cli --test local_browser --locked reference_embedded::local_embedded_references_prepare_exact_source_and_preserve_outer_cursor_authority -- --exact --nocapture (initial cold implementation)
```

**Notes**

1 passed;35 filtered out. local-embedded-after-051.log.
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
cargo test -p jj-store-cli --test local_browser --locked reference_embedded::local_embedded_references_prepare_exact_source_and_preserve_outer_cursor_authority -- --exact --nocapture (expanded initial fixture)
```

**Notes**

Fixture CLI op @ was ambiguous after creating two operation heads. Replaced with explicit operation-head metadata lookup; no reconciliation. local-embedded-expanded-051.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/local-browser.test.ts (initial test assertion)
```

**Notes**

22 passed,1 failed. Test incorrectly read localSnapshot after recovery maps it to ApiError.snapshot; corrected assertion. local-ui-051.log.
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
cargo test -p jj-store-cli --test local_browser --locked reference_embedded::local_embedded_references_prepare_exact_source_and_preserve_outer_cursor_authority -- --exact --nocapture (expanded fixture cache directories)
```

**Notes**

Missing fixture output directories caused Unavailable; create owned empty source/outer/sibling index directories. local-embedded-fixed-051.log.
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
cargo test -p jj-store-cli --test local_browser --locked reference_embedded::local_embedded_references_prepare_exact_source_and_preserve_outer_cursor_authority -- --exact --nocapture (expanded anchor fixture)
```

**Notes**

Empty bookmark oracle requires bookmark-prefixed anchors. Corrected kind-specific anchor. local-embedded-final-051.log.
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
cargo test -p jj-store-cli --test local_browser --locked reference_embedded::local_embedded_references_prepare_exact_source_and_preserve_outer_cursor_authority -- --exact --nocapture (expanded dispatch fixture)
```

**Notes**

Empty bookmark windows intentionally do not use prepared dispatch; regression now uses their ordinary metadata-only reader. local-embedded-ready-051.log.
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
cargo test -p jj-store-cli --test local_browser --locked reference_embedded::local_embedded_references_prepare_exact_source_and_preserve_outer_cursor_authority -- --exact --nocapture
```

**Notes**

1 passed;35 filtered out;8.97s.48 differential windows plus2 metadata-only comparisons at4 history visits; successful source-only HTTP preparation/cursors, exact-source cold recovery, newer/sibling warm rejection and unavailable Git-body ancestry proof. local-embedded-accepted-051.log.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check (initial generic request narrowing)
```

**Notes**

2 TypeScript property-query errors; concrete LocalCodeRequest selection narrows sourceOperation correctly. local-web-check-051.log.
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
pnpm web:check
```

**Notes**

0 errors,0 warnings. local-web-check-fixed-051.log and local-web-final-051.log.
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
pnpm catalog:check
```

**Notes**

0 errors,0 warnings;190 catalog modules. local-catalog-final-051.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/local-browser.test.ts
```

**Notes**

23 passed; local-ui-fixed-051.log.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/local-browser.test.ts web/tests/vite/history-query-transport.test.ts
```

**Notes**

24 passed,2 timed out. Existing native/local immediate identical retries joined the first aborted caller before asynchronous observer release. Release is now synchronous and idempotent on caller abort. local-ui-consumers-051.log.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/local-browser.test.ts web/tests/vite/history-query-transport.test.ts web/tests/vite/read-query.test.ts web/tests/vite/thread-query-cache.test.ts web/tests/vite/production-store-data-source.test.ts (first full direct transport/cache surface)
```

**Notes**

40 passed,1 failed. Target reaction invalidation read the new trailing epoch slot instead of argument slot. Reproduced separately in unchanged main:1 failed,1 passed; thread-cache-before-052.log. local-ui-all-consumers-051.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/local-browser.test.ts web/tests/vite/history-query-transport.test.ts web/tests/vite/read-query.test.ts web/tests/vite/thread-query-cache.test.ts web/tests/vite/production-store-data-source.test.ts
```

**Notes**

41 passed across5 files; native/local immediate retries, independent caller cancellation, old navigation cancellation, scoped target/session invalidation and source recovery pass. local-ui-final-051.log.
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
cargo test -p jj-store-cli --test local_browser --locked prepared -- --nocapture
```

**Notes**

test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 26 filtered out; finished in 24.36s; 47.44s. local-prepared-consumers-051.log.
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
cargo test -p jj-store-cli --test local_browser --locked index_tasks_acceptance -- --nocapture
```

**Notes**

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 31 filtered out; finished in 5.75s; 23.37s. local-task-consumers-051.log.
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
cargo test -p jj-store-cli --test local_browser --locked bounded_reads -- --nocapture
```

**Notes**

test result: ok. 6 passed; 0 failed; 1 ignored; 0 measured; 29 filtered out; finished in 15.52s; 32.63s. local-bounded-consumers-051.log.
<!-- /run -->
<!-- /evidence-field -->

