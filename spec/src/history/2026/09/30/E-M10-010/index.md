---
schema: 1
id: E-M10-010
---

# E-M10-010

<!-- evidence-field:id markdown -->
## Id

E-M10-010
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M10
- M10.1
- M10.3
- M10.6
- M10.7
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-30
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

native-envelope-transfer-and-zero-upstream-proof
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/workspace_migration.rs
- crates/jj-store-replication/src/transfer.rs
- crates/jj-store-replication/src/worker.rs
- crates/jj-store-replication/tests/one_way.rs
- spec/src/cli.md
- spec/src/milestones.md
- spec/src/replication.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-replication/src/transfer.rs#native_transfer_checks_the_envelope_digest_and_payload
- crates/jj-store-replication/tests/one_way.rs#one_way_import_is_atomic_idempotent_and_recovers_a_lost_response
- crates/jj-store-cli/tests/workspace_migration.rs#nonempty_workspace_migrates_without_snapshot_and_mutates_with_upstream_offline
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
cargo test -p jj-store-cli --test workspace_migration --locked nonempty_workspace_migrates_without_snapshot_and_mutates_with_upstream_offline -- --exact
```

**Notes**

Before the fix, missing CMake and Boost blocked compilation; after installing both, the runtime failed on worker native object digest. The committed parent also had two CLI JSON presentation type errors, repaired in its in-progress successor.
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
HOMEBREW_NO_AUTO_UPDATE=1 brew install cmake; HOMEBREW_NO_AUTO_UPDATE=1 brew install boost
```

**Notes**

Installed CMake 4.4.3 and Boost 1.92.0 as host build tools; no repository or service installation changed.
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
cargo test -p jj-store-replication --test one_way --locked
```

**Notes**

Both one-way cases pass using canonical native envelopes.
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
cargo test -p jj-store-replication --lib --locked transfer::tests::native_transfer_checks_the_envelope_digest_and_payload -- --exact
```

**Notes**

The verifier accepts a native envelope and rejects a changed payload or object identity.
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
cargo test -p jj-store-cli --test workspace_migration --locked nonempty_workspace_migrates_without_snapshot_and_mutates_with_upstream_offline -- --exact
```

**Notes**

The migrated workspace preserves its working copy, runs ordinary native and ReviewStore commands offline, and makes zero connections to the former upstream listener.
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
cargo test -p jj-store-replication --locked
```

**Notes**

One unit test and all 15 replication integration tests pass together.
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
cargo clippy -p jj-store-replication -p jj-store-cli --all-targets --locked -- -D warnings
```

**Notes**

All affected Rust targets pass warnings as errors.
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

Rustfmt and Prettier pass after repairing the existing ledger fence.
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
pnpm check
```

**Notes**

All repository specification, boundary, embedded RustFS, contract, web, catalog and Rust Clippy checks pass.
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
pnpm test
```

**Notes**

The complete governance, storage, RustFS, LocalTrusted, contract, web, catalog, Storybook, browser and Rust workspace test lanes pass.
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
pnpm build
```

**Notes**

Storybook, web, embedded RustFS and Rust workspace builds pass.
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
pnpm spec:check
```

**Notes**

All 18 specification lanes pass, including the cold browser suite, contract tests, Rust workspace tests, mdbook and spec-first.
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

Rust 1.98.1; jj 0.45.1; CMake 4.4.3; Boost 1.92.0; manifest format 2.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No persistence or wire format change. Native transfer checks the existing inventory digest over the canonical envelope prefix plus payload.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


