---
schema: 1
id: E-WORKING-COPY-STATUS-001
---

# E-WORKING-COPY-STATUS-001

<!-- evidence-field:id markdown -->
## Id

E-WORKING-COPY-STATUS-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M13.4
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-LOCAL-004
- JST-LOCAL-005
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-18
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-jj/src/local_browser/working_tree/status.rs
- crates/jj-store-jj/src/local_browser/working_tree.rs
- crates/jj-store-service/README.md
- crates/jj-store-service/src/working_tree/mod.rs
- crates/jj-store-service/src/working_tree/observer.rs
- crates/jj-store-service/src/working_tree/status.rs
- crates/jj-store-service/src/working_tree/walk.rs
- crates/jj-store-service/tests/working_tree_observer.rs
- crates/jj-store-service/tests/working_tree_projection.rs
- spec/src/current/contracts/jj-integration/21-filesystem-browser-reads.md
- spec/src/current/contracts/local-jj/18-bounded-stack-history-browsing.md
- spec/src/current/contracts/mutations/01-typed-mutations.md
- spec/src/current/progress/M13/index.md
- spec/src/history/2026/10/18/E-WORKING-COPY-STATUS-001/SUMMARY.nav.md
- spec/src/history/2026/10/18/E-WORKING-COPY-STATUS-001/index.md
- spec/src/history/2026/10/18/SUMMARY.nav.md
- spec/src/history/2026/10/18/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-service/tests/working_tree_observer.rs#status_compares_final_physical_tree_to_parent_without_double_counting_or_writes
- crates/jj-store-service/tests/working_tree_observer.rs#status_rejects_changes_while_counting_instead_of_returning_old_totals
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
pnpm dev:cargo -- test -p jj-store-service --test working_tree_observer --no-run
```

**Notes**

Initial compile required sibling-module visibility for the retained Observer reader; corrected with pub(super).
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
JJ_STORE_WORKING_TREE_TEST_JJ=/opt/homebrew/bin/jj pnpm dev:cargo -- test -p jj-store-service --test working_tree_observer status_ -- --ignored
```

**Notes**

One of two tests passed; fixture commit descriptions include the native trailing newline. Preserve it in the assertion.
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
JJ_STORE_WORKING_TREE_TEST_JJ=/opt/homebrew/bin/jj pnpm dev:cargo -- test -p jj-store-service --test working_tree_observer --test working_tree_projection --test working_tree_scan -- --include-ignored
```

**Notes**

16 passed: 6 observer (including both status regressions), 4 projection, 6 scan. Full source/Git/JJ byte inventory unchanged. Final tree comparison cancels a recorded edit, includes untracked text, omits ignored files, and returns absent line counts for binary data. Mid-count edits fail stale.
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
node .local/status-evidence.mjs
```

**Notes**

Passed repository root to navigation helper instead of spec/src; it added 38373 navigation/index files under hidden metadata/cache/config directories before refusing an oversized cached package index. Reversibly archived generated files, removed only newly created navigation by exact name/birth time, restored three existing cached index bodies (including empty npm index from official tarball), and verified JJ reads recover. No binary/source repository object contents changed. Corrected helper to spec/src and reran document checks.
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
pnpm check:progress
```

**Notes**

171 tasks/908 evidence records; bookkeeping only. Log status-progress-last.log.
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
pnpm spec:validate
```

**Notes**

Seven validators,4295 chapters,207 requirements. Log status-spec-last.log.
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
pnpm spec:first
```

**Notes**

Nine protected files,three canonical chapters. Log status-spec-first-last.log.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --check <six changed Markdown files>
```

**Notes**

Four changed chapters/README needed formatting; corrected only those files with prettier --write. Evidence and filesystem browser chapter already passed.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

[]
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Enclosing JJ change on b829c2ff; installed JJ 0.45.1, Rust 1.99.0, pnpm 10.34.5, Node 26.9.0.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Baseline/observation wire formats and source objects remain unchanged: private bytes are excluded from baseline serialization. New trusted status summary is not a command or transport grant.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Private bounded physical observation and native parent comparison; sole-parent conflict-free baselines. No source initialization, snapshots, writes, head reconciliation or shell command execution. UI and RustFS tests out of scope for this Rust-only library surface.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Continue desktop plugin status review; broader provider/platform and source-write gates remain separate.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-LOCAL-004/005 permit privately retained bounded bytes and direct
  final-tree-vs-parent status counts under qualified ignore/sparse policy with a
  complete second observation. Binary/structural/complexity cases omit line
  totals. Merge/conflict status remains explicitly unsupported.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M13.4 remains In progress with its existing indexed search/code-command
  and broader qualification gates; this downstream-enabling read slice completes
  no milestone.

```
<!-- /evidence-field -->

