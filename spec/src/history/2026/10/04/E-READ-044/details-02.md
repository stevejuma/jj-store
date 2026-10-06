---
schema: 1
id: E-READ-044
---

# E-READ-044 details 2

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
cargo test -p jj-store-cli --test native_lifecycle --locked native_prepared_references_select_visible_windows_before_target_reads -- --nocapture (before)
```

**Notes**

0pass/1fail,0.42s: composed native All/Changes still walks complete ancestors despite an already prepared index, failing a one-visit limit. native-reference-before-044.log.
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
cargo check -p jj-store-jj -p jj-store-service --locked (first)
```

**Notes**

21.15s; one unused Repo import warning removed before final check. native-reference-check-044.log.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-cli --test native_lifecycle --locked native_prepared_references_select_visible_windows_before_target_reads -- --nocapture (expanded fixture first)
```

**Notes**

One test compiler diagnostic: formatted bookmark key needed Into&lt;RefNameBuf&gt;; fixed test-only key conversion. native-reference-exact-044.log.
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
cargo test -p jj-store-cli --test native_lifecycle --locked native_prepared_references_select_visible_windows_before_target_reads -- --nocapture (expanded fixture second)
```

**Notes**

0pass/1fail,30.25s: test reused one short CommittedRead across separate simulated requests. Fixed each page to open a fresh exact-operation read while reusing the same generation; retained limits and all assertions. native-reference-strengthened-044.log.
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
cargo test -p jj-store-cli --test native_lifecycle --locked native_prepared_references_select_visible_windows_before_target_reads -- --nocapture (fixed)
```

**Notes**

1pass/92.58s.32 extra ancestors,256 bookmarks, diverging visible targets, explicit conflict terms, workspace labels and hidden predecessor.16 forward/backward/deep windows and all exact changes match ordinary DTOs/cursors under a4-visit prepared limit. One-row ready page:4object reads,76042charged bytes,147work; metadata1656includes repeated selected operation/view framing, not ancestor body hydration. native-reference-fixed-044.log.
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
cargo check -p jj-store-jj -p jj-store-service -p jj-store-http -p jj-store-cli --locked
```

**Notes**

59.88s including shared build-directory wait; no new warnings. native-reference-check-final-044.log.
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
cargo fmt -p jj-store-jj -p jj-store-service -p jj-store-cli; cargo fmt -p jj-store-cli; pnpm exec prettier --write spec/src/api.md spec/src/architecture.md spec/src/cli.md spec/src/compatibility.md spec/src/jj-integration.md docs/implementation-plan.md; pnpm check:boundaries; pnpm check:ui-ownership
```

**Notes**

Changed Rust/docs formatting,17package boundaries and1ownership validator pass. native-reference-format[-final]/test-format[-final]/doc-format/boundaries/ownership-044.log.
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
cargo fmt -p jj-store-jj -p jj-store-service -p jj-store-cli -- --check
```

**Notes**

Affected Rust formatting passes. native-reference-rust-format-check-044.log.
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
pnpm check:progress (initial)
```

**Notes**

E-READ-044 evidence used commands instead of required runs field; corrected ledger schema and added requirement/test links. native-reference-progress-044.log.
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
pnpm spec:first (initial)
```

**Notes**

Service browsing change requires mapped mutations.md chapter; added explicit shared-worker/read-only recovery requirement alongside existing read capability policy. native-reference-spec-first-044.log.
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
pnpm spec:validate
```

**Notes**

5validators/23chapters/172requirements pass. native-reference-spec-044.log.
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
cargo test -p jj-store-cli --test native_lifecycle --locked native_browser_windows_preserve_references_paths_groups_and_exact_diffs -- --nocapture
```

**Notes**

1pass/0.66s tests (36.81s with build). native-reference-consumer-00-044.log. Direct browser/signed recovery, shared worker/cancellation, prepared graph/cache/disk/inventory consumers; no full suite.
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
cargo test -p jj-store-cli --test native_lifecycle --locked signed_browser_cursors_bind_queries_support_backward_windows_and_reset_on_restart -- --nocapture
```

**Notes**

1pass/0.40s tests (28.72s with build). native-reference-consumer-01-044.log. Direct browser/signed recovery, shared worker/cancellation, prepared graph/cache/disk/inventory consumers; no full suite.
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
cargo test -p jj-store-cli --test native_lifecycle --locked direct_directory_rows_include_directories_and_bookmark_pages_hydrate_only_the_window -- --nocapture
```

**Notes**

1pass/0.20s tests (14.85s with build). native-reference-consumer-02-044.log. Direct browser/signed recovery, shared worker/cancellation, prepared graph/cache/disk/inventory consumers; no full suite.
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
cargo test -p jj-store-cli --test native_lifecycle --locked workspace_windows_seek_filter_and_resolve_exact_names_without_history -- --nocapture
```

**Notes**

1pass/0.26s tests (15.4s with build). native-reference-consumer-03-044.log. Direct browser/signed recovery, shared worker/cancellation, prepared graph/cache/disk/inventory consumers; no full suite.
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
cargo test -p jj-store-cli --test native_lifecycle --locked operation_pages_do_not_admit_or_build_a_commit_index -- --nocapture
```

**Notes**

1pass/0.19s tests (17.98s with build). native-reference-consumer-04-044.log. Direct browser/signed recovery, shared worker/cancellation, prepared graph/cache/disk/inventory consumers; no full suite.
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
cargo test -p jj-store-cli --test native_lifecycle --locked http_reads_index_task_and_signed_cursor_preserve_snapshot_across_publication -- --nocapture
```

**Notes**

1pass/0.93s tests (18.4s with build). native-reference-consumer-05-044.log. Direct browser/signed recovery, shared worker/cancellation, prepared graph/cache/disk/inventory consumers; no full suite.
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

3pass/0.29s tests (15.64s with build). native-reference-consumer-06-044.log. Direct browser/signed recovery, shared worker/cancellation, prepared graph/cache/disk/inventory consumers; no full suite.
<!-- /run -->
<!-- /evidence-field -->

