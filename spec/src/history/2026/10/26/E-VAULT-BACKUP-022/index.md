---
schema: 1
id: E-VAULT-BACKUP-022
---

# E-VAULT-BACKUP-022

<!-- evidence-field:id markdown -->
## Id

E-VAULT-BACKUP-022
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-26
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M34.6
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-LOCAL-007
- JST-ARCH-007
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-vault/src/search/indexed/disk.rs
- crates/jj-vault/src/search/indexed/mod.rs
- crates/jj-vault/src/search/indexed/native.rs
- crates/jj-vault/src/search/indexed/read.rs
- crates/jj-vault/tests/indexed_read_admission.rs
- crates/jj-vault/tests/native_adoption.rs
- spec/src/current/contracts/architecture/27-vault-history-search.md
- spec/src/current/contracts/local-jj/32-automatic-vault-backup.md
- spec/src/current/progress/M34/index.md
- spec/src/history/2026/10/26/E-VAULT-BACKUP-022/index.md
- spec/src/history/2026/10/26/E-VAULT-BACKUP-022/SUMMARY.nav.md
- spec/src/history/2026/10/26/index.md
- spec/src/history/2026/10/26/SUMMARY.nav.md
- spec/src/history/2026/10/index.md
- spec/src/history/2026/10/SUMMARY.nav.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-vault/tests/indexed_read_admission.rs#native_query_waits_for_the_publisher_then_reads_the_retained_generation
- crates/jj-vault/tests/indexed_read_admission.rs#sustained_publisher_contention_remains_a_bounded_busy_error
- crates/jj-vault/tests/indexed_read_admission.rs#native_query_never_creates_a_missing_projection
- crates/jj-vault/tests/indexed_read_admission.rs#native_query_never_repairs_corrupt_projection_bytes
- crates/jj-vault/src/search/indexed/read.rs#admitted_reader_releases_the_publisher_lock
- crates/jj-vault/tests/native_adoption.rs#worker_adopts_original_history_and_receipt_replay_keeps_the_current_status
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
cargo test --manifest-path /work/jj-store/Cargo.toml --locked -p jj-vault --test indexed_read_admission -- --nocapture
```

**Notes**

Before, 0 passed / 4 failed in immutable source222f0683; Docker run lapis-desktop-history-source-before-1d300740-210a-4c4a-95e4-9b904d6175a3. Native query rejected a transient publisher immediately, did not wait boundedly, initialized a missing projection and repaired corrupt bytes. The source implementation was unchanged in this before snapshot.
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
node scripts/managed-cargo.mjs test --locked -p jj-vault --test indexed_read_admission -- --nocapture
```

**Notes**

Exact after, 4 passed / 0 failed, including the real five-second Busy deadline. Source8138c336; Docker run lapis-desktop-history-source-after-9aaa06c2-8c0e-48b5-9f70-f4dbd0fe1319. Original immutable commit and unsnapshotted disk bytes are preserved.
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
node scripts/managed-cargo.mjs test --locked -p jj-vault
```

**Notes**

Initial full owning-crate widening reached native_adoption: 0 passed / 1 failed there because a test hard-coded /opt/homebrew/bin/jj on Linux. Earlier source/writer/indexed-search tests passed. Run lapis-desktop-history-source-qualified-4825c37a-8e85-42cc-b77d-5d1318cc3e4b. Use the provisioned PATH executable without changing assertions.
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
node scripts/managed-cargo.mjs test --locked -p jj-vault --test native_adoption
```

**Notes**

Exact fixture rerun: 1 passed / 0 failed. Run lapis-desktop-history-source-qualified-1a5b5f6c-657c-4cdd-87b9-edf3467660ae; source8c3fc655.
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
node scripts/managed-cargo.mjs test --locked -p jj-vault
```

**Notes**

Full owning crate: 33 passed / 0 failed across unit, integration and doc tests. Includes existing changed-version/frontier/corruption/native-source membership tests, backup/capture/close/watcher families and the new committed-reader snapshot/lock test. Same cold immutable source8c3fc655 and Docker run1a5b5f6c.
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
node scripts/managed-cargo.mjs check --locked -p jj-vault; node scripts/managed-cargo.mjs build --locked -p jj-vault; node scripts/managed-cargo.mjs clippy --locked -p jj-vault --all-targets -- -D warnings
```

**Notes**

All three owning Rust lanes pass in the same immutable Docker source8c3fc655/run1a5b5f6c. Source package and all directly affected writer/query tests are qualified; no UI code, unrelated crate or provider suite is in this source-only impact surface.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
node scripts/check-progress.mjs; JJ_STORE_VALIDATION_RUNTIME=native spec-validator first
```

**Notes**

Initial source evidence used a bare Rust test-file reference. The progress/spec-first parser requires exact file#test anchors. Added all four exact names without altering the tests or historical records.
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
node scripts/check-progress.mjs; JJ_STORE_VALIDATION_RUNTIME=native spec-validator first; JJ_STORE_VALIDATION_RUNTIME=native spec-validator validate; prettier --check changed Markdown; rustfmt --edition 2024 --check changed Rust files
```

**Notes**

Changed-source ledger and spec-first pass: 180 tasks, 961 evidence records; six protected files/two owning chapters. Static specification passes all8validators across4456chapters/208requirements. Changed-file formatting and Rust formatting pass. Only new day/evidence navigation is retained; unrelated generated rewrites and all historical records are preserved.
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
cargo fmt --all -- --check
```

**Notes**

The optional host-wide formatter cannot resolve pinned sibling source paths from this isolated workspace. No formatting assertion fails; exact rustfmt --edition2024 --check on all six changed Rust paths passes. Product source/check/test/build/Clippy remain qualified in Docker with the pinned sibling layout.
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

Linux x86_64 Docker on remote omarchy; Rust1.99.0; Jujutsu0.45.1; immutable Desktop validation image sha256:da3863d93645476fd6ceb2240fa371d73880175b352c31e188ce027df1bcaee6; 4 CPUs/12 GiB. Parent jj-store4e74e423. Docker source/cache/artifact manifests are retained under the isolated Desktop .local/validation/runs directories; independent target volumes are seeded from read-only idle caches.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No stored-format, public DTO or API changes. The read constructor is crate-private. Existing fail-fast Engine::open writer admission remains unchanged.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Native indexed queries wait at most five seconds for publisher admission, then pin a committed manual-reader generation and release publisher admission before immutable-source membership/search work. They never initialize or repair projection bytes, schedule work, acquire capture authority or alter saved files. Sustained contention remains explicit Busy.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Commit this verified source slice, then separately update the Desktop native source pin and rerun its unchanged hidden backup/query acceptance. Desktop/provider/platform/release results are separate.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-LOCAL-007 and JST-ARCH-007 now specify bounded existing-snapshot
  query admission and query independence from cache repair/capture. Original
  source membership, checkpoint identities, writer authority and formats are
  retained.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M34.6 remains Partial at its original complete cold
  application/search/restore gate. Focused source read-admission qualification
  does not close Desktop, provider, cross-platform, distribution or visual
  gates.

```
<!-- /evidence-field -->

