---
schema: 1
id: E-VAULT-BACKUP-016
---

# E-VAULT-BACKUP-016

<!-- evidence-field:id markdown -->
## Id

E-VAULT-BACKUP-016
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

- crates/jj-store-jj/src/vault_workspace/history/divergence.rs
- crates/jj-store-jj/src/vault_workspace/history.rs
- crates/jj-store-jj/src/vault_workspace/mod.rs
- crates/jj-store-jj/tests/vault_divergence.rs
- spec/src/current/contracts/local-jj/32-automatic-vault-backup.md
- spec/src/current/progress/M34/index.md
- spec/src/current/contracts/jj-integration/28-vault-working-copy.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-jj/tests/vault_divergence.rs#immutable_divergence_counts_retained_ancestry_without_snapshotting_or_moving_its_pin
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
```

**Notes**

48.3 GiB free before compilation. .local/divergence-disk.log; managed target only.
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
pnpm dev:cargo -- test -p jj-store-jj --no-default-features --features vault-backup --test vault_sync immutable_divergence_counts_retained_ancestry_without_snapshotting_or_moving_its_pin -- --exact
```

**Notes**

Initial new code did not compile: missing RevsetExpression generic. Corrected count input to ResolvedRevsetExpression.
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
pnpm dev:cargo -- test -p jj-store-jj --no-default-features --features vault-backup --test vault_sync immutable_divergence_counts_retained_ancestry_without_snapshotting_or_moving_its_pin -- --exact
```

**Notes**

Exact case passes; immutable pin, missing initialization, 2 incoming/1 outgoing, later local capture, equal anchors and unchanged op-head files. .local/divergence-native-final.log.
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
pnpm dev:cargo -- test -p jj-store-jj --no-default-features --features vault-backup --test vault_sync
```

**Notes**

Two attempts each had one transient Busy publication-lock failure, first in the new case and then in the pre-existing two-device case. .local/divergence-family.log and divergence-family-rerun.log. No product lock or assertion was relaxed.
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
pnpm dev:cargo -- test -p jj-store-jj --no-default-features --features vault-backup --test vault_sync -- --test-threads=1
```

**Notes**

All four cases pass serially. .local/divergence-family-serial.log. The new immutable reader case was then moved to its own responsibility-specific test binary; original vault_sync source is unchanged.
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
pnpm dev:cargo -- test -p jj-store-jj --no-default-features --features vault-backup --test vault_divergence
```

**Notes**

One isolated exact reader case passes, without serial flags. .local/divergence-isolated.log.
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
pnpm dev:cargo -- clippy -p jj-store-jj --no-default-features --features vault-backup --test vault_sync -- -D warnings
pnpm dev:cargo -- clippy -p jj-store-jj --no-default-features --features vault-backup --test vault_divergence -- -D warnings
```

**Notes**

Strict affected source/test Clippy passes before and after test isolation. .local/divergence-clippy.log and divergence-clippy-isolated.log.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
pnpm spec:validate
pnpm spec:first
```

**Notes**

Progress and canonical validation pass. Initial whole-working-copy spec-first also sees four pre-existing unrelated M33/docs edits with missing E-REUSABLE-PROGRESS-050; those are preserved. It additionally caught this draft record using a test name rather than a path and a pass with nonempty remaining. Correct this record and check the isolated file-specific committed revision.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first --file crates/jj-store-jj/src/vault_workspace/history/divergence.rs --file crates/jj-store-jj/src/vault_workspace/history.rs --file crates/jj-store-jj/src/vault_workspace/mod.rs --file crates/jj-store-jj/tests/vault_divergence.rs
```

**Notes**

The explicit file selection omitted its mapped jj-integration chapter. Added the owning vault working-copy integration mapping and include canonical changes in the scoped selection.
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
pnpm check:progress
pnpm spec:validate
pnpm spec:first --file crates/jj-store-jj/src/vault_workspace/history/divergence.rs --file crates/jj-store-jj/src/vault_workspace/history.rs --file crates/jj-store-jj/src/vault_workspace/mod.rs --file crates/jj-store-jj/tests/vault_divergence.rs --file spec/src/current/contracts/jj-integration/28-vault-working-copy.md --file spec/src/current/contracts/local-jj/32-automatic-vault-backup.md
```

**Notes**

Canonical validation and progress pass. Explicit source plus owning-chapter selection passes, excluding pre-existing unrelated M33/docs work. Run revision-pinned full evidence validation after file-specific commit.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first --base @-- --head @-
JJ_STORE_CHECK_REVISION=@- pnpm check:progress
```

**Notes**

Revision-pinned evidence validation correctly requires path#test references; corrected this same unshared source change before completion. Concurrent unrelated M33/docs files remain outside the selected revision.
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

Full revision-pinned source evidence/contract gate and ledger now pass on the file-specific source commit (179 tasks, 943 evidence records). Four pre-existing unrelated files remain dirty and unchanged by this task. Logs divergence-spec-committed-final.log and divergence-progress-committed-final.log.
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

Enclosing Jujutsu change; jj 0.45.1, macOS ARM64. Desktop consumes the native source through its existing sibling Cargo dependency.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive CheckpointDivergence DTO only; native operation/commit identities, repository formats and publication authority unchanged.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Expose bounded immutable incoming/outgoing checkpoint ancestry counts for the Desktop footer. No initialization, mutation lease, disk capture, remote fetch, credentials or head reconciliation. Concurrent Git-child fixture publication remains outside qualified concurrent execution; the serial four-case family and new isolated reader pass. Cold app/provider/full-suite qualification remains deferred under M34.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Consume through Desktop bounded native workers; review footer actions. Keep original M34 cold app/provider qualification separate.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-LOCAL-007 now governs immutable divergence counts, retained anchors,
  one operation pin, two-second shared budget and 100000-per-direction bound.
  Owning automatic-backup chapter updated.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M34 and M34.5 remain Partial. This focused source delivery adds optional
  graph counts; original automatic-sync cold native/provider and broader
  completion gates remain deferred.

```
<!-- /evidence-field -->

