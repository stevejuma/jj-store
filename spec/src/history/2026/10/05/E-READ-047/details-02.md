---
schema: 1
id: E-READ-047
---

# E-READ-047 details 2

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
cargo test -p jj-store-cli --test native_lifecycle --locked native_filtered_references_seek_pinned_text_before_selected_target_reads -- --nocapture (before)
```

**Notes**

1fail/0pass/0.69s tests. Prepared nonempty query fell back to the ordinary reader and exceeded its four-visit budget. native-filter-before-047.log.
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
cargo test -p jj-store-cli --test native_lifecycle --locked native_filtered_references_seek_pinned_text_before_selected_target_reads -- --nocapture (initial compile)
```

**Notes**

Compiler rejected len() on the opaque local_bookmarks iterator (E0599); corrected to the admitted view map. Tool output; intermediate log was replaced by the after run.
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
cargo test -p jj-store-cli --test native_lifecycle --locked native_filtered_references_seek_pinned_text_before_selected_target_reads -- --nocapture (after)
```

**Notes**

1pass/0failed/109.09s tests,66s compile. native-filter-after-047.log. All108 windows passed; selected page4object reads/27524bytes/2385work. Diagnostic process sample showed repeated ordinary-source hydration, not a stuck prepared reader. Oracle reuse subsequently retains all108 comparisons while eliminating repeated identical ordinary reads.
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
cargo test -p jj-store-cli --test native_lifecycle --locked native_filter -- --nocapture
```

**Notes**

3pass/0failed/28.62s tests,45.21s build. native-filter-final-047.log.108 differential windows, explicit upgrade, failed/successful pinned cold recovery, restart and idle-pressure/live-reader ownership.
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
cargo test -p jj-store-jj --lib --locked query::cache -- --nocapture
```

**Notes**

10pass/0failed/0.33s tests. native-filter-cache-unit-047.log. Complete affected private cache-unit module; no full crate suite.
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
cargo test -p jj-store-jj --lib --locked query::memory::retained -- --nocapture
```

**Notes**

4pass/0failed. native-filter-memory-unit-047.log. Complete affected retained-memory unit module.
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
cargo test -p jj-store-cli --test native_lifecycle --locked cache_acceptance -- --nocapture
```

**Notes**

18pass/0failed/5.78s tests,28.59s build. native-filter-cache-consumers-047.log. Complete focused native cache consumers, including authorization, corruption, symlinks, cancellation, cold/private construction, historical scopes and eviction.
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
pnpm check:boundaries
```

**Notes**

17package dependency boundaries pass. native-filter-boundaries-047.log.
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
cargo fmt -p jj-store-jj -p jj-store-cli; pnpm exec prettier --write spec/src/jj-integration.md spec/src/cli.md spec/src/architecture.md docs/implementation-plan.md
```

**Notes**

Changed Rust/docs formatted; native-filter-doc-format-047.log. Focused final checks follow.
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
cargo test -p jj-store-jj --lib --locked query::projection:: -- --nocapture
```

**Notes**

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 45 filtered out; finished in 0.04s. 0.46s command. native-filter-consumer-00-047.log. Focused direct consumers; no full suite.
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
cargo test -p jj-store-jj --lib --locked query::prepared::filters:: -- --nocapture
```

**Notes**

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 45 filtered out; finished in 2.14s. 2.55s command. native-filter-consumer-01-047.log. Focused direct consumers; no full suite.
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
cargo test -p jj-store-cli --test native_lifecycle --locked native_prepared_references_select_visible_windows_before_target_reads -- --nocapture
```

**Notes**

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 221 filtered out; finished in 49.67s. 67.12s command. native-filter-consumer-02-047.log. Focused direct consumers; no full suite.
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
cargo test -p jj-store-cli --test native_lifecycle --locked graph_query_ -- --nocapture
```

**Notes**

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 217 filtered out; finished in 1.51s. 17.21s command. native-filter-consumer-03-047.log. Focused direct consumers; no full suite.
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
cargo test -p jj-store-cli --test native_lifecycle --locked disk_preparation_ -- --nocapture
```

**Notes**

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 220 filtered out; finished in 0.87s. 16.66s command. native-filter-consumer-04-047.log. Focused direct consumers; no full suite.
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
cargo test -p jj-store-cli --test native_lifecycle --locked index_preparation:: -- --nocapture
```

**Notes**

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 222 filtered out; finished in 0.00s. 14.83s command. native-filter-consumer-05-047.log. Focused direct consumers; no full suite. This selected zero tests and proves no preparation behavior; corrected index_acceptance:: lane follows.
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
cargo test -p jj-store-cli --test native_lifecycle --locked http_reads_index_task_and_signed_cursor_preserve_snapshot_across_publication -- --nocapture
```

**Notes**

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 221 filtered out; finished in 0.93s. 19.41s command. native-filter-consumer-06-047.log. Focused direct consumers; no full suite.
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
cargo test -p jj-store-cli --test native_lifecycle --locked http_browsing_windows_are_authorized_pinned_cursor_bound_and_contract_captured -- --nocapture
```

**Notes**

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 221 filtered out; finished in 0.52s. 16.11s command. native-filter-consumer-07-047.log. Focused direct consumers; no full suite.
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
cargo test -p jj-store-cli --test native_lifecycle --locked query_workers_ -- --nocapture
```

**Notes**

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 219 filtered out; finished in 0.31s. 26.86s command. native-filter-consumer-08-047.log. Focused direct consumers; no full suite.
<!-- /run -->
<!-- /evidence-field -->

