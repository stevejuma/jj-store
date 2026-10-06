---
schema: 1
id: E-M3-003
---

# E-M3-003

<!-- evidence-field:id markdown -->
## Id

E-M3-003
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M3
- M3.2
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-27
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

native-commit-backend
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- Cargo.lock
- crates/jj-store-jj/Cargo.toml
- crates/jj-store-jj/src/backend.rs
- crates/jj-store-jj/src/copy_query.rs
- crates/jj-store-jj/src/lib.rs
- crates/jj-store-jj/tests/native_backend.rs
- spec/src/jj-integration.md
- spec/src/architecture.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-jj/tests/native_backend.rs#native_backend_roundtrips_roots_full_trees_commits_and_signatures
- crates/jj-store-jj/tests/native_backend.rs#file_stream_survives_backend_drop_and_rejects_unknown_corrupt_or_revoked_reads
- crates/jj-store-jj/tests/native_backend.rs#copy_inventory_is_session_local_complete_deterministic_and_reverse_topological
- crates/jj-store-jj/tests/native_backend.rs#copy_records_follow_renames_edits_dag_ranges_and_path_filters
- crates/jj-store-jj/tests/native_backend.rs#copy_records_cover_merges_multiple_sources_and_intra_commit_chains
- crates/jj-store-jj/tests/native_backend.rs#conflicted_copy_targets_preserve_positive_alternatives_and_nested_paths
- crates/jj-store-jj/tests/native_backend.rs#copy_queries_fail_explicitly_on_limits_deadlines_missing_data_and_revocation
- crates/jj-store-jj/tests/native_backend.rs#staging_limit_allows_identical_copy_retry_without_accepting_new_history
- crates/jj-store-jj/tests/native_backend.rs#signing_failure_and_unacknowledged_metadata_leave_no_successful_object
- crates/jj-store-jj/tests/native_backend.rs#record_stream_rechecks_revocation_between_results_and_rejects_bad_copy_paths
- crates/jj-store-jj/tests/native_backend.rs#canceled_tree_terms_do_not_emit_deleted_copy_records
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-jj --test native_backend
```

**Notes**

Before implementation: native_backend target absent.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo check -p jj-store-jj
```

**Notes**

Initial integration: ObjectKey does not implement Display; RepoPathBuf comparison requires borrowed RepoPath. Both corrected.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-jj --test native_backend
```

**Notes**

Test compilation: pinned RepoPathComponent uses as_internal_str rather than as_str; corrected.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-jj --test native_backend
```

**Notes**

8 focused tests passed before adding signing-failure and mid-stream-revocation coverage.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-jj --test native_backend
```

**Notes**

10 focused tests passed, including signing failure and revocation between streamed records.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check
```
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm format:check
```
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check
```

**Notes**

Initial 9-lane pass before review found canceled tree-term edge case.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-jj --test native_backend canceled_tree_terms_do_not_emit_deleted_copy_records
```

**Notes**

Review regression: considering only positive tree terms emitted a copy for a file removed by cancellation against the base.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-jj --test native_backend canceled_tree_terms_do_not_emit_deleted_copy_records
```

**Notes**

Full signed tree terms now simplify at each path using native Merge; the deleted file produces no copy record.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-jj --test native_backend
```

**Notes**

11 backend tests pass after native per-path merge simplification. Includes complete objects/signatures, streamed binary reads, scope/revocation, copy DAGs/chains/conflicts, unknown outcomes and resource limits.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check
```
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test
```

**Notes**

70 Rust tests, 14 governance tests and 3 storage-harness self-tests pass. Two external RustFS tests remain opt-in. This slice validates native trait integration using the fault store, not live CLI/server acceptance.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build
```
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check
```

**Notes**

Final nine lanes pass after merge-cancellation regression fix, including rustfmt, specification mapping, tests, governance and mdBook.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm format:check
```
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Production authenticated authority, StoreFactories and remote RPC still
  required.
- Full CLI lifecycle, independent workspaces and deployed
  transport-authentication acceptance remain M3.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M3-003 details 2](details-02.md)
