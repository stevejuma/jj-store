---
schema: 1
id: E-VAULT-BACKUP-017
---

# E-VAULT-BACKUP-017

<!-- evidence-field:id markdown -->
## Id

E-VAULT-BACKUP-017
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M34.6
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-LOCAL-007
- JST-JJ-003
- JST-JJ-005
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-19
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- Cargo.lock
- crates/jj-store-jj/src/vault_workspace/history/index.rs
- crates/jj-store-jj/src/vault_workspace/history.rs
- crates/jj-store-jj/src/vault_workspace/mod.rs
- crates/jj-vault/Cargo.toml
- crates/jj-vault/src/search/indexed/disk.rs
- crates/jj-vault/src/search/indexed/mod.rs
- crates/jj-vault/src/search/indexed/native.rs
- crates/jj-vault/src/search/indexed/query.rs
- crates/jj-vault/src/search/mod.rs
- crates/jj-vault/tests/indexed_search.rs
- spec/src/current/contracts/local-jj/32-automatic-vault-backup.md
- spec/src/current/contracts/jj-integration/28-vault-working-copy.md
- spec/src/current/progress/M34/index.md
- spec/src/current/contracts/architecture/27-vault-history-search.md
- spec/src/current/contracts/architecture/index.md
- spec/src/current/contracts/architecture/SUMMARY.nav.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-vault/tests/indexed_search.rs#indexes_changed_versions_once_and_keeps_reintroduced_checkpoint_references
- crates/jj-vault/tests/indexed_search.rs#resumes_after_failure_and_fences_changed_generation_cursors
- crates/jj-vault/tests/indexed_search.rs#traverses_each_merge_parent_and_recovers_a_corrupt_index
- crates/jj-vault/tests/indexed_search.rs#native_delta_projection_keeps_exact_deleted_versions_and_never_captures_disk
<!-- /evidence-field -->

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
pnpm dev:disk
```

**Notes**

41.0 GiB free before the targeted native builds; no cleanup or development-data reset performed.
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
pnpm dev:cargo -- check -p jj-vault
```

**Notes**

Initial implementation failed on ambiguous RepoPathBuf conversion (two compiler diagnostics); corrected to pass the string directly.
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
pnpm dev:cargo -- test -p jj-vault --test indexed_search
```

**Notes**

Initial Tantivy collector use required explicit order_by_score in 0.26.2; corrected without changing assertions.
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
pnpm dev:cargo -- test -p jj-vault --test indexed_search --test search
```

**Notes**

Four indexed-history cases and two existing lazy-projection cases pass. Covers deduplication, reintroduction, author/date/case filters, metadata-only hits, tombstones, each merge parent, resumable failure, obsolete generations, namespace change, corrupt rebuild and real native reads with unsnapshotted disk bytes preserved.
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
pnpm dev:cargo -- clippy -p jj-vault --all-targets -- -D warnings
```

**Notes**

One collapsible-if diagnostic in body deduplication; corrected.
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
pnpm dev:cargo -- fmt --package jj-vault --package jj-store-jj
pnpm dev:cargo -- test -p jj-vault --test indexed_search --test search
pnpm dev:cargo -- clippy -p jj-vault --all-targets -- -D warnings
```

**Notes**

Final formatted source: six tests pass and strict package Clippy passes.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first
```

**Notes**

The initial whole-working-copy gate sees four preserved unrelated M33/documentation edits and missing E-REUSABLE-PROGRESS-050, plus the then-unfilled source evidence. Qualify this slice with explicit source/chapter selection and the file-specific committed revision.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:validate
pnpm spec:first --file Cargo.lock --file crates/jj-store-jj/src/vault_workspace/history/index.rs --file crates/jj-vault/Cargo.toml --file spec/src/current/contracts/jj-integration/28-vault-working-copy.md --file spec/src/current/contracts/local-jj/32-automatic-vault-backup.md
```

**Notes**

Canonical validation passes. The source/dependency selection also requires an architecture chapter. Added the dedicated vault-search ownership chapter while preserving the overlapping M33 chapter. The navigation generator normalized unrelated historical pages; that generated churn was restored to the parent, leaving only the new evidence links.
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
pnpm spec:validate
pnpm spec:first --file Cargo.lock --file crates/jj-store-jj/src/vault_workspace/history/index.rs --file crates/jj-store-jj/src/vault_workspace/history.rs --file crates/jj-store-jj/src/vault_workspace/mod.rs --file crates/jj-vault/Cargo.toml --file crates/jj-vault/src/search/indexed/disk.rs --file crates/jj-vault/src/search/indexed/mod.rs --file crates/jj-vault/src/search/indexed/native.rs --file crates/jj-vault/src/search/indexed/query.rs --file crates/jj-vault/src/search/mod.rs --file crates/jj-vault/tests/indexed_search.rs --file spec/src/current/contracts/jj-integration/28-vault-working-copy.md --file spec/src/current/contracts/local-jj/32-automatic-vault-backup.md --file spec/src/current/contracts/architecture/27-vault-history-search.md --file spec/src/current/progress/M34/index.md
```

**Notes**

Canonical validation passes (4375 chapters, 208 requirements). Exact source and owning-chapter spec-first selection passes (11 protected files, three canonical chapters); four pre-existing M33/documentation edits remain excluded.
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
pnpm spec:first --base @-- --head @-
JJ_STORE_CHECK_REVISION=@- pnpm check:progress
```

**Notes**

File-specific committed source revision passes spec-first and progress: 179 tasks and 944 evidence records. Preserved M33 working-copy edits are outside this slice.
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

Enclosing Jujutsu change. Native JJ 0.45.1, Tantivy 0.26.2, Rust 1.99.0, macOS ARM64, pnpm 10.34.5.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive IndexRevision/IndexFile/IndexFiles and jj-vault indexed-search DTOs. Private history-search-v2 Tantivy projection and atomic resumable commit payload. Native repository, operation, view, checkpoint and restore-receipt formats are unchanged.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Native projection source only. Index metadata for each retained checkpoint, reuse immutable file bodies, record changed/deleted paths and every parent, and publish resumable bounded batches independently of capture. Query generations and source membership are checked; no storage initialization, working-copy lease, capture, transfer or restore authority is added. Root aggregates, UI, cold app/provider, platform and release lanes are out of scope.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Complete Search/Desktop provider integration and its focused browser/native acceptance. Preserve original M34.6 cold and wider qualification gates.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-LOCAL-007 now governs incremental deduplicated indexing, durable
  frontier and partial coverage. The owning backup and vault-working-copy
  chapters are updated; JST-JJ-003/005 identity and immutable-read authority
  remain unchanged.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M34.6 and M34 remain Partial at their original cold native/provider
  gates. The new source engine is qualified independently of the prior cached
  scan and the consuming Desktop UI.

```
<!-- /evidence-field -->

