---
schema: 1
id: E-M6-001
---

# E-M6-001 details 2

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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-wal --test publication selected_reader --locked
```

**Notes**

Selected-record acceptance cannot compile: five missing-method diagnostics in one test target.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-wal --test publication selected_reader --locked
```

**Notes**

Exact WAL regression passes: orphan exclusion, exact references, captured history and record-specific inventory survive checkpoints and reject oversized/future traversal.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-adapters --test delivery adapter_replays_only_selected_records_and_bootstraps_after_artifact_loss --locked
```

**Notes**

Adapter crate does not yet exist; baseline acceptance target cannot run.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo check -p jj-store-adapters
```

**Notes**

New crate compiles; removed one unused import warning. Lockfile records the workspace crate.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-adapters --test delivery adapter_replays_only_selected_records_and_bootstraps_after_artifact_loss
```

**Notes**

Initial ordered replay and artifact-loss acceptance passes.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-adapters --test delivery --locked
```

**Notes**

During fixture cleanup the artifact deletion was accidentally removed before a rebuild assertion. Restored restart assertions and explicit file-loss step; no product assertion weakened.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-adapters --test delivery
```

**Notes**

Six initial crash, recovery, poison, scope, ownership and limit tests pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle native_adapter_source --locked
```

**Notes**

Native JJ objects decode through current read-token binding; staged content is denied; revoked token cannot advance the projection.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-adapters --test delivery --locked
```

**Notes**

Eight expanded fixtures pass, including polling/backoff/cancellation and corrupt native/cursor data.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-adapters --test delivery adapter_bootstrap_rejects_corrupt_selected_tail_before_destination_effect --locked
```

**Notes**

Bootstrap incorrectly acknowledges a corrupt selected WAL tail because only inventory was read.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-adapters --test delivery adapter_bootstrap_rejects_corrupt_selected_tail_before_destination_effect --locked
```

**Notes**

Exact regression passes after current tail/receipt validation; no destination effect or cursor is written.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-adapters --test delivery --locked
```

**Notes**

All nine final adapter fault tests pass; remote pointer readback also rejects an unexpected competing cursor.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-adapters --test delivery adapter_replays_only_selected_records_and_bootstraps_after_artifact_loss --locked
```

**Notes**

Original acceptance rerun with exact locked command passes.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle native_adapter_source --locked
```

**Notes**

Final source boundary including bootstrap tail/receipt checks passes native token, integrity and revocation fixture.
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

Normal checks pass: all-target Clippy, rustfmt, ten package boundaries, progress, generated contracts and Svelte checks.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test
```

**Notes**

312 Rust tests pass, zero failures; six external deployment tests ignored. Existing governance, harness, contracts and browser unit tests pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build
```

**Notes**

Locked Rust workspace and unchanged browser assets build successfully.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check
```

**Notes**

Repeated Rust lane hits RPC admission fixture race at native_rpc.rs:640: no occupied permit observed before watchdog. 144 native lifecycle and nine adapter fixtures pass; specification stops before final build lanes.
<!-- /run -->
<!-- /evidence-field -->

