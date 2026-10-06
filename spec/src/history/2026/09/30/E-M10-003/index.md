---
schema: 1
id: E-M10-003
---

# E-M10-003

<!-- evidence-field:id markdown -->
## Id

E-M10-003
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M10
- M10.3
- M10.4
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-30
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verified-origin-import-and-native-head-reduction
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- Cargo.lock
- Cargo.toml
- crates/jj-store-jj/src/lib.rs
- crates/jj-store-jj/src/replication.rs
- crates/jj-store-jj/tests/replication_heads.rs
- crates/jj-store-replication/Cargo.toml
- crates/jj-store-replication/src/lib.rs
- crates/jj-store-replication/src/protocol.rs
- crates/jj-store-replication/src/transfer.rs
- crates/jj-store-replication/tests/one_way.rs
- crates/jj-store-service/src/native.rs
- crates/jj-store-wal/src/publication.rs
- crates/jj-store-wal/src/publication/native_copy.rs
- crates/jj-store-wal/src/publication/replication.rs
- crates/jj-store-wal/src/publication/replication_import.rs
- crates/jj-store-wal/src/publication/replication_transfer.rs
- spec/src/milestones.md
- spec/src/publication.md
- spec/src/replication.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-replication/tests/one_way.rs#one_way_import_is_atomic_idempotent_and_recovers_a_lost_response
- crates/jj-store-replication/tests/one_way.rs#holes_partial_closures_and_digest_reuse_never_publish
- crates/jj-store-jj/tests/replication_heads.rs#imported_heads_use_pinned_operation_ancestry_and_keep_siblings
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-replication --test one_way --locked
```

**Notes**

Both one-way cases pass: immutable closure transfer cannot publish partial data; complete imports select content and per-origin receipts atomically; duplicates are no-ops; causal holes and origin-key digest reuse fail closed; a lost manifest response resolves under the stable import request.
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
cargo test -p jj-store-jj --test replication_heads --locked
```

**Notes**

The pinned jj-lib operation-parent walk removes root and delayed ancestor candidates while retaining concurrent sibling operation heads.
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
cargo test -p jj-store-wal --locked
```

**Notes**

All WAL tests pass: 52 integration cases across publication, inventory, native copy, AgentStore and ReviewStore plus unit and documentation targets.
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
cargo check -p jj-store-wal -p jj-store-jj -p jj-store-service -p jj-store-replication --all-targets --locked
```

**Notes**

The receiver, transport-neutral replication package, pinned native ancestry reducer and production native-session wiring compile across all affected targets.
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
cargo fmt --all
```

**Notes**

All Rust sources were formatted before focused verification.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
jj --no-pager diff --check
```

**Notes**

The installed jj does not provide a diff --check option; this command changed no files and source formatting was verified with cargo fmt.
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
pnpm check
```

**Notes**

Specification, boundary, architecture, progress, formatting and the ten-minute embedded RustFS clippy lane passed. The following contracts check stopped while building Kingfisher Vectorscan because cmake is not installed on this host; this is the same environment limitation recorded by E-M10-001 and E-M10-002.
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
pnpm format:check
```

**Notes**

Rust formatting for the workspace and embedded RustFS manifest plus Prettier formatting for all tracked files pass after canonical evidence formatting.
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
pnpm spec:check
```

**Notes**

Canonical validation and the cold catalog browser lane passed. Browser interaction then stopped when its native CLI fixture reached Kingfisher Vectorscan and could not invoke absent cmake.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test
```

**Notes**

Governance, the storage harness, real embedded RustFS restart and managed-storage lock acceptance passed. The following LocalTrusted binary build stopped at the same missing-cmake Kingfisher Vectorscan dependency.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm build
```

**Notes**

Storybook, the production web application and embedded RustFS built successfully. The final workspace Cargo build stopped at the same missing-cmake Kingfisher Vectorscan dependency.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- M10.3 still requires authenticated RPC exposure, persistent peer cursors and
  full push/pull orchestration.
- M10.4 still requires distinct hydrated workspace acceptance and an explicit
  native reconciliation command; import itself deliberately does not reconcile.
- AgentStore and ReviewStore origins are rejected until M10.5 accepted-fact
  reducers exist.
- Format-2 seed upgrade, bootstrap and non-destructive migration remain M10.6
  work.
- The repository-wide contracts, check, test, build and specification lanes that
  compile Kingfisher Vectorscan require cmake on this host.
- Real RustFS, worker, outage and foreground zero-upstream-call qualification
  remain M10.7 work.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; jj 0.45.1; jj-store-replication, jj-store-wal and jj-store-jj 0.1.0; manifest format 2; origin, origin-receipt and apply-origins schema v1.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Adds a transport-neutral replication package with capability, feed and offer DTOs; bounded immutable native/control closure transfer; an authorized ApplyOriginsV1 receiver; destination-owned per-origin receipts; and a production NativeOperationHeadReducer over pinned jj-lib operation ancestry. Manifest format remains 2.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


