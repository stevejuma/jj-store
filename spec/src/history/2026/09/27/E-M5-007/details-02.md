---
schema: 1
id: E-M5-007
---

# E-M5-007 details 2

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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle conflict_upload --locked
```

**Notes**

Initial HTTP acceptance fails: upload endpoint returns 404 unknown_route.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo check -p jj-store-service --locked
```

**Notes**

Initial implementation compile error: Repo trait import missing for native store access; fixed import.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo check -p jj-store-http
```

**Notes**

Same native Repo import compile failure before correction; lockfile updated for existing tokio-util streaming adapter.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle conflict_upload --locked
```

**Notes**

First integrated test aborts with a worker stack overflow. Replaced the new 64 KiB stack array in the async native verifier with a heap buffer; unchanged runtime stack configuration.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle conflict_upload --locked
```

**Notes**

Original end-to-end upload/native-byte/atomic-resolution acceptance passes after heap-buffer correction.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle conflict_upload --locked
```

**Notes**

Expanded five fixtures pass; removed one unused fixture parameter warning.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle conflict_upload --locked
```

**Notes**

All six final fixtures pass, including 600,000-byte binary upload and actual HTTP preview/apply, empty/structural/copy resolution, corrupt/missing data, scoped handles, checkpoint receipt replay, lost acknowledgement, revocation, bounded/failed streams and admission deadlines.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-service conflict_upload_record --locked
```

**Notes**

Canonical V1 record independent SHA-256 vector and scope-integrity cases pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check
```

**Notes**

All-target Clippy, rustfmt, spec/boundaries/progress, generated contracts and 43 actual HTTP/SSE samples, schema tests and zero Svelte diagnostics pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test
```

**Notes**

299 Rust tests pass, zero failures, six opt-in external tests ignored; governance, harness, generated-contract and existing browser unit checks pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build
```

**Notes**

Existing browser asset build and full locked Rust workspace build pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check
```

**Notes**

All 13 specification lanes pass, including mdBook, spec-first, rustfmt and full locked Rust tests.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm format:check
```

**Notes**

Rustfmt and all repository Prettier checks pass.
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
pnpm check:progress
```

**Notes**

Evidence incorrectly marked pass while retaining the audited M5 report gap; restored pending evidence status. Product checks passed independently.
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
pnpm check:progress
```

**Notes**

32 tasks validate with upload evidence pending only the explicit M5 report audit gap; all implementation checks already pass.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- M5 audit found a missing explicit remaining-conflict-path report; implement
  bounded preview/receipt reporting while preserving legacy V1 canonical receipt
  bytes before closing M5.3 or M5.
- Real RustFS/TLS deployment evidence and deferred browser controls remain
  separate gates.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.97.1 with rustfmt; jj-cli/jj-lib 0.45.1 at 7c41cdeb16b6b321c64e789a966b6adf723816a5; jj-native-0.45.1-v1.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Adds ConflictUploadId SHA-256 record V1, immutable repository/epoch/principal staging namespace, binary HTTP upload endpoint/capabilities and uploaded_file resolution variant. Native/WAL/RPC codecs and existing StoredResult V1/Prepared bytes unchanged. Existing released tokio-util is added to HTTP and lockfile. Only generated web API declarations change; UI remains Deferred M4.3. Enclosing Jujutsu change is the revision reference. Real RustFS/TLS SSE qualification remains blocked by the stopped shared Docker engine.
<!-- /evidence-field -->

