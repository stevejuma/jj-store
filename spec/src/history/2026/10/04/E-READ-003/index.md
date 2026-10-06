---
schema: 1
id: E-READ-003
---

# E-READ-003

<!-- evidence-field:id markdown -->
## Id

E-READ-003
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4.1
- M13.4
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-04
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change wstuusku
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Scoped Git directory capabilities, direct-child paths and seekable bookmark windows
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; jj-lib 0.45.1; Node 26.9.0; pnpm 10.32.1.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Existing native/local wire formats and persisted formats retained. Direct path pages now include directories. Git path reads of directly referenced commits use a read-only edge-following capability. Unfiltered bookmark windows seek names and deduplicate target hydration.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/local_browser/bounded_reads.rs
- crates/jj-store-cli/tests/native_lifecycle/browsing.rs
- crates/jj-store-jj/src/local_browser/mod.rs
- crates/jj-store-jj/src/local_browser/native.rs
- crates/jj-store-jj/src/local_browser/paths.rs
- crates/jj-store-jj/src/local_browser/references.rs
- crates/jj-store-jj/src/local_browser/scoped.rs
- crates/jj-store-jj/src/query/content/paths.rs
- crates/jj-store-jj/src/query/content/references.rs
- spec/src/api.md
- spec/src/cli.md
- spec/src/jj-integration.md
- spec/src/local-jj.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-LOCAL-004
- JST-JJ-003
- JST-API-001
- JST-API-002
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/local_browser/bounded_reads.rs#local_file_rows_read_only_the_selected_commit_and_traversed_trees
- crates/jj-store-cli/tests/local_browser/bounded_reads.rs#local_metadata_and_bookmark_pages_do_not_load_history_or_index
- crates/jj-store-cli/tests/native_lifecycle/browsing.rs#direct_directory_rows_include_directories_and_bookmark_pages_hydrate_only_the_window
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
cargo test -p jj-store-cli --test local_browser --locked local_file_rows_read_only_the_selected_commit_and_traversed_trees -- --exact bounded_reads::local_file_rows_read_only_the_selected_commit_and_traversed_trees
```

**Notes**

Before: one selected path fails Limit under a two-commit history budget because Context scans operation ancestry before dispatch.
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
cargo check -p jj-store-jj --locked
```

**Notes**

Initial scoped backend capability compiles.
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
cargo test -p jj-store-cli --test local_browser --locked local_file_rows_read_only_the_selected_commit_and_traversed_trees -- --exact bounded_reads::local_file_rows_read_only_the_selected_commit_and_traversed_trees
```

**Notes**

Scoped selected row passes; directory continuation coverage then added.
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
cargo test -p jj-store-cli --test native_lifecycle --locked direct_directory_rows_include_directories_and_bookmark_pages_hydrate_only_the_window -- --nocapture
```

**Notes**

Initial new test required the fixture multithread Tokio runtime; corrected the test attribute.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-cli --test native_lifecycle --locked direct_directory_rows_include_directories_and_bookmark_pages_hydrate_only_the_window -- --nocapture
```

**Notes**

Initial native resource assertion fails Limit. Follow-up found metadata_nodes counts JSON values, not objects, so this was an invalid admission threshold rather than a native before proof. Replaced it with the intended exact object_reads=3 assertion; production limits unchanged. Directory assertions already pass. The invalid threshold was run twice before correcting the metric.
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
cargo test -p jj-store-cli --test native_lifecycle --locked browsing_acceptance -- --test-threads=2
```

**Notes**

4 passed, including exact three-object bookmark window, directory rows, bidirectional cursor bindings, exact diffs and HTTP contract captures. Escalation covers direct native content/API consumers.
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
cargo test -p jj-store-cli --test local_browser --locked -- --test-threads=2
```

**Notes**

9 passed, 1 opt-in real-repository smoke ignored. Exact path regression now covers directory continuation; 10000-bookmark deep seek and 50 shared targets require only three metadata records and page-proportional work.
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
target/debug/deps/native_lifecycle-e6652c34f7ed2d93 content_acceptance --test-threads=2
```

**Notes**

95 content/direct-consumer tests passed in 44.58s, covering HTTP, projections/search, index tasks, predicates, diffs and shared worker lifecycle.
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
JJ_STORE_LARGE_REPO=/Users/stevejuma/code/linux/.jj/repo target/debug/deps/local_browser-893d8c4057fcb950 --ignored --exact bounded_reads::registered_large_repository_metadata_smoke --nocapture
```

**Notes**

Read-only Linux: metadata+bookmark 1.3998s; first 10 direct directory rows 1.0341s, 3 metadata records, 2938179 charged bytes, 102 work units. Historical graph and raw file requests remain unqualified.
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
cargo check -p jj-store-jj -p jj-store-cli --all-targets --locked
```

**Notes**

Affected all-target Rust check passes; corrected native test subsequently compiles in its passing focused suite.
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
pnpm spec:first; pnpm check:progress; pnpm spec:validate; cargo fmt --all -- --check; pnpm exec prettier --write spec/src/cli.md; pnpm exec prettier --check spec/src/local-jj.md spec/src/jj-integration.md spec/src/api.md spec/src/cli.md
```

**Notes**

Canonical mapping, 136-task ledger, 167 requirements and focused formatting pass. No schema shape changed.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Historical Git commits still use complete membership preflight; bare file IDs
  and graph requests remain later preparation/scoped-file work. Files-first UI
  and remaining endpoint/collaboration/large-scale plan are not yet implemented.
- Root aggregate, UI, visual and real-RustFS lanes are out of scope for this
  Rust capability/read algorithm slice; cold full-flow and scale qualification
  remain in the plan.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Implement files-first UI loading, scoped file proofs, independent attribution and exact reference/workspace startup selection.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


