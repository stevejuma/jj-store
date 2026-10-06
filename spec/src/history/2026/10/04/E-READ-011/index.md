---
schema: 1
id: E-READ-011
---

# E-READ-011

<!-- evidence-field:id markdown -->
## Id

E-READ-011
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4.1
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-04
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change kmuklnxk
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Bounded reuse of verified resident native query indexes
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust1.98.1; jj-lib0.45.1; pnpm10.32.1; macOS ARM64.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Public DTOs, native persisted formats, query-cache receipt and JJ derived index format unchanged. Resident copies are process-local and disappear on executor teardown/restart.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/native_lifecycle/graph_cache.rs
- crates/jj-store-jj/src/query/cache.rs
- crates/jj-store-jj/src/query/cache/resident.rs
- crates/jj-store-jj/src/query/index.rs
- crates/jj-store-jj/src/query/mod.rs
- spec/src/jj-integration.md
- spec/src/cli.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-JJ-003
- JST-JJ-005
- JST-API-001
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-jj/src/query/cache.rs#warm_cache_verification_does_not_hash_unchanged_large_files
- crates/jj-store-cli/tests/native_lifecycle/graph_cache.rs#resident_indexes_preserve_multiple_historical_operation_scopes
- crates/jj-store-cli/tests/native_lifecycle/graph_cache.rs#graph_cache_checksums_missing_files_and_receipts_rebuild_exact_snapshots
- crates/jj-store-cli/tests/native_lifecycle/graph_cache.rs#graph_cache_recovery_rechecks_permission_and_serializes_independent_executors
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
cargo test -p jj-store-jj --lib --locked warm_cache_verification_does_not_hash_unchanged_large_files -- --nocapture
```

**Notes**

Before: repeated full verification of unchanged2MiB file exhausted5-unit work budget; first checksum used more than30units. Final fixture tests resident verification metadata separately: disk bytes themselves always retain full checksum admission.
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
cargo test -p jj-store-jj --lib --locked warm_cache_verification_does_not_hash_unchanged_large_files -- --nocapture
```

**Notes**

Final1passed: bounded warm metadata checks, same-size edit with restored mtime invalidates proof, changed file requires full work, and coarse timestamps disable resident reuse while full checks remain available.
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
cargo test -p jj-store-cli --test native_lifecycle --locked graph_acceptance:: -- --test-threads=2
```

**Notes**

21passed/1failed in new historical-scope fixture; supplied explicit workspace required by existing query-context grammar.
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
cargo test -p jj-store-cli --test native_lifecycle --locked resident_indexes_preserve_multiple_historical_operation_scopes -- --nocapture
```

**Notes**

With valid context, warm multi-operation query failed Unavailable: first scope built derived files, second scope compared them with the original receipt. Verify disk once before disk-based additional scopes under the held exclusive namespace lease; final cache receipt still covers resulting files.
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
cargo test -p jj-store-cli --test native_lifecycle --locked graph_acceptance:: -- --test-threads=2
```

**Notes**

Final22passed: warm historical-scope parity, segment/link parity, corrupt/same-size/missing files, receipts/symlinks, repository/epoch separation, eviction, revocation, cancellation, pins and quota recovery.
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
target/debug/deps/native_lifecycle-d9faf2e6f13fc0f1 content_acceptance::index_task_acceptance:: --test-threads=2
```

**Notes**

5passed: preparation deduplication, expiry/cancellation, restart/revocation and reserved interactive capacity.
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
cargo check -p jj-store-jj -p jj-store-service -p jj-store-cli --all-targets --locked
```

**Notes**

Affected targets and direct consumers pass. Existing test-linker unwind-size advisory remains.
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
pnpm spec:first; pnpm check:progress; pnpm spec:validate; pnpm exec prettier --check spec/src/jj-integration.md spec/src/cli.md; cargo fmt --all -- --check
```

**Notes**

5protected files/2canonical chapters,136ledger tasks and167requirements pass. Changed chapters formatted; ledger appended without whole-file rewriting.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Residents are bounded by existing entry count and conservative estimated
  retained bytes capped at256MiB per executor. Metadata stamps authorize reuse
  of the verified in-memory index only; any additional disk index loading
  performs full checksums. Coarse/non-Unix filesystems fall back to full
  checksums. Large-scale resident/peak-memory qualification remains pending.
- Namespace lock still spans graph evaluation/preparation. Immutable disk
  generations, short locking, process/global resident accounting,4GiB/repository
  and16GiB aggregate staging quotas, larger native/local preparation and
  remaining endpoint/card/collaboration work remain pending.
- Running application unchanged. No public contract or persisted format change;
  root aggregates, unchanged UI/browser and full contracts lanes out of scope.
  E-READ-005 registry/full-capture and real-RustFS/scale gates remain open.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Commit resident reuse after ledger checks; continue immutable generations and cache admission, then local prepared history and remaining audited endpoints.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


