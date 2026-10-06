---
schema: 1
id: E-M4-001
---

# E-M4-001

<!-- evidence-field:id markdown -->
## Id

E-M4-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4
- M4.1
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-27
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

committed-read-snapshot-foundation
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-service/src/reads.rs
- crates/jj-store-service/src/lib.rs
- crates/jj-store-cli/tests/native_lifecycle.rs
- spec/src/api.md
- spec/src/mutations.md
- spec/src/cli.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/native_lifecycle.rs#read_session_pins_history_and_reports_divergent_heads_without_publication
- crates/jj-store-cli/tests/native_lifecycle.rs#read_session_excludes_staged_operations_and_rechecks_revocation
- crates/jj-store-cli/tests/native_lifecycle.rs#read_session_enforces_inventory_and_elapsed_budgets
- crates/jj-store-cli/tests/native_lifecycle.rs#read_session_rejects_replaced_epoch_without_rebinding
<!-- /evidence-field -->

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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle read_session_
```

**Notes**

Before implementation: zero matching cases, not acceptance evidence.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle read_session_
```

**Notes**

Initial 3 cases pass: exact historical selection, explicit sibling-head response, zero read publication, immutable membership, staged rejection, current revocation and resource budgets. Epoch fault case added next.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle read_session_
```

**Notes**

Epoch fault fixture failed compilation: storage condition variant is Matches, not Match. No tests executed in this run.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle read_session_
```

**Notes**

All 4 cases pass, including explicit epoch replacement rejection.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle read_session_pins_history_and_reports_divergent_heads_without_publication
```

**Notes**

Additional deadline regression: an immediately ready multi-head response escaped a one-nanosecond budget. Pinned Tokio polls the future before its timer. Guard now checks elapsed time before and after polling as well as using timeout_at.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle read_session_pins_history_and_reports_divergent_heads_without_publication
```

**Notes**

Exact immediate-ready deadline regression passes after guarding both elapsed boundaries.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check
```
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test
```

**Notes**

140 Rust tests pass / 6 external cases ignored, including all 4 read-session cases; 15 governance tests and 3 storage-harness self-tests pass.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build
```
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check
```

**Notes**

All 9 lanes pass including repeated full Rust suite, rustfmt, mdBook and mapped spec-first.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm format:check
```
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- M4.1 still needs native graph/index sessions, bounded revsets,
  objects/files/diffs and signed snapshot cursors; M4.2 retains HTTP, SSE,
  generated contracts and UI.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.97.1; jj-cli/jj-lib 0.45.1 / 7c41cdeb16b6b321c64e789a966b6adf723816a5; memory-store native transactions (not deployment conformance).
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Owned read snapshot DTO V1 records repo/epoch, manifest revision, publication position, heads and selected operation. It is informational only; the read capability has private construction and no mutation API. No authority or native RPC format changes.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


