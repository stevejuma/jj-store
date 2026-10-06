---
schema: 1
id: E-M3-005
---

# E-M3-005

<!-- evidence-field:id markdown -->
## Id

E-M3-005
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M3
- M3.1
- M3.2
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-27
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

native-rpc
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-rpc/Cargo.toml
- crates/jj-store-rpc/proto/native.proto
- crates/jj-store-rpc/src/client.rs
- crates/jj-store-rpc/src/server.rs
- crates/jj-store-service/src/transport.rs
- crates/jj-store-jj/src/objects.rs
- crates/jj-store-cli/src/remote.rs
- crates/jj-store-cli/tests/native_rpc.rs
- scripts/check-boundaries.mjs
- scripts/governance.test.mjs
- Cargo.toml
- Cargo.lock
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/native_rpc.rs#remote_native_factories_preserve_workspace_and_history_after_reconnect
- crates/jj-store-cli/tests/native_rpc.rs#rpc_authorization_and_native_validation_precede_immutable_storage
- crates/jj-store-cli/tests/native_rpc.rs#rpc_guarded_publication_preserves_receipts_siblings_and_protection
- crates/jj-store-cli/tests/native_rpc.rs#rpc_stream_protocol_deadlines_and_permits_are_bounded
- crates/jj-store-cli/tests/native_rpc.rs#rpc_requires_credentials_and_explicit_secure_transport_configuration
- crates/jj-store-cli/tests/native_rpc.rs#rpc_tls_checks_trust_and_server_hostname
- crates/jj-store-cli/tests/native_rpc.rs#rpc_download_rechecks_revocation_and_releases_permits_on_cancellation
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_rpc
```

**Notes**

Before implementation: no native_rpc target existed; no test cases executed.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo check --workspace
```

**Notes**

Initial compile: streamed reader error type ambiguity and async upload Send lifetime error. Subsequent compile attempts isolated the remaining lifetime error; solved with an owned bounded upload producer and explicit stream error type.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_rpc
```

**Notes**

Five loopback gRPC acceptance tests passed before TLS and mid-stream revocation coverage.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_rpc
```

**Notes**

Seven loopback tests pass, including authenticated native workspace binary snapshot/describe, real server reconnection and fresh cache/history recovery, immutable validation, current revocation, crafted publication protection, sibling heads, receipt retries, deadlines/cancellation and real TLS trust/hostname verification. Memory storage is not RustFS conformance.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check
```

**Notes**

Clippy found one redundant field-name initialization, reported for library and test targets; fixed without changing behavior.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check
```

**Notes**

Clippy all targets, Rust formatting, specification, seven package boundaries and progress bookkeeping pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test
```

**Notes**

87 Rust tests, 14 governance tests and 3 storage-harness self-tests pass. Two external RustFS tests remain ignored in this lane; no deployment conformance claim for RPC.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check
```

**Notes**

All nine lanes pass, including rustfmt, mapped spec-first, mdBook and the seven native RPC tests.
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

- Executable CLI/server command, configuration and operator/token administration
  remain.
- OS credential integration and the full command presentation/capability matrix
  remain.
- M3 remains incomplete; RPC library tests do not establish executable or
  deployment acceptance.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

jj-cli/jj-lib 0.45.1 / 7c41cdeb16b6b321c64e789a966b6adf723816a5; Rust 1.97.1; tonic/prost 0.14
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Introduces jj_store.native.v1 protobuf RPC with typed immutable objects and guarded head transitions. Native codecs, WAL, manifest, token policy and local cache formats unchanged. No raw control-key access. TLS is required except explicitly enabled literal loopback.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


