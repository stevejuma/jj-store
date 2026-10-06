---
schema: 1
id: E-M4-009
---

# E-M4-009 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle query_cancel_
```

**Notes**

Before: 0 passed / 2 failed. Detached graph/content workers retained permits past the one-second cancellation check and produced secondary runtime-shutdown panics.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle query_cancel_
```

**Notes**

After request-drop cancellation: original 2 cases pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle graph_query_cancellation_retains_worker_permit_until_exit_and_deadlines_bound_waits
```

**Notes**

The old async-pause fixture expected Busy after the worker had correctly exited. Replaced only its first pause with a synchronous gate, retaining the exact permit/cache ownership assertion and independent deadline case.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle query_cancel_
```

**Notes**

One fixture compile diagnostic: non-binding synchronization guard. Bound the guard within its synchronous scope.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle query_cancel_
```

**Notes**

All 3 cases pass, including cancellation dropping actual pending remote response bodies. Temporary unused-guard warning subsequently corrected.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle graph_query_cancellation_retains_worker_permit_until_exit_and_deadlines_bound_waits
```

**Notes**

Exact permit/cache/deadline case passes with deterministic synchronous work.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo run --release --locked -p jj-store-jj --example query_qualification -- 50000
```

**Notes**

Before benchmark creation: example target absent.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo build --release --locked -p jj-store-jj --example query_qualification
```

**Notes**

One fixture compile diagnostic: native View head_ids is a HashSet, not BTreeSet. Fixed explicit collection; removed unused import.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo build --release --locked -p jj-store-jj --example query_qualification
```

**Notes**

Release fixture compiles.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
python3 scripts/qualify-query-resources.py --binary target/release/examples/query_qualification --commits 50000 --output .scratch/m4-native-query-budget.json
```

**Notes**

Independent process timeout at 180.01 seconds / 177.47 CPU seconds; no complete query report. Live sampling located a Tokio cooperative-budget spin inside native synchronous metadata predicate reads. Immutable before report retained.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-jj runtime::tests
```

**Notes**

Both bounded-poll regressions fail: native future and stream yield Pending after exhausting Tokio cooperative budget under a foreign executor.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-jj runtime::tests
```

**Notes**

Both regressions pass after runtime boundary unconstrained polling; real pending oneshot I/O remains pending.
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
python3 scripts/qualify-query-resources.py --binary target/release/examples/query_qualification --commits 50000 --output .scratch/m4-native-query-budget-after.json
```

**Notes**

First repaired release run passes: 4.68 seconds elapsed, 4.15 CPU seconds, 243204096 peak RSS bytes. All 8 native query cases pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo build --release --locked -p jj-store-jj --example query_qualification
```

**Notes**

Final release fixture asserts exact row counts and records query strings and configured limits.
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
python3 scripts/qualify-query-resources.py --binary target/release/examples/query_qualification --commits 50000 --output docs/evidence/m4-native-query-budget.json
```

**Notes**

Final report passes: 8 query cases plus 4 negative admission checks; 4.74 seconds elapsed, 4.13 CPU seconds, 246366208 peak RSS bytes. Binary/lock hashes and scope retained.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle
```

**Notes**

All 57 native lifecycle cases pass after cancellation and shared runtime fixes.
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
python3 scripts/rustfs-contract.py --run --rust-client --server --output .scratch/m4-runtime-rustfs.json
```

**Notes**

All 11 real RustFS harness cases pass; compiled native transport, cache-free cold publication recovery and TLS server/CLI restart/revocation requalify the shared runtime adapter.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check
```

**Notes**

57 requirements, boundaries, ledger, rustfmt and all-target Clippy pass.
<!-- /run -->
<!-- /evidence-field -->

