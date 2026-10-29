---
schema: 1
id: E-VAULT-BACKUP-013
---

# E-VAULT-BACKUP-013

<!-- evidence-field:id markdown -->
## Id

E-VAULT-BACKUP-013
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

- crates/jj-store-jj/README.md
- crates/jj-store-jj/src/vault_workspace/adoption/mod.rs
- crates/jj-store-jj/src/vault_workspace/adoption/publish.rs
- crates/jj-store-jj/src/vault_workspace/adoption/source.rs
- crates/jj-store-jj/src/vault_workspace/capture.rs
- crates/jj-store-jj/src/vault_workspace/mod.rs
- crates/jj-store-jj/src/vault_workspace/remote/adoption.rs
- crates/jj-store-jj/src/vault_workspace/remote/mod.rs
- crates/jj-store-jj/tests/vault_adoption.rs
- crates/jj-vault/README.md
- crates/jj-vault/src/adoption.rs
- crates/jj-vault/src/lib.rs
- crates/jj-vault/src/native.rs
- crates/jj-vault/tests/native_adoption.rs
- spec/src/current/contracts/jj-integration/28-vault-working-copy.md
- spec/src/current/contracts/local-jj/32-automatic-vault-backup.md
- spec/src/current/progress/M34/index.md
- spec/src/history/2026/10/19/E-VAULT-BACKUP-013/SUMMARY.nav.md
- spec/src/history/2026/10/19/E-VAULT-BACKUP-013/index.md
- spec/src/history/2026/10/19/SUMMARY.nav.md
- spec/src/history/2026/10/19/index.md
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
result: fail
failures: 1
hasNote: true
noteEncoding: markdown
```

```shell
CARGO_INCREMENTAL=0 pnpm dev:cargo -- test -p jj-store-jj --no-default-features --features vault-backup --test vault_adoption --test vault_backup --test vault_storage
```

**Notes**

Initial functional14cases passed, but the managed command failed its independent post-build30GiB reserve at29.8GiB. This is not a successful command. .local/vault-backup/adoption-native.log.
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
pnpm dev:disk
```

**Notes**

Guarded build-storage active/cleanTarget helpers proved the isolated desktop target idle and cleaned only its cache:4632268800 to4096bytes. No protected running target, compiler worker, review app, source or vault data was touched. The older macro-worker cleanup request was not exercised.
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
CARGO_INCREMENTAL=0 pnpm dev:cargo -- test -p jj-store-jj --no-default-features --features vault-backup --test vault_adoption --test vault_backup --test vault_storage
```

**Notes**

All14cases and the managed reserve check pass after final stop-before-delivery fencing. Proof includes both Git layouts, original repository byte preservation, pinned ancestry, current disk tree preservation, original request replay, prepared/published interruption, invalid source pins and storage/capture regressions. adoption-native-final.log.
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
CARGO_INCREMENTAL=0 pnpm dev:cargo -- test -p jj-vault
```

**Notes**

All10tests pass, including actual worker adoption and receipt replay retaining current status, scheduling/lifetime, native restore/remote and search. adoption-worker.log.
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
CARGO_INCREMENTAL=0 pnpm dev:cargo -- clippy -p jj-store-jj --no-default-features --features vault-backup --lib -- -D warnings
CARGO_INCREMENTAL=0 pnpm dev:cargo -- clippy -p jj-vault --all-targets -- -D warnings
```

**Notes**

Both affected strict Clippy commands pass. adoption-jj-clippy.log and adoption-worker-clippy.log.
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
pnpm dev:cargo -- fmt --all
```

**Notes**

Rust formatting completed. Source evidence generation was restricted back to owning topics/M34/new dated evidence; unrelated generated navigation churn was discarded before checks.
<!-- /run -->
<!-- run -->
### Run 7

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

Final package boundaries, progress, canonical validation and spec-first pass.
The native source and worker command results above remain distinct from desktop
native/catalog qualification. Full suites, live provider, cold app and release
lanes are deferred by the user, not replaced by this focused evidence.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Complete desktop native source scopes, adoption controls and source-specific tabs.
- Cold actual-app lifecycle, live forge, full suites and platform/release qualification remain deferred.
- Integrate verified delivery into main workspaces after implementation and focused native acceptance.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ0.45.1; Git2.55.0; rustc1.99.0; source parentcea94fb2; desktop parentcc811c94; enclosing Jujutsu change owns this evidence.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Private adoption receipt v1, active request pointer and terminal stop-before-delivery marker; immutable candidate IDs retain upstream native codecs. Standard Git import retains recorded Git ancestry; original native operation/evolution metadata stays in its original repository.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Explicit bounded local ancestry import into independent managed backup. The source operation and working revision are pinned and revalidated; source heads/configuration and vault files remain unchanged. A retained join keeps the captured managed tree. The injectable jj-vault coordinator serializes import/stop with capture and updates current status after receipt replay. JST-JJ-003/005 identities, codecs and immutable reads are unchanged; JST-LOCAL-007 adds explicit adoption.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Complete desktop source/adoption focused proof, then actual native app acceptance and main integration. M34 remains in progress.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-LOCAL-007 adds explicit recoverable ancestry adoption. JST-JJ-003/005 native identity and immutable-reader contracts remain unchanged as confirmed in 28-vault-working-copy.md.
```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M34.7 remains the only active stage. Source adoption passes focused native and worker checks. Desktop native acceptance, cold provider gates and main integration remain open; earlier stages stay Partial.
```
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


