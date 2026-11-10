---
schema: 1
id: E-VAULT-BACKUP-025
---

# E-VAULT-BACKUP-025

<!-- evidence-field:id markdown -->
## Id

E-VAULT-BACKUP-025
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M34.2
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-LOCAL-007
- JST-DOC-002
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-11-10
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-jj/src/vault_workspace/binding.rs
- crates/jj-store-jj/src/vault_workspace/mod.rs
- crates/jj-store-jj/tests/vault_backup.rs
- docs/maintenance/coverage.json
- spec/src/current/contracts/jj-integration/28-vault-working-copy.md
- spec/src/current/contracts/local-jj/32-automatic-vault-backup.md
- spec/src/current/contracts/documentation/03-delivery-and-validation.md
- spec/src/current/progress/M34/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-jj/tests/vault_backup.rs#released_session_and_relink_leases_do_not_block_the_next_command
- crates/jj-store-jj/tests/vault_backup.rs#rejects_concurrent_commands_and_another_vault_binding
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
cargo test --locked -p jj-store-jj --features vault-backup --test vault_backup released_session_and_relink_leases_do_not_block_the_next_command -- --exact
```

**Notes**

The exact source regression passed 1/1 on the local Linux machine. Its 512
session-drop/relink cycles retain the original same-binding command boundary.
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
cargo test --locked -p jj-store-jj --features vault-backup --test vault_backup
```

**Notes**

The directly affected vault-backup binary passed 6/6, including the existing
proof that a genuinely concurrent same-binding command is rejected.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown
```

```shell
cargo check --locked -p jj-store-jj --features vault-backup --all-targets
cargo clippy --locked -p jj-store-jj --features vault-backup --all-targets -- -D warnings
cargo fmt --all -- --check
```
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
pnpm spec:search 'automatic vault backup command lease unlock'
pnpm exec spec-validator doctor
node scripts/spec/progress-validation.mjs
pnpm spec:validate
pnpm spec:first --base a630e97f23a78572b3cc11619e1540f5fabf2ad8 --head @
```

**Notes**

QMD found both owning chapters; doctor passed with its pre-existing script-alias
warning. Progress, all eight specification validators across 4,629 chapters and
211 requirements, and the mapped spec-first gate all passed. The first broader
check attempt correctly stopped on the stale `local` documentation fingerprint;
the three mapped pages were reviewed and their unchanged public guidance was
recorded before this final pass.
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

Rust 1.99.0, Cargo 1.99.0 and jj-lib 0.46.0 on local Linux.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No stored format, binding identity, repository identity or receipt change.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

The per-binding native command lock is now an owned guard that explicitly
unlocks on drop for both Session and Binding::relink. Sequential commands no
longer inherit a stale Busy result, while an overlapping command remains Busy.
No retry, timeout, admission or assertion was weakened.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Publish the owning fix, refresh lapis-server and prove the original Desktop
repository_original boundary before rerunning a complete hosted workflow. The
macOS ARM64 Desktop quarantine remains until its original platform evidence
passes.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-LOCAL-007 now makes command-lease release explicit. The lock still
  rejects real overlap; the change prevents only a completed session or relink
  from spuriously blocking the next command.
```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M34.2 remains Partial at its existing cold native application lifecycle
  gate. Focused source and local Desktop evidence do not complete cross-platform
  or hosted qualification.
```
<!-- /evidence-field -->
