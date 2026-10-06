---
schema: 1
id: E-M1-002
---

# E-M1-002

<!-- evidence-field:id markdown -->
## Id

E-M1-002
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M1
- M1.2
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-27
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

real-rustfs
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- Cargo.toml
- Cargo.lock
- crates/jj-store-object/Cargo.toml
- crates/jj-store-object/src/lib.rs
- crates/jj-store-object/src/staging.rs
- crates/jj-store-object/src/memory.rs
- crates/jj-store-object/src/s3.rs
- crates/jj-store-object/tests/contract.rs
- crates/jj-store-object/tests/sigv4.txt
- crates/jj-store-jj/src/objects.rs
- crates/jj-store-jj/tests/native_objects.rs
- crates/jj-store-jj/tests/rustfs.rs
- scripts/rustfs-contract.py
- docs/evidence/m1-native-rustfs.json
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-object/tests/contract.rs#conditional_create_and_cas_have_one_winner
- crates/jj-store-object/tests/contract.rs#unknown_before_and_after_apply_are_distinct_from_conflicts
- crates/jj-store-object/src/s3.rs#sigv4_matches_independent_python_vector
- crates/jj-store-object/src/s3.rs#streaming_reader_rejects_short_and_excess_bodies
- crates/jj-store-jj/tests/native_objects.rs#streamed_binary_blobs_match_codec_and_resolve_lost_acknowledgements
- crates/jj-store-jj/tests/native_objects.rs#streamed_reader_rejects_late_corruption_truncation_and_forged_identity
- crates/jj-store-jj/tests/rustfs.rs#real_rustfs_transport_and_native_blob_contract
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-object --test contract
```

**Notes**

Before implementation: object crate absent.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-jj --test native_objects
```

**Notes**

Streamed binary fixture aborted with async stack overflow; moved bounded buffers to heap.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-jj --test native_objects
```

**Notes**

4 tests, including late-stream corruption and forged native identity.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-object
```

**Notes**

2 internal + 4 contract tests.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo clippy --workspace --all-targets --locked -- -D warnings
```

**Notes**

Duplicate test import removed; no lint suppression.
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
python3 scripts/rustfs-contract.py --run --rust-client --output .scratch/m1-rustfs.json
```

**Notes**

9 real probes including 20x16 Python create/CAS races, process kill/restart and 8x8 independent Rust client races; compiled native 8 MiB file and dropped real acknowledgement. Owned container/volume cleaned up.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check
```
<!-- /run -->

<!-- run -->
### Run 9

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
### Run 10

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
### Run 11

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

jj 0.45.1; Rust 1.97.1; reqwest 0.12.28; RustFS sha256:b7014e0ce2bc703c1316b3ef760e29dfae61fe4a50d1a66fa89638e0f8ea211f; single-node independent connections only
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Existing V1 native envelope unchanged; repository-local typed keys; opaque conditional version tokens. Uploads use at most two full private disk spools plus bounded 64 KiB buffers; configured single PUT cap &lt;= 5 GiB.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


