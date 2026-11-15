---
schema: 1
id: E-ORIGINAL-REMOTE-002
---

# E-ORIGINAL-REMOTE-002

<!-- evidence-field:id markdown -->
## Id

E-ORIGINAL-REMOTE-002
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
- JST-ARCH-001
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-jj/tests/vault_https_transport.rs
- crates/jj-store-jj/tests/vault_original_remotes.rs
- docs/site/technical/local.md
- package.json
- scripts/test-vault-https.mjs
- scripts/test-vault-ssh.mjs
- scripts/validation/lanes.json
- scripts/validation.test.mjs
- scripts/vault-https-fixture.mjs
- spec/src/current/contracts/architecture/23-public-distribution.md
- spec/src/current/contracts/documentation/03-delivery-and-validation.md
- spec/src/current/contracts/jj-integration/28-vault-working-copy.md
- spec/src/current/contracts/local-jj/33-original-repository-remotes.md
- spec/src/current/progress/M34/index.md
<!-- /evidence-field -->

<!-- evidence-field:removed_files list -->
## Removed files

- crates/jj-store-jj/tests/original_remotes.rs
<!-- /evidence-field -->

<!-- evidence-field:removed_tests list -->
## Removed tests

- crates/jj-store-jj/tests/original_remotes.rs#discovery_is_read_only_and_never_returns_embedded_credentials
- crates/jj-store-jj/tests/original_remotes.rs#mutation_admission_and_backup_cache_isolation_refuse_before_network
- crates/jj-store-jj/tests/original_remotes.rs#an_externally_changed_endpoint_refuses_the_stale_transfer
- crates/jj-store-jj/tests/original_remotes.rs#explicit_remote_save_preserves_origin_and_rejects_stale_configuration_without_transfer
- crates/jj-store-jj/tests/original_remotes.rs#original_fetch_moves_tracked_bookmarks_without_capturing_or_writing_files
- crates/jj-store-jj/tests/original_remotes.rs#original_push_publishes_one_reviewed_bookmark_and_recovers_without_replay
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-jj/tests/vault_https_transport.rs#real_https_tokens_anonymous_reads_and_original_publication_preserve_scope
- crates/jj-store-jj/tests/vault_original_remotes.rs#discovery_is_read_only_and_never_returns_embedded_credentials
- crates/jj-store-jj/tests/vault_original_remotes.rs#mutation_admission_and_backup_cache_isolation_refuse_before_network
- crates/jj-store-jj/tests/vault_original_remotes.rs#an_externally_changed_endpoint_refuses_the_stale_transfer
- crates/jj-store-jj/tests/vault_original_remotes.rs#explicit_remote_save_preserves_origin_and_rejects_stale_configuration_without_transfer
- crates/jj-store-jj/tests/vault_original_remotes.rs#original_fetch_moves_tracked_bookmarks_without_capturing_or_writing_files
- crates/jj-store-jj/tests/vault_original_remotes.rs#original_push_publishes_one_reviewed_bookmark_and_recovers_without_replay
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
pnpm test:vault-https
```

**Notes**

Before this source qualification slice no HTTPS acceptance command exists; pnpm refuses the missing script. This records absent real transport coverage.
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
pnpm test:vault-https
```

**Notes**

One real TLS smart-HTTP Git case passes on the disposable Linux Docker worker. Backup token publication, anonymous reads, 401 refusal, original fetch/selected push, token replacement, redirect refusal and cross-target isolation pass. Exactly two explicit receive-pack POSTs occur; the anonymous endpoint sees no Authorization. Normal Git certificate validation stays enabled with a generated container-only CA retired afterward.
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
GitHub PR #80 run 43673669111: Tooling regression tests; node --test scripts/local-history-features.test.mjs
```

**Notes**

The new original integration filename fell outside the existing vault-backup family audit; default native was therefore expected incorrectly. The exact local audit reproduces one failed / two passing cases. Rename the binary to vault_original_remotes, retaining its vault-backup gate and every assertion; do not weaken the feature audit.
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
node --test scripts/local-history-features.test.mjs
```

**Notes**

All three feature/binary admission regressions pass with the correctly named original target.
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
node --test scripts/validation.test.mjs
```

**Notes**

All 12 managed runner regressions pass; SSH and HTTPS select the full native toolchain.
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
pnpm validation:rust -- clippy --locked -p jj-store-jj --no-default-features --features vault-backup --lib --test vault_https_transport --test vault_original_remotes -- -D warnings
```

**Notes**

Strict native lint passes, including the new HTTPS source test and renamed original binary.
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
pnpm validation:rust -- test --locked -p jj-store-jj --no-default-features --features vault-backup --test vault_original_remotes; pnpm test:vault-ssh -- --original
```

**Notes**

Renamed exact binary passes four ordinary cases (two opt-in ignored), then both real SSH cases pass cold through the unchanged fixture assertions.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_VALIDATION_RUNTIME=native pnpm test:governance
```

**Notes**

Cache pruning removed offline package metadata for TypeScript/lossless-json. Three existing archive/fresh-workspace cases fail with ERR_PNPM_NO_OFFLINE_META. Restore those exact released entries with pnpm store add; no test or product code changes.
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
node --test scripts/conversation-client.test.mjs scripts/history-client.test.mjs; JJ_STORE_VALIDATION_RUNTIME=native pnpm test:governance
```

**Notes**

All seven original archive/fresh-workspace regressions pass after cache restoration; the full governance lane passes all 121 cases.
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
JJ_STORE_VALIDATION_RUNTIME=native JJ_STORE_DOC_REFERENCE=.local/remotes-qualification/docs_reference pnpm docs:check
```

**Notes**

Nine Mermaid diagrams and the 144-file, 26,743,252-byte documentation closure pass.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_VALIDATION_RUNTIME=native pnpm spec:validate; pnpm spec:first
```

**Notes**

The existing delivery chapter exceeded 400 physical lines after its new paragraph; condense the two adjacent source-proof paragraphs. Spec-first correctly refuses an unfinished unchanged-contract receipt; finalize only after source qualification completes.
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
JJ_STORE_VALIDATION_RUNTIME=native pnpm spec:validate; pnpm spec:first; pnpm check:progress; cargo fmt --all -- --check; pnpm exec prettier --check <affected files>
```

**Notes**

Final source/document closure passes: 8 validators, 4,661 chapters and 212 requirements; 10 protected files and 4 mapped chapters; 187 tasks and 1,052 evidence records. The new receipt navigation include is retained even when its continuation list is empty. Rust and all affected authored files pass formatting.
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

Rust/Cargo 1.99.0; native JJ 0.46.0/gix 0.88.0; fixture CLI JJ 0.45.1; real Git 2.55.0; Docker Node 22.22.0; local Node 24.10.0; root Deno-free spec-validator 0.2.0.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Unchanged native object, credential and publication formats. Disposable HTTPS smart Git backend, authenticated endpoint and isolated CA trust only inside the root-owned Linux validation worker.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Real source HTTPS transport qualification and exact feature-family audit repair; no product source or credential custody policy changes. Neither cold application host is qualified by these source checks.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Complete canonical closure and exact-head hosted PR #80 verification; then consume the qualified native source in Core/host adapters. Both cold application hosts remain unqualified.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: JST-LOCAL-007/008 native transfer, TLS verification, endpoint scope and
  exact publication semantics are unchanged; tests now exercise actual HTTPS and
  preserve the feature-family boundary.

```
<!-- /evidence-field -->

