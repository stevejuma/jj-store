---
schema: 1
id: E-VAULT-BACKUP-019
---

# E-VAULT-BACKUP-019

<!-- evidence-field:id markdown -->
## Id

E-VAULT-BACKUP-019
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M34.2
- M34.3
- M34.5
- M34.6
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

- crates/jj-store-jj/src/vault_workspace/mod.rs
- crates/jj-store-jj/src/vault_workspace/path_policy.rs
- crates/jj-store-jj/src/vault_workspace/restore_io.rs
- crates/jj-store-jj/src/vault_workspace/scan.rs
- crates/jj-store-jj/tests/vault_backup.rs
- crates/jj-store-jj/tests/vault_transcript_sync.rs
- crates/jj-vault/src/search/cache.rs
- crates/jj-vault/tests/search.rs
- spec/src/current/contracts/local-jj/32-automatic-vault-backup.md
- spec/src/current/progress/M34/index.md
- spec/src/history/2026/10/20/E-VAULT-BACKUP-019/SUMMARY.nav.md
- spec/src/history/2026/10/20/E-VAULT-BACKUP-019/index.md
- spec/src/history/2026/10/20/SUMMARY.nav.md
- spec/src/history/2026/10/20/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-jj/tests/vault_backup.rs#transcript_files_are_backed_up_without_private_runtime_siblings
- crates/jj-store-jj/tests/vault_transcript_sync.rs#transcript_sync_conflicts_require_explicit_resolution
- crates/jj-vault/tests/search.rs#history_search_indexes_transcripts_but_not_private_runtime_files
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1: before

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown
```

```shell
pnpm dev:cargo -- test -p jj-store-jj --no-default-features --features vault-backup --test vault_backup transcript_files_are_backed_up_without_private_runtime_siblings
```

**Notes**

0/1 passed before implementation: transcript file_at returned None. Log .local/transcript-backup-before.log.
<!-- /run -->

<!-- run -->
### Run 2: after

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown
```

```shell
pnpm dev:cargo -- test -p jj-store-jj --no-default-features --features vault-backup --test vault_backup
```

**Notes**

5/5 passed. Log .local/transcript-backup-after.log.
<!-- /run -->

<!-- run -->
### Run 3: families

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown
```

```shell
pnpm dev:cargo -- test -p jj-store-jj --no-default-features --features vault-backup --test vault_backup --test vault_history --test vault_restore --test vault_restore_all --test vault_sync
```

**Notes**

15 tests passed; wrapper then exited 1 because free space reached 30.0 GiB at the 30 GiB reserve. No test failed. Log .local/transcript-backup-families.log.
<!-- /run -->

<!-- run -->
### Run 4: maintenance

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
pnpm dev:clean -- --target target --dry-run
pnpm dev:clean -- --target target --apply
```

**Notes**

Managed idle source artifacts only; no cleanup concurrent with raw Cargo, no application data or active target cleanup. The final source-only cleanup permits the next focused run without reducing the reserve.
<!-- /run -->

<!-- run -->
### Run 5: transcript-families

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown
```

```shell
pnpm dev:cargo -- test -p jj-store-jj --no-default-features --features vault-backup --test vault_backup --test vault_history --test vault_restore --test vault_restore_all --test vault_sync --test vault_transcript_sync
```

**Notes**

16 tests passed: 5 backup, 1 history, 4 restore, 2 whole-vault restore, 3 sync and 1 transcript-conflict proof. Root/nested files, explicit exclusions and transcript restore/reopen are covered. Wrapper then exited 1 with 28.2 GiB free after compilation, below its reserve. Log .local/transcript-backup-final.log. Further Rust commands wait for safe idle-target maintenance.
<!-- /run -->
<!-- run -->
### Run 6: classifier

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown
```

```shell
pnpm dev:cargo -- test -p jj-store-jj --no-default-features --features vault-backup --lib vault_workspace::path_policy::tests
```

**Notes**

1 passed, 63 filtered; root/nested file/ancestor/private metadata classification. .local/transcript-classifier.log.
<!-- /run -->

<!-- run -->
### Run 7: search-fixture-before-repair

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown
```

```shell
pnpm dev:cargo -- test -p jj-vault --test search
```

**Notes**

2 passed, 1 failed: the new test manually reused an Index namespace after changing source exclusions, contrary to its documented namespace contract. Corrected the test to use production NativeHistory::search, which binds current exclusions; retained the exclusion assertion and added indexed-search coverage. .local/transcript-search.log.
<!-- /run -->

<!-- run -->
### Run 8: search

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown
```

```shell
pnpm dev:cargo -- test -p jj-vault --test search
```

**Notes**

3 passed. Transcript body/path hits are visible through bounded and indexed native search; private runtime files are absent. Current explicit exclusions remove transcript hits from both projections. .local/transcript-search-final.log.
<!-- /run -->

<!-- run -->
### Run 9: source-checks

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown
```

```shell
pnpm dev:cargo -- clippy -p jj-store-jj --no-default-features --features vault-backup --lib -- -D warnings
pnpm dev:cargo -- clippy -p jj-vault --lib -- -D warnings
```

**Notes**

Both affected package checks pass with strict Clippy. .local/transcript-clippy-source.log and .local/transcript-clippy-vault.log.
<!-- /run -->

<!-- run -->
### Run 10: working-copy-ledger

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

The four pre-existing unrelated M33/documentation edits refer to missing E-REUSABLE-PROGRESS-050 and lack fresh evidence coverage (six errors). They remain byte-for-byte preserved and are excluded from this slice. Validate the committed source revision separately.
<!-- /run -->

<!-- run -->
### Run 11: canonical-documents

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown
```

```shell
pnpm spec:validate
pnpm exec prettier --check <changed authored documents>
rustfmt --edition 2024 --check <changed Rust files>
```

**Notes**

Canonical validation passes (7 validators, 4381 chapters, 208 requirements), and changed authored-document Prettier plus changed Rust rustfmt checks pass. Isolated committed-ledger check follows. Managed idle Desktop package-cache maintenance recovered 22.8 GiB after all native processes/builds were idle; existing app bundles, native data and source caches were preserved. No reserve override was used.
<!-- /run -->

<!-- run -->
### Run 12: committed slice ledger

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown
```

```shell
JJ_STORE_CHECK_REVISION=@- pnpm check:progress
```

**Notes**

179 tasks and 946 evidence records validate for this source slice's enclosing committed revision. The four pre-existing M33/documentation edits remain uncommitted in the descendant working copy. No unrelated content was included.
<!-- /run -->

<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Original M34 cold/provider/platform/aggregate release gates remain outside this focused source slice.
- Unrelated uncommitted M33 evidence work remains owned by its existing change; it blocks the current-working-copy ledger check until completed there.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.99.0; JJ 0.45.1; macOS ARM64. Source parent df31b5caeaabd562b0b76806f89bc50dec073ec5; this enclosing change owns the classifier. Desktop checkpoint c7118f9e preserves its native-chat experiment and E-AI-092 dependency archive identities.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Existing vault binding/checkpoint/restore/sync receipts and transcript YAML/JSONL formats are unchanged. The backup eligibility policy admits three existing portable files at every valid scope; historical earlier bytes cannot be reconstructed.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

JST-LOCAL-007 now includes root/nested local ACP metadata.yaml, agents.jsonl and transcript.jsonl under UUIDv4 session directories. One shared Rust classifier governs scanning, native backup watch filtering, historical search, restore and synchronization application. Ancestors are traversable; private runtime siblings, caches, credentials and Git/JJ state remain excluded. Explicit user backup exclusions win. No native AgentStore/ThreadStore object or journal is migrated/deleted. Desktop owns writer protection and presentation; its E-AI-093 records consumer qualification.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Commit the verified transcript inclusion slice and validate its revision independently of unrelated M33 work. Desktop E-AI-093 owns the qualified writer guards, UI and cold backup/restore/reopen proof; remaining original provider/release gates stay deferred.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-LOCAL-007 portable backup now includes the three allowlisted ACP transcript files; owning automatic-vault-backup chapter updated. Existing identity, receipts, explicit exclusions and conflict authority remain unchanged.
```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M34.2/M34.3/M34.5/M34.6 remain Partial at their original cold native and provider gates. This source slice adds transcript inclusion and regression evidence; it does not complete deferred milestone gates.
```
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


