---
schema: 1
id: E-VAULT-BACKUP-018
---

# E-VAULT-BACKUP-018

<!-- evidence-field:id markdown -->
## Id

E-VAULT-BACKUP-018
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M34.2
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-LOCAL-007
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-20
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-vault/README.md
- crates/jj-vault/src/lib.rs
- crates/jj-vault/src/lifetime.rs
- crates/jj-vault/tests/shutdown.rs
- spec/src/current/contracts/local-jj/32-automatic-vault-backup.md
- spec/src/current/progress/M34/index.md
- spec/src/history/2026/10/20/E-VAULT-BACKUP-018/index.md
- spec/src/history/2026/10/20/E-VAULT-BACKUP-018/SUMMARY.nav.md
- spec/src/history/2026/10/20/index.md
- spec/src/history/2026/10/20/SUMMARY.nav.md
- spec/src/history/2026/10/index.md
- spec/src/history/2026/10/SUMMARY.nav.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-vault/tests/shutdown.rs#consuming_close_retires_the_worker_without_replacing_failure_with_closed
- crates/jj-vault/tests/shutdown.rs#dropping_the_owner_retires_the_worker_despite_retained_change_handles
- crates/jj-vault/tests/shutdown.rs#failed_close_keeps_the_original_worker_and_retry_captures_repaired_files
- crates/jj-vault/tests/shutdown.rs#dropping_after_failed_retryable_close_retires_the_retained_worker
- crates/jj-vault/tests/shutdown.rs#full_queue_preserves_close_barrier_without_blocking_the_async_runtime
- crates/jj-vault/tests/shutdown.rs#cancelled_close_wait_can_retry_the_already_queued_barrier
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
pnpm dev:disk
pnpm dev:clean -- --dry-run
pnpm dev:clean -- --apply
```

**Notes**

Initial 25.3 GiB free; managed maintenance cleans only idle first-party target outputs and recovers 30.5 GiB. Running Desktop targets, development data and fixture roots are preserved.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown
```

```shell
pnpm dev:cargo -- test -p jj-vault --test shutdown
```

**Notes**

Before source changes, 0/2 pass: failed close reports Closed instead of Failed; dropping the owner leaves its worker alive through notification handles. The command also reports 29.9 GiB after compilation, below the 30 GiB reserve; no subsequent build bypasses the preflight.
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
pnpm dev:cargo -- clean
```

**Notes**

Managed Cargo rejects the unsupported clean command before any Cargo action. Use the existing locked storage-policy cleanTarget helper for complete idle-cache cleanup instead.
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
node --input-type=module -e 'import {storageContext,snapshot} from "./scripts/build-storage-inventory.mjs"; import {active,cleanTarget} from "./scripts/build-storage-policy.mjs"; const context=await storageContext(process.cwd()); const entry=(await snapshot(context)).find(e=>e.path===process.cwd()+"/target"); const busy=await active(context,entry); if(busy.length) throw Error(busy.join("; ")); console.log(await cleanTarget(context,entry,"all"));'
```

**Notes**

Idle target active-check and exclusive POSIX lock protect complete cleanup: 3,414,368,256 bytes to 4,096 bytes. Available storage returns to 33 GiB. Lower-debug nonincremental focused compilation leaves 32 GiB; required reserve is retained.
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
pnpm dev:cargo -- fmt --package jj-vault
CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 pnpm dev:cargo -- test -p jj-vault --test shutdown
```

**Notes**

source-after.log: all six regressions pass. Covers failure retention/repair retry, consuming failure cleanup, ordinary/failed-close abandonment, saturated queue barrier and cancelled-await recovery. No UI thread blocks on send or join.
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
CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 pnpm dev:cargo -- test -p jj-vault
```

**Notes**

package-tests.log: 20/20 across scheduler, lifetime/shutdown, native adoption/remote/restore and search direct consumers pass; zero documentation examples. This is the affected package, not the repository-wide Rust suite.
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
CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 pnpm dev:cargo -- clippy -p jj-vault --all-targets -- -D warnings
```

**Notes**

clippy.log reports one collapsible-if diagnostic at the join guard (reported for lib and lib-test). Collapsed the condition without changing lifecycle behavior or assertions.
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
CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 pnpm dev:cargo -- clippy -p jj-vault --all-targets -- -D warnings
```

**Notes**

clippy-final.log: strict affected-package all-target lint passes. Boundary check also passes all 26 packages. Canonical validation passes seven validators, 4379 chapters and 208 requirements; exact spec-first selection passes four protected files and one owning chapter. Final committed progress checks follow.
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
CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 pnpm dev:cargo -- test -p jj-vault --test shutdown --test lifetime
```

**Notes**

lifecycle-final.log: final source after lint-only condition collapse passes all nine direct lifecycle tests. Boundary and canonical checks pass independently; whole repository Rust/UI suites are out of scope.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 6
hasNote: true
noteEncoding: markdown
```

```shell
pnpm check:progress
```

**Notes**

progress-before.log reports only the four preserved unrelated M33/documentation paths lacking fresh coverage and two M33 references to missing E-REUSABLE-PROGRESS-050. This slice neither edits nor includes those paths. File-specific committed revision checks follow.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown
```

```shell
pnpm exec tauri build --debug --bundles app --features e2e --config src-tauri/tauri.e2e.conf.json --config '{"build":{"beforeBuildCommand":null}}'
LAPIS_DESKTOP_ACCEPTANCE_ROOT=/Users/stevejuma/code/lapismd/lapis-desktop/.local/verification/native-backup node --test --test-concurrency=1 --test-name-pattern='failed capture' tests/desktop/repository-backup-recovery.test.mjs
LAPIS_DESKTOP_ACCEPTANCE_ROOT=/Users/stevejuma/code/lapismd/lapis-desktop/.local/verification/native-backup node --test --test-concurrency=1 tests/desktop/repository-backup-recovery.test.mjs
```

**Notes**

Commands run in sibling lapis-desktop. Native Acceptance build passes with the retained nonfatal compact-unwind advisory. Original close-retry regression passes 1/1; combined actual native crash/capture/final-close suite passes 2/2. Two unrepaired close attempts reject, then repaired bytes and previous version are retained. Desktop E-REPO-048 records consumer evidence. Running review app, Vite and data are preserved.
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
pnpm spec:first --base @-- --head @-
JJ_STORE_CHECK_REVISION=@- pnpm check:progress
```

**Notes**

The file-specific source commit passes exact spec-first and progress (179 tasks,
945 evidence records). Its parent working copy retains only the four unrelated
M33/documentation paths. Their missing record remains outside this slice.
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

Enclosing Jujutsu change; macOS ARM64, Rust 1.99.0, upstream JJ 0.45.1, Node 26.9.0 and pnpm 10.34.5.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive borrowed Backup::try_close API; consuming close remains source-compatible. No persistence, native identity, checkpoint, operation or receipt format changes.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

JST-LOCAL-007 final-capture lifecycle only. A failed borrowed close retains the worker and observable failure for explicit retry; success joins off-thread and remains idempotent after cancellation. Abandonment wakes and retires the worker even with retained notification senders, without claiming capture success. No server, network, storage codec or UI authority changes. Four pre-existing M33/documentation paths remain excluded. Root aggregate, UI/platform/release suites are out of scope under targeted verification.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Finish ordinary-App backup editor-save/Quit acceptance after guarded package refresh. Retain all original M34 lifecycle, restoration/search and real-provider gates; no release work is started.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-LOCAL-007 now explicitly requires retryable final capture, original
  owner retention, idempotent success and worker retirement on abandonment.
  Native identities and persistence/publication authority are unchanged.
```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M34.2 and M34 remain Partial at their original complete cold native
  lifecycle/provider gates. Focused source correctness and consuming Desktop
  acceptance are recorded independently; no original gate is weakened.
```
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


