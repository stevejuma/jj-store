---
schema: 1
id: E-READ-032
---

# E-READ-032

<!-- evidence-field:id markdown -->
## Id

E-READ-032
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4.1
- M13.4
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-04
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change pkonqplr
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Owned bounded native interactive/additional-operation construction with small ordered scratch
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust1.98.1; jj-lib0.45.1; macOS ARM64.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No authoritative, storage or wire changes. Owned format6 segment/control writer now covers missing native inline/additional operations as well as explicit preparation. Compact scratch caps64MiB and shares process resident admission; large explicit native and local builds retain disk scratch. Actual receipt bytes are admitted before temporary writes.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/native_lifecycle/graph_cache.rs
- crates/jj-store-cli/tests/native_lifecycle/graphs.rs
- crates/jj-store-jj/src/query/cache.rs
- crates/jj-store-jj/src/query/index.rs
- crates/jj-store-jj/src/query/mod.rs
- crates/jj-store-jj/src/query/prepared/mod.rs
- crates/jj-store-jj/src/query/prepared/segment.rs
- crates/jj-store-jj/src/query/prepared/tables.rs
- docs/implementation-plan.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/jj-integration.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

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
cargo test -p jj-store-cli --test native_lifecycle --locked native_preparation_fits_small_cache_and_preserves_historical_scopes -- --nocapture (before)
```

**Notes**

Small explicit native preparation rejects64KiB before construction because it reserves a fixed1MiB receipt plus disk scratch. native-construction-before.log.
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
cargo test -p jj-store-cli --test native_lifecycle --locked native_preparation_fits_small_cache_and_preserves_historical_scopes -- --nocapture (after)
```

**Notes**

1pass/2.12s. Small preparation and multiple historical scopes match the pinned oracle; repeated warm reads and unchanged manifest. native-construction-after.log.
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
cargo test -p jj-store-cli --test native_lifecycle --locked graph_acceptance:: -- --nocapture
```

**Notes**

30pass/8.95s. Direct native graph/cache/preparation/predicate/decorator/identity/permission/cancellation consumers. Quota fixture now uses1024bytes below complete framing; assertion, cleanup and retry remain unchanged because small scratch removes the old minimum. native-graph-consumers-032.log.
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
cargo test -p jj-store-jj --lib --locked query::prepared:: -- --nocapture
```

**Notes**

3pass/0.24s. Small ordered seeks, before-retention admission, scratch disk before-growth bounds and batching/quota guard. native-scratch-units-032.log.
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
cargo test -p jj-store-cli --test local_browser --locked preparation_acceptance:: -- --nocapture
```

**Notes**

3pass/1intentionally ignored large-repository smoke/7.27s. Shared writer preserves byte-identical upstream skew/merge/divergence format, pinned source semantics, cancellation and cold/retry behavior. native-local-writer-consumers-032.log. Large fixture remains deferred.
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
cargo check -p jj-store-jj -p jj-store-cli --lib --locked
```

**Notes**

Affected integration and direct CLI libraries pass/48.30s. native-packages-032.log. Public contracts unchanged; no client/schema generation required.
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
cargo fmt -p jj-store-jj -p jj-store-cli -- --check; pnpm exec prettier --write spec/src/jj-integration.md spec/src/cli.md spec/src/architecture.md docs/implementation-plan.md; pnpm spec:first; pnpm spec:validate; pnpm check:progress
```

**Notes**

Focused formatting,9protected files/4chapters,5validators/23chapters/172requirements and138task bookkeeping pass. native-rust-format-032.log/native-doc-format-032.log/native-spec-first-032.log/native-spec-032.log/native-progress-032.log. No root aggregates, UI rebuild, full contracts, scale fixture or comprehensive qualification. CLI linking retains the existing oversized __eh_frame warning.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Immutable native cache generations, auxiliary-operation reuse and shorter
  namespace locking remain implementation work; missing-operation builds are
  bounded but still add derived files in the selected slot under the existing
  lock.
- Lightweight repository summaries, cross-request local WorkStore projections,
  local operation/prefix/comparison/deep/predecessor/copy/search and paged
  native RPC delivery remain open.
- Complete combined integration/contracts, Linux/native large fixtures, isolated
  RSS/release latency, cold production browser,
  restart/revocation/publication/fault/real-RustFS, registry-only and
  Help/source-review qualification remain deferred. Focused delivery results do
  not promote milestones. Main/original dev services/Linux source/visual
  baselines unchanged.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Continue remaining high-priority native generation/locking and local lookup performance delivery in the retained temporary workspace; qualify the combined implementation later.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Replace upstream missing-operation construction with compact pre-growth-admitted scratch and streamed bounded output, including tiny explicit preparation without a database minimum.
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/native_lifecycle/graph_cache.rs#native_preparation_fits_small_cache_and_preserves_historical_scopes
- crates/jj-store-cli/tests/native_lifecycle/graph_cache.rs#resident_indexes_preserve_multiple_historical_operation_scopes
- crates/jj-store-cli/tests/native_lifecycle/graphs.rs#disk_preparation_matches_pinned_upstream_index_bytes
- crates/jj-store-cli/tests/native_lifecycle/graphs.rs#disk_preparation_quota_failure_discards_stage_and_allows_retry
- crates/jj-store-jj/src/query/prepared/tables.rs#small_tables_preserve_order_and_reject_growth_before_retaining_data
- crates/jj-store-cli/tests/local_browser/preparation.rs#local_preparation_matches_upstream_without_trees_or_source_index_writes
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


