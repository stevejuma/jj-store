---
schema: 1
id: E-VAULT-BACKUP-014
---

# E-VAULT-BACKUP-014

<!-- evidence-field:id markdown -->
## Id

E-VAULT-BACKUP-014
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
- JST-DOC-001
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
- Cargo.toml
- crates/jj-forge/README.md
- crates/jj-forge/src/accounts/forgejo.rs
- crates/jj-forge/src/accounts/http.rs
- crates/jj-forge/src/accounts/mod.rs
- crates/jj-forge/src/accounts/store.rs
- crates/jj-forge/tests/forgejo_accounts.rs
- crates/jj-store-jj/Cargo.toml
- crates/jj-store-jj/README.md
- crates/jj-store-jj/src/lib.rs
- crates/jj-store-jj/src/vault_workspace/adoption/mod.rs
- crates/jj-store-jj/src/vault_workspace/adoption/publish.rs
- crates/jj-store-jj/src/vault_workspace/adoption/source.rs
- crates/jj-store-jj/src/vault_workspace/batch_plan.rs
- crates/jj-store-jj/src/vault_workspace/batch_restore.rs
- crates/jj-store-jj/src/vault_workspace/binding.rs
- crates/jj-store-jj/src/vault_workspace/capture.rs
- crates/jj-store-jj/src/vault_workspace/content.rs
- crates/jj-store-jj/src/vault_workspace/history.rs
- crates/jj-store-jj/src/vault_workspace/initialize.rs
- crates/jj-store-jj/src/vault_workspace/mod.rs
- crates/jj-store-jj/src/vault_workspace/remote/adoption.rs
- crates/jj-store-jj/src/vault_workspace/remote/import.rs
- crates/jj-store-jj/src/vault_workspace/remote/mod.rs
- crates/jj-store-jj/src/vault_workspace/remote/process.rs
- crates/jj-store-jj/src/vault_workspace/remote/publication.rs
- crates/jj-store-jj/src/vault_workspace/remote/publication/recovery.rs
- crates/jj-store-jj/src/vault_workspace/restore.rs
- crates/jj-store-jj/src/vault_workspace/restore_io.rs
- crates/jj-store-jj/src/vault_workspace/scan.rs
- crates/jj-store-jj/src/vault_workspace/storage/copy.rs
- crates/jj-store-jj/src/vault_workspace/storage/mod.rs
- crates/jj-store-jj/src/vault_workspace/storage/record.rs
- crates/jj-store-jj/src/vault_workspace/storage/rollback.rs
- crates/jj-store-jj/src/vault_workspace/storage/transition.rs
- crates/jj-store-jj/src/vault_workspace/sync/mod.rs
- crates/jj-store-jj/src/vault_workspace/sync/resolution.rs
- crates/jj-store-jj/tests/vault_adoption.rs
- crates/jj-store-jj/tests/vault_backup.rs
- crates/jj-store-jj/tests/vault_history.rs
- crates/jj-store-jj/tests/vault_publication.rs
- crates/jj-store-jj/tests/vault_remote.rs
- crates/jj-store-jj/tests/vault_restore.rs
- crates/jj-store-jj/tests/vault_restore_all.rs
- crates/jj-store-jj/tests/vault_storage.rs
- crates/jj-store-jj/tests/vault_sync.rs
- crates/jj-vault/Cargo.toml
- crates/jj-vault/README.md
- crates/jj-vault/src/adoption.rs
- crates/jj-vault/src/lib.rs
- crates/jj-vault/src/native.rs
- crates/jj-vault/src/remote.rs
- crates/jj-vault/src/restore.rs
- crates/jj-vault/src/schedule.rs
- crates/jj-vault/src/search/cache.rs
- crates/jj-vault/src/search/mod.rs
- crates/jj-vault/src/search/native.rs
- crates/jj-vault/src/sync.rs
- crates/jj-vault/tests/lifetime.rs
- crates/jj-vault/tests/native_adoption.rs
- crates/jj-vault/tests/native_remote.rs
- crates/jj-vault/tests/native_restore.rs
- crates/jj-vault/tests/search.rs
- docs/maintenance/reusable-agent-workflows-progress.md
- scripts/check-boundaries.mjs
- spec-validator.config.mjs
- spec/src/current/contracts/architecture/24-forge-stacks.md
- spec/src/current/contracts/architecture/26-reusable-agent-workflows.md
- spec/src/current/contracts/documentation/01-application-documentation.md
- spec/src/current/contracts/jj-integration/28-vault-working-copy.md
- spec/src/current/contracts/jj-integration/SUMMARY.nav.md
- spec/src/current/contracts/jj-integration/index.md
- spec/src/current/contracts/local-jj/02-requirements.md
- spec/src/current/contracts/local-jj/32-automatic-vault-backup.md
- spec/src/current/contracts/local-jj/SUMMARY.nav.md
- spec/src/current/contracts/local-jj/index.md
- spec/src/current/governance/spec-governance.md
- spec/src/current/progress/M33/history/M33.4/index.md
- spec/src/current/progress/M33/history/M33/index.md
- spec/src/current/progress/M33/index.md
- spec/src/current/progress/M34/SUMMARY.nav.md
- spec/src/current/progress/M34/index.md
- spec/src/current/progress/SUMMARY.nav.md
- spec/src/current/progress/index.md
- spec/src/current/verification/SUMMARY.nav.md
- spec/src/current/verification/index.md
- spec/src/current/verification/local-03.md
- spec/src/history/2026/10/19/E-REUSABLE-PROGRESS-034/SUMMARY.nav.md
- spec/src/history/2026/10/19/E-REUSABLE-PROGRESS-034/index.md
- spec/src/history/2026/10/19/E-REUSABLE-PROGRESS-035/SUMMARY.nav.md
- spec/src/history/2026/10/19/E-REUSABLE-PROGRESS-035/index.md
- spec/src/history/2026/10/19/E-REUSABLE-PROGRESS-036/SUMMARY.nav.md
- spec/src/history/2026/10/19/E-REUSABLE-PROGRESS-036/index.md
- spec/src/history/2026/10/19/E-REUSABLE-PROGRESS-037/SUMMARY.nav.md
- spec/src/history/2026/10/19/E-REUSABLE-PROGRESS-037/index.md
- spec/src/history/2026/10/19/E-REUSABLE-PROGRESS-038/SUMMARY.nav.md
- spec/src/history/2026/10/19/E-REUSABLE-PROGRESS-038/index.md
- spec/src/history/2026/10/19/E-REUSABLE-PROGRESS-039/SUMMARY.nav.md
- spec/src/history/2026/10/19/E-REUSABLE-PROGRESS-039/index.md
- spec/src/history/2026/10/19/E-REUSABLE-PROGRESS-040/SUMMARY.nav.md
- spec/src/history/2026/10/19/E-REUSABLE-PROGRESS-040/index.md
- spec/src/history/2026/10/19/E-REUSABLE-PROGRESS-041/SUMMARY.nav.md
- spec/src/history/2026/10/19/E-REUSABLE-PROGRESS-041/index.md
- spec/src/history/2026/10/19/E-REUSABLE-PROGRESS-042/SUMMARY.nav.md
- spec/src/history/2026/10/19/E-REUSABLE-PROGRESS-042/index.md
- spec/src/history/2026/10/19/E-VAULT-BACKUP-001/SUMMARY.nav.md
- spec/src/history/2026/10/19/E-VAULT-BACKUP-001/index.md
- spec/src/history/2026/10/19/E-VAULT-BACKUP-002/SUMMARY.nav.md
- spec/src/history/2026/10/19/E-VAULT-BACKUP-002/index.md
- spec/src/history/2026/10/19/E-VAULT-BACKUP-003/SUMMARY.nav.md
- spec/src/history/2026/10/19/E-VAULT-BACKUP-003/index.md
- spec/src/history/2026/10/19/E-VAULT-BACKUP-004/SUMMARY.nav.md
- spec/src/history/2026/10/19/E-VAULT-BACKUP-004/index.md
- spec/src/history/2026/10/19/E-VAULT-BACKUP-005/SUMMARY.nav.md
- spec/src/history/2026/10/19/E-VAULT-BACKUP-005/index.md
- spec/src/history/2026/10/19/E-VAULT-BACKUP-006/SUMMARY.nav.md
- spec/src/history/2026/10/19/E-VAULT-BACKUP-006/index.md
- spec/src/history/2026/10/19/E-VAULT-BACKUP-007/SUMMARY.nav.md
- spec/src/history/2026/10/19/E-VAULT-BACKUP-007/index.md
- spec/src/history/2026/10/19/E-VAULT-BACKUP-008/SUMMARY.nav.md
- spec/src/history/2026/10/19/E-VAULT-BACKUP-008/index.md
- spec/src/history/2026/10/19/E-VAULT-BACKUP-009/SUMMARY.nav.md
- spec/src/history/2026/10/19/E-VAULT-BACKUP-009/index.md
- spec/src/history/2026/10/19/E-VAULT-BACKUP-010/SUMMARY.nav.md
- spec/src/history/2026/10/19/E-VAULT-BACKUP-010/index.md
- spec/src/history/2026/10/19/E-VAULT-BACKUP-011/SUMMARY.nav.md
- spec/src/history/2026/10/19/E-VAULT-BACKUP-011/index.md
- spec/src/history/2026/10/19/E-VAULT-BACKUP-012/SUMMARY.nav.md
- spec/src/history/2026/10/19/E-VAULT-BACKUP-012/index.md
- spec/src/history/2026/10/19/E-VAULT-BACKUP-013/SUMMARY.nav.md
- spec/src/history/2026/10/19/E-VAULT-BACKUP-013/index.md
- spec/src/history/2026/10/19/E-VAULT-BACKUP-014/SUMMARY.nav.md
- spec/src/history/2026/10/19/E-VAULT-BACKUP-014/index.md
- spec/src/history/2026/10/19/SUMMARY.nav.md
- spec/src/history/2026/10/19/index.md
- spec/src/history/2026/10/SUMMARY.nav.md
- spec/src/history/2026/10/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-jj/tests/vault_adoption.rs#adopts_recorded_git_ancestry_without_touching_source_metadata_or_saved_files
- crates/jj-store-jj/tests/vault_adoption.rs#startup_capture_recovers_prepared_or_published_adoption_before_recording_later_edits
- crates/jj-store-jj/tests/vault_adoption.rs#stopping_an_unfinished_import_retains_the_receipt_and_never_replays_it
- crates/jj-vault/tests/native_adoption.rs#worker_adopts_original_history_and_receipt_replay_keeps_the_current_status
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
jj new -r 150d88a3 -r d45b03eb -m "Integrate automatic vault backup into the current jj-store workspace"
jj diff --name-only --from d45b03eb --to @ -- crates packages Cargo.toml Cargo.lock
```

**Notes**

The merge preserves current M33 work and all M34 implementation. Four generated
history navigation conflicts are resolved by retaining both evidence families
and the current dated title. The exact production/package/manifest diff from the
verified source slice is empty. Native source behavior and formats are unchanged;
the14native and10worker cases and strict Clippy from E-VAULT-BACKUP-013 remain
applicable. No Rust rebuild is needed for this documentation-only integration.
The main workspace's unrelated progress-file edit remains separate after
`jj rebase -r @ -d vkkkokku`. Its SHA-256 before and after is
`e7103da653e3bd747602ff9d8ff22bca82f5654edc46144eabb5325d923d2d66`.
The user's working change remains uncommitted with only that original file.
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
node scripts/check-boundaries.mjs
pnpm check:progress
pnpm spec:validate
pnpm spec:first
```

**Notes**

Package boundaries pass. The merge ledger initially rejects milestone status
changed (the schema requires updated/unchanged) and requires fresh evidence for
the complete union of both parent diffs. Record that full integration inventory,
preserve the original evidence and use the governed milestone-review status.
Native code remains identical to d45b03eb; no native tests are substituted.
Further schema checks reject a pending unchanged-contract result and then a
passing result with milestone-wide gaps. This record qualifies only the completed
documentation integration; native/provider gaps stay explicit in M34 and the
prior records, outside this narrow result.
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
pnpm check:progress
pnpm spec:validate
pnpm spec:first
```

**Notes**

Final merged ledger, canonical validation and spec-first pass after covering the
complete parent-diff inventory and retaining both native-backup and M33 evidence.
The unchanged-contract gate also required this documentation-only scope to have
a completed passing result; milestone qualification remains Partial.
The integration has no production-code diff from the verified d45b03eb slice.
Native/provider/full-suite/release checks stay deferred; no gate is promoted.
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

Verified source adoption d45b03eb; current main parent150d88a3; desktop adoption a4ce183a; enclosing merge change owns this evidence.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Native codecs, identities, private binding/storage/adoption/publication receipts and public contracts remain unchanged. Only Markdown navigation is merged.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Integrate the complete automatic-vault-backup implementation into the current source workspace, preserving independent M33 progress and the user's uncommitted progress edit. Production code is byte-identical to the verified source slice. This is documentation/navigation integration with no native behavior change.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Review the integrated working plugin. Complete deferred native/provider and wider qualification when resumed; do not call M34 Done from focused evidence.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: JST-LOCAL-007 and JST-JJ-003/005 retain the exact verified production code and formats; JST-DOC-001 retains the current independently qualified M33 documentation. The integration changes only documentation navigation and delivery bookkeeping.
```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: updated
reason: Functional implementation is integrated into main. M34 and M34.7 become Partial with no active implementation stage; cold app/provider and user-deferred wider qualification keep their original gates. Earlier stages remain Partial.
```
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


