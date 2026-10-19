---
schema: 1
id: E-VAULT-BACKUP-012
---

# E-VAULT-BACKUP-012

<!-- evidence-field:id markdown -->
## Id

E-VAULT-BACKUP-012
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M34
- M34.7
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

- crates/jj-store-jj/src/vault_workspace/binding.rs
- crates/jj-store-jj/src/vault_workspace/initialize.rs
- crates/jj-store-jj/src/vault_workspace/mod.rs
- crates/jj-store-jj/src/vault_workspace/remote/process.rs
- crates/jj-store-jj/src/vault_workspace/remote/publication/recovery.rs
- crates/jj-store-jj/src/vault_workspace/storage/copy.rs
- crates/jj-store-jj/src/vault_workspace/storage/mod.rs
- crates/jj-store-jj/src/vault_workspace/storage/record.rs
- crates/jj-store-jj/src/vault_workspace/storage/rollback.rs
- crates/jj-store-jj/src/vault_workspace/storage/transition.rs
- crates/jj-store-jj/tests/vault_storage.rs
- crates/jj-vault/README.md
- crates/jj-vault/src/lib.rs
- spec/src/current/contracts/jj-integration/28-vault-working-copy.md
- spec/src/current/contracts/local-jj/32-automatic-vault-backup.md
- spec/src/current/progress/M34/index.md
- spec/src/history/2026/10/19/E-VAULT-BACKUP-012/SUMMARY.nav.md
- spec/src/history/2026/10/19/E-VAULT-BACKUP-012/index.md
- spec/src/history/2026/10/19/SUMMARY.nav.md
- spec/src/history/2026/10/19/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-jj/tests/vault_storage.rs#relocation_preserves_native_history_receipts_and_unsnapshotted_files_across_both_layouts
- crates/jj-store-jj/tests/vault_storage.rs#interrupted_copy_replaces_owned_readonly_stage_files_on_recovery
- crates/jj-store-jj/tests/vault_backup.rs#external_capture_retains_edit_bursts_and_never_creates_vault_metadata
- crates/jj-store-jj/tests/vault_remote.rs#ordinary_git_backup_restores_files_and_native_history_on_a_fresh_device
- crates/jj-store-jj/tests/vault_publication.rs#explicit_stop_preserves_the_unknown_receipt_and_never_replays_or_clears_a_newer_push
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
pnpm dev:cargo -- test -p jj-store-jj --no-default-features --features vault-backup --test vault_storage
```

**Notes**

Before repair: four pass, one rollback fails with Busy because rollback reacquires its held writer lease. Post-command storage reserve also fails at 29.8 GiB. Preserve both findings.
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
pnpm dev:cargo -- test -p jj-store-jj --no-default-features --features vault-backup --test vault_storage
```

**Notes**

After repair: all six cases pass, including read-only staged-file recovery. Command still fails its separate 30 GiB post-build reserve check. This is not an accepted command pass.
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
pnpm dev:disk; pnpm dev:clean -- --dry-run; pnpm dev:clean -- --apply
```

**Notes**

Dry-run selects only idle source workspace build artifacts. Guarded cleanup recovers 29.8 to 30.6 GiB. Subsequent reserve pressure is resolved by the same guarded helper: verify the desktop target is idle, then clean its first-party workspace artifacts while retaining dependencies and the managed exclusive lease. Target allocation falls from 6861873152 to 4628545536 bytes. No review data, vaults, dependencies or running executable is removed.
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
CARGO_INCREMENTAL=0 pnpm dev:cargo -- test -p jj-store-jj --no-default-features --features vault-backup --test vault_storage --test vault_backup --test vault_remote --test vault_publication
```

**Notes**

All16native regressions pass and managed wrapper exits successfully. Six storage cases exercise both directions, pending receipt bytes, old-request replay, original-operation identities, interrupted archive/copy/install, explicit rollback, read-only files, destination preservation, root relocation and excluded locks. Ten direct capture and Git/publication regressions pass. .local/vault-backup/storage-native.log.
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
CARGO_INCREMENTAL=0 pnpm dev:cargo -- clippy -p jj-store-jj --no-default-features --features vault-backup --lib --test vault_storage --test vault_backup --test vault_remote --test vault_publication -- -D warnings
```

**Notes**

Focused strict Clippy passes. .local/vault-backup/storage-clippy.log.
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
CARGO_INCREMENTAL=0 pnpm dev:cargo -- check -p jj-vault
```

**Notes**

Direct worker-library consumer check passes. storage-worker-check.log.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 12
hasNote: true
noteEncoding: markdown
```

```shell
node scripts/check-boundaries.mjs
pnpm check:progress
pnpm spec:validate
pnpm spec:first
```

**Notes**

Package boundaries pass. Authored evidence initially lists four unnamed test files and a failed reserve run with zero failures: five progress errors. Including every generator-only whitespace change also exceeds document line/byte limits: two errors. Spec-first repeats the five progress errors. Correct the evidence references/count and restrict its file inventory to the owning slice; format generated navigation to avoid unrelated whitespace churn.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 30
hasNote: true
noteEncoding: markdown
```

```shell
pnpm check:progress
```

**Notes**

After correcting the evidence, generated unrelated navigation still causes30fresh-coverage errors. Restore only this session's unrelated generator edits from the parent; preserve all source changes, owning chapters, M34 and new dated evidence. The actual delivery slice is20files.
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
node scripts/check-boundaries.mjs
pnpm check:progress
pnpm spec:validate
pnpm spec:first
```

**Notes**

Final26package boundaries,179tasks/922records, sevenvalidators/4330chapters/208requirements and spec-first13protected files/two owning chapters pass. Rustfmt and changed Markdown formatting pass. Source native/Clippy/worker checks passed in runs4-6. This is focused source qualification; desktop adoption, cold app/provider and all deferred wider lanes remain open.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Complete desktop storage settings, binding resolution and existing-source adoption.
- Cold native app, live provider, full Rust/UI/catalog and platform/release lanes remain deferred by the user.
- Integrate into main only after the functional implementation and focused checks are complete.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

jj 0.45.1; rustc 1.99.0 (b940084d7 2026-09-28); git version 2.55.0; source parent890d8220; desktop parent798424a7.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Private vault binding v1 accepts optional path/relocation lineage; storage current/move v1 records use atomic synced replacement. Native JJ objects and original publication receipt bytes remain unchanged.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Native external/root metadata relocation and reusable jj-vault exports. Capture and transport take separate leases; movement fences both. Archives retain the original complete repository. Copy validates native operation identity and refuses unrelated destinations. Recovery resumes the original move; rollback is allowed only before installation. Root controls and staging remain excluded from captured notes. No desktop UI or live vault has been migrated.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Complete desktop storage selection and existing-repository history-source adoption, then cold native acceptance and main integration.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-LOCAL-007 defines explicit recoverable storage relocation and path lineage. Mapped JST-JJ-003/005 confirmation retains native identities, codecs and immutable read behavior in 28-vault-working-copy.md.
```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M34.7 remains the only active stage. Native storage proofs pass; desktop selection, adoption, cold app/provider qualification and main integration are unfinished. Earlier partial stages and deferred heavy gates retain their original scope.
```
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


