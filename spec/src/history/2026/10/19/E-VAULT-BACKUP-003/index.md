---
schema: 1
id: E-VAULT-BACKUP-003
---

# E-VAULT-BACKUP-003

<!-- evidence-field:id markdown -->
## Id

E-VAULT-BACKUP-003
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M34
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

- crates/jj-store-jj/src/vault_workspace/batch_plan.rs
- crates/jj-store-jj/src/vault_workspace/batch_restore.rs
- crates/jj-store-jj/src/vault_workspace/content.rs
- crates/jj-store-jj/src/vault_workspace/mod.rs
- crates/jj-store-jj/src/vault_workspace/scan.rs
- crates/jj-store-jj/tests/vault_restore_all.rs
- crates/jj-vault/src/lib.rs
- crates/jj-vault/src/native.rs
- crates/jj-vault/src/restore.rs
- crates/jj-vault/tests/native_restore.rs
- spec/src/current/contracts/architecture/26-reusable-agent-workflows.md
- spec/src/current/contracts/jj-integration/28-vault-working-copy.md
- spec/src/current/contracts/local-jj/32-automatic-vault-backup.md
- spec/src/current/progress/M34/index.md
- spec/src/history/2026/10/19/E-VAULT-BACKUP-003/SUMMARY.nav.md
- spec/src/history/2026/10/19/E-VAULT-BACKUP-003/index.md
- spec/src/history/2026/10/19/SUMMARY.nav.md
- spec/src/history/2026/10/19/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-jj/tests/vault_restore_all.rs#restores_a_whole_vault_and_preserves_original_safety_and_runtime_files
- crates/jj-store-jj/tests/vault_restore_all.rs#stale_vault_preview_and_interrupted_restore_preserve_newer_edits
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
CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 pnpm dev:cargo -- check -p jj-store-jj --no-default-features --features vault-backup
```

**Notes**

Native source check passes. Whole-vault planning resolves included regular-file trees, bounds work and rejects unsupported transitions before source writes.
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
CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 pnpm dev:cargo -- test -p jj-store-jj --no-default-features --features vault-backup --test vault_restore_all
```

**Notes**

Two actual native tests pass: additions/modifications/deletions, binary preservation, original safety checkpoint, runtime exclusion, cold reopen/replay, stale preview and partial-progress recovery with later edit refusal.
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
CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 pnpm dev:cargo -- test -p jj-vault; pnpm dev:cargo -- test -p jj-vault --test native_restore
```

**Notes**

Coordinator2scheduler/3lifecycle/1native tests pass; extended native worker case then passes file and whole-vault preview/apply, status and close on the same serialized owner. Logs batch-worker-tests.log and batch-worker-final.log.
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
pnpm dev:cargo -- test -p jj-store-jj --no-default-features --features vault-backup --test vault_backup --test vault_restore --test vault_restore_all
node scripts/check-boundaries.mjs
pnpm check:progress
pnpm spec:validate
pnpm spec:first
```

**Notes**

Nine native tests pass across the three directly affected binaries.26package
boundaries,179tasks/913evidence records, seven validators4312chapters/208requirements
and spec-first10protected files/threecanonical chapters pass. Consumer native check
and whole-vault authority frame test pass; source/type/renderer and focused catalog
results remain separately recorded in E-REPO-028. Full suites and complete cold app
qualification are deferred by user instruction; remaining implementation stays open.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Complete cold actual-WebView capture/restore/reopen acceptance remains deferred.
- GitHub/Forgejo, synchronization, historical search, storage/adoption and main integration remain required.
- Full suites/platform/release lanes are deferred by explicit user instruction.
- Whole-vault restore rejects symlinks and file/directory type transitions until guarded profiles are qualified.
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

Resumable whole-vault restoration over included regular files; native JJ identity and Git codecs are unchanged. One original safety checkpoint remains reachable while the durable receipt records per-file progress. Later edits are never overwritten by replay.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Complete focused source/consumer bookkeeping and native cold acceptance; proceed to forge accounts and remote backup implementation. Retain all deferred gates.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-LOCAL-007 now owns whole-tree previews, original safety checkpoint,
  guarded per-file progress and complete-result verification. JST-ARCH-007
  injection and JST-JJ-003/005 native identity contracts remain unchanged.
```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: updated
reason: M34.3 remains the sole active stage. File and whole-vault restore
  have focused proofs; cold app, remote, search and main integration gates
  remain pending. No Done claim follows from compilation or mock UI.
```
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


