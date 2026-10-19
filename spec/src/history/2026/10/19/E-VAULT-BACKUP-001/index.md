---
schema: 1
id: E-VAULT-BACKUP-001
---

# E-VAULT-BACKUP-001

<!-- evidence-field:id markdown -->
## Id

E-VAULT-BACKUP-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M34
- M34.1
- M34.2
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
- Cargo.toml
- crates/jj-store-jj/Cargo.toml
- crates/jj-store-jj/README.md
- crates/jj-store-jj/src/lib.rs
- crates/jj-store-jj/src/vault_workspace/binding.rs
- crates/jj-store-jj/src/vault_workspace/capture.rs
- crates/jj-store-jj/src/vault_workspace/content.rs
- crates/jj-store-jj/src/vault_workspace/initialize.rs
- crates/jj-store-jj/src/vault_workspace/mod.rs
- crates/jj-store-jj/src/vault_workspace/scan.rs
- crates/jj-store-jj/tests/vault_backup.rs
- crates/jj-vault/Cargo.toml
- crates/jj-vault/README.md
- crates/jj-vault/src/lib.rs
- crates/jj-vault/src/native.rs
- crates/jj-vault/src/schedule.rs
- crates/jj-vault/tests/lifetime.rs
- scripts/check-boundaries.mjs
- spec/src/current/contracts/architecture/26-reusable-agent-workflows.md
- spec/src/current/contracts/jj-integration/28-vault-working-copy.md
- spec/src/current/contracts/jj-integration/SUMMARY.nav.md
- spec/src/current/contracts/jj-integration/index.md
- spec/src/current/contracts/local-jj/02-requirements.md
- spec/src/current/contracts/local-jj/32-automatic-vault-backup.md
- spec/src/current/contracts/local-jj/SUMMARY.nav.md
- spec/src/current/contracts/local-jj/index.md
- spec/src/current/governance/spec-governance.md
- spec/src/current/progress/M34/SUMMARY.nav.md
- spec/src/current/progress/M34/index.md
- spec/src/current/progress/SUMMARY.nav.md
- spec/src/current/progress/index.md
- spec/src/current/verification/SUMMARY.nav.md
- spec/src/current/verification/index.md
- spec/src/current/verification/local-03.md
- spec/src/history/2026/10/19/E-VAULT-BACKUP-001/SUMMARY.nav.md
- spec/src/history/2026/10/19/E-VAULT-BACKUP-001/index.md
- spec/src/history/2026/10/19/SUMMARY.nav.md
- spec/src/history/2026/10/19/index.md
- spec/src/history/2026/10/SUMMARY.nav.md
- spec/src/history/2026/10/index.md
- spec-validator.config.mjs
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-jj/tests/vault_backup.rs#external_capture_retains_edit_bursts_and_never_creates_vault_metadata
- crates/jj-store-jj/tests/vault_backup.rs#captures_nested_repo_contents_but_excludes_its_metadata_and_deleted_files
- crates/jj-store-jj/tests/vault_backup.rs#rejects_concurrent_commands_and_another_vault_binding
- crates/jj-store-jj/tests/vault_backup.rs#relinking_the_same_physical_vault_preserves_history_and_rejects_replacement
- crates/jj-vault/src/schedule.rs#captures_on_open_and_after_quiet_or_maximum_delay
- crates/jj-vault/src/schedule.rs#idle_reconciliation_and_failure_retry_remain_bounded
- crates/jj-vault/tests/lifetime.rs#idle_start_waits_for_watcher_registration_and_explicit_capture
- crates/jj-vault/tests/lifetime.rs#runs_on_worker_and_drains_final_capture_before_close_returns
- crates/jj-vault/tests/lifetime.rs#changes_during_in_flight_capture_schedule_another_capture
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
pnpm dev:cargo -- check -p jj-store-jj --no-default-features --features vault-backup --test vault_backup
```

**Notes**

Disk preflight stopped before compilation:22GiB free versus30GiB reserve. The live jj-store target remains protected.
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
Verified no Cargo/rustc owner; hold lapis-desktop/src-tauri/target/debug/.cargo-lock; remove only its idle incremental directory
```

**Notes**

Recovered14.3GiB:21.9 to36.2GiB free. Existing native bundles, source work, running review app and review vault preserved.
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
CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 pnpm dev:cargo -- check -p jj-vault --tests
```

**Notes**

Initial6missing Repo-trait errors fixed; repeat passes. Source local log .local/vault-backup/native-check-after.log.
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
CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 pnpm dev:cargo -- test -p jj-store-jj --no-default-features --features vault-backup --test vault_backup
```

**Notes**

Initial fixture ID-conversion compile errors retained in native-tests*.log; first runtime1pass/2fail exposed missing native descendant rebase. Corrected transaction rebases descendants before advancing WC. Final4pass (including same-physical-root relocation and replacement rejection): external metadata, retained commit ancestry, no-op capture, ignored attachments, nested repository content/deletion, independent binding and concurrent lease rejection. Logs native-tests-rebase.log and relink-tests.log.
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
CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 pnpm dev:cargo -- test -p jj-vault
```

**Notes**

2 scheduler and 3 actual worker tests pass, including change during capture and final drain/off-thread execution; doc lane0examples. Log coordinator-tests.log.
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
node scripts/check-boundaries.mjs; pnpm --dir packages/jj-history build
```

**Notes**

26package boundaries pass and public JS history dist builds for isolated desktop consumption.
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
pnpm dev:cargo -- check --manifest-path ../lapis-desktop/src-tauri/Cargo.toml --lib --bin lapis-desktop
```

**Notes**

Wrapper correctly rejects a manifest outside its managed family. Consumer guide owns native commands; focused consumer check is running in its separate default target after disk reserve recovery.
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
pnpm check:progress; pnpm spec:validate; pnpm spec:first; cargo fmt --all
```

**Notes**

Final progress179tasks/911records, seven validators4308chapters/208requirements,
spec-first20protected files/fivechapters pass. Initial progress/spec-first used
stale pinned pre-evidence state and rejected pass-with-gaps/missing task evidence;
kept this delivery pending, added M34.2 evidence and snapshotted before recheck.
Latest consumer native tests: four real watch/admission/identity cases pass;
15 renderer and17governance tests pass, strict types zero errors/warnings.
Full suites and cold WebView qualification remain explicitly deferred.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Cold native WebView qualification remains deferred; consumer native library, watch, identity and renderer checks have passed.
- Root storage, restore, remote transport and search remain required delivery work.
- Stages M34.2 through M34.7 remain required; full suites deferred by user.
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

First external-storage and automatic-capture foundation. New reusable crate and native primitives; UI/forge/search follow in the approved plan. Main checkouts and unrelated M33 documentation edits preserved.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Finish lifecycle and crash recovery, then restore/timeline, accounts/sync, search and Forgejo; integrate into main only after implementation completion.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-LOCAL-007 adds the separate host-admitted backup writer;
  JST-ARCH-007 and JST-JJ-003/005 retain owned interfaces and native identities.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M34 remains In progress; M34.1 is Partial with cold app qualification deferred and M34.2 becomes the sole active stage. M33 and all original
  native/conversation/release gates remain unchanged; no compiled or
  screenshot-only completion claim.

```
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


