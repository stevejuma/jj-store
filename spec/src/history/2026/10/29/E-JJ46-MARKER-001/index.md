---
schema: 1
id: E-JJ46-MARKER-001
---

# E-JJ46-MARKER-001

<!-- evidence-field:id markdown -->
## Id

E-JJ46-MARKER-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M30
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-JJ-001
- JST-LOCAL-001
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-29
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-jj/tests/stack_operation_archive/history.rs
- crates/jj-store-jj/tests/stack_reword_preparation/lifecycle.rs
- spec/src/current/progress/M30/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-jj/tests/stack_operation_archive/history.rs#archive_capture_never_imports_missing_extras_or_creates_nogc_refs
- crates/jj-store-jj/tests/stack_operation_archive/history.rs#historical_only_code_remains_reachable_after_gc_and_packed_ref_replay
- crates/jj-store-jj/tests/stack_reword_preparation/lifecycle.rs#missing_original_extras_fail_before_reword_import_or_candidate_write
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
pnpm dev:cargo -- test --locked -p jj-store-jj --test stack_operation_archive --test stack_reword_preparation -- archive_capture_never_imports_missing_extras_or_creates_nogc_refs historical_only_code_remains_reachable_after_gc_and_packed_ref_replay missing_original_extras_fail_before_reword_import_or_candidate_write
```

**Notes**

The comparisons ignore only `.jj/repo/workspace_store/type` when jj-lib 0.46 writes `simple_workspace_store` on first open. The archive and reword tests passed, 3 tests.
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

jj-lib 0.46.0 opened a stock jj 0.45.1 repository.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No stored format change beyond the one-time jj-lib 0.46 workspace-store type marker.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Keep archive and reword snapshots stable when jj-lib writes its one-time workspace-store type marker.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Required CI must pass on this head. M30 stays In progress.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: JST-JJ-001 and JST-LOCAL-001 still cover the same archive and reword
  behavior. The fixtures now accept jj-lib's one-time workspace-store type
  marker.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M30 stays In progress. These fixture baselines do not complete release
  qualification.

```
<!-- /evidence-field -->
