---
schema: 1
id: E-M2-001
---

# E-M2-001

<!-- evidence-field:id markdown -->
## Id

E-M2-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M2
- M2.1
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-27
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

native-dependency-inventory
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- Cargo.toml
- Cargo.lock
- crates/jj-store-wal/Cargo.toml
- crates/jj-store-wal/src/lib.rs
- crates/jj-store-wal/src/inventory.rs
- crates/jj-store-wal/tests/inventory.rs
- crates/jj-store-jj/Cargo.toml
- crates/jj-store-jj/src/dependencies.rs
- crates/jj-store-jj/src/objects.rs
- crates/jj-store-jj/tests/native_inventory.rs
- docs/evidence/m2-inventory-native-rustfs.json
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-wal/tests/inventory.rs#complete_dag_closure_retains_history_and_reuses_verified_base
- crates/jj-store-wal/tests/inventory.rs#missing_dependencies_cycles_wrong_keys_and_budgets_fail_closed
- crates/jj-store-wal/tests/inventory.rs#verified_bases_cannot_cross_storage_sources_even_with_same_repository_id
- crates/jj-store-wal/tests/inventory.rs#deadline_cancels_a_stalled_source
- crates/jj-store-jj/tests/native_inventory.rs#inventory_includes_invisible_evolution_copies_and_every_reference_term
- crates/jj-store-jj/tests/native_inventory.rs#missing_or_corrupt_native_dependencies_never_certify_inventory
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-wal --test inventory
```

**Notes**

Before implementation: WAL crate absent.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-wal --test inventory
```

**Notes**

4 tests: closure, incremental retention, cycles/missing objects/resource limits, cancellation and source-bound bases.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-jj --test native_inventory
```

**Notes**

3 tests: complete seven-kind native closure, all view/evolution references, copies, external submodules and missing/corrupt objects.
<!-- /run -->

<!-- run -->
### Run 4

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
### Run 5

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
### Run 6

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test
```
<!-- /run -->

<!-- run -->
### Run 7

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
### Run 8

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm format:check
```
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
python3 scripts/rustfs-contract.py --run --rust-client --output .scratch/m2-native-rustfs.json
```

**Notes**

Fresh single-node pinned container: all 9 probes passed; native metadata/file transport remains conformant after inventory-source integration. No publication claim. Owned container and volume cleaned up.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Attempt-unique WAL candidates and manifest CAS are not implemented.
- Durable receipts, unknown-outcome recovery and bounded persistent
  checkpoint/catalog formats remain M2.2/M2.3.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

jj-lib 0.45.1; Rust 1.97.1
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No persisted format change. Process-local verified inventories retain the full dependency closure; source capabilities are not serialized or used as publication authority.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


