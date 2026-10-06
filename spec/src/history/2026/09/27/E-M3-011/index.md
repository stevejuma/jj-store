---
schema: 1
id: E-M3-011
---

# E-M3-011

<!-- evidence-field:id markdown -->
## Id

E-M3-011
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M3
- M3.1
- M3.2
- M3.3
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-27
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

durable-cli-initialization
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/src/attachment.rs
- crates/jj-store-cli/src/commands.rs
- crates/jj-store-cli/src/credentials.rs
- crates/jj-store-cli/src/initialization.rs
- crates/jj-store-cli/src/lib.rs
- crates/jj-store-cli/tests/attachment_recovery.rs
- crates/jj-store-cli/tests/executable_native.rs
- crates/jj-store-cli/tests/server_executable.rs
- docs/cli/commands.md
- docs/cli/init.md
- docs/cli/store.md
- docs/evidence/m3-init-rustfs.json
- spec/src/cli.md
- spec/src/compatibility.md
- spec/src/security.md
- README.md
- spec/src/architecture.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/attachment_recovery.rs#executable_init_reuses_durable_identity_after_denial_and_lost_creation_ack
- crates/jj-store-cli/tests/attachment_recovery.rs#executable_init_rejects_changed_epoch_or_credential_and_preserves_existing_files
- crates/jj-store-cli/tests/executable_native.rs#os_credential_initial_admin_provisioning_is_repeatable
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
target/debug/jj store init --help
```

**Notes**

Before implementation: unrecognized init subcommand; exit 2.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo check -p jj-store-cli
```

**Notes**

Initial implementation compiles.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test attachment_recovery executable_init
```

**Notes**

Two executable cases pass: denied creation then same-ID retry, lost acknowledgement after operator revocation, existing source preservation, changed epoch/credential denial and native history after init.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test attachment_recovery
```

**Notes**

All six init/attachment cases pass after shared helper extraction.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools python3 scripts/rustfs-contract.py --run --server --output docs/evidence/m3-init-rustfs.json
```

**Notes**

Nine real-storage probes pass using actual jj store init followed by exact repeat, CLI edits, server/storage kill, cache deletion, history and revocation. No cleanup errors.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test executable_native os_credential_initial_admin_provisioning_is_repeatable -- --ignored
```

**Notes**

One real OS-facility case passes: random initial-admin account is generated once, reused, read and cleaned up.
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

All-target Clippy, rustfmt, specification, boundaries and progress pass.
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

125 Rust tests, 15 governance tests and 3 storage-harness self-tests pass. Six external cases remain ignored in the normal lane; real RustFS and initial-admin OS-facility acceptance ran separately.
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

Workspace and both development executables build.
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

All nine lanes pass including full Rust suite, rustfmt, mapped spec-first and mdBook.
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
pnpm format:check
```

**Notes**

Rustfmt and Prettier pass.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Repository token administration CLI commands and complete ordinary-command
  semantic parity remain.
- Shared TLS/platform qualification remain.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.97.1; jj-cli/jj-lib 0.45.1 / 7c41cdeb16b6b321c64e789a966b6adf723816a5; pinned RustFS image in retained report.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

New local secret-free creation journal V1 preserves generated repository ID, epoch, credential ID, connection and attachment paths. Initial credentials are durable separately in the OS facility or automation environment. Authority/object/manifest/receipt/RPC formats unchanged.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


