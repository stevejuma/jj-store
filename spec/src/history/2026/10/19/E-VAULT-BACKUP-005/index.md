---
schema: 1
id: E-VAULT-BACKUP-005
---

# E-VAULT-BACKUP-005

<!-- evidence-field:id markdown -->
## Id

E-VAULT-BACKUP-005
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M34
- M34.4
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

- crates/jj-store-jj/Cargo.toml
- crates/jj-store-jj/README.md
- crates/jj-store-jj/src/vault_workspace/capture.rs
- crates/jj-store-jj/src/vault_workspace/content.rs
- crates/jj-store-jj/src/vault_workspace/mod.rs
- crates/jj-store-jj/src/vault_workspace/remote/import.rs
- crates/jj-store-jj/src/vault_workspace/remote/mod.rs
- crates/jj-store-jj/src/vault_workspace/remote/process.rs
- crates/jj-store-jj/src/vault_workspace/remote/publication.rs
- crates/jj-store-jj/tests/vault_remote.rs
- spec/src/current/contracts/jj-integration/28-vault-working-copy.md
- spec/src/current/contracts/local-jj/32-automatic-vault-backup.md
- spec/src/current/progress/M34/index.md
- spec/src/history/2026/10/19/E-VAULT-BACKUP-005/SUMMARY.nav.md
- spec/src/history/2026/10/19/E-VAULT-BACKUP-005/index.md
- spec/src/history/2026/10/19/SUMMARY.nav.md
- spec/src/history/2026/10/19/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-jj/tests/vault_remote.rs#ordinary_git_backup_restores_files_and_native_history_on_a_fresh_device
- crates/jj-store-jj/tests/vault_remote.rs#remote_drift_rejects_publication_and_unknown_intents_are_observed_without_replay
- crates/jj-store-jj/tests/vault_remote.rs#remote_targets_reject_embedded_secrets_and_ref_injection
- crates/jj-store-jj/tests/vault_remote.rs#a_stalled_https_child_does_not_hold_the_native_capture_lease
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
pnpm dev:disk; pnpm dev:clean -- --dry-run
```

**Notes**

Initial available storage28.3GiB was below30GiB reserve. A subsequent dry-run found34.8GiB already free; no cleanup was performed by this slice. Normal managed Cargo preflight then admitted source builds.
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
CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 pnpm dev:cargo -- check -p jj-store-jj --no-default-features --features vault-backup
```

**Notes**

One borrow error: workspace name read during active mutable working-copy lease. Copy the name before leasing.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 pnpm dev:cargo -- test -p jj-store-jj --no-default-features --features vault-backup --test vault_remote
```

**Notes**

One validation case passed; two actual local-Git remote tests exposed pending JJ rewrites after checking out a fresh working-copy child. Drain descendants before publishing the native view. Exact three-case rerun then passed.
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
CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 pnpm dev:cargo -- clippy -p jj-store-jj --no-default-features --features vault-backup --lib --test vault_remote -- -D warnings
```

**Notes**

Two lint findings: identical completion blocks in new publication handling and a pre-existing nested-if in capture. Combine the completion condition and collapse the unchanged capture guard. No semantic or acceptance relaxation.
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
CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 pnpm dev:cargo -- test -p jj-store-jj --no-default-features --features vault-backup --test vault_backup --test vault_restore --test vault_restore_all --test vault_remote
```

**Notes**

Twelve native cases pass:4capture,3file restore,2whole-vault restore and3remote cases. Remote tests use disposable actual bare Git remotes and ordinary Git clones. They prove native commit/change-ID retention, readonly fetch/import, fresh-device guarded restoration, lost-result/reopen and later-edit protection, second-device fast-forward publication, exact remote drift rejection, forged-intent ancestry rejection and an unresolved original push blocking new intents.
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
CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 pnpm dev:cargo -- clippy -p jj-store-jj --no-default-features --features vault-backup --lib --test vault_remote -- -D warnings
```

**Notes**

Focused library and exact remote test Clippy pass with warnings denied. Network calls hold no capture/workspace lease. Secrets remain non-serializable and only reach a scoped child environment, with redirects, hooks, helpers, submodules and ambient Git configuration disabled.
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
CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 pnpm dev:cargo -- test -p jj-store-jj --no-default-features --features vault-backup --test vault_remote; pnpm dev:cargo -- clippy -p jj-store-jj --no-default-features --features vault-backup --lib --test vault_remote -- -D warnings
```

**Notes**

Final four remote cases pass. A real Git HTTPS child is held during TCP/TLS handshake on loopback; another native saved-file capture completes while that child remains pending. Closing the fixture socket then produces the expected failed observation. This tests lease independence with actual pending network work, without personal credentials or external effects. Focused Clippy passes again. Existing nine capture/file/whole-vault regressions passed in the preceding complete affected-surface run.
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
node scripts/check-boundaries.mjs; pnpm check:progress; pnpm spec:validate; pnpm spec:first
```

**Notes**

26package boundaries,179tasks/915evidence records, sevenvalidators/4316chapters/208requirements and spec-first10protected files/twoowning chapters pass. This source-only slice does not qualify the pending desktop remote UI or live GitHub/provider gate.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Desktop remote account/repository/branch configuration and guarded
  fresh-device restore UI remain required.
- Automatic synchronization, conflict handling and richer unknown-push
  observation remain stage5 work. Current unknown reconciliation confirms only
  an exact observed target; otherwise it retains the original intent.
- Historical search, advanced storage/adoption, Forgejo and main integration
  remain required.
- Complete cold actual-WebView, real GitHub/TLS/auth fault and
  full-suite/platform/release acceptance are deferred by user instruction.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ0.45.1; Git2.55.0; Rust1.99.0; Node26.9.0; pnpm10.32.1. Actual macOS ARM64 local filesystem/Git fixture proof.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Native JJ and SHA-1 Git codecs unchanged. Device-local remote-push and remote-restore receipts version1; none is server/WAL/manifest state. Native imported change IDs retain standard Git headers.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Reusable bounded remote transfer and guarded native import/restore/publication. Explicit HTTPS endpoints reject embedded credentials; local-file remotes require a separate native-only constructor. Capture can proceed while the transfer capability lives. Imported heads become explicit native pins; restore retains imported/local safety ancestry. Exact immutable publication targets are revalidated from native retained history even when receipts deserialize. Durable active intent blocks replacement after an unknown external outcome.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Integrate remote account/repository setup and guarded restoration into the desktop settings, then automatic two-way sync. Retain all deferred original gates.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-LOCAL-007 adds explicit native remote transfer, imported-head
  pinning, guarded fresh-device restoration and exact durable publication
  intent. JST-JJ-003/005 retain upstream identities/codecs; immutable reads
  still cannot capture, transfer or publish. The capture conditional change is a
  semantics-preserving Clippy repair.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: updated
reason: M34.4 remains the only active stage. Source remote primitives have
  focused native proof; the original desktop setup/fresh-device restore gate
  remains pending. Accounts are committed in source52b2f24a and desktop867ee73f.
  No main integration, live personal forge effects or release qualification is
  claimed.

```
<!-- /evidence-field -->

