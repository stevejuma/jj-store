---
schema: 1
id: E-READ-022
---

# E-READ-022

<!-- evidence-field:id markdown -->
## Id

E-READ-022
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4.1
- M13.5
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-04
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change zpmrzput
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Selected local diff/conflict reads without complete repository preflight
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust1.98.1; jj-lib0.45.1; macOS ARM64; Linux existing JJ metadata checkout.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Public DTOs and persistence unchanged. Internal scoped query context admits at most the requested from/to commits before selected tree reads.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/local_browser/scoped_differences.rs
- crates/jj-store-cli/tests/local_browser.rs
- crates/jj-store-jj/src/local_browser/mod.rs
- crates/jj-store-jj/src/local_browser/preparation/graph.rs
- crates/jj-store-jj/src/local_browser/preparation.rs
- spec/src/cli.md
- spec/src/jj-integration.md
- spec/src/local-jj.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-JJ-003
- JST-JJ-005
- JST-LOCAL-004
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/local_browser/scoped_differences.rs#selected_diffs_and_conflicts_do_not_preflight_unrelated_history
- crates/jj-store-cli/tests/local_browser.rs#local_code_reads_cover_graphs_diffs_roots_conflicts_and_multiple_heads
- crates/jj-store-cli/tests/local_browser/preparation.rs#registered_large_repository_preparation_smoke
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
cargo test -p jj-store-cli --test local_browser --locked selected_diffs_and_conflicts_do_not_preflight_unrelated_history -- --nocapture
```

**Notes**

Before: Limit with max_history_visits4, although only current/parent selected.txt diff was requested. scoped-diff-before.log.
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
cargo test -p jj-store-cli --test local_browser --locked selected_diffs_and_conflicts_do_not_preflight_unrelated_history -- --nocapture
```

**Notes**

1pass/9.09s after: exact before/after hunks, one-row changed paths, selected conflict descriptor and unchanged extra heads; metadata nodes below16. scoped-diff-after.log.
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
cargo test -p jj-store-cli --test local_browser --locked
```

**Notes**

22pass/2ignored/28.24s. Shared local context change justifies complete local-reader consumers; conflict terms/roots/multiple heads/operations, history queries, pinned reads and HTTP/task behavior retained. scoped-diff-suite.log.
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
cargo check -p jj-store-jj -p jj-store-service -p jj-store-http -p jj-store-cli --all-targets --locked
```

**Notes**

Affected package/direct consumers pass; scoped-diff-check.log.
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
cargo fmt --all; pnpm exec prettier --write spec/src/local-jj.md spec/src/jj-integration.md spec/src/cli.md; pnpm spec:first; pnpm spec:validate; pnpm check:boundaries; cargo fmt --all -- --check
```

**Notes**

5protected files/3chapters,167requirements,17package boundaries and formatting pass. scoped-diff-governance.log.
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
JJ_STORE_LARGE_REPO=/Users/stevejuma/code/linux/.jj/repo /usr/bin/time -l cargo test -p jj-store-cli --test local_browser --locked registered_large_repository_preparation_smoke -- --ignored --nocapture
```

**Notes**

Previously running E-READ-018 probe completed. Original pre-E019/E020 binary:1pass/2537.27s, preparation2531.07s;1,484,686metadata records,2,852,322,513read bytes,78,253,573work. Three distinct50-row pages1.703/1.742/2.720s. Source op/extra heads unchanged and private output cleaned. Time report maximumRSS2,043,789,312bytes includes compilation/process descendants and is not isolated serving RSS. local-linux-preparation.log.
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
JJ_STORE_LARGE_REPO=/Users/stevejuma/code/linux/.jj/repo cargo test -p jj-store-cli --test local_browser --locked registered_large_repository_metadata_smoke -- --ignored --nocapture
```

**Notes**

Latest ordinary reader1pass/5.37s: operation page230.6ms, metadata/bookmark1.164s, directory873.4ms, historical directory1.279s and selected .clang-format bytes. Source-preservation assertions pass. scoped-diff-linux-point.log.
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
pnpm check:progress
```

**Notes**

136task evidence bookkeeping passes. scoped-diff-progress.log.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Historical point proof remains a bounded edge walk. Deep selected targets need
  prepared membership; detailed commits/evolution/prefix/operation comparisons
  still need their prepared reader conversion. Diff page seeks and copy lineage
  remain separate work. No new copy profile is enabled.
- Linux full correctness probe passed with original construction binary,
  but42min debug preparation is slow. Latest shared-memory policy, isolated RSS,
  release performance, native2m/10krefs/deep pages/near-capacity and
  fault/real-RustFS acceptance pending.
- UI/wire-generation/root aggregates out of scope. Original services untouched;
  inherited registry/full-contract gates retained.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Commit scoped differences; reuse prepared operation-bound change visibility and add prepared detailed local endpoints, then continue remaining large-scale and collaboration/card work.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


