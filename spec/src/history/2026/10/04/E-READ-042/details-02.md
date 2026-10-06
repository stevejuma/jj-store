---
schema: 1
id: E-READ-042
---

# E-READ-042 details 2

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
cargo test -p jj-store-cli --test local_browser --locked prepared_operation_prefixes_resolve_with_only_selected_metadata -- --nocapture (before)
```

**Notes**

1failure/12.52s: prepared prefix path returns Invalid before implementation. operation-prefix-before-042.log.
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
cargo check -p jj-store-jj --locked (initial)
```

**Notes**

Initial owning package check passes. operation-prefix-check-initial-042.log.
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
cargo test -p jj-store-cli --test local_browser --locked prepared_operation_prefixes_resolve_with_only_selected_metadata -- --nocapture (initial after)
```

**Notes**

1pass/9.77s: ready full/odd/uppercase/parent prefixes. operation-prefix-after-042.log.
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
cargo test -p jj-store-cli --test local_browser --locked prepared_operation_prefixes_resolve_with_only_selected_metadata -- --nocapture (strengthened child)
```

**Notes**

1failure/10.35s: child suffix still walks retained operations and correctly hits tiny budget. Recorded indexed child neighbors as separate work; retained default semantic equivalence and explicit small-budget Limit. operation-prefix-final-042.log.
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
cargo test -p jj-store-cli --test local_browser --locked prepared_operation_prefixes_resolve_with_only_selected_metadata -- --nocapture (fingerprints)
```

**Notes**

1pass/21.97s: remove all disposable Git objects, damage commit indexes, reject later/ambiguous prefixes and replaced historical protobuf, then restore/retry. operation-prefix-fingerprint-042.log. Denied access added afterward and passes in final subset.
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
cargo test -p jj-store-jj --lib --locked query::prepared::operations:: -- --nocapture; cargo test -p jj-store-jj --lib --locked query::prepared:: -- --nocapture (initial/final)
```

**Notes**

Initial2lookup tests; then6builder tests pass twice (0.23s/0.40s), including writer disk admission and corruption/selection/order/checksum/limit/cancellation guards. operation-prefix-unit/builder/builder-final-042.log.
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
cargo test -p jj-store-jj --lib --locked query::language::operations:: -- --nocapture
```

**Notes**

2parser tests pass. operation-prefix-parser-042.log.
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
cargo check -p jj-store-jj -p jj-store-service -p jj-store-http -p jj-store-cli --locked
```

**Notes**

Affected JJ and direct service/HTTP/CLI packages pass (2m31s with lock waits). operation-prefix-check-042.log.
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
cargo test -p jj-store-cli --test local_browser --locked prepared_ -- --nocapture (first)
```

**Notes**

6pass/1fail (29.62s): exact prepared/upstream directory assertion sees new intentional local-operations file; upstream segment/link bytes match. operation-prefix-prepared-regressions-042.log.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-cli --test local_browser --locked local_preparation_ -- --nocapture (first)
```

**Notes**

1pass/1fail (4.74s): same new disposable artifact in exact directory comparison. Updated exactly two fixtures to validate artifact framing/selection/order/checksum and compare every remaining upstream file byte-for-byte. operation-prefix-preparation-042.log.
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
cargo test -p jj-store-cli --test local_browser --locked local_point_http_uses_ready_membership_and_only_falls_back_when_cold -- --nocapture
```

**Notes**

1pass/10.40s: exact cold409 rebuild-required pin, ready historical prefix and selected source/point proofs. Disposable service cleaned up. operation-prefix-http-042.log.
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
cargo test -p jj-store-cli --test local_browser --locked operation_pages_ignore_corrupt_commit_indexes -- --nocapture
```

**Notes**

1pass/5.55s: plain metadata-only cold pages retain independence. operation-prefix-cold-042.log.
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
cargo test -p jj-store-cli --test native_lifecycle --locked history_queries_filter_sparse_ancestry_before_paging_and_never_publish -- --nocapture
```

**Notes**

1pass/1.17s: native selector and sparse-filter semantics preserved. operation-prefix-native-042.log.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-cli --test local_browser --locked local_preparation_matches_upstream_without_trees_or_source_index_writes -- --nocapture; cargo test -p jj-store-cli --test local_browser --locked prepared_local_log_preserves_conflicts_and_filtered_native_edges -- --nocapture
```

**Notes**

Both original failed tests repaired:1pass/5.26s and1pass/5.65s. Exact native index bytes retained. operation-prefix-preparation-repaired/log-repaired-042.log.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-cli --test local_browser --locked prepared_ -- --nocapture; cargo test -p jj-store-cli --test local_browser --locked local_preparation_ -- --nocapture (final)
```

**Notes**

7prepared tests/34.12s and2preparation tests/6.17s. Direct point/detail/reference/operation/visibility/HTTP/preparation consumers pass. operation-prefix-prepared-final/preparation-final-042.log.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
cargo fmt -p jj-store-jj -p jj-store-cli; cargo fmt -p jj-store-jj -p jj-store-cli -- --check; pnpm check:boundaries; pnpm check:ui-ownership; pnpm exec prettier --write spec/src/api.md spec/src/architecture.md spec/src/cli.md spec/src/compatibility.md spec/src/jj-integration.md spec/src/local-jj.md docs/implementation-plan.md
```

**Notes**

Changed Rust/docs formatting,17package boundaries and1ownership validator pass; exact log prefixes operation-prefix-rust-format/boundaries/ownership/format[-final]-042.log. Final fixture Rust formatted again.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --write spec/src/milestones.md; pnpm check:progress; pnpm spec:first; pnpm spec:validate; cargo fmt -p jj-store-jj -p jj-store-cli -- --check
```

**Notes**

Final ledger formatting,138task bookkeeping,13protected files/7canonical chapters,5validators/23chapters/172requirements and Rust formatting pass. operation-prefix-ledger-format/progress/spec-first/spec/rust-format-final-042.log.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- 19 unique focused Rust tests pass. Operation-prefix delivery complete; overall
  bounded-read qualification and milestone remain pending. Source fingerprints
  preserve SimpleOpStore normalization and reject replaced historical operation
  data even with a retained lookup.
- Prepared operation comparisons, indexed filtered references/child neighbors
  and deep operation/predecessor/copy/search continuations, paged native RPC,
  local operation-difference/change-group paging and remaining local application
  search/index delivery remain open.
- Full combined integration/contracts, Linux/native/near-capacity fixtures,
  measured RSS/release latency, complete cold production browser,
  fault/revocation/restart/publication/real-RustFS, registry/release and
  Help/source-review refresh remain deferred. UI code/public schemas unchanged;
  UI suites/build and contract regeneration are out of scope. Main/Linux source
  and original dev services untouched. Visual baselines remain unapproved;
  inherited __eh_frame linker warning and earlier strict-lint gaps remain
  deferred.
<!-- /evidence-field -->

