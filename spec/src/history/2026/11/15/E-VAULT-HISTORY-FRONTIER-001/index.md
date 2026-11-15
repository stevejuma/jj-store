---
schema: 1
id: E-VAULT-HISTORY-FRONTIER-001
---

# E-VAULT-HISTORY-FRONTIER-001

<!-- evidence-field:id markdown -->
## Id

E-VAULT-HISTORY-FRONTIER-001
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-11-15
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M34.6
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-LOCAL-007
- JST-JJ-005
- JST-DOC-002
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-jj/src/vault_workspace/history.rs
- crates/jj-store-jj/src/vault_workspace/history/operation_frontier.rs
- crates/jj-store-jj/src/vault_workspace/history/publication_tests.rs
- spec/src/current/contracts/local-jj/32-automatic-vault-backup.md
- spec/src/current/contracts/jj-integration/28-vault-working-copy.md
- spec/src/current/contracts/documentation/03-delivery-and-validation.md
- spec/src/current/progress/M34/index.md
- spec/src/current/verification/local-03.md
- docs/site/technical/local.md
- docs/maintenance/coverage.json
- spec/src/history/2026/11/15/index.md
- spec/src/history/2026/11/15/SUMMARY.nav.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-jj/src/vault_workspace/history/publication_tests.rs#immutable_reader_accepts_ancestral_heads_without_changing_the_published_frontier
- crates/jj-store-jj/tests/vault_history.rs#history_reads_pin_saved_versions_and_page_changed_text_without_snapshots
- crates/jj-store-jj/tests/vault_counts.rs#empty_remote_counts_exclude_the_native_root_and_never_capture_unsaved_files
- crates/jj-store-jj/tests/vault_divergence.rs#immutable_divergence_counts_retained_ancestry_without_snapshotting_or_moving_its_pin
- crates/jj-store-jj/tests/vault_backup.rs#external_capture_retains_edit_bursts_and_never_creates_vault_metadata
- crates/jj-vault/tests/search.rs#history_search_indexes_transcripts_but_not_private_runtime_files
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
cargo +1.99.0 test --workspace --locked (Lapis Server consumer)
```

**Notes**

The hosted backup http_files target failed 53 passed/1 failed with HistoryReader::open returning Binding. The exact single case passed 1/1; the complete http_files target reproduced 53/1. No consumer assertion was changed and adoption remains pending.
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
node scripts/validation.mjs run validation:rust test -p jj-store-jj --features vault-backup --lib vault_workspace::history::publication_tests::immutable_reader_accepts_ancestral_heads_without_changing_the_published_frontier --locked -- --exact
```

**Notes**

Before fix: 0 passed/1 failed, actual Linux native JJ repository with both a published operation and its ancestor. Retained run jj-store-validation-38cc8aa1-eb24-4dae-a74e-8f9fd09fee1a at source 810b8aa3. Docker report collection initially closed prematurely; the supported explicit ssh-cli transport collected the same retained run.
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
node scripts/validation.mjs run validation:rust test -p jj-store-jj --features vault-backup --lib vault_workspace::history::publication_tests::immutable_reader_accepts_ancestral_heads_without_changing_the_published_frontier --locked -- --exact
```

**Notes**

After fix: 1/1, including latest retained checkpoint, unchanged head set and unsnapshotted files, plus genuine divergent-operation refusal. Run jj-store-validation-a59f9373-a416-4ddd-b414-fc24fa1503cf at 9ee21722.
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
node scripts/validation.mjs run validation:rust test -p jj-store-jj --features vault-backup --lib vault_workspace --locked
```

**Notes**

4/4 native vault_workspace unit cases, run jj-store-validation-1352e2db-1d47-432a-8afa-ed81fa2487c7.
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
node scripts/validation.mjs run validation:rust test -p jj-store-jj --features vault-backup --test vault_history --test vault_counts --test vault_divergence --test vault_backup --locked
```

**Notes**

10/10 directly affected native integration cases, run jj-store-validation-eb8fffff-c7ed-4565-bed6-4b7c598818b3.
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
node scripts/validation.mjs run validation:rust test -p jj-vault -p jj-history-service --locked
```

**Notes**

47 passed, 0 failed, 1 existing Q-RUST-002 quarantine: literal-filename pinned original history, documented in governance/25-test-quarantines.md. It is not a pass or new helper and remains unqualified; this change neither edits nor uses it as proof. Managed vault history and the cold native-history tests execute normally. Run jj-store-validation-a779c09c-fba4-408d-a996-0d17df2fb008.
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
node scripts/validation.mjs run validation:rust clippy -p jj-store-jj -p jj-vault -p jj-history-service --features jj-store-jj/vault-backup --all-targets --locked -- -D warnings
```

**Notes**

Strict affected all-target Clippy, run jj-store-validation-4825c880-ec18-428a-9ee4-a52b6c010d51.
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
pnpm dev:disk
```

**Notes**

Read-only local preflight reported 8.6 GiB free against the retained 30 GiB reserve; no local managed Cargo build or cleanup of unrelated targets was attempted. The supported isolated Linux Docker worker was selected explicitly.
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
node scripts/spec/evidence-new.mjs --id E-VAULT-HISTORY-FRONTIER-001 --task M34.6 --requirement JST-LOCAL-007
```

**Notes**

The helper wrote the new receipt, then global navigation regeneration refused the unrelated 399-line E-UI-CATALOG-HELP-001 historical index. All 750 unrelated generated changes were restored. The exported writeEvidence API authors this new receipt, with append-only links in its date index; historical receipt payloads remain unchanged.
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
node scripts/validation.mjs run docs:check
```

**Notes**

Managed Linux worker rebuilt the strict manual and nine offline Mermaid diagrams: 144 files, 26,745,974 bytes, all links/anchors valid. Run jj-store-validation-856fbcec-3ecb-44bd-8ed4-a9abf30672b0. Reviewed only the local source topic; unrelated JSON formatting was restored.
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
pnpm spec:validate; pnpm spec:first; pnpm check:progress; pnpm check:architecture; pnpm check:boundaries (explicit native documentation/tooling runtime)
```

**Notes**

Source governance and spec-first pass with the new owning chapters, 187-task/1,056-record ledger, 2,108-module architecture and 27-package boundary audit. QMD search and doctor pass after rebuilding only this checkout’s declared better-sqlite3 dependency; no global installation was changed. An early evidence draft used invalid reference/result syntax and a format write changed the revision during one pinned check; those guards refused, the draft was corrected, and final checks ran without concurrent edits.
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
node scripts/validation.mjs run validation:rust build -p jj-store-jj -p jj-vault -p jj-history-service --features jj-store-jj/vault-backup --locked
```

**Notes**

Ordinary affected library build passed in Linux run jj-store-validation-344be950-5c32-457d-b7cf-429f4ca124a5. No binaries, packages or installed applications were published or replaced.
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

Rust 1.99.0; native library jj-lib 0.46.0; source qualification based on f0d01ced; rebased onto main a7a1df3d with unrelated working-copy status changes retained; Linux x86_64 managed image sha256:7d3aa64f81dce0988978ac830b4cc53ff5713d3acbf19163bd4987ff3e858efe; worker Node 22.22.0, pnpm 10.32.1, JJ 0.45.1 and Git 2.55.0. Local QMD/tooling Node 24.10.0 and released spec-validator 0.2.0.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No operation, commit, binding, checkpoint or receipt codec change. Public HistoryReader signatures remain unchanged.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Remove ancestral candidates in memory before opening one immutable operation. Bound admission to 64 heads, 4,096 parent reads and two seconds. Do not reconcile, write heads, snapshot, capture or acquire a working-copy writer lease. Empty, genuinely divergent and exhausted frontiers remain unavailable. Qualification is source-only; Lapis Server currently pins a different upstream revision and its consumer boundary remains unqualified.

Rebased again onto main 6fdce65c after PR #85. Preserved its independent immutable
publication module and all new evidence. At source snapshot 7ae619ae the same
managed Linux runner passed vault_workspace 4/4 (including the deterministic
regression) and vault_publication 4/4 (including capture-lease independence).
Reports: jj-store-validation-9e2b1347-f48f-4157-b34e-8ae0d4fe44fb and
jj-store-validation-f7fda931-74f3-4220-aa72-8b6402a5ef61. Strict all-target
Clippy for jj-store-jj, jj-vault and jj-history-service also passed, report
jj-store-validation-6ec02bc7-2bef-4666-be14-4978efe9b26c. Governance checks
passed after conflict resolution; final evidence-only bookkeeping is rechecked.
Earlier Server candidate c2f743c8 passed 862 tests with seven existing ignores;
this rebased revision still requires distinct consumer intake qualification.

<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Complete source governance, commit/publish the focused upstream fix, monitor/merge its checked head and qualify Lapis Server adoption before publishing the pending account-recovery slice.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-LOCAL-007 and JST-JJ-005 now explicitly admit a unique maximal
  ancestral operation frontier without writes; ambiguity and read-only authority
  remain unchanged.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M34.6 remains Partial at its existing cold application/search/restore
  gate. This native source repair and Linux consumer-crate tests do not
  establish Lapis Sync, Tauri, OS or release acceptance.

```
<!-- /evidence-field -->

