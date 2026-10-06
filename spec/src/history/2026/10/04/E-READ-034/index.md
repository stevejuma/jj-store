---
schema: 1
id: E-READ-034
---

# E-READ-034

<!-- evidence-field:id markdown -->
## Id

E-READ-034
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

enclosing Jujutsu change pmtoksup
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Immutable primary native generations and private bounded historical construction outside graph namespace locking
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust1.98.1; jj-lib0.45.1; macOS ARM64.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No authoritative, storage, HTTP schema or client changes. Primary derived receipts retain only their selected-operation generation. Missing historical operations use private full-allowance reservations and exact checked format6 output; immutable owned arrays and original frame leases survive scratch cleanup. Auxiliary residents bind repository/epoch/operation, bound count/weight and join the shared1GiB native/local allocation pool. Fresh capabilities and current authorization remain request-owned.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/native_lifecycle/graph_cache.rs
- crates/jj-store-jj/src/query/cache.rs
- crates/jj-store-jj/src/query/cache/constructed.rs
- crates/jj-store-jj/src/query/cache/reservation.rs
- crates/jj-store-jj/src/query/cache/resident.rs
- crates/jj-store-jj/src/query/cache/resident/additional.rs
- crates/jj-store-jj/src/query/index.rs
- crates/jj-store-jj/src/query/index/secondary.rs
- crates/jj-store-jj/src/query/memory.rs
- crates/jj-store-jj/src/query/mod.rs
- crates/jj-store-jj/src/query/projection.rs
- docs/implementation-plan.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/jj-integration.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-JJ-003
- JST-JJ-005
- JST-CLI-003
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
cargo test -p jj-store-cli --test native_lifecycle --locked cold_historical_indexes_leave_the_primary_generation_immutable -- --nocapture (before)
```

**Notes**

Primary receipt gained historical links/segments. native-generation-before.log.
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
target/debug/deps/native_lifecycle-d9faf2e6f13fc0f1 warm_filtered_graph_starts_while_another_operation_is_preparing --nocapture (before)
```

**Notes**

Compiled pre-change binary returned Busy for a filtered warm graph. native-filtered-warm-before.log.
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
cargo test -p jj-store-cli --test native_lifecycle --locked cold_historical_indexes_leave_the_primary_generation_immutable -- --nocapture (after)
```

**Notes**

1pass/1.36s. Exact primary file receipt unchanged, repeated historical data equal with fewer object reads, private scratch removed. native-generation-after.log.
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
cargo test -p jj-store-cli --test native_lifecycle --locked warm_filtered_graph_starts_while_another_operation_is_preparing -- --nocapture
```

**Notes**

1pass/0.67s. Filtered warm graph runs while a different primary is preparing. native-filtered-warm-after.log.
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
cargo test -p jj-store-cli --test native_lifecycle --locked private_historical_construction_reserves_quota_without_blocking_warm_graphs -- --nocapture
```

**Notes**

1pass/0.95s. One-shot gated constructor proves charged reservation, namespace-free warm filtered reads, unchanged primary receipt with two entries, one-entry eviction with retained memory fallback and worker-exit cleanup. native-private-quota-034.log.
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
cargo test -p jj-store-cli --test native_lifecycle --locked graph_acceptance:: -- --nocapture (initial)
```

**Notes**

33pass/1fail/8.96s. Recovery fixture expected no retained slot after failed revset. Primary installation now precedes evaluation; replaced that old lifecycle assumption with exact verified-file-set preservation and successful subsequent read. native-graph-consumers-034.log.
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
cargo test -p jj-store-cli --test native_lifecycle --locked graph_cache_recovery_rechecks_permission_and_serializes_independent_executors -- --nocapture
```

**Notes**

1pass/0.28s. Denied queries leave damage untouched, independent executor lock contention remains Busy, interrupted stage is removed, failed revset preserves complete verified files and subsequent read. native-recovery-034.log.
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
cargo test -p jj-store-cli --test native_lifecycle --locked graph_acceptance:: -- --nocapture (final)
```

**Notes**

34pass/8.70s. Complete directly affected native graph/cache/preparation/predicate/decorator/permission/identity/cancellation consumers. native-graph-consumers-034-final.log.
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
cargo test -p jj-store-jj --lib --locked query::cache:: -- --nocapture
```

**Notes**

6pass/0.19s. Private checksum/link verification, cross-profile reservations, stale cleanup, repository/total admission and warm metadata verification. native-cache-units-034.log.
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
cargo test -p jj-store-jj --lib --locked query::memory:: -- --nocapture
```

**Notes**

5pass/0.02s. Shared allocation lifetime/idle eviction, busy reader preservation and framing admission. native-memory-units-034.log.
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
cargo test -p jj-store-cli --test local_browser --locked preparation_acceptance:: -- --nocapture
```

**Notes**

3pass/1intentionally ignored large fixture/6.60s. Direct local consumers of unchanged public reservation wrapper and shared owned writer. native-local-reservation-consumers-034.log.
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
cargo fmt -p jj-store-jj -p jj-store-cli -- --check; pnpm exec prettier --write spec/src/jj-integration.md spec/src/cli.md spec/src/architecture.md docs/implementation-plan.md spec/src/milestones.md; pnpm spec:first; pnpm spec:validate; pnpm check:progress; jj --no-pager diff --git plus added-line whitespace validation
```

**Notes**

Targeted formatting,12protected files/4chapters,5validators/23chapters/172requirements,138task bookkeeping and JJ whitespace pass. native-rust-format-034.log/native-doc-format-034-final.log/native-spec-first-034.log/native-spec-034.log/native-progress-034.log/native-diff-034.patch. No root aggregates, full contracts, UI/release build, scale fixture or comprehensive qualification. Inherited oversized __eh_frame linker warning remains.
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
cargo check -p jj-store-jj -p jj-store-cli --lib --locked
```

**Notes**

Affected JJ/CLI library checks pass/41.77s with direct consumers compiling. native-packages-034.log. Public wire contracts unchanged.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Cold primary construction still holds the namespace lock through its build;
  separately reserved/staged primary admission remains high-priority delivery
  work.
- Lightweight repository summaries, local WorkStore/root reuse, local
  operation/prefix/comparison/deep/predecessor/copy/search and paged native RPC
  remain implementation work.
- Complete combined integration/contracts, Linux/native large fixtures, isolated
  RSS/release latency, cold production browser,
  restart/revocation/publication/fault/real-RustFS, registry-only and
  Help/source-review qualification remain deferred. Focused delivery does not
  promote milestones. Main/original dev services/Linux source/visual baselines
  unchanged.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Continue high-priority native primary staging/locking and remaining lookup/summary delivery; complete combined qualification later.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Prevent historical evaluation from extending primary disk generations or holding the namespace while privately constructing immutable data-only historical indexes.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-READ-034 details 2](details-02.md)
