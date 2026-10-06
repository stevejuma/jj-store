---
schema: 1
id: E-M3-007
---

# E-M3-007

<!-- evidence-field:id markdown -->
## Id

E-M3-007
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

typed-administration
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-service/src/administration.rs
- crates/jj-store-service/src/auth.rs
- crates/jj-store-service/src/lib.rs
- crates/jj-store-service/src/native.rs
- crates/jj-store-service/tests/administration.rs
- crates/jj-store-wal/src/publication.rs
- crates/jj-store-wal/tests/publication.rs
- spec/src/mutations.md
- spec/src/publication.md
- spec/src/security.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-service/tests/administration.rs#creation_requires_fresh_operator_permission_before_storage_and_is_retryable
- crates/jj-store-service/tests/administration.rs#token_grants_lists_and_revoke_are_admin_scoped_and_never_persist_secrets
- crates/jj-store-service/tests/administration.rs#token_retries_recover_original_receipts_after_later_changes_and_restart
- crates/jj-store-service/tests/administration.rs#stale_concurrent_token_mutations_cannot_restore_revoked_grants
- crates/jj-store-service/tests/administration.rs#token_action_identity_and_current_authorization_apply_to_receipt_queries
- crates/jj-store-service/tests/administration.rs#client_generated_grants_are_exact_idempotent_and_reject_token_id_collisions
- crates/jj-store-service/tests/administration.rs#unknown_token_publication_resolves_by_original_request_without_issuing_again
- crates/jj-store-wal/tests/publication.rs#historical_settings_bind_selected_snapshot_and_survive_checkpoint
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-service --test administration
```

**Notes**

Before implementation: administration target absent; no test cases executed.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-service --test administration
```

**Notes**

Initial fixture compile had ambiguous str::contains argument; corrected explicit string slice.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-service --test administration
```

**Notes**

Initial six administration tests pass before unknown-outcome and checkpoint extensions.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-wal --test publication historical_settings_bind_selected_snapshot_and_survive_checkpoint
```

**Notes**

Historical policy lookup verifies exact selected snapshots across checkpoint/restart and fails on corruption.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-service --test administration
```

**Notes**

All seven tests pass, including explicit unknown-outcome recovery, late receipt replay after checkpoint/restart, secret-free persistence and a concurrent revocation race.
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

Clippy all targets, rustfmt, specification, boundaries and progress pass.
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

101 Rust tests, 14 governance tests and 3 storage-harness self-tests pass. Four external tests remain explicitly ignored in this lane.
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

Workspace and development executable build pass.
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

All nine specification lanes pass, including full Rust tests, mapped spec-first and mdBook.
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

- Service administration is not yet exposed through RPC or server commands.
  Operator file provisioning and refresh wiring remain.
- M3 executable server creation, configuration, diagnostics, serve-local, attach
  recovery and complete ordinary-command parity remain.
- Administration fixtures use memory storage and do not establish real RustFS
  deployment conformance.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

jj-cli/jj-lib 0.45.1 / 7c41cdeb16b6b321c64e789a966b6adf723816a5; Rust 1.97.1
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No persisted format change: token policy, native objects, WAL, manifests and receipts remain V1. Token administration derives deterministic publication IDs and encodes secret-free typed results; historical settings lookup follows selected catalogs.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


