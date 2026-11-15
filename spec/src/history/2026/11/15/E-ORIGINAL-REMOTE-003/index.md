---
schema: 1
id: E-ORIGINAL-REMOTE-003
---

# E-ORIGINAL-REMOTE-003

<!-- evidence-field:id markdown -->
## Id

E-ORIGINAL-REMOTE-003
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-11-15
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M34.9
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-LOCAL-007
- JST-LOCAL-008
- JST-DOC-002
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

implementation
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-jj/src/original_remotes/counts.rs
- crates/jj-store-jj/src/original_remotes/mod.rs
- crates/jj-store-jj/src/original_remotes/push.rs
- crates/jj-store-jj/src/vault_workspace/history/divergence.rs
- crates/jj-store-jj/src/vault_workspace/history.rs
- crates/jj-store-jj/src/vault_workspace/mod.rs
- crates/jj-store-jj/src/vault_workspace/remote/publication/selected.rs
- crates/jj-store-jj/tests/vault_counts.rs
- crates/jj-store-jj/tests/vault_https_transport.rs
- crates/jj-store-jj/tests/vault_original_remotes.rs
- docs/maintenance/coverage.json
- docs/site/technical/local.md
- spec/src/current/contracts/documentation/03-delivery-and-validation.md
- spec/src/current/contracts/documentation/13-native-remotes-source-qualification.md
- spec/src/current/contracts/documentation/SUMMARY.nav.md
- spec/src/current/contracts/documentation/index.md
- spec/src/current/contracts/jj-integration/28-vault-working-copy.md
- spec/src/current/contracts/local-jj/33-original-repository-remotes.md
- spec/src/current/progress/M34/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-jj/tests/vault_counts.rs#empty_remote_counts_exclude_the_native_root_and_never_capture_unsaved_files
- crates/jj-store-jj/tests/vault_counts.rs#push_counts_keep_the_reviewed_commit_when_a_later_capture_advances
- crates/jj-store-jj/tests/vault_divergence.rs#immutable_divergence_counts_retained_ancestry_without_snapshotting_or_moving_its_pin
- crates/jj-store-jj/tests/vault_original_remotes.rs#original_fetch_moves_tracked_bookmarks_without_capturing_or_writing_files
- crates/jj-store-jj/tests/vault_https_transport.rs#real_https_tokens_anonymous_reads_and_original_publication_preserve_scope
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm validation:rust -- test --locked -p jj-store-jj --no-default-features --features vault-backup --test vault_counts
```

**Notes**

Before the source addition, the exact new counter target reports four E0599 missing-method errors for checkpoint_divergence.
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
pnpm validation:rust -- test --locked -p jj-store-jj --no-default-features --features vault-backup --test vault_counts --test vault_divergence --lib
```

**Notes**

Initial focused boundary passes 62 library, one empty-branch and one existing divergence regression; three existing opt-in library cases remain ignored.
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
pnpm validation:rust -- test --locked -p jj-store-jj --no-default-features --features vault-backup --test vault_counts --test vault_divergence --test vault_original_remotes --test vault_publication --lib
```

**Notes**

Final impact surface passes 71 cases: 62 library, 2 empty/review counts, 1 retained divergence, 4 original admission/save and 2 publication cases; five opt-in cases remain separately inventoried.
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
pnpm test:vault-ssh -- --original; pnpm test:vault-https
```

**Notes**

Both real original SSH cases and the real TLS Git case pass with native count assertions. Empty original destination reports outgoing one; tracked fetch reports zero/zero; HTTPS review pins outgoing one and the exact observed remote SHA.
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
pnpm validation:rust -- clippy --locked -p jj-store-jj --no-default-features --features vault-backup --lib --test vault_counts --test vault_divergence --test vault_original_remotes --test vault_https_transport -- -D warnings
```

**Notes**

Strict affected-library and public source test lint passes.
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
JJ_STORE_VALIDATION_RUNTIME=native pnpm test:governance
JJ_STORE_VALIDATION_RUNTIME=native JJ_STORE_DOC_REFERENCE=.local/remotes-qualification/docs_reference pnpm docs:check
JJ_STORE_VALIDATION_RUNTIME=native pnpm check:progress
JJ_STORE_VALIDATION_RUNTIME=native pnpm spec:validate
JJ_STORE_VALIDATION_RUNTIME=native pnpm spec:first
cargo fmt --all --check
pnpm exec prettier --check <all changed Markdown and JSON paths>
pnpm exec spec-validator doctor
pnpm spec:search "JST-LOCAL-008"
```

**Notes**

All 121 governance cases pass, including source feature audits. Documentation
closure validates 144 files and 9 Mermaid diagrams. Progress validates 187 tasks
and 1,053 records; eight specification validators pass 4,664 chapters and 212
requirements. Spec-first accepts 12 protected files and four canonical chapters.
Rust and changed-document formatting pass. Doctor reports ten passes and one
existing script-alias warning; QMD opens and returns the owning contract.
The initial search attempt with a literal `--` reports unsupported syntax; the
corrected positional query passes. No product acceptance is claimed.
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

Rust/Cargo 1.99.0; native JJ 0.46.0/gix 0.88.0; Docker fixture JJ 0.45.1 and Git 2.55.0; root Deno-free spec-validator 0.2.0.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive CommitDivergence and private original review count metadata; existing retained CheckpointDivergence and durable PushIntent/receipt formats remain unchanged.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Native graph reads only. Known empty remote excludes the synthetic root; missing local checkpoint stays unknown. Original metadata reads and exact retained review counts share the existing two-second / 100,000-per-direction graph budget. No snapshot, transfer or application pin movement.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Commit this verified source slice separately and verify hosted PR #80 before Core/host adoption. Cold application acceptance remains separate.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: Add native empty-branch and exact-review count access required by the
  approved shared status/log/settings contract. Existing retained-checkpoint API
  remains compatible.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M34.9 remains In progress at shared Core/Protocol orchestration, host
  adapters and both cold applications.

```
<!-- /evidence-field -->

