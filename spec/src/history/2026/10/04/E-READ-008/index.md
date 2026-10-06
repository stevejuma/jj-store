---
schema: 1
id: E-READ-008
---

# E-READ-008

<!-- evidence-field:id markdown -->
## Id

E-READ-008
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M13.4
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-04
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change nsrrnsrs
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Bounded local historical point membership without source indexes or tree preflight
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; jj-lib 0.45.1; pnpm 10.32.1.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No public DTO or persisted format changes. Historical commit_summary, paths and file_at retain exact operation pins and scoped identity proof.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/local_browser/bounded_reads.rs
- crates/jj-store-jj/src/local_browser/history.rs
- crates/jj-store-jj/src/local_browser/mod.rs
- crates/jj-store-jj/src/local_browser/scoped.rs
- spec/src/cli.md
- spec/src/jj-integration.md
- spec/src/local-jj.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-LOCAL-004
- JST-JJ-003
- JST-JJ-005
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/local_browser/bounded_reads.rs#historical_file_reads_stop_after_proving_the_selected_commit
- crates/jj-store-cli/tests/local_browser/bounded_reads.rs#historical_point_membership_retains_abandoned_commits_and_excludes_later_operations
- crates/jj-store-cli/tests/local_browser/bounded_reads.rs#registered_large_repository_metadata_smoke
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
cargo test -p jj-store-cli --test local_browser --locked historical_file_reads_stop_after_proving_the_selected_commit -- --nocapture
```

**Notes**

Initial fixture accidentally selected an already-proven reference; failure was only an untrimmed description newline. Strengthened to a three-generation ancestor with four-commit visit budget and corrupt source indexes.
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
cargo test -p jj-store-cli --test local_browser --locked historical_file_reads_stop_after_proving_the_selected_commit -- --nocapture (parent Context::open restored temporarily by try/finally)
```

**Notes**

Meaningful before state: Limit while opening historical directory. Parent module supplied from jj file show; current module restored byte-for-byte automatically after baseline.
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
cargo test -p jj-store-cli --test local_browser --locked -- --test-threads=2
```

**Notes**

12 passed,1 opt-in ignored. Includes directory/file/header small-budget proof, unchanged corrupt index bytes, retained abandoned commit and later-operation exclusion, HTTP, working files, conflicts and history aliases.
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
cargo check -p jj-store-jj -p jj-store-cli --all-targets --locked
```

**Notes**

Affected package targets and direct consumers checked.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_LARGE_REPO=/Users/stevejuma/code/linux/.jj/repo target/debug/deps/local_browser-893d8c4057fcb950 --exact bounded_reads::registered_large_repository_metadata_smoke --ignored --nocapture
```

**Notes**

Historical directory returned successfully, but an unjustified &lt;=10 metadata-record test assumption failed: 944 records due to many selected-view roots. Replaced that assumption with an explicit 1024-commit and100000-work request budget. Prepared lookup remains necessary for consistently cheap arbitrary historical IDs.
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
cargo test -p jj-store-cli --test local_browser --locked bounded_reads:: -- --test-threads=2
```

**Notes**

Final focused5 passed,1 opt-in ignored after tightening Linux request budgets; production unchanged since full local binary pass.
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
JJ_STORE_LARGE_REPO=/Users/stevejuma/code/linux/.jj/repo target/debug/deps/local_browser-893d8c4057fcb950 --exact bounded_reads::registered_large_repository_metadata_smoke --ignored --nocapture
```

**Notes**

Final Linux: historical directory10 rows in1.797s,944 metadata records,7,227,557 charged bytes,27,292work, within1024-commit/100000-work budget. Current directory10 rows:1.912s,3 records,72work. Op Log0.383s; scoped .clang-format24,291bytes; total9.97s. Concurrent-run observations, not isolated benchmarks.
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
pnpm spec:first; pnpm exec prettier --check spec/src/cli.md spec/src/local-jj.md spec/src/jj-integration.md; cargo fmt --all
```

**Notes**

Final check:progress, spec:first, spec:validate, cargo fmt --check and focused Prettier checks pass:136 tasks,4 protected files/4 chapters,167 requirements.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Deep historical lookup can still exhaust bounded edge proof. Commit graph
  queries and ID-prefix operation selectors retain old preparation/admission.
  Prepared historical indexes, immutable cache segments/quotas, actual-work
  graph admission, operation/copy/search/RPC and collaboration projections
  remain pending.
- Native2m, deep-page, near-capacity collaboration,
  cache-loss/corruption/revocation/races, cold and real-RustFS gates and
  E-READ-005 registry/full-capture gaps remain pending. Running dev app
  unchanged. Native-only and UI/browser suites out of scope for this local Rust
  point-read change.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Validate ledger and commit historical point reads, then remove complete-inventory graph admission and implement bounded preparation.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


