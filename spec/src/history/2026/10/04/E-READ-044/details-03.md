---
schema: 1
id: E-READ-044
---

# E-READ-044 details 3

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
cargo test -p jj-store-cli --test native_lifecycle --locked graph_query_ -- --nocapture
```

**Notes**

5pass/1.64s tests (13.25s with build). native-reference-consumer-07-044.log. Direct browser/signed recovery, shared worker/cancellation, prepared graph/cache/disk/inventory consumers; no full suite.
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
cargo test -p jj-store-cli --test native_lifecycle --locked graph_cache_ -- --nocapture
```

**Notes**

6pass/2.79s tests (17.97s with build). native-reference-consumer-08-044.log. Direct browser/signed recovery, shared worker/cancellation, prepared graph/cache/disk/inventory consumers; no full suite.
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
cargo test -p jj-store-cli --test native_lifecycle --locked warm_ -- --nocapture
```

**Notes**

4pass/1.31s tests (14.73s with build). native-reference-consumer-09-044.log. Direct browser/signed recovery, shared worker/cancellation, prepared graph/cache/disk/inventory consumers; no full suite.
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
cargo test -p jj-store-cli --test native_lifecycle --locked disk_preparation_ -- --nocapture
```

**Notes**

2pass/0.60s tests (12.21s with build). native-reference-consumer-10-044.log. Direct browser/signed recovery, shared worker/cancellation, prepared graph/cache/disk/inventory consumers; no full suite.
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
cargo test -p jj-store-cli --test native_lifecycle --locked native_graph_pages_and_preparation_do_not_enumerate_inventory -- --nocapture
```

**Notes**

1pass/0.46s tests (14.75s with build). native-reference-consumer-11-044.log. Direct browser/signed recovery, shared worker/cancellation, prepared graph/cache/disk/inventory consumers; no full suite.
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
cargo test -p jj-store-cli --test native_lifecycle --locked existing_disk_projection_recovery_releases_namespace_and_preserves_generations -- --nocapture
```

**Notes**

1pass/1.72s tests (19.84s with build). native-reference-consumer-12-044.log. Direct browser/signed recovery, shared worker/cancellation, prepared graph/cache/disk/inventory consumers; no full suite.
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
cargo test -p jj-store-cli --test native_lifecycle --locked canceled_disk_projection_recovery_cleans_up_while_namespace_is_busy -- --nocapture
```

**Notes**

1pass/0.44s tests (13.27s with build). native-reference-consumer-13-044.log. Direct browser/signed recovery, shared worker/cancellation, prepared graph/cache/disk/inventory consumers; no full suite.
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
cargo test -p jj-store-cli --test native_lifecycle --locked query_cancel_ -- --nocapture
```

**Notes**

3pass/0.31s tests (14.76s with build). native-reference-consumer-14-044.log. Direct browser/signed recovery, shared worker/cancellation, prepared graph/cache/disk/inventory consumers; no full suite.
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
cargo test -p jj-store-cli --test native_lifecycle --locked native_preparation_fits_small_cache_and_preserves_historical_scopes -- --nocapture
```

**Notes**

1pass/1.29s tests (29.08s with build). native-reference-consumer-15-044.log. Direct browser/signed recovery, shared worker/cancellation, prepared graph/cache/disk/inventory consumers; no full suite.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
cargo check -p jj-store-jj --locked (loaded-view optimization first)
```

**Notes**

3compiler diagnostics: upstream RefNameBuf lacks Display. Use as_str for lexical bookmark keys; no contract/semantic change. native-reference-view-check-044.log.
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
cargo check -p jj-store-jj -p jj-store-service -p jj-store-http -p jj-store-cli --locked (loaded-view fixed)
```

**Notes**

13.52s. Borrow freshly verified upstream view; convert only selected bookmark terms and remove redundant operation/view reads and whole-view DTO copies. native-reference-view-check-fixed-044.log.
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
cargo fmt -p jj-store-jj; cargo fmt -p jj-store-cli; pnpm exec prettier --write spec/src/architecture.md spec/src/jj-integration.md spec/src/mutations.md spec/src/milestones.md
```

**Notes**

Loaded-view production/fixture and owning-chapter/ledger formatting pass. native-reference-view-format/view-test-format/view-doc-format/ledger-format-fixed/ledger-consumer-format-044.log.
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
cargo test -p jj-store-cli --test native_lifecycle --locked native_prepared_references_select_visible_windows_before_target_reads -- --nocapture (borrowed view)
```

**Notes**

1pass/67.28s. Same complete DTO/cursor equivalence after borrowing fresh native view. One-row virtual root now needs2object reads,38021charged bytes,143work and828metadata records (half previous operation/view admission). native-reference-view-exact-044.log.
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
cargo test -p jj-store-cli --test native_lifecycle --locked native_prepared_references_select_visible_windows_before_target_reads -- --nocapture (final)
```

**Notes**

1pass/59.99s (84.1s with build). native-reference-final-00-044.log. Final strict reference counts, actual cold HTTP preparation/retry and signed backward pages across publication, plus shared file-lifetime and cancellation admission.
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
cargo test -p jj-store-cli --test native_lifecycle --locked http_reads_index_task_and_signed_cursor_preserve_snapshot_across_publication -- --nocapture (final)
```

**Notes**

1pass/0.62s (16.98s with build). native-reference-final-01-044.log. Final strict reference counts, actual cold HTTP preparation/retry and signed backward pages across publication, plus shared file-lifetime and cancellation admission.
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
cargo test -p jj-store-cli --test native_lifecycle --locked query_workers_ -- --nocapture (final)
```

**Notes**

3pass/0.42s (18.89s with build). native-reference-final-02-044.log. Final strict reference counts, actual cold HTTP preparation/retry and signed backward pages across publication, plus shared file-lifetime and cancellation admission.
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
cargo fmt -p jj-store-jj -p jj-store-service -p jj-store-cli -- --check (final)
```

**Notes**

Final affected Rust formatting passes. native-reference-rust-format-final-044.log.
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
pnpm exec prettier --write spec/src/milestones.md; pnpm check:progress; pnpm spec:first; pnpm spec:validate (final)
```

**Notes**

Final ledger formatting,138task bookkeeping,11protected files/7canonical chapters and5validators/23chapters/172requirements pass. native-reference-ledger-final-format/progress-fixed/spec-first-fixed/spec-final-044.log. No milestone promotion or complete qualification.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-JJ-003
- JST-API-001
- JST-MUT-001
<!-- /evidence-field -->

