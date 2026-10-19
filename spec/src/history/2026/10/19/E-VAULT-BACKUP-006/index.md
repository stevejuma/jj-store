---
schema: 1
id: E-VAULT-BACKUP-006
---

# E-VAULT-BACKUP-006

<!-- evidence-field:id markdown -->
## Id

E-VAULT-BACKUP-006
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M34
- M34.4
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-ARCH-007
- JST-LOCAL-007
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

- crates/jj-vault/Cargo.toml
- crates/jj-vault/README.md
- crates/jj-vault/src/lib.rs
- crates/jj-vault/src/native.rs
- crates/jj-vault/src/remote.rs
- crates/jj-vault/tests/native_remote.rs
- spec/src/current/contracts/architecture/26-reusable-agent-workflows.md
- spec/src/current/progress/M34/index.md
- spec/src/history/2026/10/19/E-VAULT-BACKUP-006/SUMMARY.nav.md
- spec/src/history/2026/10/19/E-VAULT-BACKUP-006/index.md
- spec/src/history/2026/10/19/SUMMARY.nav.md
- spec/src/history/2026/10/19/index.md
- spec/src/current/contracts/local-jj/32-automatic-vault-backup.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-vault/tests/native_remote.rs#native_remote_import_and_restore_share_the_serialized_backup_worker
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
CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 pnpm dev:cargo -- test -p jj-vault
```

**Notes**

Seven cases pass: two scheduler, three lifecycle, one native local restore and one native remote import/restore case. The actual native worker serializes an opaque fetched proof with capture, leaves files untouched during import, guards restoration, publishes local checkpoint status and preserves later edits when replaying an original request.
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
CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 pnpm dev:cargo -- clippy -p jj-vault --all-targets -- -D warnings
```

**Notes**

The new opaque fetched-head command enlarged the channel error type. Strict Clippy flagged the close-barrier spawn closure returning that large error. Map it to the existing small owned message within the closure; no queue/barrier timing or error handling weakened.
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
CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 pnpm dev:cargo -- test -p jj-vault; pnpm dev:cargo -- clippy -p jj-vault --all-targets -- -D warnings
```

**Notes**

Final complete seven-case coordinator suite and all-targets Clippy pass. The backup worker performs native import/restore only; network transfer remains host-owned and outside the capture command lease.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
node scripts/check-boundaries.mjs; pnpm check:progress; pnpm spec:validate; pnpm spec:first
```

**Notes**

26package boundaries,179tasks/916records and sevenvalidators/4318chapters/208requirements passed. Spec-first required the local-vault owning chapter in addition to the architecture contract; added the explicit worker/import/status statement under JST-LOCAL-007.
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

26package boundaries,179tasks/916evidence records, sevenvalidators/4318chapters/208requirements and spec-first6protected files/twoowning canonical chapters pass. Source worker implementation is qualified at its focused boundary; desktop setup and original native app/provider gates remain open.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Desktop remote account/repository configuration, restoration controls and
  actual-WebView acceptance remain required.
- Automatic sync/conflicts, historical search, advanced storage/adoption,
  Forgejo and main integration remain required.
- Full repository/provider/platform/release qualification remains deferred by
  explicit user instruction.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ0.45.1; Git2.55.0; Rust1.99.0; Node26.9.0; pnpm10.32.1; source remote primitives26c4c417.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Native JJ/Git, capture status and restore request/receipt codecs unchanged. Additive owned worker methods accept opaque fetched proof and existing guarded whole-vault request.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Add injectable Repository import_remote/restore_remote methods and Backup queue dispatch. NativeRepository reopens/revalidates the admitted binding on its single worker. Import does not masquerade as local capture progress; remote restoration updates the ordinary resulting local checkpoint status. No upstream jj-lib imports enter the coordinator or desktop host.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Wire admitted desktop remote setup, fetch and guarded restore into backup settings using this worker boundary.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-ARCH-007/JST-LOCAL-007 explicitly govern the additive injected
  remote import/restore boundary. Existing scheduling, coalescing, final-capture
  barriers, immutable readers and native identity contracts remain unchanged.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: updated
reason: M34.4 stays the only active stage. Source remote transfer and serialized
  native worker now have focused proof, but desktop setup/fresh-device
  restoration and original native app gate remain pending. No milestone
  completion or main integration is claimed.

```
<!-- /evidence-field -->

