---
schema: 1
id: E-VAULT-BACKUP-020
---

# E-VAULT-BACKUP-020

<!-- evidence-field:id markdown -->
## Id

E-VAULT-BACKUP-020
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-21
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing-change
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M34.8
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-LOCAL-007
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Shared native AI observation boundaries, bounded portable commit-description context and selected-path undo, independent of renderer and AgentStore.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-jj/src/vault_workspace/ai_turns.rs
- crates/jj-store-jj/src/vault_workspace/ai_turn_capture.rs
- crates/jj-store-jj/src/vault_workspace/ai_undo.rs
- crates/jj-store-jj/src/vault_workspace/ai_undo_apply.rs
- crates/jj-store-jj/src/vault_workspace/mod.rs
- crates/jj-vault/src/ai.rs
- crates/jj-vault/src/lib.rs
- crates/jj-vault/src/native.rs
- crates/jj-vault/tests/ai_history.rs
- spec/src/current/contracts/local-jj/32-automatic-vault-backup.md
- spec/src/current/progress/M34/index.md
- crates/jj-vault/tests/ai_undo_recovery.rs
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-vault/tests/ai_history.rs#checkpoints_keep_context_and_guarded_undo_preserves_later_and_unrelated_edits
- crates/jj-vault/tests/ai_history.rs#cold_native_history_survives_two_processes_and_undo
- crates/jj-vault/tests/native_restore.rs#the_worker_serializes_restore_with_capture_and_publishes_its_status
- crates/jj-vault/tests/shutdown.rs#failed_close_keeps_the_original_worker_and_retry_captures_repaired_files
- crates/jj-vault/tests/ai_undo_recovery.rs#interrupted_batch_checks_completed_paths_before_touching_remaining_files
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; Rust 1.99.0; Node 26.9.0; pnpm 10.34.5; source parent 3998e2b0.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Version-1 native turn/undo receipts outside vaults; Lapis-AI-Observation-Version and JSON Lapis-AI-Observation trailers in Git-compatible descriptions; existing checkpoint/restore formats unchanged.
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
pnpm dev:disk; pnpm dev:cargo -- check -p jj-vault --locked
```

**Notes**

Initial report showed 18.5 GiB free. Managed Cargo admitted and completed the cached check without an override or cleanup of protected state.
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
pnpm dev:cargo -- test -p jj-vault --test ai_history --test native_restore --test shutdown --locked
```

**Notes**

13 cases pass: six AI test entries (including the helper entry), one ordinary restore and six shutdown cases. The cold parent launches two fresh native processes, captures a real file update, reopens exact boundaries and restores original bytes. Earlier iterations passed 2 then 4 AI entries before added coverage.
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
pnpm dev:cargo -- clippy -p jj-vault --all-targets --locked -- -D warnings
```

**Notes**

Initial run found large_enum_variant in AiHistoryResponse. Boxed the turn record without changing serialized JSON.
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
pnpm dev:cargo -- clippy -p jj-vault --all-targets --locked -- -D warnings
```

**Notes**

Corrected affected library and tests pass strict Clippy.
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
rustfmt --edition 2024 (changed Rust files)
```

**Notes**

Only the affected sources and tests were formatted.
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
pnpm dev:cargo -- test -p jj-vault --test ai_undo_recovery --locked
```

**Notes**

One additional partial-progress fault test passes: completed-path later edits stop the batch before any remaining write, stale atomic temporary files are ignored, original request recovery completes after review. Fourteen top-level source cases pass across focused runs.
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
pnpm dev:cargo -- clippy -p jj-vault --all-targets --locked -- -D warnings
```

**Notes**

New fault test initially triggered cloned_ref_to_slice_refs. Replaced the assertion slice with std::slice::from_ref; no lint suppression.
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
pnpm dev:cargo -- clippy -p jj-vault --all-targets --locked -- -D warnings > .local/ai-history-clippy-final.log 2>&1
```

**Notes**

Final native source and all target tests pass strict Clippy.
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
pnpm evidence:new -- --id E-VAULT-BACKUP-020 --task M34.8 --requirement JST-LOCAL-007; pnpm check:progress
```

**Notes**

The progress check detected concurrent changes to its pinned revision while evidence was being completed. The evidence scaffold also rewrote unrelated navigation/history formatting; restored only those generated collateral changes from pre-task snapshot 61c3f60d while preserving initial unrelated work and new evidence/date navigation. Immutable historical evidence is retained.
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
pnpm check:progress; pnpm spec:validate
```

**Notes**

Specification validation passed. Progress found invalid evidence result partial, missing named test references and milestone review status changed. Corrected to pending, explicit test names and updated. Independent initial M33 edits still lack E-REUSABLE-PROGRESS-050; these remain untouched and this slice will be checked at its selected revision.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress > .local/ai-history-progress-corrected.log 2>&1
```

**Notes**

All E-VAULT-BACKUP-020 schema, test references and review fields now validate. Only the four initial unrelated M33 files and their missing E-REUSABLE-PROGRESS-050 remain. Preserve them; run the final ledger check on this selected change.
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
rustfmt --edition 2024 --check (ten changed Rust files); JJ_STORE_CHECK_REVISION=@- pnpm check:progress
```

**Notes**

Affected native sources/tests are formatted. Selected enclosing change xypolszs passes ledger validation: 180 tasks and 947 evidence records. Exactly four unrelated initial M33/documentation edits remain in the working copy, excluded from this commit and qualification.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Desktop visible provider/native-window integration and Git transfer/recovery
  of linked conversation records remain separate acceptance.
- Full Rust/UI/catalog, platform and release lanes are out of scope under the
  targeted verification policy.
- Hunk/merge-aware/whole-conversation undo and isolated agent workspaces remain
  deferred.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Finish Desktop integration evidence E-AI-114 and qualify its visible provider/native gates.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: Extended JST-LOCAL-007 with serialized AI observations, truthful
  mixed-edit attribution, startup incompleteness and guarded selected-path undo.
  Ordinary backup cadence, identity and immutable read authority are unchanged.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: updated
reason: Added M34.8 as Partial for Desktop product integration. Existing M34
  children and M33 gates retain their original statuses.

```
<!-- /evidence-field -->

