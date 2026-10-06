---
schema: 1
id: E-M3-012
---

# E-M3-012

<!-- evidence-field:id markdown -->
## Id

E-M3-012
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M3
- M3.1
- M3.3
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-27
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

typed-token-cli
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/src/commands.rs
- crates/jj-store-cli/src/lib.rs
- crates/jj-store-cli/src/server_commands.rs
- crates/jj-store-cli/src/token_commands.rs
- crates/jj-store-cli/tests/administration_rpc.rs
- crates/jj-store-cli/tests/server_executable.rs
- docs/cli/commands.md
- docs/cli/server-commands.md
- docs/cli/server-operator.md
- docs/cli/tokens.md
- docs/evidence/m3-token-cli-rustfs.json
- spec/src/cli.md
- spec/src/compatibility.md
- spec/src/security.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/administration_rpc.rs#executable_token_commands_share_receipts_and_never_expose_credentials
- crates/jj-store-cli/tests/administration_rpc.rs#executable_token_retry_rejects_changed_intent_stale_snapshot_and_wrong_authority
- crates/jj-store-cli/tests/administration_rpc.rs#executable_token_admin_rotation_and_bounded_stdin_preserve_request_identity
- crates/jj-store-cli/tests/administration_rpc.rs#executable_token_help_is_authored_for_both_families
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
target/debug/jj store token --help
```

**Notes**

Before implementation: token subcommand absent; exit 2.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo check -p jj-store-cli
```

**Notes**

Initial compilation found missing Display implementations for WalPosition and upstream CommandError. Use the position value and a shared owned connection error boundary.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo check -p jj-store-cli
```

**Notes**

Shared command implementation compiles after the corrections.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test administration_rpc executable_token
```

**Notes**

Four new executable acceptance tests pass: both command families, stable/late receipts, stale and changed intents, permission/actor isolation, same-principal admin rotation, bounded stdin, private paths and authored help.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test administration_rpc --test server_executable
```

**Notes**

13 cases pass; one real-storage case ignored in this focused lane.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools python3 scripts/rustfs-contract.py --run --server --output docs/evidence/m3-token-cli-rustfs.json
```

**Notes**

Nine real RustFS probes pass. Actual CLI grant, standalone revoke, late grant replay and live access denial are verified after cold recovery. Owned resources cleaned without errors.
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

All-target Clippy, rustfmt, boundaries, progress and specification pass.
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

129 Rust tests, 15 governance tests and 3 storage-harness self-tests pass; six external cases ignored in the normal lane.
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

Both development binaries and the workspace build.
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

All nine lanes pass including Rust tests, rustfmt, mapped spec-first and mdBook.
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

- Complete ordinary-command semantic parity and shared TLS/platform
  qualification remain.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.97.1; jj-cli/jj-lib 0.45.1 / 7c41cdeb16b6b321c64e789a966b6adf723816a5; pinned RustFS digest in report.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Local token-request journal V1 contains connection, actor, request ID, exact snapshot and intent; grant secrets are represented only by public ID and domain-separated fingerprint. Stored authority and native RPC formats unchanged.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


