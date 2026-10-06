---
schema: 1
id: E-M4-012
---

# E-M4-012

<!-- evidence-field:id markdown -->
## Id

E-M4-012
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

shared-native-query-workers
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/native_lifecycle/content.rs
- crates/jj-store-cli/tests/native_lifecycle/graphs.rs
- crates/jj-store-cli/tests/native_lifecycle/query_workers.rs
- crates/jj-store-jj/src/query/content.rs
- crates/jj-store-jj/src/query/mod.rs
- crates/jj-store-jj/src/query/workers.rs
- spec/src/api.md
- spec/src/cli.md
- spec/src/jj-integration.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/native_lifecycle/query_workers.rs#query_workers_file_lifetime_limits_graphs_and_content_together
- crates/jj-store-cli/tests/native_lifecycle/query_workers.rs#query_workers_graph_and_content_cancel_release_shared_capacity
- crates/jj-store-cli/tests/native_lifecycle/query_workers.rs#query_workers_mixed_capacity_clones_and_errors_release_slots
- crates/jj-store-cli/tests/native_lifecycle/graphs.rs#graph_query_cancellation_retains_worker_permit_until_exit_and_deadlines_bound_waits
<!-- /evidence-field -->

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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle query_workers_
```

**Notes**

Before: independent graph/content pools admit a competing route while a worker or file handle occupies the intended shared capacity. Both tests failed; no compile errors.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle query_workers_
```

**Notes**

Original 2 regressions pass after shared composition. Final expanded suite passes all 3 cases, including mixed capacity, clones, alternate cache roots and error cleanup.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle graph_query_cancellation_retains_worker_permit_until_exit_and_deadlines_bound_waits
```

**Notes**

Synchronous canceled graph worker retains both shared content capacity and its cache lock until actual exit.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check
```

**Notes**

57 requirements, boundaries, progress, rustfmt and all-target Clippy pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test
```

**Notes**

209 Rust tests pass / 6 external cases intentionally ignored; all 71 native lifecycle cases, 15 governance tests and 3 storage-harness self-tests pass. No unresolved failures.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build
```

**Notes**

Locked workspace build passes.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check
```

**Notes**

All 9 lanes pass, including Rust tests, rustfmt, mdBook and mapped spec-first checks.
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
pnpm format:check
```

**Notes**

Rustfmt and Prettier pass.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Controlled long cold-index tasks/status; HTTP admission for snapshot capture
  and response delivery, JSON/media/SSE, generated contracts and browser
  integration. Shared native pool is a tested composition interface;
  whole-server concurrent resource qualification remains M8a.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.97.1; jj-cli/jj-lib 0.45.1 at 7c41cdeb16b6b321c64e789a966b6adf723816a5. Existing per-query shape/byte/work/deadline policies unchanged.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive shared query worker composition and capacity/in-use diagnostics. No native identity, envelope, publication, HTTP or RPC format changes.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


