---
schema: 1
id: E-M1-001
---

# E-M1-001

<!-- evidence-field:id markdown -->
## Id

E-M1-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M1
- M1.1
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-28
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

native-codec
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- Cargo.lock
- crates/jj-store-model/Cargo.toml
- crates/jj-store-model/src/lib.rs
- crates/jj-store-model/src/ids.rs
- crates/jj-store-model/src/envelope.rs
- crates/jj-store-model/tests/framing.rs
- crates/jj-store-jj/Cargo.toml
- crates/jj-store-jj/src/lib.rs
- crates/jj-store-jj/src/codec.rs
- crates/jj-store-jj/src/roots.rs
- crates/jj-store-jj/tests/native_codec.rs
- fixtures/native-profile/m1-hashes.json
- fixtures/native-profile/v1-envelopes.json
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-jj/tests/native_codec.rs#all_native_fields_roundtrip_without_identity_loss
- crates/jj-store-jj/tests/native_codec.rs#codec_ids_match_pinned_simple_stores
- crates/jj-store-jj/tests/native_codec.rs#frozen_v1_envelopes_remain_readable
- crates/jj-store-jj/tests/native_codec.rs#malformed_native_structures_never_panic
- crates/jj-store-model/tests/framing.rs#envelope_rejects_corruption_truncation_wrong_key_and_limits
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-jj --test native_codec
```

**Notes**

Before implementation: native_codec target absent.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo check -p jj-store-jj
```

**Notes**

Initial compilation: five generic-array conversions required explicit Into.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-jj --test native_codec
```

**Notes**

5 passed; initial golden vector deliberately empty before recording pinned reference outputs.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-jj --test native_codec
```

**Notes**

8 tests covering all seven stored kinds, pinned upstream comparison, virtual roots, frozen V1 bytes and malformed inputs.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-model --test framing
```

**Notes**

2 tests; every single-byte envelope corruption and truncation rejected.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check
```

**Notes**

Initial lint run: unused import and comparison-to-empty; both corrected.
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

Rust 1.97.1; jj-lib 0.45.1 / 7c41cdeb16b6b321c64e789a966b6adf723816a5
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

V1 envelope (113-byte header); profile 1; explicit canonical JSON metadata, raw file/symlink payloads; separate SHA-256 envelope digest and BLAKE2b native identities.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


