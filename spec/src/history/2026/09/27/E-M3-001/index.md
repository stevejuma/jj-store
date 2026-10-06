---
schema: 1
id: E-M3-001
---

# E-M3-001

<!-- evidence-field:id markdown -->
## Id

E-M3-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M3
- M3.1
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-27
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

authentication-foundation
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- Cargo.lock
- Cargo.toml
- crates/jj-store-model/src/ids.rs
- crates/jj-store-service/Cargo.toml
- crates/jj-store-service/src/auth.rs
- crates/jj-store-service/src/guard.rs
- crates/jj-store-service/src/lib.rs
- crates/jj-store-service/tests/authorization.rs
- crates/jj-store-wal/Cargo.toml
- crates/jj-store-wal/src/publication.rs
- crates/jj-store-wal/tests/publication.rs
- docs/evidence/m3-auth-storage-rustfs.json
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-service/tests/authorization.rs#tokens_are_secret_scope_bound_revocable_and_expiring
- crates/jj-store-service/tests/authorization.rs#operator_capability_never_implies_repository_access
- crates/jj-store-service/tests/authorization.rs#malformed_credentials_policies_and_limits_fail_closed
- crates/jj-store-service/tests/authorization.rs#listener_configuration_rejects_insecure_shared_or_implicit_development_access
- crates/jj-store-service/tests/authorization.rs#actor_permission_expiry_and_semantic_denial_precede_dependency_reads
- crates/jj-store-service/tests/authorization.rs#revocation_wins_publication_race_and_blocks_receipt_replay
- crates/jj-store-service/tests/authorization.rs#replacement_policy_must_retain_scope_and_can_be_read_after_restart
- crates/jj-store-wal/tests/publication.rs#cancellation_preserves_unknown_identity_before_and_after_manifest_application
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-service --test authorization
```

**Notes**

Before implementation: service crate absent.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-service --test authorization
```

**Notes**

7 tests: token scope/permissions/verifiers/expiry/revocation, limits/malformed inputs, listener configuration and publication/receipt authorization races.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test
```

**Notes**

51 Rust tests passed; cancellation fixture timed out during staging under concurrent compiler load, before its intended after-CAS fault.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-wal --test publication cancellation_preserves_unknown_identity_before_and_after_manifest_application
```

**Notes**

Synchronized with the exact injected before/after-CAS boundary before advancing a controlled Tokio clock; both original publication-state assertions retained.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools python3 scripts/rustfs-contract.py --run --rust-client --output .scratch/m3-auth-storage-rustfs.json
```

**Notes**

All 10 storage/publication probes pass after manifest-reader extraction and token-ID addition. This is storage compatibility evidence, not deployed transport-authentication evidence.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check
```
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

52 Rust tests, 14 governance tests and 3 harness self-tests pass after the deterministic cancellation fixture fix. The two external RustFS tests ran separately.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build
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
pnpm format:check
```
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- HTTP/RPC authentication middleware and production native semantic policy are
  not connected.
- OS credential integration, operator administration and server TLS enforcement
  remain M3.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.97.1; jj 0.45.1; SHA-256, subtle constant-time comparison, zeroize secret buffers.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

V1 repository/operator policy selected as manifest settings; 128 credentials and 64 KiB limit; jst1 token carries 128-bit ID and 256-bit secret; only domain/scope-bound verifiers persisted. No native/envelope/control-record format change.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


