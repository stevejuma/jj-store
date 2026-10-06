---
schema: 1
id: E-READ-033
---

# E-READ-033

<!-- evidence-field:id markdown -->
## Id

E-READ-033
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

enclosing Jujutsu change vkxvponx
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Reuse separately prepared historical native residents without selected-slot growth
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust1.98.1; jj-lib0.45.1; macOS ARM64.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No authority, storage, HTTP schema or client changes. The native resident registry admits secondary indexes only by exact repository/epoch/operation. Fresh readers hold their saved allocation leases and enforce current hard commit limits; primary generation accounting stays separate from secondary reader leases. Generation parent handles deduplicate shared allocations. Current native capabilities still admit operation/view and each returned object.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/native_lifecycle/graph_cache.rs
- crates/jj-store-jj/src/query/cache/resident.rs
- crates/jj-store-jj/src/query/index.rs
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
cargo test -p jj-store-cli --test native_lifecycle --locked historical_predicates_reuse_separately_prepared_residents_without_slot_growth -- --nocapture (before)
```

**Notes**

Multiple prepared ancestor operations still invoke missing inline construction inside the current slot and return IndexRebuildRequired under threshold1. native-history-resident-before.log.
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
cargo test -p jj-store-cli --test native_lifecycle --locked historical_predicates_reuse_separately_prepared_residents_without_slot_growth -- --nocapture (after)
```

**Notes**

1pass/4.29s. Multiple exact historical scopes reuse separately prepared indexes under threshold1, match pinned ordinary output, leave primary file inventory unchanged and repeat without publication. native-history-resident-after.log.
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
cargo test -p jj-store-cli --test native_lifecycle --locked historical_predicates_reuse_separately_prepared_residents_without_slot_growth -- --nocapture (strengthened)
```

**Notes**

1pass/4.72s. A read-only principal uses owner-prepared data, loss of both old disk segments preserves exact historical output, primary files stay unchanged and revocation returns Denied. native-history-resident-final.log.
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
cargo test -p jj-store-cli --test native_lifecycle --locked graph_acceptance:: -- --nocapture (before final reader lease ownership adjustment)
```

**Notes**

31pass/11.91s. Direct native graph/cache/decorator/preparation/permission/binding/cancellation consumers. native-history-consumers-033.log. Final lease ownership adjustment reruns these direct consumers separately.
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
cargo test -p jj-store-cli --test native_lifecycle --locked historical_predicates_reuse_separately_prepared_residents_without_slot_growth -- --nocapture (final reader-owned leases)
```

**Notes**

1pass/1.53s. Strengthened disk-loss/current reader authorization test passes with secondary leases attached to the request reader rather than generation accounting. native-history-resident-leases-final.log.
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
cargo test -p jj-store-cli --test native_lifecycle --locked graph_acceptance:: -- --nocapture (final)
```

**Notes**

31pass/8.37s. Complete directly affected native graph/cache/preparation/decorator/predicate/identity/permission/cancellation family. native-history-consumers-033-final.log.
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
cargo test -p jj-store-jj --lib --locked query::memory:: -- --nocapture
```

**Notes**

5pass/0.01s. Shared native/local LRU, oversized denials, busy slots, active reader lease lifetime and allocating-frame refusal. native-history-memory-units-033.log.
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
cargo check -p jj-store-jj -p jj-store-cli --lib --locked
```

**Notes**

Affected integration/direct CLI libraries pass/16.37s. native-history-packages-033.log. No public schema changes.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
git -C /Users/stevejuma/code/lapismd/jj-store-bounded-reads diff --check (optional inspection probe)
```

**Notes**

Inapplicable read-only probe: this temporary workspace is Jujutsu-managed without a colocated Git working tree. No files changed. Corrected through jj --no-pager diff --git plus added-line whitespace validation; governed formatting/acceptance remain passing.
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
cargo fmt -p jj-store-jj -p jj-store-cli -- --check; pnpm exec prettier --write spec/src/jj-integration.md spec/src/cli.md spec/src/architecture.md docs/implementation-plan.md; pnpm spec:first; pnpm spec:validate; pnpm check:progress; jj --no-pager diff --git plus added-line whitespace check
```

**Notes**

Final targeted Rust/document formatting,7protected files/4chapters,5validators/23chapters/172requirements,138task bookkeeping and JJ diff whitespace pass. native-history-rust-format-033-final.log/native-history-doc-format-033-final.log/native-history-spec-first-033.log/native-history-spec-033.log/native-history-progress-033.log/native-history-diff-033.patch. Complete verification and UI/root aggregate lanes not run.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Cold additional-operation construction still mutates the retained primary slot
  under its namespace lock. Immutable cold generations and shorter/general warm
  locking remain implementation work. Secondary resident reuse requires those
  generations already be prepared and retained; it does not infer readiness
  after cache loss/restart.
- Lightweight repository summaries, local WorkStore/root reuse, local
  operation/prefix/comparison/deep/predecessor/copy/search and paged native RPC
  delivery remain open.
- Complete combined integration/contracts, Linux/native large fixtures, isolated
  RSS/release latency, cold production browser,
  restart/revocation/publication/fault/real-RustFS, registry-only and
  Help/source-review qualification remain deferred. No milestone promotion or
  changes to main/original dev services/Linux source/visual baselines.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Continue native immutable additional-operation generations/short locking and remaining high-priority local lookup and lightweight summary delivery; qualify the combined implementation in the later slice.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Avoid repeated historical native construction and copying/hashing another generation when its exact prepared resident is already retained.
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/native_lifecycle/graph_cache.rs#historical_predicates_reuse_separately_prepared_residents_without_slot_growth
- crates/jj-store-cli/tests/native_lifecycle/graph_cache.rs#resident_indexes_preserve_multiple_historical_operation_scopes
- crates/jj-store-cli/tests/native_lifecycle/graph_cache.rs#graph_cache_bindings_separate_repository_and_epoch_for_identical_root_ids
- crates/jj-store-cli/tests/native_lifecycle/graph_cache.rs#warm_graph_survives_concurrent_preparation_and_cache_eviction
- crates/jj-store-jj/src/query/memory.rs#shared_resident_admission_lives_until_the_last_reader_exits
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


