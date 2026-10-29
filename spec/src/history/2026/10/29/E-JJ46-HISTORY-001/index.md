---
schema: 1
id: E-JJ46-HISTORY-001
---

# E-JJ46-HISTORY-001

<!-- evidence-field:id markdown -->
## Id

E-JJ46-HISTORY-001
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

- crates/jj-store-jj/src/local_browser/mod.rs
- spec/src/current/progress/M30/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-jj/tests/local_history.rs#standalone_history_preserves_source_pins_diffs_cancellation_and_current_admission
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
pnpm dev:cargo -- test --locked -p jj-store-jj --test local_history standalone_history_preserves_source_pins_diffs_cancellation_and_current_admission
```

**Notes**

The snapshot is taken before heads(). Loading a stock jj 0.45.1 repo no longer writes .jj/repo/workspace_store/type. Cancellation left that tree unchanged. 1 passed.
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

No stored format change. A stock 0.45.1 checkout stays byte-identical across the read.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Keep a stock jj 0.45.1 read from writing the jj-lib 0.46 workspace-store type marker.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Required CI must pass on this head. M30 stays In progress.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: JST-JJ-001 and JST-LOCAL-001 already require reads not to initialize
  repository state. The loader follows that rule for a missing workspace store
  type file.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M30 stays In progress. This read fix does not complete release
  qualification.

```
<!-- /evidence-field -->
