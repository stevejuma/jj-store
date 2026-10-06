---
schema: 1
id: E-M6-009
---

# E-M6-009

<!-- evidence-field:id markdown -->
## Id

E-M6-009
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M6
- M6.2
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-27
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

authorized-immutable-snapshot-artifacts
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-adapters/Cargo.toml
- crates/jj-store-adapters/src/artifacts.rs
- crates/jj-store-adapters/src/lib.rs
- crates/jj-store-adapters/src/source.rs
- crates/jj-store-adapters/tests/artifacts.rs
- docs/evidence/m6-snapshot-artifacts.md
- spec/src/adapters.md
- spec/src/compatibility.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-adapters/tests/artifacts.rs#adapter_snapshot_streams_verify_before_pointer_and_preserve_authority
- crates/jj-store-adapters/tests/artifacts.rs#adapter_snapshot_missing_or_corrupt_files_never_select_a_pointer
- crates/jj-store-adapters/tests/artifacts.rs#adapter_snapshot_unknown_upload_descriptor_and_pointer_outcomes_reconcile
- crates/jj-store-adapters/tests/artifacts.rs#adapter_snapshot_rechecks_revocation_and_confines_identity_and_lineage
- crates/jj-store-adapters/tests/artifacts.rs#adapter_snapshot_rejects_unsafe_names_forged_selected_records_and_rollback
- crates/jj-store-adapters/tests/artifacts.rs#adapter_snapshot_rejects_corrupt_and_noncanonical_descriptors_and_pointers
- crates/jj-store-adapters/tests/artifacts.rs#adapter_snapshot_bounds_file_bytes_and_shared_transfer_budgets
- crates/jj-store-adapters/tests/artifacts.rs#adapter_snapshot_competing_pointer_candidates_preserve_one_complete_selection
- crates/jj-store-adapters/tests/artifacts.rs#adapter_snapshot_revocation_during_download_returns_no_content
- crates/jj-store-adapters/tests/artifacts.rs#adapter_snapshot_cancellation_and_deadline_bound_streaming_work
- crates/jj-store-adapters/tests/artifacts.rs#adapter_snapshot_epoch_changes_and_request_exhaustion_stop_access
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-adapters --test artifacts adapter_snapshot_streams_verify_before_pointer_and_preserve_authority --locked
```

**Notes**

Initial named target cannot compile: four missing artifact capability/type diagnostics.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-adapters --test artifacts adapter_snapshot_streams_verify_before_pointer_and_preserve_authority --locked
```

**Notes**

Original target passes after implementation. Shared fixture imports initially emit three dead-code warnings; the isolated fixture import is now annotated.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-adapters --test artifacts --locked
```

**Notes**

All eleven expanded snapshot storage fixtures pass, including competing CAS candidates, every write-stage unknown outcome, corruption, missing dependencies, namespace/epoch checks, revocation, shared budgets, cancellation and deadline.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-adapters --locked
```

**Notes**

All 22 adapter delivery and snapshot fixtures pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle adapter_projection --locked
```

**Notes**

All 27 native search/extraction/server projection fixtures pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-tantivy --test durability --locked
```

**Notes**

All ten local Tantivy durability fixtures pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check
```

**Notes**

Normal checks pass, including rustfmt, all-target Clippy, progress, boundaries, contracts and existing Svelte checks.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test
```

**Notes**

Full suite passes: 368 Rust tests, zero failures, six external fixtures ignored; governance, storage harness, generated contracts and existing client unit checks pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build
```

**Notes**

Workspace binaries and existing browser assets build successfully.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check
```

**Notes**

All 13 lanes pass, including existing cold browser interaction/accessibility, native tests, contracts, rustfmt, governance, mdBook and mapped spec-first.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm format:check
```

**Notes**

Rustfmt and all matched Prettier files pass.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Integrate committed Tantivy generation export and compatible restore with
  complete payload/schema/index validation and rebuild fallback.
- Configure foreground snapshot scheduling and record measured indexing/rebuild
  and real RustFS/TLS deployment acceptance. UI remains deferred; the original
  external Docker qualification remains stalled.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.97.1 with rustfmt; jj-cli/jj-lib 0.45.1 at 7c41cdeb16b6b321c64e789a966b6adf723816a5; jj-native-0.45.1-v1; Tantivy 0.26.2 at 72d1ef9a6468aa68bbc69dcc80cdf60aaf64364d.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive optional adapter snapshot namespace v1: SHA-256 blobs, canonical V1 bundle descriptors and conditional snapshot.json pointers scoped to repository/epoch/full adapter identity. No native, WAL, receipt, replay cursor, local Tantivy schema 2 or HTTP format changes. Enclosing Jujutsu change is the revision reference.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


