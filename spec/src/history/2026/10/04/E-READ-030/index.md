---
schema: 1
id: E-READ-030
---

# E-READ-030

<!-- evidence-field:id markdown -->
## Id

E-READ-030
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M11.4
- M13.4
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-04
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change mrumlvxk
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Root-bound native WorkStore order/count/search projections and selected-window summary hydration
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust1.98.1; jj-lib0.45.1; macOS ARM64. Existing bounded collaboration cache and WorkStore capacities retained.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Authoritative WorkStore records, native/companion snapshots, wire DTOs and opaque cursor format unchanged. Native first pages share audited root data keyed by repository/epoch/root; browse cursors retain compact matching keys. Local browse uses the same window projector but still reconstructs each request.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-service/src/collab_reads/materialization.rs
- crates/jj-store-service/src/collab_reads.rs
- crates/jj-store-service/src/work/browse.rs
- crates/jj-store-service/src/work/browse_tests.rs
- crates/jj-store-service/src/work/cursors.rs
- crates/jj-store-service/src/work/projection.rs
- crates/jj-store-service/src/work/reads.rs
- crates/jj-store-service/src/work.rs
- crates/jj-store-service/tests/work_store/cache.rs
- crates/jj-store-service/tests/work_store.rs
- docs/implementation-plan.md
- spec/src/architecture.md
- spec/src/mutations.md
- spec/src/projects.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-WORK-005
- JST-WORK-009
- JST-MUT-001
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
cargo test -p jj-store-service --lib --locked work_browse_hydrates_only_the_returned_window -- --nocapture (initial fixture)
```

**Notes**

Fixture imported a WorkActorV1 type not exported by WAL. Repaired fixture through the existing ProjectV1 serde contract; no product/dependency change. work-window-before.log.
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
cargo test -p jj-store-service --lib --locked work_browse_hydrates_only_the_returned_window -- --nocapture (before)
```

**Notes**

One-row offset89/query matches100rows and eagerly hydrates100summaries. Exact red baseline after fixture repair: work-window-before-after-fixture.log.
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
cargo test -p jj-store-service --lib --locked work_browse_hydrates_only_the_returned_window -- --nocapture (after)
```

**Notes**

1pass. Deep project/task windows hydrate only returned rows while preserving complete-root matches, counts, identity ordering and tombstone handling. work-window-after.log.
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
cargo test -p jj-store-service --test work_store --locked work_first_pages_share_selected_roots_and_enforce_fresh_limits -- --nocapture
```

**Notes**

1pass. Concurrent warm browse/detail perform0WorkStore object reads; fresh lower inventory admission rejects warm and retained pages; a new root reloads and old query keys/counts remain pinned. work-cache-after.log.
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
cargo check -p jj-store-service -p jj-store-http -p jj-store-rpc --lib --locked
```

**Notes**

Affected package and direct HTTP/RPC libraries pass/28.95s; unchanged public contracts. work-packages-030.log.
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
cargo test -p jj-store-service --test work_store --test local_work --locked
```

**Notes**

15pass:5native/10local direct WorkStore consumers, filtering/counts/pins/receipts/staging/identity/GC/expiry. work-consumers-030.log.
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
cargo test -p jj-store-service --test native_collab --locked native_work_comment_reactions_pin_versions_authorize_and_survive_cold_sessions -- --nocapture
```

**Notes**

1pass/0.21s. Direct WorkComment reaction consumer remains exact-version/home-authorized through cold sessions. work-reactions-030.log.
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
cargo fmt -p jj-store-service -- --check; pnpm exec prettier --write spec/src/projects.md spec/src/mutations.md spec/src/architecture.md docs/implementation-plan.md; pnpm spec:first; pnpm spec:validate; pnpm check:progress
```

**Notes**

Focused formatting passes;11protected files/4chapters,5validators/23chapters/172requirements and138task bookkeeping pass. Logs work-rust-format-030.log/work-doc-format-030.log/work-spec-first-030.log/work-spec-030.log/work-progress-030.log. No UI/RustFS/root aggregates or comprehensive qualification rerun.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Cross-request local WorkStore root reuse, repository card
  aggregate/selected-tip summaries, application content search projections,
  native immutable additional-operation construction and prepared local
  operation/prefix/comparison/deep/native RPC delivery remain open.
- Native root cache retention is capped at64MiB plus the existing64MiB cursor
  registry; ordered keys and normalized fields are included in charges. Delivery
  tests are small deterministic regressions, not combined retained-root pressure
  or isolated RSS qualification.
- Complete combined contracts/integration, cold production browser, Linux/native
  large fixtures, release latency/RSS,
  restart/revocation/publication/fault/real-RustFS, registry-only and
  Help/source-review refresh remain the separate later qualification slice. No
  milestone promotion; main/original dev services/Linux source/visual baselines
  untouched.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Continue lightweight repository cards and shared local/root-bound application search delivery in this retained workspace; comprehensive verification stays deferred.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Remove repeated native WorkStore reconstruction, repeated sorting and quadratic summary count scans; hydrate only selected native/local browse windows under existing authority/capacity contracts.
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-service/src/work/browse_tests.rs#work_browse_hydrates_only_the_returned_window
- crates/jj-store-service/tests/work_store/cache.rs#work_first_pages_share_selected_roots_and_enforce_fresh_limits
- crates/jj-store-service/tests/work_store.rs#work_browse_filters_complete_roots_and_pins_summary_counts
- crates/jj-store-service/tests/local_work.rs#local_browser_browse_uses_complete_root_and_explicit_committed_pin
- crates/jj-store-service/tests/native_collab.rs#native_work_comment_reactions_pin_versions_authorize_and_survive_cold_sessions
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


