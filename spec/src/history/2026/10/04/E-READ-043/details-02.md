---
schema: 1
id: E-READ-043
---

# E-READ-043 details 2

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
cargo test -p jj-store-cli --test local_browser --locked prepared_operation_comparison_skips_shared_history_and_git_objects -- --nocapture (before)
```

**Notes**

1failure/9.78s: prepared OperationDiff returns Invalid before dispatch. operation-comparison-before-043.log.
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

4.81s; two temporary unused private-method warnings before change-page integration; removed by the completed implementation. operation-comparison-check-initial-043.log.
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
cargo test -p jj-store-cli --test local_browser --locked prepared_operation_change_pages_hydrate_only_selected_groups -- --nocapture (before selected hydration)
```

**Notes**

1failure/11.17s: two-group page reads42 metadata nodes, violating the9-node selected-page bound. operation-change-pages-before-043.log.
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
cargo test -p jj-store-cli --test local_browser --locked prepared_operation_comparison_skips_shared_history_and_git_objects -- --nocapture (after)
```

**Notes**

1pass/9.01s: historical comparison with a4-visit budget, Git objects/source index removed and replaced historical view rejected/restored. operation-comparison-after-043.log.
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
cargo test -p jj-store-cli --test local_browser --locked prepared_operation_change_pages_hydrate_only_selected_groups -- --nocapture (after)
```

**Notes**

1pass/10.12s: exact ordinary forward/backward groups/anchors; full window joined without hydrating unselected bodies. operation-change-pages-after-043.log.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 8
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-jj --lib --locked local_browser::preparation::comparison:: -- --nocapture (initial)
```

**Notes**

8 compiler diagnostics: test glob selected preparation Limits instead of ordinary read Limits. Fixed the test-only explicit import. operation-comparison-units-043.log.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-jj --lib --locked local_browser::preparation::comparison:: -- --nocapture (first compiled)
```

**Notes**

1pass/2fail: compact decoder confused parent-overflow offset zero with an empty parent list. Corrected production decoding to inspect both words; preserved all assertions. operation-comparison-units-final-043.log.
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
cargo test -p jj-store-jj --lib --locked local_browser::preparation::comparison:: -- --nocapture (fixed)
```

**Notes**

3pass/0.07s. Exhaustive4096 head-set pairs match independent full ancestry on branches, merges, redundant heads and3-parent overflow. Short tails over100 shared commits need2 admitted identities; malformed framing/lookup/offset/parent/generation, unknown IDs, work/visit/cancel limits reject explicitly. operation-comparison-units-fixed-043.log.
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
cargo test -p jj-store-cli --test local_browser --locked prepared_ -- --nocapture; cargo test -p jj-store-cli --test local_browser --locked local_preparation_ -- --nocapture
```

**Notes**

9prepared tests/35.41s plus2preparation tests/7.90s. Direct prefix/reference/log/detail/evolution/point/diff/HTTP consumers including10,000 bookmarks, cancellation/binding and exact upstream local index bytes pass. operation-comparison-prepared/preparation-043.log.
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
cargo check -p jj-store-jj -p jj-store-service -p jj-store-http -p jj-store-cli --locked (first/final)
```

**Notes**

20.21s/12.55s; affected JJ and direct service/HTTP/CLI packages pass without new warnings after final production edits. operation-comparison-check/check-final-043.log.
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
cargo test -p jj-store-cli --test local_browser --locked prepared_operation_comparison_skips_shared_history_and_git_objects -- --nocapture (strengthened admission)
```

**Notes**

1pass/20.08s. Warm current denial/generation change rejected, invalid parent explicit, virtual root preserved and later-operation lookup rejected. Resident historical detail/difference still succeeds with source Git/index objects and prepared segment files removed. operation-comparison-admission-043.log.
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
cargo test -p jj-store-jj --lib --locked query::prepared:: -- --nocapture
```

**Notes**

6pass/0.26s: v2 fingerprint framing plus operation/view lookup, checksum/selection/order, disk-before-write admission and bounded scratch tables. operation-comparison-builder-043.log.
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
cargo test -p jj-store-cli --test native_lifecycle --locked disk_preparation_matches_pinned_upstream_index_bytes -- --nocapture
```

**Notes**

1pass/1.35s: native default-writer output remains byte-identical to pinned JJ. operation-comparison-native-043.log.
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
cargo test -p jj-store-cli --test local_browser --locked local_code_reads_cover_graphs_diffs_roots_conflicts_and_multiple_heads -- --nocapture
```

**Notes**

1pass/6.92s: ordinary local operation/change, root, graph, conflict and multiple-head behavior preserved. operation-comparison-ordinary-043.log.
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
cargo test -p jj-store-cli --test local_browser --locked prepared_operation_change_pages_hydrate_only_selected_groups -- --nocapture (final associations; measured; no Git)
```

**Notes**

Three exact passes/9.34s,10.49s,9.69s after association-memory admission and removal of unnecessary selected hydration. Final first two-group page uses4 metadata nodes,5713 charged bytes,2683 work vs42 metadata nodes before. Explicit predecessor groups replay with all Git objects removed;4-visit oversized rewrite still returns Limit. operation-change-pages-final/measured/no-git-043.log.
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
cargo test -p jj-store-cli --test local_browser --locked local_point_http_uses_ready_membership_and_only_falls_back_when_cold -- --nocapture (final)
```

**Notes**

1pass/8.96s after final production edits: diff/change cold409 exact-operation recovery and ready responses join existing point/prefix/reference/signed-page/source-seal proofs. Disposable filesystem server stopped. operation-comparison-http-final-043.log.
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
cargo fmt -p jj-store-jj -p jj-store-cli; cargo fmt -p jj-store-jj -p jj-store-cli -- --check; pnpm exec prettier --write spec/src/api.md spec/src/architecture.md spec/src/cli.md spec/src/compatibility.md spec/src/jj-integration.md spec/src/local-jj.md docs/implementation-plan.md; pnpm check:boundaries; pnpm check:ui-ownership
```

**Notes**

Changed Rust/docs formatting,17package boundaries and1ownership validator pass. operation-comparison-rust-format/format[-final]/boundaries/ownership-043.log. Final fixture edits formatted; no root aggregates or unrelated lanes run.
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
pnpm exec prettier --write spec/src/milestones.md; pnpm check:progress; pnpm spec:first; pnpm spec:validate
```

**Notes**

Final138task bookkeeping,16protected files/7canonical chapters and5validators/23chapters/172requirements pass. operation-comparison-ledger-format/progress/spec-first/spec-043.log. No complete qualification or milestone promotion.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

