---
schema: 1
id: E-M3-008
---

# E-M3-008

<!-- evidence-field:id markdown -->
## Id

E-M3-008
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

typed-administration-rpc
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- Cargo.lock
- crates/jj-store-cli/tests/administration_rpc.rs
- crates/jj-store-rpc/Cargo.toml
- crates/jj-store-rpc/build.rs
- crates/jj-store-rpc/proto/native.proto
- crates/jj-store-rpc/src/administration.rs
- crates/jj-store-rpc/src/client.rs
- crates/jj-store-rpc/src/lib.rs
- crates/jj-store-rpc/src/server.rs
- crates/jj-store-service/src/administration.rs
- crates/jj-store-service/src/native.rs
- crates/jj-store-service/src/transport.rs
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/jj-integration.md
- spec/src/mutations.md
- spec/src/security.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/administration_rpc.rs#rpc_created_repositories_attach_with_real_cli_and_survive_host_restart
- crates/jj-store-cli/tests/administration_rpc.rs#rpc_administration_separates_operator_admin_and_cross_repository_access
- crates/jj-store-cli/tests/administration_rpc.rs#rpc_token_retries_and_competing_snapshots_preserve_accepted_policy
- crates/jj-store-cli/tests/administration_rpc.rs#rpc_rejects_malformed_admin_requests_without_publication_and_redacts_secrets
- crates/jj-store-rpc/src/administration.rs#token_receipt_validation_binds_action_request_and_exact_snapshot
- crates/jj-store-rpc/src/administration.rs#admin_decoding_clears_credentials_and_rejects_unbounded_or_duplicate_results
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test administration_rpc
```

**Notes**

Before implementation: administration_rpc target absent; no cases executed.
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

Initial compile produced two diagnostics for one ambiguous Result error type in optional receipt conversion. Focused test compilation reproduced the same two diagnostics. Fixed explicit tonic Status type.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test administration_rpc
```

**Notes**

Same receipt-conversion compile ambiguity, before explicit Status annotation.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test administration_rpc
```

**Notes**

Four loopback acceptance tests pass: typed creation to real CLI attachment/describe/restart, permission and cross-repository denial, competing stale updates with durable receipts, malformed requests and secret redaction.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-rpc --lib
```

**Notes**

Two protocol validation tests pass: exact receipt action/request/snapshot binding, plaintext clearing and response bounds.
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

Clippy all targets, rustfmt, specification, dependency boundaries and progress pass.
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

107 Rust tests, 14 governance tests and 3 storage-harness self-tests pass. Four external tests remain explicitly ignored in this lane.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build
```

**Notes**

Workspace and custom development jj executable build pass.
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

All nine specification lanes pass, including full Rust tests, mapped spec-first, rustfmt and mdBook.
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

- Standalone server commands, durable operator provisioning/configuration, init
  and serve-local remain.
- Real executable RustFS deployment, attachment recovery, full CLI semantic
  parity and release platform acceptance remain gated.
- Loopback production RPC uses memory storage in these tests; no additional
  deployment-conformance claim.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

jj-cli/jj-lib 0.45.1 / 7c41cdeb16b6b321c64e789a966b6adf723816a5; Rust 1.97.1; tonic/prost 0.14
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive native RPC V1 creation and typed token list/apply/status messages. No native, token-policy, WAL, manifest or receipt storage-format changes. Credential-bearing protobuf Debug is redacted and plaintext fields are zeroized on drop.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


