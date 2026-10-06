---
schema: 1
id: E-M4-008
---

# E-M4-008 details 2

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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle graph_cache_
```

**Notes**

Before new fixture: zero matching cases; not acceptance evidence.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle graph_cache_corrupt_operation_link_rebuilds_without_publication
```

**Notes**

Original regression fails: malformed native index operation link returns Unavailable instead of rebuilding the derived cache.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 16
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo check -p jj-store-service
```

**Notes**

Initial cache module missed its std::fs import after removing the parent import; 16 diagnostics.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo check -p jj-store-service
```

**Notes**

Service compile passes after local import correction.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle graph_cache_corrupt_operation_link_rebuilds_without_publication
```

**Notes**

Exact regression passes after receipt inventory and derived-file reconstruction.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle graph_query_
```

**Notes**

All 5 original graph acceptance cases pass under bounded slot caching.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle graph_cache_
```

**Notes**

Two fixture compile diagnostics: unavailable NativeObjects::scope trait and nonexistent MemoryStore::corrupt helper. Fixture now uses ObjectKey::storage_path and a checked conditional store replacement.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle graph_cache_
```

**Notes**

Initial 5 cache cases pass: malformed links, checksum/missing-file/receipt/index-type damage, LRU/byte/file/slot limits, revocation, independent executors, interrupted-build cleanup, symlink isolation and authoritative-object errors.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle graph_cache_
```

**Notes**

Final 6 cases pass, including separate repository/epoch bindings for identical virtual roots and atomic local receipts.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle graph_query_
```

**Notes**

All 5 existing graph cases pass, including retained namespace lock across caller cancellation until worker exit.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check
```

**Notes**

57 requirements, boundaries, ledger, rustfmt and all-target Clippy pass before shared RPC fixture repair.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test
```

**Notes**

Governance/storage self-tests pass. Administration RPC target: 7 passed / 1 failed. rpc_token_retries_and_competing_snapshots_preserve_accepted_policy hit AddrInUse at restart because the fixture released its listener reservation.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test administration_rpc rpc_fixture_reserves_listener_across_restarts
```

**Notes**

Deterministic source regression proves another listener can acquire a stopped fixture endpoint. The fixture now owns one nonblocking listener and restarts using cloned handles.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test administration_rpc rpc_fixture_reserves_listener_across_restarts
```

**Notes**

Exact listener-reservation regression passes with retained socket ownership.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test administration_rpc rpc_token_retries_and_competing_snapshots_preserve_accepted_policy
```

**Notes**

Original failing retry/competing-snapshot case passes.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test administration_rpc
```

**Notes**

All 9 cases pass after the shared fixture change.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check
```

**Notes**

57 requirements, boundaries, ledger, rustfmt and all-target Clippy pass after final changes.
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

185 Rust tests pass / 6 external cases intentionally ignored. 15 governance and 3 storage-harness tests pass; no unresolved failures.
<!-- /run -->
<!-- /evidence-field -->

