---
schema: 1
id: E-M4-014
---

# E-M4-014 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 9
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle http_reads_
```

**Notes**

Before HTTP implementation: missing crate/Axum/Tower imports produced 9 compile diagnostics; no tests ran.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle http_reads_
```

**Notes**

Initial implementation: 3 Send-future compile errors and 1 unused import warning; fixed an owned VerifiedFile info future and middleware request borrow scope.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle http_reads_
```

**Notes**

Initial 2 route fixtures passed, then expanded 6 cases passed including range, revocation, cursor/task and unpolled response expiry.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test server_executable cold_tls_http_reads_support_http1_and_http2
```

**Notes**

Before production integration: HTTP/1.1 TLS client rejected with NoApplicationProtocol; one failed test.
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
pnpm test:governance
```

**Notes**

New allowed server-to-HTTP composition assertion failed before boundary allowlist update. HTTP direct storage/native import rejection passed.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test server_executable cold_tls_
```

**Notes**

Both HTTP/1.1 + HTTP/2 and native certificate/hostname/plaintext TLS fixtures passed after shared listener integration.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle http_reads_reject_unframed_request_content
```

**Notes**

Unexpected streamed GET body without framing headers returned 200 instead of 400. Fixed bounded EOF verification before bodyless route execution.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle http_reads_
```

**Notes**

Intermediate compile diagnostic: HeaderMap has no get_all_mut; changed sensitive-header marking to iter_mut. No tests ran.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle http_reads_
```

**Notes**

All 7 focused HTTP fixtures passed, including the exact unframed-body regression.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-server listener_
```

**Notes**

2 listener fixtures pass: shared connection admission and activity/idle deadline retaining then releasing the socket permit.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test server_executable
```

**Notes**

7 cold executable tests pass / 1 external RustFS case ignored; trust, protocols, state lock, commands and shutdown remain intact.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools python3 scripts/rustfs-contract.py --run --rust-client --server --output .scratch/m4-http-rustfs.json
```

**Notes**

All 11 pinned single-node harness cases pass, including real HTTP/CLI fixed-operation parity, unchanged GET manifest snapshot, ranges, process/storage kills, removed caches and revoked HTTP access. Report retained in docs/evidence/m4-http-rustfs.json.
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
pnpm test:governance
```

**Notes**

All 16 cases pass, including corrected allowed composition and forbidden direct HTTP storage/native dependencies.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check
```

**Notes**

One Clippy manual_option_zip diagnostic, repeated for library/test targets. Replaced nested option mapping with zip; no behavior or assertion weakened.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check
```

**Notes**

57 requirements, 9 package boundaries, progress, rustfmt and all-target Clippy pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test
```

**Notes**

Environment failure before Rust tests: attachment_recovery linker write failed with errno=28 (disk full). Governance 16/16 and harness self-tests 3/3 passed. No product test failure was observed.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build
```

**Notes**

Environment failure: rustc LLVM output could not write (disk full) for jj-cli and jj-store-cli. Removed only jj-store target/debug/incremental generated cache (14 GiB); no source, release artifacts, credentials or other checkout files removed.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test
```

**Notes**

226 Rust tests pass / 6 external tests ignored, including 85 native lifecycle tests and 7 cold executable server tests. Governance 16/16, storage harness self-tests 3/3 pass after generated-cache cleanup. No unresolved product failures.
<!-- /run -->
<!-- /evidence-field -->

