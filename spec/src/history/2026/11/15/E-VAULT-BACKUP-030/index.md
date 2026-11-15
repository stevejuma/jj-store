---
schema: 1
id: E-VAULT-BACKUP-030
---

# E-VAULT-BACKUP-030

<!-- evidence-field:id markdown -->
## Id

E-VAULT-BACKUP-030
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-11-15
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M34.5
- M34.9
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-LOCAL-007
- JST-LOCAL-008
- JST-JJ-003
- JST-JJ-005
- JST-DOC-002
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

implementation
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-jj/src/vault_workspace/history.rs
- crates/jj-store-jj/src/vault_workspace/history/publication.rs
- crates/jj-store-jj/src/vault_workspace/remote/mod.rs
- crates/jj-store-jj/src/vault_workspace/remote/publication.rs
- crates/jj-store-jj/src/vault_workspace/remote/publication/prepare.rs
- crates/jj-store-jj/src/vault_workspace/remote/publication/selected.rs
- crates/jj-store-jj/tests/vault_publication.rs
- docs/site/technical/local.md
- docs/maintenance/coverage.json
- spec/src/current/contracts/jj-integration/28-vault-working-copy.md
- spec/src/current/contracts/local-jj/32-automatic-vault-backup.md
- spec/src/current/contracts/documentation/13-native-remotes-source-qualification.md
- spec/src/current/progress/M34/index.md
- spec/src/current/verification/local-03.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-jj/tests/vault_publication.rs#initialized_transfer_does_not_take_the_capture_lease
- crates/jj-store-jj/tests/vault_publication.rs#retained_publication_does_not_take_the_capture_lease
- crates/jj-store-jj/tests/vault_remote.rs#a_stalled_https_child_does_not_hold_the_native_capture_lease
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
node .local/native-remotes-validation.mjs test --locked -p jj-store-jj --no-default-features --features vault-backup --test vault_publication
```

**Notes**

Baseline: initialized connect and retained publish both return Busy while a real Session holds the capture lease. Existing stop/recovery cases pass.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 9
hasNote: true
noteEncoding: markdown

```

```shell
node .local/native-remotes-validation.mjs test --locked -p jj-store-jj --no-default-features --features vault-backup --test vault_publication
```

**Notes**

Initial extraction exposed nine Rust private-field/method and cascading type errors. Immutable publication methods now live inside the owning HistoryReader module and delegate to shared native-private validation; private fields were not exposed.
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
node .local/native-remotes-validation.mjs test --locked -p jj-store-jj --no-default-features --features vault-backup --test vault_publication
```

**Notes**

All four exact cases pass after correction. Later strengthening additionally publishes a pinned review after a subsequent capture and verifies unchanged counts/commit and real remote exclusion of unsnapshotted files.
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
node .local/native-remotes-validation.mjs test --locked -p jj-store-jj --no-default-features --features vault-backup --test vault_publication --test vault_counts --test vault_divergence --test vault_remote --test vault_original_remotes --lib
```

**Notes**

Complete affected native library/transport impact: 77 passed, zero failed, five existing opt-in ignores. Native Session callers preserve compatibility through the same shared validation helpers.
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
node .local/native-remotes-validation.mjs --https
```

**Notes**

Real TLS smart-HTTP source acceptance passes 1/1 across backup and original token/anonymous transport, replacement and redirect checks. Trust changes are confined to the disposable Linux container.
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
node .local/native-remotes-validation.mjs --ssh
```

**Notes**

Real original SSH acceptance passes 2/2, including tracking fetch, one reviewed bookmark, rejected divergence and lost-response observation without replay.
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
node .local/native-remotes-validation.mjs --ssh-backup
```

**Notes**

Real backup SSH acceptance passes 1/1 with wrong-host-key refusal and lost-response recovery. Disposable keys, agent and endpoint are retired.
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
node .local/native-remotes-validation.mjs clippy --locked -p jj-store-jj --no-default-features --features native,vault-backup --all-targets -- -D warnings
```

**Notes**

Strict all-target Clippy passes for the complete native/vault-backup source profile.
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
node .local/native-remotes-validation.mjs build --locked -p jj-store-jj --no-default-features --features native,vault-backup
```

**Notes**

The affected native SDK builds successfully. The public Session methods remain compatible.
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
JJ_STORE_VALIDATION_RUNTIME=native JJ_STORE_DOC_REFERENCE=$PWD/.local/remotes-qualification/docs_reference pnpm docs:check
```

**Notes**

Unchanged native CLI-schema helper, reviewed source fingerprints, dual-theme Mermaid rendering and strict complete documentation/link closure pass.
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
JJ_STORE_VALIDATION_RUNTIME=native pnpm test:governance; cargo fmt --all -- --check
```

**Notes**

121 governance cases and Rustfmt pass. No UI implementation or visual baseline changes.
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
JJ_STORE_VALIDATION_RUNTIME=native pnpm check:progress; JJ_STORE_VALIDATION_RUNTIME=native pnpm spec:validate; JJ_STORE_VALIDATION_RUNTIME=native pnpm spec:first
```

**Notes**

Progress, canonical specification and spec-first pass. Unrelated UI and whole-workspace product lanes are outside the owning Rust-only targeted policy; hosted required CI and later consumer intake remain separate gates.
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

Rust 1.99.0; pinned JJ library 0.46.0; fixture JJ 0.45.1; Node 24.10.0; spec-validator 0.2.0. Remote image sha256:7d3aa64f81dce0988978ac830b4cc53ff5713d3acbf19163bd4987ff3e858efe with exact tracked archives, 2 CPUs/8 GiB and the existing workspace-isolated cache.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive HistoryReader prepare_push/push_divergence methods. Session compatibility methods, PushIntent/receipt persistence and network DTO formats are unchanged.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

An initialized transfer opens the existing Git backend under the independent transport fence without a capture Session. First absent-binding initialization still uses the explicit native writer; partial existing state refuses. Backup review/counts pin immutable native state and publication revalidates retained ancestry without the capture lease. Original publication retains its separate MutationAccess and exact source checks. No consumer Busy retry or scanner copy is introduced.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Commit the qualified source slice, verify hosted CI and merge while preserving the checked SDK commit; then intake the immutable review path into shared backup orchestration. Both application hosts and the whole remotes release remain incomplete.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: Clarify JST-LOCAL-007/008 and JST-JJ-003/005 capture/network separation
  for initialized connect and retained validation; add an explicit immutable
  native backup review entry point without changing Session compatibility or
  receipt identities.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M34.5 remains Partial and M34.9 remains In progress. Native/real
  transport qualification cannot substitute for shared orchestration,
  credentials, both host adapters and cold application delivery.

```
<!-- /evidence-field -->

