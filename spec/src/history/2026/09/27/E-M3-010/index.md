---
schema: 1
id: E-M3-010
---

# E-M3-010

<!-- evidence-field:id markdown -->
## Id

E-M3-010
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M3
- M3.2
- M3.3
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-27
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

resumable-native-attachment
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/Cargo.toml
- crates/jj-store-cli/src/attachment.rs
- crates/jj-store-cli/src/commands.rs
- crates/jj-store-cli/src/lib.rs
- crates/jj-store-cli/tests/attachment_recovery.rs
- docs/cli/attach.md
- docs/cli/commands.md
- docs/evidence/m3-attachment-rustfs.json
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/compatibility.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/attachment_recovery.rs#accepted_attachment_retries_original_operation_after_lost_ack_and_later_publication
- crates/jj-store-cli/tests/attachment_recovery.rs#prepared_attachment_survives_cache_loss_and_rejects_changed_retry_identity
- crates/jj-store-cli/tests/attachment_recovery.rs#revoked_attachment_cannot_replay_receipt_or_expose_physical_metadata
- crates/jj-store-cli/tests/attachment_recovery.rs#recovery_preserves_unowned_physical_metadata_stage
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test attachment_recovery
```

**Notes**

Before implementation: test target absent; zero cases executed.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo check -p jj-store-cli
```

**Notes**

Workspace-name comparison used RefSymbol against String; changed to WorkspaceName accessor.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test attachment_recovery
```

**Notes**

Follow-up compile used a nonexistent RefSymbol::as_str; fixed to WorkspaceName::as_str.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test attachment_recovery
```

**Notes**

Initial two cases lacked operation.hostname in their constructed test settings.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test attachment_recovery
```

**Notes**

After hostname correction the same two cases lacked signing.behavior; the expanded four-case target reproduced four fixture failures. Fixed by using the upstream default settings stack.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test attachment_recovery
```

**Notes**

Expanded target still used incomplete fixture settings before default-stack correction.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test attachment_recovery
```

**Notes**

Two pass; two completed-retry paths fail because upstream new-workspace settings helper assumes .jj/repo is a directory. Completed retries now reuse settings without that initializer.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test attachment_recovery
```

**Notes**

Four cases pass: original-operation receipt retry after later publication, cache-loss recovery and changed-intent/lock rejection, revoked receipt denial, and preservation of an unowned physical stage.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test executable_native --test administration_rpc --test server_executable
```

**Notes**

15 cases pass; three external cases ignored in this focused lane.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools python3 scripts/rustfs-contract.py --run --server --output docs/evidence/m3-attachment-rustfs.json
```

**Notes**

Nine pinned RustFS probes pass; actual server/CLI cold recovery and revocation retained. No cleanup errors.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check
```

**Notes**

All-target Clippy, rustfmt, specification, boundaries and progress pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test
```

**Notes**

123 Rust tests, 15 governance tests and 3 storage-harness self-tests pass. Five external tests ignored in normal lane.
<!-- /run -->

<!-- run -->
### Run 13

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
### Run 14

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

All nine lanes pass including full Rust suite, mapped spec-first, rustfmt and mdBook.
<!-- /run -->

<!-- run -->
### Run 15

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

- Repository init and token administration CLI commands and complete semantic
  parity remain.
- Shared TLS/platform qualification remain.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M3-010 details 2](details-02.md)
