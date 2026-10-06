---
schema: 1
id: E-READ-004
---

# E-READ-004 details 2

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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-file-reader.test.ts
```

**Notes**

Before files-first implementation: rows remained loading while metadata was unresolved. After initial implementation the existing synchronous-header assertion failed; changed it to await the independent header. Final four behavior regressions pass.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/file-read-queue.test.ts
```

**Notes**

One active worker, 32 pending jobs, keyed coalescing and cancellation: 1 passed.
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
cargo test -p jj-store-cli --test local_browser --locked -- --test-threads=2
```

**Notes**

9 passed, 1 opt-in smoke ignored. Includes row-only one-commit admission, exact summary and file membership/wrong-path denial, 10k bookmarks, registration and pinned read consumers.
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
cargo test -p jj-store-cli --test native_lifecycle --locked browsing:: -- --test-threads=2
```

**Notes**

Incorrect filter selected zero tests; not accepted as evidence. Corrected to compiled native test binary browser_ filter.
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
target/debug/deps/native_lifecycle-e6652c34f7ed2d93 browser_ --test-threads=2
```

**Notes**

2 passed: native attribution regression and signed cursor metadata-flag mismatch.
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
target/debug/deps/native_lifecycle-e6652c34f7ed2d93 content_acceptance:: --test-threads=4
```

**Notes**

95 direct native content consumers passed. Widened because public path/read interfaces affect multiple native query and HTTP consumers.
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
JJ_STORE_LARGE_REPO=/Users/stevejuma/code/linux/.jj/repo cargo test -p jj-store-cli --test local_browser --locked registered_large_repository_metadata_smoke -- --ignored --nocapture
```

**Notes**

1 read-only Linux smoke passed: metadata plus bookmark 2.672s; 10 root rows 2.848s, 3 metadata records, 2938179 charged bytes, 72 work units; lightweight summary and scoped .clang-format file (24291 bytes) succeeded. Concurrent verification was running; no service restart or checkout writes.
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
cargo check -p jj-store-jj -p jj-store-service -p jj-store-http -p jj-store-cli --all-targets --locked
```

**Notes**

All affected Rust target checks pass.
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
pnpm contracts:generate
```

**Notes**

Regenerated OpenAPI and TypeScript for additive metadata flag, not_requested attribution, local commit_summary and file_at.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 26
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check
```

**Notes**

Initial check found 3 new typing errors plus 23 dependency errors. Fixed the new inference and summary/detailed-revision contract errors. Final registry check has only the inherited 23 errors.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 23
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec svelte-check --tsconfig .local/bounded-reads/baseline/web/tsconfig.json
```

**Notes**

Parent revision with the same registry dependencies reproduces all 23 errors in 9 files; Design Core release lacks already-used source APIs.
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
pnpm install --offline --ignore-scripts --no-frozen-lockfile
```

**Notes**

Explicit temporary source validation using existing repository Design Core hook; restored registry pnpm-lock.yaml byte-for-byte. Existing M4.31 release gate remains pending. No package published.
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
pnpm web:check (temporary source validation)
```

**Notes**

0 errors, 0 warnings after lightweight file loader/transport extraction.
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
pnpm catalog:check
```

**Notes**

Directory story still supplied graph-only fields to the new summary prop; updated fixture to the actual summary contract.
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
pnpm catalog:check (after story update)
```

**Notes**

0 errors, 0 warnings; 182 catalog modules pass boundaries.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-file-reader.test.ts web/tests/vite/production-store-data-source.test.ts web/tests/vite/production-store-bootstrap.test.ts web/tests/vite/production-store-events.test.ts web/tests/vite/local-browser.test.ts web/tests/vite/local-browser-merge.test.ts web/tests/vite/native-activity.test.ts web/tests/vite/production-store-workspaces.test.ts
```

**Notes**

Initial run 66 passed/1 failed: HTTP test expected old query shape. Added the metadata flag to exact wire expectation; focused data-source rerun 11 passed. Later concurrent run had 65 passed/2 bootstrap settle failures; isolated bootstrap rerun 3 passed.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit --maxWorkers=2 web/tests/vite/production-file-reader.test.ts web/tests/vite/production-store-data-source.test.ts web/tests/vite/production-store-bootstrap.test.ts web/tests/vite/production-store-events.test.ts web/tests/vite/local-browser.test.ts web/tests/vite/local-browser-merge.test.ts web/tests/vite/native-activity.test.ts web/tests/vite/production-store-workspaces.test.ts
```

**Notes**

67 passed across 8 direct consumer files without changing bootstrap assertions or production deadlines.
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
pnpm exec tsx --test web/tests/production-store-loaders.test.ts
```

**Notes**

10 loader tests passed.
<!-- /run -->
<!-- /evidence-field -->

