---
schema: 1
id: E-VAULT-BACKUP-026
---

# E-VAULT-BACKUP-026

<!-- evidence-field:id markdown -->
## Id

E-VAULT-BACKUP-026
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M34.5
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-LOCAL-007
- JST-DOC-002
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-11-15
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-jj/src/vault_workspace/mod.rs
- crates/jj-store-jj/src/vault_workspace/remote/mod.rs
- crates/jj-store-jj/src/vault_workspace/remote/endpoint.rs
- crates/jj-store-jj/src/vault_workspace/remote/process.rs
- crates/jj-store-jj/src/vault_workspace/remote/adoption.rs
- crates/jj-store-jj/tests/vault_transport_validation.rs
- crates/jj-vault/src/lib.rs
- docs/site/technical/local.md
- docs/maintenance/coverage.json
- spec/src/current/contracts/local-jj/32-automatic-vault-backup.md
- spec/src/current/contracts/jj-integration/28-vault-working-copy.md
- spec/src/current/contracts/documentation/03-delivery-and-validation.md
- spec/src/current/progress/M34/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-jj/tests/vault_transport_validation.rs#admits_host_managed_ssh_endpoints
- crates/jj-store-jj/tests/vault_transport_validation.rs#rejects_embedded_secrets_and_executable_transports
- crates/jj-store-jj/tests/vault_transport_validation.rs#authentication_protocol_mismatch_refuses_before_initializing_storage
- crates/jj-store-jj/tests/vault_remote.rs#ordinary_git_backup_restores_files_and_native_history_on_a_fresh_device
- crates/jj-store-jj/tests/vault_remote.rs#remote_drift_rejects_publication_and_unknown_intents_are_observed_without_replay
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
pnpm dev:cargo -- test -p jj-store-jj --no-default-features --features vault-backup --test vault_transport_validation
```

**Notes**

Before: 1 passed and 1 failed; existing target validation rejects SSH. The original vault_remote baseline passed 4 tests, but the wrapper rejected its post-build free-space threshold. No gate was reported as passing from that wrapper failure.
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
pnpm dev:cargo -- test -p jj-store-jj --no-default-features --features vault-backup --test vault_transport_validation
```

**Notes**

Final exact target: 3 passed. Endpoint validation, embedded-secret/executable-transport rejection, and auth/protocol mismatch before storage initialization. Earlier after-runs passed assertions but failed disk postflight; eligible idle compiler caches were cleared before the final pass.
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
pnpm dev:cargo -- test -p jj-store-jj --no-default-features --features vault-backup --test vault_transport_validation --test vault_remote --lib
```

**Notes**

Final complete selected surface: 62 library tests passed, 3 existing platform/opt-in tests ignored; 4 existing real local-Git remote cases and 3 transport validation cases passed. An intermediate credentials-only filter selected zero integration cases and was not counted as their qualification.
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
pnpm dev:cargo -- check -p jj-vault
pnpm dev:cargo -- clippy -p jj-store-jj -p jj-vault --no-default-features --features jj-store-jj/vault-backup --lib -- -D warnings
cargo fmt --all --check
```

**Notes**

Direct consumer check and strict Clippy passed. Initial Clippy found one collapsible-if diagnostic; corrected without suppression.
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
pnpm spec:search --semantic "vault backup remote SSH original repository fetch selected bookmark push"
pnpm exec spec-validator doctor
```

**Notes**

QMD found both governing chapters. After a fresh ignore-scripts install, doctor detected unbuilt native modules; explicit rebuild of better-sqlite3 and node-llama-cpp restored doctor to 10 passes and the existing script-alias warning.
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
node scripts/spec/progress-validation.mjs
pnpm spec:validate
pnpm spec:first -- --base 3cc59c48 --head @
```

**Notes**

Progress and mapped spec-first passed; 8 validators passed across 4,651 chapters and 211 requirements. The global evidence:new attempt failed on an existing oversized generated index. Restored all 747 unrelated generated paths and added only the new receipt/day navigation. Initial receipt validation rejected path-only test references and an unsupported partial result; references now name actual tests and result remains pending for real transport/host qualification.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Real SSH/HTTPS-token transport, original-repository remote operations, shared
  Core/Protocol integration and cold Desktop/Server acceptance remain
  unqualified.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust/Cargo 1.99.0; jj-lib 0.46.0; Node 24.10.0; pnpm 10.32.1; spec-validator 0.2.0; local macOS ARM64.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No stored binding, publication receipt, native identity or checkpoint format changes.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Separate anonymous HTTPS and host-agent/default SSH backup transport entry points. HTTPS tokens cannot be sent through SSH. Child execution keeps strict known-host checks, disables interactive authentication and scopes token headers to the approved repository endpoint. No host adopts the new capabilities in this slice.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Complete source gates, then qualify actual disposable SSH/HTTPS transport during host integration; retain M34.5 Partial.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-LOCAL-007 now admits anonymous HTTPS and host-managed SSH through
  explicit native entry points; original-repository operations remain outside
  this source slice.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: "M34.5 remains Partial: source primitives and regression proof are
  separate from real transport, provider and cold consumer qualification."

```
<!-- /evidence-field -->

