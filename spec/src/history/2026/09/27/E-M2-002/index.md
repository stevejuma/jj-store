---
schema: 1
id: E-M2-002
---

# E-M2-002

<!-- evidence-field:id markdown -->
## Id

E-M2-002
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M2
- M2.1
- M2.2
- M2.3
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-27
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

manifest-publication-recovery
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- Cargo.lock
- crates/jj-store-jj/tests/rustfs.rs
- crates/jj-store-model/src/ids.rs
- crates/jj-store-wal/Cargo.toml
- crates/jj-store-wal/src/catalog.rs
- crates/jj-store-wal/src/inventory.rs
- crates/jj-store-wal/src/lib.rs
- crates/jj-store-wal/src/publication.rs
- crates/jj-store-wal/src/records.rs
- crates/jj-store-wal/tests/publication.rs
- scripts/rustfs-contract.py
- docs/evidence/m2-publication-rustfs.json
- spec/src/verification.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-wal/tests/publication.rs#atomic_publication_retains_receipts_through_restart_and_checkpoint
- crates/jj-store-wal/tests/publication.rs#every_staging_or_manifest_fault_preserves_atomic_visibility_and_retry_identity
- crates/jj-store-wal/tests/publication.rs#competing_candidates_preserve_siblings_and_only_selected_records_are_committed
- crates/jj-store-wal/tests/publication.rs#delayed_duplicate_cannot_resurrect_superseded_heads
- crates/jj-store-wal/tests/publication.rs#lost_acknowledgement_resolves_from_history_after_another_publication
- crates/jj-store-wal/tests/publication.rs#strict_stale_intent_reuse_and_missing_dependencies_publish_nothing
- crates/jj-store-wal/tests/publication.rs#checkpoint_race_preserves_newer_tail_receipts_and_inventory
- crates/jj-store-wal/tests/publication.rs#settings_are_selected_atomically_and_initialization_is_idempotent
- crates/jj-store-wal/tests/publication.rs#persistent_catalog_splits_are_bounded_and_old_roots_remain_readable
- crates/jj-store-wal/tests/publication.rs#cold_recovery_rejects_missing_native_data_and_control_corruption
- crates/jj-store-wal/tests/publication.rs#deterministic_publication_sequences_replay_with_original_results
- crates/jj-store-wal/tests/publication.rs#authorization_is_rechecked_after_cas_conflict_and_before_receipt_replay
- crates/jj-store-wal/tests/publication.rs#semantic_denial_stages_no_publication_records
- crates/jj-store-wal/tests/publication.rs#cold_recovery_verifies_retained_historical_roots_and_settings
- crates/jj-store-wal/tests/publication.rs#cancellation_preserves_unknown_identity_before_and_after_manifest_application
- crates/jj-store-jj/tests/rustfs.rs#real_rustfs_manifest_publication_and_cold_recovery
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-wal --test publication
```

**Notes**

Before implementation: publication target absent.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-wal --test publication cold_recovery_verifies_retained_historical_roots_and_settings
```

**Notes**

Review regression reproduced accepted corruption in a retained historical inventory root.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-wal --test publication
```

**Notes**

15 publication, fault, race, randomized replay, cancellation, guard and recovery tests. Historical-root regression now passes.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools python3 scripts/rustfs-contract.py --run --rust-client --output .scratch/m2-publication-rustfs-final.json
```

**Notes**

10 real probes; 19 accepted native publications with concurrent siblings, randomized head updates, unknown acknowledgements, late receipts and checkpoints. A fresh process after container kill/restart verifies all retained native and control data. Owned services cleaned up.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check
```
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check
```
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test
```

**Notes**

45 Rust tests, 14 governance tests and 3 harness self-tests; 2 external RustFS tests run separately by the real harness.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build
```
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm format:check
```

**Notes**

Two canonical Markdown files required formatting; corrected.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

Ledger status-edit script matched four dependency cells; restored original requirement IDs before commit.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm format:check
```
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm check:progress
```
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:build
```
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

jj 0.45.1 / 7c41cdeb16b6b321c64e789a966b6adf723816a5; Rust 1.97.1; pinned single-node RustFS.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

New V1 canonical repository-bound JSON control envelopes, SHA-256 references; attempt-unique WAL keys; bounded radix catalogs; manifest revision distinct from publication sequence; native V1 envelope unchanged.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


