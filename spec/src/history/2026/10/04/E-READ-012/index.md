---
schema: 1
id: E-READ-012
---

# E-READ-012

<!-- evidence-field:id markdown -->
## Id

E-READ-012
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4.1
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-04
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change tlsrvwxp
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Warm native graph pages remain available during cache preparation
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust1.98.1; jj-lib0.45.1; pnpm10.32.1; macOS ARM64.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Public DTOs, native persistence and derived cache formats unchanged. Memory-only readers retain exact operation pins and fresh request authorization.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/native_lifecycle/graph_cache.rs
- crates/jj-store-jj/src/factories.rs
- crates/jj-store-jj/src/query/cache/resident.rs
- crates/jj-store-jj/src/query/cache.rs
- crates/jj-store-jj/src/query/index.rs
- crates/jj-store-jj/src/query/mod.rs
- spec/src/cli.md
- spec/src/jj-integration.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-JJ-003
- JST-JJ-005
- JST-API-001
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/native_lifecycle/graph_cache.rs#warm_graph_survives_concurrent_preparation_and_cache_eviction
- crates/jj-store-cli/tests/native_lifecycle/graph_cache.rs#warm_graph_starts_while_another_operation_is_preparing
<!-- /evidence-field -->

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
cargo test -p jj-store-cli --test native_lifecycle --locked warm_graph_survives_concurrent_preparation_and_cache_eviction -- --nocapture
```

**Notes**

Before: same-namespace preparation returned Busy while warm page held the namespace lease. Final regression passed.
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
cargo test -p jj-store-cli --test native_lifecycle --locked warm_graph_starts_while_another_operation_is_preparing -- --nocapture
```

**Notes**

Before: warm page returned Busy after preparation evicted its old disk slot. Final memory-only loader passes with a different reader principal and current revocation.
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
cargo test -p jj-store-cli --test native_lifecycle --locked graph_acceptance:: -- --test-threads=2
```

**Notes**

Strengthened reader fixture initially passed CommittedRead instead of Arc&lt;CommittedRead&gt;; corrected compile failure without changing production admission.
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
cargo test -p jj-store-cli --test native_lifecycle --locked graph_acceptance:: -- --test-threads=2
```

**Notes**

Final24passed. Both concurrency orders, exact pinned data after disk eviction, reader revocation, corruption, cache separation, cancellation and native graph semantics. Existing linker unwind-size advisory remains.
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
target/debug/deps/native_lifecycle-d9faf2e6f13fc0f1 content_acceptance::index_task_acceptance:: --test-threads=2
```

**Notes**

5passed, including reserved interactive capacity, cancellation, expiry, restart and revocation.
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
target/debug/deps/jj_store_jj-830d932ccb18ef00 query::cache:: --nocapture; cargo test -p jj-store-jj --lib --locked query::cache:: -- --nocapture
```

**Notes**

1passed in each run; rebuilt current unit binary confirms warm verification and same-size corruption detection.
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
cargo check -p jj-store-jj -p jj-store-service -p jj-store-cli --all-targets --locked
```

**Notes**

Affected package and direct consumer targets pass.
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
pnpm spec:first; pnpm check:progress; pnpm spec:validate; pnpm exec prettier --check spec/src/jj-integration.md spec/src/cli.md; cargo fmt --all -- --check
```

**Notes**

Progress checker rejected pending as a run status; corrected evidence schema before rerunning all document consumers.
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
pnpm spec:first; pnpm check:progress; pnpm spec:validate; pnpm exec prettier --check spec/src/jj-integration.md spec/src/cli.md; cargo fmt --all -- --check
```

**Notes**

6protected files/3chapters,136ledger tasks,167requirements and affected format checks pass. Ledger was appended without whole-file formatting.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Only exact warm all() pages release the namespace lease or use the memory-only
  fallback. Other expressions retain existing exclusive admission. Retained
  memory charges survive eviction until executing owners release their copies.
- Immutable disk generations, process-wide quota admission,4GiB/repository
  and16GiB aggregate staging quotas, larger native/local preparation and
  remaining endpoint/card/collaboration work are still pending.
- Running app unchanged; no public contract/persistence change. Unchanged
  UI/browser, full contracts and root aggregate lanes out of scope. E-READ-005
  registry/full-capture and large-scale/real-RustFS gates remain open.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Commit this verified concurrency slice, then continue staging-inclusive cache quotas and local prepared history.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


