---
schema: 1
id: E-READ-031
---

# E-READ-031

<!-- evidence-field:id markdown -->
## Id

E-READ-031
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M15.6
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-04
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change tkqyuwkq
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Root-bound current-content search fields/order with retained matching keys and selected-hit Unicode snippets
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust1.98.1; jj-lib0.45.1; macOS ARM64. Domain capacities and wire cursor contract unchanged.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No authoritative/storage/HTTP schema changes. Derived Thread/Work field indexes retain current body/normalized text and separate recent/query order, keyed by repository/epoch/immutable root. Opaque cursor pins retain matching source keys and share root bytes. Captured manifest envelope, viewer and inventory admission remain fresh.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-service/src/content_search/cursors.rs
- crates/jj-store-service/src/content_search/extract.rs
- crates/jj-store-service/src/content_search/index.rs
- crates/jj-store-service/src/content_search/index_tests.rs
- crates/jj-store-service/src/content_search/materialization.rs
- crates/jj-store-service/src/content_search.rs
- crates/jj-store-service/tests/content_search/cache.rs
- crates/jj-store-service/tests/content_search.rs
- docs/implementation-plan.md
- spec/src/api.md
- spec/src/architecture.md
- spec/src/collaboration.md
- spec/src/mutations.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-API-009
- JST-COLLAB-007
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
cargo test -p jj-store-service --test content_search --locked content_search_reuses_unchanged_roots_across_filters_and_rechecks_limits -- --nocapture (before)
```

**Notes**

Changing the query from needle to other reconstructs unchanged WorkStore data with4object reads instead of0. content-index-before.log.
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
cargo test -p jj-store-service --test content_search --locked content_search_reuses_unchanged_roots_across_filters_and_rechecks_limits -- --nocapture (after)
```

**Notes**

1pass/0.19s. New filters perform0domain reads; fresh lower inventory admission rejects warm and continued pages; changed root reloads, historical keys/snapshot remain pinned. content-index-after.log.
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
cargo test -p jj-store-service --test content_search --locked
```

**Notes**

7pass/0.77s. Complete direct content search test binary: current edits/tombstones/scopes, recent archived/entity rules, exact filters/page identity, viewer revocation and source limits. content-consumers-031.log.
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
cargo test -p jj-store-service --lib --locked content_search:: -- --nocapture
```

**Notes**

Initial2unit targets pass; content-units-031.log.
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
cargo test -p jj-store-service --lib --locked content_search:: -- --nocapture (final deep-window/before-growth regressions)
```

**Notes**

4pass. Deep offset49window materializes exactly2hits, stable domain/ID ties and recent exclusion preserved; field growth rejected before any document is retained, plus Unicode expansion and64cursor/expiry/restart guards. content-units-031-final.log.
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
cargo check -p jj-store-service -p jj-store-http --lib --locked
```

**Notes**

Affected service and direct HTTP boundary pass/27.73s. No schema/client generation required because contracts are unchanged. content-packages-031.log.
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
cargo fmt -p jj-store-service -- --check; pnpm exec prettier --write spec/src/api.md spec/src/collaboration.md spec/src/mutations.md spec/src/architecture.md docs/implementation-plan.md; pnpm spec:first; pnpm spec:validate; pnpm check:progress
```

**Notes**

Focused formatting passes;9protected files/5chapters,5validators/23chapters/172requirements and138task bookkeeping pass. Logs content-rust-format-031.log/content-doc-format-031.log/content-spec-first-031.log/content-spec-031.log/content-progress-031.log. Clarified request-principal exclusion separately from stored actor attribution, final doc formatter content-doc-format-031-final.log. Complete qualification not run.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Lightweight repository summaries, cross-request local WorkStore
  reconstruction, native immutable additional-operation construction, prepared
  local operation/prefix/comparison and deep/predecessor/copy/native RPC
  delivery remain implementation work.
- Field index estimates are capped at64MiB per constructed root and64MiB total
  cache retention; existing64MiB/64token cursor capacity and2search permits
  remain. Roots held by cursor pins are charged separately from cache retention;
  tests do not constitute isolated RSS or near-capacity/concurrent-root
  qualification.
- Complete combined integration/contracts, cold production browser, Linux/native
  large fixtures,
  release/RSS/pressure/restart/revocation/publication/fault/real-RustFS,
  registry-only and Help/source-review qualification remain deferred to the
  separate later slice. Main/original dev services/Linux source/visual baselines
  unchanged; no milestone promotion.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Continue performance delivery with lightweight repository summaries and remaining native/local index/generation features in the retained temporary workspace; do comprehensive qualification later.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Reuse full-domain validated current-content fields across first queries and avoid all-match snippet construction/repeated normalization/sorting during browse and deep continuations.
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-service/tests/content_search/cache.rs#content_search_reuses_unchanged_roots_across_filters_and_rechecks_limits
- crates/jj-store-service/tests/content_search.rs#current_full_bodies_edits_tombstones_and_scopes_are_manifest_selected
- crates/jj-store-service/tests/content_search.rs#fresh_and_resumed_content_never_disclose_after_token_revocation
- crates/jj-store-service/src/content_search/index_tests.rs#search_merges_root_orders_and_hydrates_only_the_selected_deep_window
- crates/jj-store-service/src/content_search/index_tests.rs#search_projection_refuses_body_growth_before_retaining_fields
- crates/jj-store-service/src/content_search/extract.rs#excerpt_keeps_unicode_expansion_match_in_original_text
- crates/jj-store-service/src/content_search/cursors.rs#capacity_restart_and_query_deadline_fail_closed_without_eviction
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


