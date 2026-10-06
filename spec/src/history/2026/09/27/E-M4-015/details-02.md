---
schema: 1
id: E-M4-015
---

# E-M4-015 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle events_
```

**Notes**

Before service implementation: 3 compile diagnostics for absent events module/types; no tests ran.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle events_
```

**Notes**

Initial 3 service fixtures pass: selected replay/restart/checkpoint, explicit cursor resets, queued-event revocation.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle http_events_
```

**Notes**

Before HTTP route implementation: /events returned 404 instead of 200.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle events_
```

**Notes**

Expanded 10 service/HTTP cases pass, including local wake, selected-record corruption, live epoch reset, separate stream admission and unpolled stream expiry.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle events_
```

**Notes**

Test helper initially assumed an ObjectStore delete API; 3 compile diagnostics. Removed those nonexistent methods from the fixture adapter, retaining the real trait contract.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle events_
```

**Notes**

All 11 service/HTTP cases pass including bounded stalled storage and retry from the original delivered cursor.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

Expected bookkeeping diagnostics before this entry existed: missing same-change ledger update and missing new/updated evidence entry.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle events_
```

**Notes**

All 12 event cases pass, including lagged 64-slot wake hints replaying 70 selected publications through two-record pages.
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

57 requirements, package boundaries, updated ledger, rustfmt and all-target Clippy pass.
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

238 Rust tests passed / 6 external cases ignored. Includes 97 native lifecycle tests. Governance 16/16 and storage harness self-tests 3/3 passed.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build
```

**Notes**

Production-only compile error: Axum MatchedPath was unavailable without the matched-path feature; dev dependencies had enabled it for tests. Added the explicit production feature without changing assertions.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build
```

**Notes**

Production workspace build passes after explicitly enabling Axum matched-path.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check
```

**Notes**

All 9 lanes pass after the production feature fix, including 238 Rust tests / 6 external ignored, rustfmt, governance, mdBook and mapped spec-first checks. Real SSE deployment remains unverified while Docker restart approval is pending.
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
pnpm format:check
```

**Notes**

Rustfmt and Prettier pass.
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
python3 scripts/rustfs-contract.py --run --server --output .scratch/m4-sse-rustfs.json
```

**Notes**

All nine full-concurrency external probes pass against the pinned RustFS digest, including 20 rounds with 16 contenders for create and CAS plus the ignored real_rustfs_server_cli_restart_and_revocation process test. The generated localhost TLS server accepts explicitly trusted native and HTTP clients; publication wakes SSE, Last-Event-ID resumes after server/RustFS restart and cache deletion, authoritative graph/file history recovers, queued revocation closes the stream and subsequent native/HTTP reads remain denied. The uniquely named container and volume were removed.
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
pnpm spec:first
```

**Notes**

The first evidence-only draft updated the governed deployment report without its mapped Compatibility chapter. The chapter now records the passing single-node TLS/RustFS/SSE boundary and retains multi-node, external certificate and load-balancer work outside this gate.
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
pnpm spec:first
```

**Notes**

The deployment report and ledger map to both required canonical chapters after the Compatibility update.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools VITEST_MAX_WORKERS=1 pnpm spec:check
```

**Notes**

All 18 canonical lanes pass with M4.2 closed, including cold catalog and production-browser acceptance, 68 Storybook interaction/accessibility cases, generated contracts, architecture and boundaries, the complete locked Rust workspace and mdBook. Visual baselines remain skipped.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

