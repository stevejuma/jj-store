---
schema: 1
id: E-READ-010
---

# E-READ-010

<!-- evidence-field:id markdown -->
## Id

E-READ-010
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

enclosing Jujutsu change rmqokryw
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Disk-backed native commit-index preparation with bounded scratch and streamed pinned format
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; jj-lib0.45.1; redb4.3.0 (std only, MIT OR Apache-2.0); pnpm10.32.1.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Native objects, manifest/WAL and public DTOs unchanged. Derived JJ commit-index segment format6 is byte-identical to the pinned cold builder. New redb scratch is private, newly created and discarded; no retained database format or migration. Existing hard index50000 and cache64MiB/slot defaults retained.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- Cargo.lock
- crates/jj-store-jj/Cargo.toml
- crates/jj-store-jj/src/query/mod.rs
- crates/jj-store-jj/src/query/index.rs
- crates/jj-store-jj/src/query/cache.rs
- crates/jj-store-jj/src/query/prepared/mod.rs
- crates/jj-store-jj/src/query/prepared/disk.rs
- crates/jj-store-jj/src/query/prepared/segment.rs
- crates/jj-store-cli/tests/native_lifecycle/graphs.rs
- spec/src/architecture.md
- spec/src/jj-integration.md
- spec/src/cli.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-ARCH-001
- JST-ARCH-002
- JST-JJ-003
- JST-JJ-005
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/native_lifecycle/graphs.rs#disk_preparation_matches_pinned_upstream_index_bytes
- crates/jj-store-cli/tests/native_lifecycle/graphs.rs#disk_preparation_quota_failure_discards_stage_and_allows_retry
- crates/jj-store-jj/src/query/prepared/disk.rs#scratch_file_refuses_growth_before_touching_the_file
- crates/jj-store-jj/src/query/prepared/disk.rs#scratch_tables_survive_batch_boundaries_and_refuse_quota_overflow
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
Inspect pinned jj-lib0.45.1 default_index/store.rs::build_index_at_operation and jj-core0.45.1 dag_walk_async::topo_order_reverse_ord
```

**Notes**

Before state: upstream cold preparation retains all historical operation/commit roots, nodes, edges and final commit Vec in memory before constructing/serializing a complete mutable index. E-READ-009 leaves that bottleneck explicit. This is source evidence, not measured large-scale memory acceptance.
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
cargo search redb --limit 1; cargo info redb@4.3.0; cargo add redb@=4.3.0 --package jj-store-jj
```

**Notes**

One package added to Cargo.lock; exact version and license checked. No experimental features; adapter-only dependency passes package boundaries.
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
cargo check -p jj-store-jj --all-targets --locked
```

**Notes**

Initial compiler diagnostics: explicit redb key/value lifetimes, operation op_store accessor, and database error conversion. Corrected; next check passed.
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
cargo test -p jj-store-cli --test native_lifecycle --locked disk_preparation_matches_pinned_upstream_index_bytes -- --nocapture
```

**Notes**

Initial preparation failed Limit. Backend must admit both set_len and writes that extend a fresh file. No over-quota write is performed.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-cli --test native_lifecycle --locked graph_acceptance:: -- --test-threads=2
```

**Notes**

15passed/5failed while new scratch initialization returned Unavailable. redb invokes the repair callback during fresh initialization; removed unconditional abort because create_new already forbids reused scratch. Existing interactive graph checks passed.
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
cargo test -p jj-store-jj --lib --locked query::prepared:: -- --nocapture
```

**Notes**

Three intermediate runs each1passed/1failed: initial fresh-store failure, diagnostic RepairAborted, then quota error lost through redb Error::source. Replaced source-chain mapping with typed redb::Error conversion; kept quota assertions.
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
cargo test -p jj-store-jj --lib --locked query::prepared:: -- --nocapture
```

**Notes**

Final2passed: before-growth quota refusal, shared scratch/output allowance, ordered table seeks across transaction batches and typed initialization quota refusal.
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
cargo test -p jj-store-cli --test native_lifecycle --locked graph_acceptance:: -- --test-threads=2
```

**Notes**

Final21passed: exact segment/link parity for equal/skewed timestamps, multi-parent overflow and divergent changes; quota failure cleanup/retry; native graphs, cursors, cache corruption/eviction, cancellation, pins, membership and revocation. Warm reuse does not reserve unused scratch headroom.
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
target/debug/deps/native_lifecycle-d9faf2e6f13fc0f1 content_acceptance::index_task_acceptance:: --test-threads=2
```

**Notes**

5passed: deduplication, cancellation, expiry, revocation, restart and reserved interactive capacity.
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
cargo check -p jj-store-jj -p jj-store-service -p jj-store-cli --all-targets --locked
```

**Notes**

Affected package and direct consumer targets pass. Existing linker unwind-size advisory retained in test builds.
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
pnpm spec:first
```

**Notes**

Initial gate required architecture chapter for new adapter dependency; added its ownership and scratch-only contract.
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
pnpm check:boundaries; pnpm spec:first; pnpm check:progress; pnpm spec:validate; cargo fmt --all -- --check; pnpm exec prettier --check spec/src/jj-integration.md spec/src/architecture.md spec/src/cli.md
```

**Notes**

17package boundaries,9protected files/3chapters,136ledger tasks and167canonical requirements pass. Only changed canonical chapters formatted; ledger appended without whole-file reformat.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Explicit native construction now uses an8MiB page cache,512-write
  transactions, durable scratch checkpoints every16batches, disk-backed
  operation/commit/Kahn state and streaming output. Large-scale memory/latency
  qualification is still pending; neither2m native commits nor complete Linux
  history is qualified by this slice.
- Inline construction and upstream resident indexes retain existing hard bounds.
  Immutable generations, warm reuse without full checksums, short cache
  locks,4GiB/repository and16GiB total staging-inclusive quotas, local
  preparation, predicate/copy/prefix/search/RPC and collaboration/card
  projections remain pending.
- Running dev app and installed tools unchanged. Contracts/persisted formats
  unchanged; UI/browser and root aggregate lanes out of scope. E-READ-005
  registry/full-capture limitations and real-RustFS qualification remain open.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Commit verified disk-backed construction, then implement immutable generation reuse and staging-inclusive repository/global quotas before qualifying larger/local prepared indexes.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


