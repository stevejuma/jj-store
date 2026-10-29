---
schema: 1
id: E-VAULT-BACKUP-010
---

# E-VAULT-BACKUP-010

<!-- evidence-field:id markdown -->
## Id

E-VAULT-BACKUP-010
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M34
- M34.6
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-LOCAL-007
- JST-JJ-003
- JST-JJ-005
- JST-ARCH-007
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

- Cargo.lock
- crates/jj-store-jj/README.md
- crates/jj-store-jj/src/vault_workspace/content.rs
- crates/jj-store-jj/src/vault_workspace/history.rs
- crates/jj-store-jj/src/vault_workspace/mod.rs
- crates/jj-store-jj/tests/vault_history.rs
- crates/jj-vault/Cargo.toml
- crates/jj-vault/README.md
- crates/jj-vault/src/lib.rs
- crates/jj-vault/src/search/cache.rs
- crates/jj-vault/src/search/mod.rs
- crates/jj-vault/src/search/native.rs
- crates/jj-vault/tests/search.rs
- spec/src/current/contracts/architecture/26-reusable-agent-workflows.md
- spec/src/current/contracts/jj-integration/28-vault-working-copy.md
- spec/src/current/contracts/local-jj/32-automatic-vault-backup.md
- spec/src/current/progress/M34/index.md
- spec/src/history/2026/10/19/E-VAULT-BACKUP-010/SUMMARY.nav.md
- spec/src/history/2026/10/19/E-VAULT-BACKUP-010/index.md
- spec/src/history/2026/10/19/SUMMARY.nav.md
- spec/src/history/2026/10/19/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-jj/tests/vault_history.rs#history_reads_pin_saved_versions_and_page_changed_text_without_snapshots
- crates/jj-vault/tests/search.rs#search_finds_lost_text_metadata_and_paths_after_cache_loss_and_revalidates_retained_anchors
- crates/jj-vault/tests/search.rs#projection_recovers_corrupt_cache_and_applies_author_date_and_path_filters
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 6
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 pnpm dev:cargo -- test -p jj-store-jj --no-default-features --features vault-backup --test vault_history
```

**Notes**

Before implementation: missing immutable history paging APIs produce six compile errors. search-native-before.log.
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
CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 pnpm dev:cargo -- test -p jj-store-jj --no-default-features --features vault-backup --test vault_history
```

**Notes**

First implementation reaches an upstream index panic when probing a nonexistent supplied commit. Guard retained_commit with native index.has_id before ancestry. The exact unknown-anchor assertion is preserved. search-native-first.log; subsequent reader proof passes.
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
CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 pnpm dev:cargo -- test -p jj-vault --test search
```

**Notes**

Missing search module before projection implementation; compile boundary. search-index-before.log.
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
CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 pnpm dev:cargo -- clippy -p jj-vault --all-targets -- -D warnings
```

**Notes**

Initial strict Clippy reports a collapsible native parent-tree condition; corrected without suppressions. search-source-clippy-first.log.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 pnpm dev:cargo -- test -p jj-vault
```

**Notes**

New authored range-filter test accidentally had from124/to123 while expecting an empty result. Correct its valid out-of-range request to from124/to125 and retain a separate explicit invalid-range assertion. search-index-qualified.log.
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
CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 pnpm dev:cargo -- test -p jj-store-jj --no-default-features --features vault-backup --test vault_history --test vault_backup --test vault_remote --test vault_restore --test vault_restore_all --test vault_sync --test vault_publication
```

**Notes**

All20directly affected native cases pass. New history proof pages changed UTF-8 text, binary/large path-only versions and exclusions, pins saved versions despite unsnapshotted edits, rejects unknown anchors and preserves operation/disk bytes. A separate immutable reader opens while the writer lease is held. Existing capture/remote/restore/sync/publication proofs retain their gates. search-native-accepted.log.
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
CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 pnpm dev:cargo -- test -p jj-vault; CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 pnpm dev:cargo -- clippy -p jj-vault --all-targets -- -D warnings; CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 pnpm dev:cargo -- clippy -p jj-store-jj --no-default-features --features vault-backup --test vault_history --test vault_backup --test vault_remote --test vault_restore --test vault_restore_all --test vault_sync --test vault_publication -- -D warnings
```

**Notes**

Nine coordinator/native-worker/search cases and both focused strict Clippy lanes pass after the bounded-cache addition. Search finds lost deleted-file text, preserves exact checkpoint/line, rebuilds after corruption/loss, applies inclusive author/date/path filters, isolates exclusion policies and revalidates retained anchors even with cache. search-index-final, search-source-clippy-final and search-native-clippy-final logs.
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
node scripts/check-boundaries.mjs; pnpm check:progress; pnpm spec:validate; pnpm spec:first; cargo fmt --all -- --check
```

**Notes**

26package boundaries,179tasks/920records, sevenvalidators/4326chapters/208requirements and spec-first pass. Changed Markdown formatting and Rust format check pass. Heavy full repository/native app/provider/platform lanes remain out of scope under the approved implementation-first deferral; no milestone completion inferred. search-bookkeeping.log.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Finish desktop search/native frame, navigation/restore and mapped browser
  proof before advancing the sole active stage.
- Cold actual native-app/provider acceptance and full platform/release lanes
  remain deferred.
- Root storage/adoption/source selector, Forgejo and integration into main
  remain required.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ0.45.1; Git2.55.0; Rust1.99.0; source parent e500adb1; desktop recovery c05a7d7d.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Owned HistoryRevision/HistoryFiles and checkpoint-anchored search cursor/query/result DTOs. Disposable search cache V1, native paths only, best-effort256MiB/4096entry quota; no upstream JJ/Git format or identity change.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Immutable fixed-ancestry/changed-file paging and injected jj-vault::search::Source, NativeHistory and rebuildable projection. Text includes regular UTF-8 files up to oneMiB; other files are explicitly path-only. Reader takes no writer lease and never initializes, snapshots, reconciles, restores, transfers or publishes. Native restore revalidation remains authority.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Finish consumer proof then root storage/adoption/source selector and Forgejo.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-LOCAL-007 now defines bounded historical search and cache/text
  limits. JST-JJ-003/005 retain full native identity and immutable reads;
  explicit unknown-object guards prevent an upstream panic. JST-ARCH-007
  preserves injected DTO/source and upstream import boundaries.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: updated
reason: M34.5 becomes Partial with original cold native/provider gates open;
  M34.6 is the sole active stage. Source focused implementation is verified but
  desktop/cold acceptance is separate. No stage or overall goal completion
  claimed.

```
<!-- /evidence-field -->

