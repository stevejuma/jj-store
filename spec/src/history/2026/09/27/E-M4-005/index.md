---
schema: 1
id: E-M4-005
---

# E-M4-005

<!-- evidence-field:id markdown -->
## Id

E-M4-005
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

signed-directory-snapshot-pagination
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-service/src/pagination.rs
- crates/jj-store-service/src/pagination/cursors.rs
- crates/jj-store-service/src/pagination/directory.rs
- crates/jj-store-cli/tests/native_lifecycle/content.rs
- crates/jj-store-cli/tests/native_lifecycle/directory.rs
- spec/src/api.md
- spec/src/mutations.md
- spec/src/cli.md
- spec/src/security.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/native_lifecycle/directory.rs#directory_cursor_retains_snapshot_through_writes_checkpoint_and_retries
- crates/jj-store-cli/tests/native_lifecycle/directory.rs#directory_cursor_binds_all_inputs_and_rechecks_revocation_before_lookup
- crates/jj-store-cli/tests/native_lifecycle/directory.rs#directory_cursor_shares_graph_capacity_and_rejects_cross_query_restart_and_expiry
- crates/jj-store-cli/tests/native_lifecycle/directory.rs#directory_cursor_keeps_head_selection_explicit_and_rejects_invalid_paths
- crates/jj-store-cli/tests/native_lifecycle/directory.rs#directory_cursor_oversized_name_fails_without_consuming_snapshot_capacity
- crates/jj-store-cli/tests/native_lifecycle/pagination.rs#graph_cursor_rejects_tampering_changed_context_and_current_revocation
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle directory_cursor_
```

**Notes**

Before implementation: zero matching cases, not acceptance evidence.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo check -p jj-store-service
```

**Notes**

Compile diagnostic: GraphExecutor has no Debug implementation. The graph pager now explicitly formats only its redacted cursor registry.
<!-- /run -->

<!-- run -->
### Run 3

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
### Run 4

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle directory_cursor_
```

**Notes**

First 4 cases pass: exact snapshot and retry through writes/checkpoint, input tampering and revocation, shared graph capacity and cross-query rejection, expiry/restart/epoch, explicit multi-head selection and invalid paths.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle directory_cursor_
```

**Notes**

All 5 cases pass including oversized continuation rejection without leaked snapshot capacity.
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

All 4 existing graph cursor regressions pass after shared registry extraction.
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

57 requirements, boundaries, ledger, rustfmt and all-target Clippy pass.
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

163 Rust tests pass / 6 external cases intentionally ignored. 15 governance and 3 storage-harness tests pass.
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

All 9 lanes pass including 163 Rust tests, rustfmt, mdBook and spec-first.
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

- Structured diffs/conflict descriptors and operation diffs, resource
  qualification, HTTP JSON/media/SSE, generated contracts and browser
  integration remain M4 work.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.97.1; jj-cli/jj-lib 0.45.1. Production service/native queries over disposable in-memory stores, not RustFS or browser evidence.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Shared process-local V1 read cursor prefix jsr1 and domain bind traversal kind, snapshot, query, page size and typed continuation. Replaces unreleased jsg1 graph tokens; retained pins remain ephemeral across restarts. No native object, authority or RPC format change.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


