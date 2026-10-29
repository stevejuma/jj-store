---
schema: 1
id: E-VAULT-BACKUP-007
---

# E-VAULT-BACKUP-007

<!-- evidence-field:id markdown -->
## Id

E-VAULT-BACKUP-007
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M34
- M34.3
- M34.4
- M34.5
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

- spec/src/current/contracts/architecture/26-reusable-agent-workflows.md
- Cargo.lock
- crates/jj-store-jj/Cargo.toml
- crates/jj-store-jj/src/vault_workspace/restore_io.rs
- crates/jj-store-jj/tests/vault_restore.rs
- spec/src/current/contracts/jj-integration/28-vault-working-copy.md
- spec/src/current/contracts/local-jj/32-automatic-vault-backup.md
- spec/src/current/progress/M34/index.md
- spec/src/history/2026/10/19/E-VAULT-BACKUP-007/index.md
- spec/src/history/2026/10/19/E-VAULT-BACKUP-007/SUMMARY.nav.md
- spec/src/history/2026/10/19/index.md
- spec/src/history/2026/10/19/SUMMARY.nav.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-jj/tests/vault_restore.rs#restore_cannot_follow_an_ancestor_alias_into_private_vault_metadata
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
pnpm dev:cargo -- test -p jj-store-jj --no-default-features --features vault-backup --test vault_restore restore_cannot_follow_an_ancestor_alias_into_private_vault_metadata
```

**Notes**

Before: a previously valid restore followed a replaced parent directory symlink into .lapis and overwrote private runtime data in a disposable fixture. Recorded restore-alias-before.log.
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
CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 pnpm dev:cargo -- test -p jj-store-jj --no-default-features --features vault-backup --test vault_restore restore_cannot_follow_an_ancestor_alias_into_private_vault_metadata
```

**Notes**

After: exact regression passes. Parent directories use cap-fs-ext4.0.3 open_dir_nofollow component by component. Preview and apply reject the alias and private contents remain unchanged.
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
pnpm dev:cargo -- test -p jj-store-jj --no-default-features --features vault-backup --test vault_backup --test vault_restore --test vault_restore_all --test vault_remote
```

**Notes**

All14affected native regressions pass. All Rust commands use CARGO_INCREMENTAL=0 and dev/test debug=0. No remote provider or personal data.
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
pnpm dev:cargo -- test -p jj-vault; pnpm dev:cargo -- clippy -p jj-store-jj --no-default-features --features vault-backup --test vault_backup --test vault_restore --test vault_restore_all --test vault_remote -- -D warnings
```

**Notes**

Seven coordinator cases and focused strict Clippy pass. Local logs restore-alias-worker.log and restore-alias-clippy.log.
<!-- /run -->
<!-- run -->
### Bookkeeping

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown
```

```shell
pnpm check:progress; pnpm spec:validate; pnpm spec:first
```

**Notes**

Progress and seven validators pass; spec-first requires an explicit architecture ownership confirmation for the added optional native dependency. Added the unchanged-contract statement without altering gates.
<!-- /run -->
<!-- run -->
### Bookkeeping final

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown
```

```shell
pnpm check:progress; pnpm spec:validate; pnpm spec:first
```

**Notes**

Progress179tasks/917records and sevenvalidators/4320chapters/208requirements pass. Final spec-first4protected files/threecanonical chapters passes after explicit architecture confirmation. No cold app/provider/full-suite gate is claimed.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Cold actual-app/provider and full-suite qualification remain deferred.
- Automatic sync/conflicts, historical search, advanced storage/adoption and
  Forgejo remain required.
- Integrate into main only after the implementation stages.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ0.45.1; Git2.55.0; Rust1.99.0; Node26.9.0; cap-fs-ext4.0.3.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Native JJ/Git and receipt formats unchanged.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Reject restore aliases into excluded vault runtime metadata while preserving original native identities and guarded restore receipt semantics. Desktop remote settings and restoration now have focused proof in its own E-REPO-031.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Implement automatic synchronization as the sole active stage.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-LOCAL-007 explicitly requires no-follow directory ancestors in
  preview and apply. JST-JJ-003/005 identity/codecs remain unchanged; owning
  native chapter records this confirmation.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: updated
reason: M34.4 implementation/focused source and desktop proofs are present; mark
  Partial because original cold app/provider gate remains open. M34.5 becomes
  the only active stage; later stages remain required.

```
<!-- /evidence-field -->

