---
schema: 1
id: E-VAULT-BACKUP-021
---

# E-VAULT-BACKUP-021

<!-- evidence-field:id markdown -->
## Id

E-VAULT-BACKUP-021
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M34.3
- M34.7
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-LOCAL-007
- JST-ARCH-005
- JST-GOV-002
- JST-GOV-011
- JST-JJ-003
- JST-JJ-005
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-25
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-jj/src/vault_workspace/restore_io.rs
- scripts/validation/repository.mjs
- scripts/validation/source.mjs
- scripts/validation/pins.mjs
- scripts/validation/runs.mjs
- scripts/validation/lanes.json
- scripts/validation.test.mjs
- crates/jj-store-jj/src/vault_workspace/storage/copy.rs
- crates/jj-store-jj/tests/vault_adoption.rs
- package.json
- spec/src/current/contracts/architecture/23-public-distribution.md
- spec/src/current/contracts/local-jj/32-automatic-vault-backup.md
- spec/src/current/governance/03-bounded-specification-and-evidence.md
- spec/src/current/progress/M34/index.md
- spec/src/current/contracts/jj-integration/28-vault-working-copy.md
- .changeset/linux-vault-directory-sync.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-jj/src/vault_workspace/restore_io.rs#root_file_restore_and_delete_synchronize_the_directory
- crates/jj-store-jj/tests/vault_storage.rs#relocation_preserves_native_history_receipts_and_unsnapshotted_files_across_both_layouts
- crates/jj-store-jj/tests/vault_adoption.rs#adopts_recorded_git_ancestry_without_touching_source_metadata_or_saved_files
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
pnpm validation:rust -- test --locked -p jj-store-jj --features vault-backup --lib root_file_restore_and_delete_synchronize_the_directory
```

**Notes**

Before: 0 passed, 1 failed, 101 filtered. Linux fsync on a search-only directory capability returned EBADF. restore-before.log.
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
pnpm validation:rust -- test --locked -p jj-store-jj --features vault-backup --lib root_file_restore_and_delete_synchronize_the_directory
```

**Notes**

After: 1 passed, 0 failed. Reopen the same capability-relative parent for reading before sync; file guards and atomic replacement remain. restore-after.log.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm validation:rust -- test --locked -p jj-store-jj --features vault-backup
```

**Notes**

Initial widening reached four vault_adoption failures because its fixture hard-coded /opt/homebrew/bin/jj. Using the provisioned PATH executable retains all assertions. crate-tests.log.
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
pnpm validation:rust -- test --locked -p jj-store-jj --features vault-backup --test vault_adoption
```

**Notes**

4 passed, 0 failed on Linux. adoption-after.log.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
pnpm validation:rust -- test --locked -p jj-store-jj --features vault-backup --test vault_storage
```

**Notes**

Exact storage reproduction: 1 passed, 5 failed. Recursive migration copy had the same search-only directory fsync defect. storage-before.log.
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
pnpm validation:rust -- test --locked -p jj-store-jj --features vault-backup --test vault_storage
```

**Notes**

6 passed, 0 failed after synchronizing a readable capability-relative handle. Guards, read-only stage recovery and original history remain. storage-after.log.
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
pnpm validation:rust -- test --locked -p jj-store-jj --features vault-backup
```

**Notes**

Complete owning crate: 379 passed, 0 failed, 13 explicitly ignored cases across 41 test binaries/doc-test summaries. crate-qualified.log. Wider scope follows the shared Docker transport and vault restore/copy surface.
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
pnpm validation:rust -- clippy --locked -p jj-store-jj --features vault-backup --all-targets -- -D warnings
```

**Notes**

Strict owning crate/all-target Clippy passes. crate-clippy.log.
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
node --test scripts/validation.test.mjs scripts/validation-pins.test.mjs
```

**Notes**

11 passed, 0 failed. Includes dirty separate-JJ-workspace source/parent preservation and pinned-source transfer. runner-final.log.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm validation:rust -- build --locked -p jj-store-jj --features vault-backup
```

**Notes**

Complete owning crate build passes. crate-build.log.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
node scripts/check-progress.mjs; JJ_STORE_VALIDATION_RUNTIME=native pnpm spec:first
```

**Notes**

Initial evidence omitted five changed package/spec/ledger paths. Add their explicit owner coverage; no test or product failure is waived.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_VALIDATION_RUNTIME=native pnpm spec:first
```

**Notes**

Mapped gate required the native JJ owning chapter as well as the local-backup topic. Both now record the same unchanged identities, guards and receipts.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
node scripts/check-progress.mjs; JJ_STORE_VALIDATION_RUNTIME=native pnpm spec:first; JJ_STORE_VALIDATION_RUNTIME=native pnpm spec:validate
```

**Notes**

Ledger validates 180 tasks and 960 evidence records. Spec-first passes the changed native/tooling owners; static specification validates all 8 validators, 4451 chapters and 208 requirements. Unrelated navigation rewrites from the scaffold were restored; only the new day/evidence links remain.
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

Linux x86_64 Docker on remote omarchy; Rust 1.99.0; Jujutsu 0.45.1; 4 CPUs, 12 GiB memory, 2 GiB shared memory.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No stored-format or public-contract changes.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Linux regular-file restore/deletion and storage migration synchronize capability-relative readable directories. Separate Jujutsu workspaces retain exact Docker snapshots, parent ancestry and pinned siblings. No stored-format or public-API changes.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Finish source bookkeeping and commit, then refresh the Notes source pin and qualify its exact backup regression and full native lane. Cold application, provider and distribution acceptance remains outside this slice.
<!-- /evidence-field -->

<!-- generated-navigation -->

## Pages

- [E-VAULT-BACKUP-021 details 2](details-02.md)
