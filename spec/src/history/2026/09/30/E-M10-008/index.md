---
schema: 1
id: E-M10-008
---

# E-M10-008

<!-- evidence-field:id markdown -->
## Id

E-M10-008
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M10
- M10.6
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-30
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

replication-seed-checkpoint-bootstrap-and-workspace-migration
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- Cargo.lock
- crates/jj-store-cli/Cargo.toml
- crates/jj-store-cli/src/commands.rs
- crates/jj-store-cli/src/lib.rs
- crates/jj-store-cli/src/migration.rs
- crates/jj-store-cli/src/replication_endpoint.rs
- crates/jj-store-cli/src/store_commands.rs
- crates/jj-store-cli/tests/workspace_migration.rs
- crates/jj-store-replication/Cargo.toml
- crates/jj-store-replication/src/worker.rs
- crates/jj-store-replication/tests/bootstrap.rs
- crates/jj-store-service/src/guard.rs
- crates/jj-store-service/src/native.rs
- crates/jj-store-wal/src/publication.rs
- crates/jj-store-wal/src/publication/agent_catalog.rs
- crates/jj-store-wal/src/publication/replication.rs
- crates/jj-store-wal/src/publication/replication_domains.rs
- crates/jj-store-wal/src/publication/replication_import.rs
- crates/jj-store-wal/src/publication/replication_protocol.rs
- crates/jj-store-wal/src/publication/replication_review.rs
- crates/jj-store-wal/src/publication/replication_seed.rs
- crates/jj-store-wal/src/publication/replication_transfer.rs
- crates/jj-store-wal/src/publication/review_catalog.rs
- crates/jj-store-wal/src/records.rs
- docs/cli/commands.md
- docs/cli/migrate-local.md
- docs/cli/store.md
- spec/src/milestones.md
- spec/src/replication.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-replication/tests/bootstrap.rs#seed_checkpoint_plus_tail_merges_three_roots_without_replacing_local_authority
- crates/jj-store-replication/tests/bootstrap.rs#worker_recovers_a_retained_feed_gap_from_seed_then_replays_the_pinned_tail
- crates/jj-store-cli/tests/workspace_migration.rs#nonempty_workspace_migrates_without_snapshot_and_mutates_with_upstream_offline
<!-- /evidence-field -->

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
cargo test -p jj-store-replication --test bootstrap --locked
```

**Notes**

A format-2 repository without replication metadata publishes one retryable four-domain seed. A destination with its own pending origin imports that complete baseline plus a concurrently authored tail while retaining its local manifest, replica stream, pending AgentStore session and every source code, agent and review identity.
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
cargo test -p jj-store-wal -p jj-store-replication -p jj-store-service -p jj-store-rpc --locked
```

**Notes**

All WAL, replication, service and RPC unit, integration and documentation tests pass. This includes 16 replication cases, 31 WAL publication cases and the existing authenticated bidirectional RPC worker acceptance.
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
cargo clippy -p jj-store-wal -p jj-store-replication -p jj-store-service -p jj-store-rpc --all-targets --locked -- -D warnings
```

**Notes**

The complete changed native package set passes warnings as errors.
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
pnpm spec:validate
```

**Notes**

All 4 validators accept the 18 canonical chapters and 102 requirements.
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
pnpm check:progress
```

**Notes**

The 69-task implementation ledger is internally consistent with tracked tests and files.
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
pnpm format:check
```

**Notes**

Rustfmt and Prettier accept the complete change.
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
pnpm check:boundaries
```

**Notes**

All 14 package boundaries remain valid.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:check
```

**Notes**

Specification validation, the cold catalog browser lane, Storybook build and web build passed. Browser interaction fixture compilation then stopped because the pinned Kingfisher Vectorscan build could not execute missing cmake (os error 2).
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check
```

**Notes**

Specification, package-boundary, web-architecture, progress, Rust formatting and embedded-RustFS Clippy checks passed. Contract capture then stopped at the pinned Kingfisher Vectorscan build because cmake is unavailable.
<!-- /run -->

<!-- run -->
### Run 10

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

Governance, storage-harness self-tests and both embedded and managed real-RustFS restart tests passed. The LocalTrusted CLI executable lane then stopped at the pinned Kingfisher Vectorscan build because cmake is unavailable.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm build
```

**Notes**

Storybook, web and embedded-RustFS builds passed. The workspace Cargo build then stopped at the pinned Kingfisher Vectorscan build because cmake is unavailable.
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
cargo test -p jj-store-replication --locked
```

**Notes**

All 15 replication integration cases pass after exposing an explicit one-direction migration step; ordinary worker fairness behavior remains unchanged.
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
cargo clippy -p jj-store-replication --all-targets --locked -- -D warnings
```

**Notes**

The migration-specific directional worker API passes warnings as errors.
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
cargo +1.98.1 check --offline (temporary focused harness over the production attachment, binding, remote and migration modules)
```

**Notes**

The production migration module and every directly used client module type-check together without the unrelated Kingfisher CLI dependency. This is compile evidence, not a substitute for the executable acceptance.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-cli --test workspace_migration --locked
```

**Notes**

The authored nonempty-workspace, exact working-copy preservation and offline-local-mutation acceptance could not compile because the pinned Kingfisher Vectorscan build could not execute missing cmake (os error 2).
<!-- /run -->
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M10-008 details 2](details-02.md)
