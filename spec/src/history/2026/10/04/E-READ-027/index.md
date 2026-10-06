---
schema: 1
id: E-READ-027
---

# E-READ-027

<!-- evidence-field:id markdown -->
## Id

E-READ-027
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

enclosing Jujutsu change kumwspzq
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Native prepared selected visibility and reverse bookmark/workspace annotations
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust1.98.1; jj-lib0.45.1; Node26.9.0; pnpm10.32.1; macOS ARM64.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Native64-byte commit identities, JJ format6 and authoritative/wire formats unchanged. Prepared immutable memory retains selected visibility and reverse labels under shared allocation admission. Fresh request budgets/access remain mandatory. Large disk-only generations use existing explicit rebuild-required recovery.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/native_lifecycle/graph_projection.rs
- crates/jj-store-cli/tests/native_lifecycle/graphs.rs
- crates/jj-store-jj/src/query/cache/resident.rs
- crates/jj-store-jj/src/query/cache.rs
- crates/jj-store-jj/src/query/graphs.rs
- crates/jj-store-jj/src/query/index.rs
- crates/jj-store-jj/src/query/mod.rs
- crates/jj-store-jj/src/query/projection.rs
- spec/src/jj-integration.md
- spec/src/cli.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-JJ-003
- JST-JJ-005
- JST-API-001
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
cargo test -p jj-store-cli --test native_lifecycle --locked native_prepared_rows_reuse_visibility_and_charge_only_returned_labels -- --nocapture (before)
```

**Notes**

Selected base returned5,000 bookmark labels at work238 under max_work300 instead of Limit; native-projection-before.log. Exact failing assertion corrected to avoid dumping the entire row.
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
cargo test -p jj-store-cli --test native_lifecycle --locked native_prepared_rows_reuse_visibility_and_charge_only_returned_labels -- --nocapture (after)
```

**Notes**

1pass/8.83s. Two warm one-row windows reuse selected visibility/workspace/local/remote labels under work300; selected5,000-label row is refused before cloning. Manifest unchanged. native-projection-after.log.
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

29direct graph/cache/preparation/resource tests pass/7.48s,175unrelated tests filtered out. Includes exact after regression, explicit disk-generation projection recovery after executor restart, hidden evolution/divergence/conflicts, pinned three histories, revocation, quota/corruption and concurrent readers. native-projection-consumers.log. Linker compact-unwind size warning retained; no compilation failure.
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
cargo check -p jj-store-jj --lib --locked
```

**Notes**

Affected package passes/21.20s; CLI/server/service composition compiled through focused consumer tests. package-check-027.log.
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
cargo fmt -p jj-store-jj -p jj-store-cli; cargo fmt -p jj-store-jj -p jj-store-cli -- --check; pnpm exec prettier --write spec/src/jj-integration.md spec/src/cli.md; pnpm exec prettier --check spec/src/jj-integration.md spec/src/cli.md; pnpm spec:first
```

**Notes**

Focused formatting and spec-first8protected files/2chapters pass. format-rust-027.log/format-docs-027.log/spec-first-027.log.
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
pnpm spec:validate; pnpm check:progress
```

**Notes**

5validators/23chapters/172requirements and138tasks pass; spec-027.log/progress-027.log. Historical evidence bytes preserved; no milestone promotion.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Shared global eviction of idle resident generations, immutable native
  cache/additional-operation construction and before-growth admission remain
  delivery work. Current projection reserves256bytes/indexed native commit plus
  exact head/name/term charges; this is an allocation estimate, not RSS
  qualification.
- Prepared local operation/prefix/comparison and indexed filtering, deep
  lookup/native RPC, root-bound collaboration/search and lightweight visible
  repository cards remain implementation slices.
- "Complete combined qualification is deferred: Linux/native large fixtures,
  RSS/release latency, comprehensive
  contracts/cold-browser/restart/fault/real-RustFS, registry-only and Help
  review refresh. No milestone promotion; main/original dev services/Linux
  source untouched."
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Deliver shared native/local resident eviction and remaining high-priority performance features in this temporary workspace; comprehensive verification remains its own later slice.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Prepared native visibility/reverse-label delivery with exact regression and direct graph/cache consumers; no full-plan completion.
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/native_lifecycle/graph_projection.rs#native_prepared_rows_reuse_visibility_and_charge_only_returned_labels
- crates/jj-store-cli/tests/native_lifecycle/graph_projection.rs#native_prepared_projection_is_recovered_explicitly_after_executor_restart
- crates/jj-store-cli/tests/native_lifecycle/graphs.rs#graph_query_three_histories_filtered_edges_and_snapshot_pagination
- crates/jj-store-cli/tests/native_lifecycle/graph_cache.rs#warm_graph_starts_while_another_operation_is_preparing
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


