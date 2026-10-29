---
schema: 1
id: E-VAULT-BACKUP-008
---

# E-VAULT-BACKUP-008

<!-- evidence-field:id markdown -->
## Id

E-VAULT-BACKUP-008
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M34
- M34.5
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-LOCAL-007
- JST-JJ-003
- JST-JJ-005
- JST-ARCH-007
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

- crates/jj-store-jj/src/vault_workspace/content.rs
- crates/jj-store-jj/src/vault_workspace/mod.rs
- crates/jj-store-jj/src/vault_workspace/remote/import.rs
- crates/jj-store-jj/src/vault_workspace/remote/mod.rs
- crates/jj-store-jj/src/vault_workspace/remote/publication.rs
- crates/jj-store-jj/src/vault_workspace/sync/mod.rs
- crates/jj-store-jj/src/vault_workspace/sync/resolution.rs
- crates/jj-store-jj/tests/vault_sync.rs
- crates/jj-vault/README.md
- crates/jj-vault/src/lib.rs
- crates/jj-vault/src/native.rs
- crates/jj-vault/src/sync.rs
- crates/jj-vault/tests/native_remote.rs
- spec/src/current/contracts/architecture/26-reusable-agent-workflows.md
- spec/src/current/contracts/jj-integration/28-vault-working-copy.md
- spec/src/current/contracts/local-jj/32-automatic-vault-backup.md
- spec/src/current/progress/M34/index.md
- spec/src/history/2026/10/19/E-VAULT-BACKUP-008/SUMMARY.nav.md
- spec/src/history/2026/10/19/E-VAULT-BACKUP-008/index.md
- spec/src/history/2026/10/19/SUMMARY.nav.md
- spec/src/history/2026/10/19/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-jj/tests/vault_sync.rs#two_devices_merge_offline_edits_and_preserve_git_ancestry
- crates/jj-store-jj/tests/vault_sync.rs#conflicts_are_retained_without_touching_files_and_stale_sync_cannot_overwrite_edits
- crates/jj-store-jj/tests/vault_sync.rs#unknown_push_is_confirmed_when_the_remote_has_advanced_to_its_descendant
- crates/jj-vault/tests/native_remote.rs#native_remote_import_and_restore_share_the_serialized_backup_worker
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 13
hasNote: true
noteEncoding: markdown

```

```shell
pnpm dev:cargo -- test -p jj-store-jj --no-default-features --features vault-backup --test vault_sync
```

**Notes**

Before implementation: acceptance code cannot compile because SyncResolution and Session sync methods are absent. This is a missing-capability compile boundary, not a runtime failure count. First implementation exposed private sibling helper visibility; fixed narrowly to vault_workspace scope.
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
pnpm dev:cargo -- test -p jj-store-jj --no-default-features --features vault-backup --test vault_sync two_devices_merge_offline_edits_and_preserve_git_ancestry
```

**Notes**

Stronger convergence assertion reproduced redundant merge publication after pulling a remote descendant. Initial two-device and conflict proofs passed but did not prove stable convergence. sync-convergence-before.log retains the exact differing heads.
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
CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 pnpm dev:cargo -- test -p jj-store-jj --no-default-features --features vault-backup --test vault_sync --test vault_remote --test vault_backup --test vault_restore --test vault_restore_all
```

**Notes**

All17cases pass, including stable two-device convergence, retained conflicts/no file writes, explicit native resolution, stale disk rejection, known request replay, reserved exact publication and unknown accepted ancestor observation. sync-convergence-after.log. Native fast-forward adoption preserves the exact remote head; divergent merges retain both parents. No personal credentials/provider network.
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
pnpm dev:cargo -- test -p jj-vault; pnpm dev:cargo -- clippy -p jj-vault --all-targets -- -D warnings; pnpm dev:cargo -- clippy -p jj-store-jj --no-default-features --features vault-backup --test vault_sync --test vault_remote --test vault_backup --test vault_restore --test vault_restore_all -- -D warnings
```

**Notes**

Seven coordinator cases and both strict Clippy lanes pass. All Rust runs set incremental=0 and dev/test debug=0. Extended actual native worker proof imports, merges and applies a remote update while retaining later local files; transfer remains outside the worker. sync-worker-qualified, sync-worker-clippy and sync-native-clippy-qualified logs.
<!-- /run -->
<!-- run -->
### Bookkeeping

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

26package boundaries,179tasks/918records, sevenvalidators/4322chapters/208requirements and spec-first13protected files/threecanonical chapters pass. No full repository, cold app/provider or platform gate is substituted.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Complete desktop automatic scheduler/conflict controls and focused actual-app
  acceptance.
- Historical search, root storage/adoption, Forgejo and main integration remain
  required.
- Cold provider/full-suite/platform/release qualification remains user-deferred.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ0.45.1; Git2.55.0; Rust1.99.0; source base00c54474; desktop remote settingsd7db2f94.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Device-private sync-proposal.jsonV1 and sync restore receipts; additive owned SyncProposal/SyncConflict/SyncResolution DTOs. Upstream native JJ/Git codecs unchanged.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Native retained merge proposals, exclusion sanitization, guarded current-tree apply, immutable text/term/delete conflict resolution, durable recovery candidates, exact publication reservation and descendant observation. Injected jj-vault methods serialize native mutation without network IO.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Complete desktop automatic sync integration; preserve all later delivery stages.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-LOCAL-007 owns synchronization and guarded materialization.
  JST-JJ-003/005 retain upstream identity and immutable-reader behavior.
  JST-ARCH-007 retains injected worker/host separation and no upstream JJ import
  outside native package.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: updated
reason: M34.5 remains the sole active stage. Source17native and sevenworker
  cases qualify focused implementation; desktop controls and original cold
  app/provider gate remain open. No milestone completion or main integration is
  claimed.

```
<!-- /evidence-field -->

