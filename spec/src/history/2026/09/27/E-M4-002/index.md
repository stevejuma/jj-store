---
schema: 1
id: E-M4-002
---

# E-M4-002

<!-- evidence-field:id markdown -->
## Id

E-M4-002
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

native-three-graph-query-foundation
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- Cargo.lock
- crates/jj-store-cli/tests/native_lifecycle/graphs.rs
- crates/jj-store-cli/tests/native_lifecycle.rs
- crates/jj-store-jj/Cargo.toml
- crates/jj-store-jj/src/lib.rs
- crates/jj-store-jj/src/query/access.rs
- crates/jj-store-jj/src/query/graphs.rs
- crates/jj-store-jj/src/query/mod.rs
- crates/jj-store-service/src/native.rs
- crates/jj-store-service/src/reads.rs
- spec/src/api.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/jj-integration.md
- spec/src/mutations.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/native_lifecycle/graphs.rs#graph_query_three_histories_filtered_edges_and_snapshot_pagination
- crates/jj-store-cli/tests/native_lifecycle/graphs.rs#graph_query_divergent_operations_bookmarks_and_explicit_workspace
- crates/jj-store-cli/tests/native_lifecycle/graphs.rs#graph_query_rebuild_limits_staged_rejection_and_current_revocation
- crates/jj-store-cli/tests/native_lifecycle/graphs.rs#graph_query_cancellation_retains_worker_permit_until_exit_and_deadlines_bound_waits
- crates/jj-store-cli/tests/native_lifecycle/graphs.rs#graph_query_marks_absent_evolution_data_and_rejects_missing_continuations
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle graph_query_
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo check -p jj-store-cli
```
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle graph_query_
```

**Notes**

Initial 3 cases: 2 pass / 1 fixture assertion failure. The native symbol resolver rejects a staged commit ID as Invalid before reading it; explicit evolution entry rejects the same staged ID as NotCommitted. Assertions now retain both distinctions.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle graph_query_
```

**Notes**

Expanded fixture compile failure: native View.head_ids is a HashSet rather than BTreeSet. One unused-import warning also removed. No cases ran.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle graph_query_
```

**Notes**

All 5 graph cases pass: explicit pinned history, filtered graph edges, hidden predecessors, divergent bookmarks, cache deletion/rebuild, limits, current revocation, worker cancellation and authorization deadlines.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo clippy -p jj-store-cli --all-targets --locked -- -D warnings
```
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle graph_query_
```

**Notes**

Added full operation-metadata assertions expose 3 missing DTO fields at compilation: is_snapshot, workspace and attributes. Fields are now mapped from native operation metadata. No cases ran.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle graph_query_
```

**Notes**

All 5 cases pass after metadata mapping, including corrupt cache backend rejection and explicit cache removal/rebuild.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test
```

**Notes**

145 Rust tests pass / 6 external cases ignored; 15 governance and 3 storage-harness tests pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build
```
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

All 9 lanes pass, including the repeated full Rust suite, rustfmt, mdBook and mapped spec-first.
<!-- /run -->

<!-- run -->
### Run 13

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

- M4 retains signed snapshot-bound transport cursors, native
  object/tree/file/diff and operation-diff queries, HTTP/SSE/generated contracts
  and browser acceptance.
- Current index-size, revset admission, I/O, output and worker limits are
  tested; synchronous native CPU algorithms are not preemptible by the async
  deadline. Adversarial CPU/memory qualification, index corruption recovery and
  large-index rebuild tasks/status remain M4 work. Per-operation cache
  retention/eviction requires operational qualification.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.97.1; jj-cli/jj-lib 0.45.1 / 7c41cdeb16b6b321c64e789a966b6adf723816a5; in-memory store and native transactions, not deployment conformance.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Owned graph DTO V1: native commit, operation and evolution nodes with ordered parents, filtered edges, annotations and absent-versus-empty predecessor metadata. Internal continuation is the last native node ID; signing/request binding remains separate. Cache key includes upstream/profile/repository/epoch/operation. No native persistence or RPC format change.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


