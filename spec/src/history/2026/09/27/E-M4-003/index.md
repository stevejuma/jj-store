---
schema: 1
id: E-M4-003
---

# E-M4-003

<!-- evidence-field:id markdown -->
## Id

E-M4-003
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

signed-snapshot-bound-graph-continuations
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- Cargo.lock
- crates/jj-store-cli/tests/native_lifecycle/pagination.rs
- crates/jj-store-cli/tests/native_lifecycle.rs
- crates/jj-store-service/Cargo.toml
- crates/jj-store-service/src/lib.rs
- crates/jj-store-service/src/pagination.rs
- crates/jj-store-service/src/reads.rs
- spec/src/api.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/mutations.md
- spec/src/security.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/native_lifecycle/pagination.rs#graph_cursor_retains_exact_snapshot_through_publication_checkpoint_and_replay
- crates/jj-store-cli/tests/native_lifecycle/pagination.rs#graph_cursor_rejects_tampering_changed_context_and_current_revocation
- crates/jj-store-cli/tests/native_lifecycle/pagination.rs#graph_cursor_expiry_restart_capacity_and_epoch_changes_require_reselection
- crates/jj-store-cli/tests/native_lifecycle/pagination.rs#graph_cursor_renews_request_deadline_keeps_head_selection_explicit_and_pages_evolution
- crates/jj-store-service/src/pagination.rs#graph_cursor_rejects_immediately_ready_results_after_deadline
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle graph_cursor_
```

**Notes**

Before implementation: zero matching cases, not acceptance evidence.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo check -p jj-store-service
```
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle graph_cursor_
```

**Notes**

Fixture compilation produced 2 errors from one StagedUpload::from_bytes call: missing byte limit and missing await. No tests ran.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle graph_cursor_
```

**Notes**

Initial 3 native pagination cases pass: snapshot replay across publications/checkpoint, changed-input/signature rejection, current revocation, expiry, registry budgets, restart and epoch replacement.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli -p jj-store-service graph_cursor_
```

**Notes**

Expanded 4 native cases plus 1 deadline unit case pass. Adds valid cross-repository credential rejection, all log-context bindings, explicit sibling head selection, terminal pages consuming no pin slot, evolution pagination and new request deadlines for retained snapshots.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle graph_cursor_
```

**Notes**

Exact narrow target rerun: all 4 native cursor cases pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check
```

**Notes**

Includes rustfmt, Clippy, progress and boundary checks.
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

150 Rust tests pass / 6 external cases ignored; 15 governance and 3 storage-harness tests pass.
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
pnpm format:check
```

**Notes**

Rust and Prettier both pass.
<!-- /run -->

<!-- run -->
### Run 10

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
### Run 11

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

All 9 lanes pass, including the repeated full Rust suite, rustfmt, progress/boundary checks, mdBook and mapped spec-first.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Cursor service is not yet exposed through HTTP or browser routes. M4 still
  needs native objects/files/trees/diffs, operation diffs,
  HTTP/SSE/contracts/browser and production query CPU/index/cache qualification.
- Cursor pins expire and reset after restart. A future multi-instance deployment
  requires sticky routing or a separately qualified shared pin design.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.97.1; jj-cli/jj-lib 0.45.1 / 7c41cdeb16b6b321c64e789a966b6adf723816a5; HMAC-SHA256 and base64url; native in-memory transaction fixtures, not deployment evidence.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Application graph cursor V1 binds process instance, pin, repo/epoch, op, publication position/revision, native sort profile, full query context, page size and last node. Registry holds immutable snapshot/inventory pins without credentials; current authorization and epoch are checked on resume. No native object, authority or RPC format change.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


