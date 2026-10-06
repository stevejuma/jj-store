---
schema: 1
id: E-M6-010
---

# E-M6-010

<!-- evidence-field:id markdown -->
## Id

E-M6-010
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M6
- M6.2
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-28
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

tantivy-snapshot-restore-deployment
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-adapters/src/context.rs
- crates/jj-store-adapters/src/runner.rs
- crates/jj-store-adapters/src/source.rs
- crates/jj-store-cli/tests/native_lifecycle/server_search.rs
- crates/jj-store-cli/tests/server_executable.rs
- crates/jj-store-server/src/config.rs
- crates/jj-store-server/src/search.rs
- crates/jj-store-server/tests/configuration.rs
- crates/jj-store-tantivy/Cargo.toml
- crates/jj-store-tantivy/src/disk.rs
- crates/jj-store-tantivy/src/lib.rs
- crates/jj-store-tantivy/src/projection.rs
- crates/jj-store-tantivy/src/snapshots.rs
- crates/jj-store-tantivy/tests/snapshots.rs
- docs/cli/server-config.md
- docs/evidence/m6-snapshot-artifacts.md
- docs/evidence/m6-snapshot-rustfs.json
- spec/src/adapters.md
- spec/src/cli.md
- spec/src/compatibility.md
- spec/src/milestones.md
- spec/src/verification.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-tantivy/tests/snapshots.rs#tantivy_snapshot_restores_an_older_generation_then_replays_forward
- crates/jj-store-tantivy/tests/snapshots.rs#tantivy_snapshot_rebuilds_and_repairs_a_schema_incompatible_bundle
- crates/jj-store-server/tests/configuration.rs#search_configuration_is_explicit_bounded_and_contains_no_credentials
- crates/jj-store-cli/tests/native_lifecycle/server_search.rs#server_search_runs_recovers_and_restores_after_cache_loss
- crates/jj-store-cli/tests/server_executable.rs#real_rustfs_server_cli_restart_and_revocation
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-tantivy --test snapshots --locked
```

**Notes**

The specification-first target initially failed after test-authoring corrections because adapter limits did not expose snapshot scheduling.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-tantivy --test snapshots --locked
```

**Notes**

Both restore cases pass: older compatible generations replay forward without bootstrap, and a checksum-valid incompatible bundle rebuilds, repairs and restores on the next cold start.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-server --test configuration search_configuration_is_explicit_bounded_and_contains_no_credentials --locked
```

**Notes**

The explicit bounded search configuration accepts the snapshot interval, rejects values above one million and retains credential-name-only serialization.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle server_search_runs_recovers_and_restores_after_cache_loss --locked
```

**Notes**

The strengthened exact-generation assertion first restored the older default-interval snapshot, then exposed a test race where an active generation was observed before its remote snapshot upload completed.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle server_search_runs_recovers_and_restores_after_cache_loss --locked
```

**Notes**

The fixture explicitly publishes every position and waits for the matching remote pointer before shutdown; complete local cache loss restores the exact generation, result order and scope snapshots.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test server_executable standalone_help_configuration_completions_and_build_info_are_consistent --locked
```

**Notes**

The cold executable target compiles the extended ignored RustFS scenario and retains ordinary CLI/configuration behavior.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools python3 scripts/rustfs-contract.py --run --server --output .scratch/m6-rustfs-snapshot.json
```

**Notes**

Eight storage probes passed; the server scenario correctly restored the newest snapshot but the assertion had captured an earlier generation before a later bookmark publication.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools python3 scripts/rustfs-contract.py --run --server --output .scratch/m6-rustfs-snapshot.json
```

**Notes**

All nine pinned RustFS/TLS probes pass. After complete local server/search-state deletion and RustFS restart, generation 762cbef1ff5ed32f4c1c30129c59d1602ce72a7711aed2678d61f815c7d1e790 restores exactly with identical results; cold rebuild measured 5436 ms and restore 2007 ms under the 30000 ms budget.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check
```

**Notes**

The first aggregate check reached Clippy and found one type-complexity diagnostic in the snapshot file loader. Extracting the owned disk::SnapshotFiles type preserved behavior and made the boundary explicit.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check
```

**Notes**

The repaired aggregate check passes TypeScript/Svelte diagnostics, architecture and provenance policy, generated-contract drift checks, Clippy, rustfmt and the mapped progress gates.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools VITEST_MAX_WORKERS=1 pnpm test
```

**Notes**

The complete functional run passes 23 governance, three storage-harness, six contract, 40 production-web, 34 catalog-unit and 68 Storybook interaction/accessibility cases, the cold static-catalog browser contract, and all normal Rust workspace suites. The external opt-in RustFS cases remain separately proven by the pinned nine-probe run.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build
```

**Notes**

The cold Storybook catalog, production browser bundle and locked Rust workspace build successfully. Existing CSS minifier and large-chunk diagnostics remain warnings rather than failed gates.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools VITEST_MAX_WORKERS=1 pnpm spec:check
```

**Notes**

All 18 governed specification lanes pass, including the cold catalog contract, catalog diagnostics, full normal functional suite, mdBook, protected-file mapping and spec-first coverage.
<!-- /run -->

<!-- run -->
### Run 14

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

Rustfmt and all matched Prettier files pass after recording the final aggregate outcomes.
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

Rust 1.97.1 with rustfmt; jj-cli/jj-lib 0.45.1 at 7c41cdeb16b6b321c64e789a966b6adf723816a5; jj-native-0.45.1-v1; Tantivy 0.26.2 at 72d1ef9a6468aa68bbc69dcc80cdf60aaf64364d; RustFS rustfs/rustfs@sha256:b7014e0ce2bc703c1316b3ef760e29dfae61fe4a50d1a66fa89638e0f8ea211f.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

TOML search entries add snapshot_every with default 32 committed positions and zero disabling snapshots. Existing adapter snapshot namespace V1, SHA-256 blobs, canonical descriptors and conditional pointer format are retained. Tantivy local schema 2 and generation descriptors are retained. No native object, WAL, receipt, replay cursor, HTTP or credential format changes. Visual-baseline tests remain skipped at the user's request.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


