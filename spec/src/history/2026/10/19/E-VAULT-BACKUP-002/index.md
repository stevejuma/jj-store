---
schema: 1
id: E-VAULT-BACKUP-002
---

# E-VAULT-BACKUP-002

<!-- evidence-field:id markdown -->
## Id

E-VAULT-BACKUP-002
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M34
- M34.2
- M34.3
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-LOCAL-007
- JST-ARCH-007
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
- crates/jj-store-jj/Cargo.toml
- crates/jj-store-jj/src/vault_workspace/mod.rs
- crates/jj-store-jj/src/vault_workspace/restore.rs
- crates/jj-store-jj/src/vault_workspace/restore_io.rs
- crates/jj-store-jj/tests/vault_restore.rs
- crates/jj-vault/README.md
- crates/jj-vault/src/lib.rs
- crates/jj-vault/src/native.rs
- crates/jj-vault/src/restore.rs
- crates/jj-vault/tests/native_restore.rs
- spec/src/current/contracts/architecture/26-reusable-agent-workflows.md
- spec/src/current/contracts/jj-integration/28-vault-working-copy.md
- spec/src/current/contracts/local-jj/32-automatic-vault-backup.md
- spec/src/current/progress/M34/index.md
- spec/src/history/2026/10/19/E-VAULT-BACKUP-002/SUMMARY.nav.md
- spec/src/history/2026/10/19/E-VAULT-BACKUP-002/index.md
- spec/src/history/2026/10/19/SUMMARY.nav.md
- spec/src/history/2026/10/19/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-jj/tests/vault_restore.rs#restores_retained_bytes_after_a_safety_checkpoint_and_replays_only_its_receipt
- crates/jj-store-jj/tests/vault_restore.rs#stale_previews_and_pending_receipts_never_overwrite_later_edits
- crates/jj-store-jj/tests/vault_restore.rs#restores_binary_deletion_and_execution_mode_without_touching_excluded_files
- crates/jj-vault/tests/native_restore.rs#the_worker_serializes_restore_with_capture_and_publishes_its_status
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
CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 pnpm dev:cargo -- test -p jj-store-jj --no-default-features --features vault-backup --test vault_backup
```

**Notes**

Four existing capture/relink tests pass with the new restore module.
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
CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 pnpm dev:cargo -- test -p jj-store-jj --no-default-features --features vault-backup --test vault_restore
```

**Notes**

Three native restore tests pass (latest includes binary/deletion/executable and exclusion guards). Retained binary/text bytes, safety ancestry, cold session reopen, stale previews, interrupted receipt completion, replay after later edits and request collision/traversal are covered.
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
CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 pnpm dev:cargo -- test -p jj-vault
```

**Notes**

Two scheduler, three lifecycle and one actual native restore-worker test pass. Zero doc examples. Logs restore-worker-final.log and restore-tests.log.
<!-- /run -->
<!-- run -->
### Run 4

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown
```

```shell
pnpm check:progress; pnpm spec:first
```

**Notes**

First progress rejected the milestone review status `changed`; use its specified
`updated` status. Next spec-first required the architecture/JJ chapters for this
cross-crate interface extension. Added explicit unchanged-contract confirmations
with JST-ARCH-007 and JST-JJ-003/005, retaining source ownership and native codecs.
No original gate was weakened. Final progress and document results follow.
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
node scripts/check-boundaries.mjs; pnpm check:progress; pnpm spec:validate; pnpm spec:first
```

**Notes**

26package boundaries,179tasks/912evidence records, seven validators for4310chapters/
208requirements and spec-first11protected files/threecanonical chapters pass.
Consumer:50focused renderer/governance tests, native compile and one exact native
permission regression, strict types and24catalog interactions/accessibility runs
pass. Full vault restore and cold complete app qualification remain pending.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Full-vault restore and complete native WebView lifecycle acceptance remain required.
- GitHub/Forgejo, sync, historical search, storage/adoption and main integration remain required.
- Full suites/platform/release lanes are deferred by explicit user instruction.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ library0.45.1; Rust1.99; Node26.9; source pnpm10.32.1, desktop pnpm10.34.5.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Native JJ and ordinary Git codecs unchanged. New private vault binding version1. No server storage/WAL/manifest profile change.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Guarded file restore primitives and reusable worker dispatch; no server or manifest change. Source metadata stays external by default. Existing component/data ownership is retained.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Complete native local-history integration and full-vault restore, then forge accounts, synchronization, search and advanced storage.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-LOCAL-007 now specifies current-disk preview guards, retained safety
  checkpoints and durable request replay; unsupported symlink restore fails
  explicitly. JST-ARCH-007 and JST-JJ-003/005 identity/ownership are unchanged.
```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: updated
reason: M34.2 becomes Partial with original cold app acceptance deferred;
  M34.3 is the sole active stage. File restore passes native proofs but full
  vault restoration and remaining delivery gates prevent Done.
```
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


