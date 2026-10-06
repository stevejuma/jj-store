---
schema: 1
id: E-M10-009
---

# E-M10-009 details 2

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
cargo check -p jj-store-server -p jj-store-rpc -p jj-store-service --locked
```

**Notes**

The production server, RPC and service activation surface compiled after adding lifecycle-owned replication controls and status DTOs.
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
cargo test -p jj-store-rpc --test replication_control --locked; cargo test -p jj-store-server --test configuration replication_configuration_is_one_upstream_per_repository_bounded_and_secret_free --locked; cargo test -p jj-store-server --lib upstream_outage_keeps_local_authority_ready_and_controls_responsive --locked
```

**Notes**

Authenticated controls preserve finite issuer-bound frontiers; strict configuration permits one secret-free upstream per repository; an unreachable upstream leaves local authority and controls responsive.
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
cargo test -p jj-store-rpc --test replication authenticated_rpc_applies_a_closure_and_checkpoints_only_after_ack --offline
```

**Notes**

An authenticated import publishes only after durable acknowledgement, wakes committed-change SSE and advances the destination search projection through its imported local WAL position.
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
cargo +1.98.1 test --offline (temporary focused harness over the production sync_commands module)
```

**Notes**

The first focused CLI compile exposed two reconciliation JSON branches passing typed outcomes to the presentation function instead of serialized JSON values; both production call sites were corrected.
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
cargo +1.98.1 test --offline (temporary focused harness over the production sync_commands module)
```

**Notes**

The repaired production sync module compiles without the unrelated Kingfisher dependency and its bounded timeout and stable reconciliation identity tests pass.
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
cargo fmt --all -- --check && cargo test -p jj-store-server --locked
```

**Notes**

Rust formatting and all 24 server unit and integration tests pass; the two separately built embedded-RustFS runtime cases remain intentionally ignored in this package lane.
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
cargo test -p jj-store-rpc --locked && cargo test -p jj-store-replication --locked
```

**Notes**

All ten RPC tests and all 15 replication integration cases pass, covering controls, bidirectional convergence, retries, durable cursors, bootstrap and collaboration reducers.
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
cargo clippy -p jj-store-service -p jj-store-rpc -p jj-store-server -p jj-store-replication --all-targets --locked -- -D warnings
```

**Notes**

Every affected service, RPC, server and replication target passes warnings as errors.
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
cargo test -p jj-store-cli --lib sync_commands::tests::wait_timeout_parser_is_explicit_and_bounded --locked
```

**Notes**

The real custom CLI target compiles with its pinned dependencies and accepts explicit bounded wait units while rejecting zero, unitless, unsupported and over-five-minute timeouts.
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
cargo fmt --all && pnpm exec prettier --write docs/cli/commands.md docs/cli/server-config.md docs/cli/sync.md spec/src/milestones.md spec/src/replication.md && pnpm format:check && pnpm spec:validate && pnpm check:progress && pnpm check:boundaries
```

**Notes**

Rust and tracked-text formatting, all 102 requirements, the 70-task evidence ledger and all 14 package boundaries pass after documenting the activation and deferred hardening profiles.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 20
hasNote: true
noteEncoding: markdown

```

```shell
VITEST_MAX_WORKERS=1 pnpm spec:check
```

**Notes**

Specification validation and the cold catalog browser stage passed. The unrelated browser-interaction stage then timed out in all 20 existing native browser cases while waiting for the shared connect fixture to expose Repository ID; no replication assertion ran or failed.
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
pnpm check
```

**Notes**

Specification, boundaries, 243-module architecture, progress, Rust formatting, embedded RustFS clippy, 70 captured HTTP/SSE contracts, six contract tests, web/catalog diagnostics and locked workspace Clippy all pass.
<!-- /run -->

<!-- run -->
### Run 13

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

Governance, storage harness, both real RustFS restart cases, LocalTrusted executable, contracts, 54 web unit tests, 60 catalog tests, 85 Storybook tests, the cold catalog browser and the workspace suite through the CLI control tests passed. The unrelated Kingfisher redaction timing case then measured a 5.18-second nested scan against its strict under-five-second assertion while another verification workspace was active.
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
cargo test -p jj-store-cli --test agent_kingfisher kingfisher_redacts_secrets_and_normalizes_workspace_paths --locked -- --exact
```

**Notes**

The isolated retry passed with its nested cached scan at 3.38 seconds; the complete pnpm test command was not repeated.
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
pnpm build
```

**Notes**

Storybook, the production browser, the embedded RustFS runtime and the complete locked Rust workspace build successfully; existing CSS pseudo-class and bundle-size warnings remain non-fatal.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
cargo fmt --all; pnpm exec prettier --write spec/src/architecture.md spec/src/cli.md spec/src/jj-integration.md spec/src/mutations.md spec/src/milestones.md; pnpm spec:first
```

**Notes**

The four additional protected ownership chapters describe only the active replication contract, the tracked text is formatted and the 25-file protected change is covered by six canonical chapters.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress; pnpm spec:validate; pnpm check:boundaries; pnpm format:check
```

**Notes**

The 70-task evidence ledger, all 102 canonical requirements, all 14 package boundaries and Rust and tracked-text formatting pass after the activation ownership update.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- The repository-wide specification gate remains red in the unrelated
  browser-interaction connect fixture, and the complete pnpm test command
  retains one timing-only Kingfisher failure even though its isolated retry
  passed.
- M10.8 defers exhaustive pressure and fault soak, expanded memory and disk
  matrices, browser replication UI, cold two-instance real-RustFS replication,
  pressure archive/export tooling and additional deployment topology
  qualification.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; jj 0.45.1; tonic/prost 0.14.6; manifest format 2; replication protocol, peer checkpoint, control and wait schema v1.
<!-- /evidence-field -->

