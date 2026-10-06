---
schema: 1
id: E-READ-046
---

# E-READ-046 details 2

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
cargo test -p jj-store-jj --lib --locked native_projection_admits_two_million_identities_with_index_headroom -- --nocapture (before)
```

**Notes**

1failed/0passed as intended: raw417048576+projection1025048576 bytes exceed the unchanged1GiB pool.6.39s build. native-projection-before-046.log. Admission arithmetic only; no actual two-million-commit fixture or measured RSS.
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
cargo test -p jj-store-jj --lib --locked native_projection_admits_two_million_identities_with_index_headroom -- --nocapture (after)
```

**Notes**

1pass/0failed,12.02s build. native-projection-after-046.log. Fixed conservative reservation; exact regression retained.
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
cargo test -p jj-store-jj --lib --locked query::projection:: -- --nocapture (initial)
```

**Notes**

3pass/0.03s tests,9.79s build. native-projection-unit-046.log.6144identities use909312array-capacity bytes and66differential arbitrary-anchor/direction/limit windows match a tree-based oracle, with2048exact divergent group checks. Sort matches standard ordering and rejects work/cancellation/deadline exhaustion.
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
cargo test -p jj-store-cli --test native_lifecycle --locked native_prepared_references_select_visible_windows_before_target_reads -- --nocapture (initial)
```

**Notes**

1pass/63.70s test,75s build. native-projection-references-046.log. Complete selected-target equivalence, hidden predecessor/divergence and exact requests; ready virtual-root page has2object reads,38021charged bytes,143work. Metadata admission828 reflects fresh operation/view validation with256bookmarks, not source commit hydration.
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
cargo fmt -p jj-store-jj; pnpm exec prettier --write spec/src/jj-integration.md spec/src/architecture.md docs/implementation-plan.md
```

**Notes**

Changed Rust/docs formatted; native-projection-doc-format-046.log. Focused final formatter checks recorded below.
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
pnpm check:progress (first); pnpm spec:first
```

**Notes**

Spec-first passed7protected files/3chapters. Progress rejected the initial ledger run combining an intended failure with a passing result. Split before-fail and after-pass records; no product regression or assertion change. native-projection-progress-first/spec-first-046.log. Final governance commands recorded below.
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
cargo test -p jj-store-jj --lib --locked query::projection:: -- --nocapture
```

**Notes**

3pass/0failed/0.04s tests. 5.59s command. native-projection-final-00-046.log. Focused direct consumers; no full suite.
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
cargo test -p jj-store-cli --test native_lifecycle --locked native_prepared_references_select_visible_windows_before_target_reads -- --nocapture
```

**Notes**

1pass/0failed/49.56s tests. 88.92s command. native-projection-final-01-046.log. Focused direct consumers; no full suite.
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
cargo test -p jj-store-cli --test native_lifecycle --locked graph_query_ -- --nocapture
```

**Notes**

5pass/0failed/1.33s tests. 16.92s command. native-projection-final-02-046.log. Focused direct consumers; no full suite.
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
cargo test -p jj-store-cli --test native_lifecycle --locked graph_cache_ -- --nocapture
```

**Notes**

6pass/0failed/1.64s tests. 13.06s command. native-projection-final-03-046.log. Focused direct consumers; no full suite.
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
cargo test -p jj-store-cli --test native_lifecycle --locked warm_ -- --nocapture
```

**Notes**

4pass/0failed/1.37s tests. 13.41s command. native-projection-final-04-046.log. Focused direct consumers; no full suite.
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
cargo test -p jj-store-cli --test native_lifecycle --locked native_preparation_fits_small_cache_and_preserves_historical_scopes -- --nocapture
```

**Notes**

1pass/0failed/1.06s tests. 13.82s command. native-projection-final-05-046.log. Focused direct consumers; no full suite.
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
cargo test -p jj-store-cli --test native_lifecycle --locked historical_predicates_reuse_separately_prepared_residents_without_slot_growth -- --nocapture
```

**Notes**

1pass/0failed/1.23s tests. 14.15s command. native-projection-final-06-046.log. Focused direct consumers; no full suite.
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
cargo test -p jj-store-cli --test native_lifecycle --locked resident_indexes_preserve_multiple_historical_operation_scopes -- --nocapture
```

**Notes**

1pass/0failed/1.09s tests. 13.97s command. native-projection-final-07-046.log. Focused direct consumers; no full suite.
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
cargo test -p jj-store-cli --test native_lifecycle --locked query_workers_ -- --nocapture
```

**Notes**

3pass/0failed/0.36s tests. 22.1s command. native-projection-final-08-046.log. Focused direct consumers; no full suite.
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
cargo test -p jj-store-cli --test native_lifecycle --locked http_browsing_windows_are_authorized_pinned_cursor_bound_and_contract_captured -- --nocapture
```

**Notes**

1pass/0failed/0.48s tests. 19.26s command. native-projection-final-09-046.log. Focused direct consumers; no full suite.
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
cargo test -p jj-store-cli --test native_lifecycle --locked disk_preparation_ -- --nocapture
```

**Notes**

2pass/0failed/0.63s tests. 16.39s command. native-projection-final-10-046.log. Focused direct consumers; no full suite.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-cli --test native_lifecycle --locked native_graph_pages_and_preparation_do_not_enumerate_inventory -- --nocapture
```

**Notes**

1pass/0failed/0.47s tests. 15.75s command. native-projection-final-11-046.log. Focused direct consumers; no full suite.
<!-- /run -->
<!-- /evidence-field -->

