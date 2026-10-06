---
schema: 1
id: E-M10-009
---

# E-M10-009

<!-- evidence-field:id markdown -->
## Id

E-M10-009
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M10
- M10.7
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-30
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

minimal-production-replication-activation
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- Cargo.lock
- crates/jj-store-cli/src/commands.rs
- crates/jj-store-cli/src/sync_commands.rs
- crates/jj-store-rpc/Cargo.toml
- crates/jj-store-rpc/proto/native.proto
- crates/jj-store-rpc/src/client.rs
- crates/jj-store-rpc/src/server.rs
- crates/jj-store-rpc/tests/replication.rs
- crates/jj-store-rpc/tests/replication_control.rs
- crates/jj-store-server/Cargo.toml
- crates/jj-store-server/src/config/format-2-example.toml
- crates/jj-store-server/src/config/types.rs
- crates/jj-store-server/src/config.rs
- crates/jj-store-server/src/installation.rs
- crates/jj-store-server/src/lib.rs
- crates/jj-store-server/src/process.rs
- crates/jj-store-server/src/replication.rs
- crates/jj-store-server/tests/configuration.rs
- crates/jj-store-service/src/lib.rs
- crates/jj-store-service/src/native.rs
- crates/jj-store-service/src/replication_control.rs
- crates/jj-store-service/src/transport.rs
- docs/cli/commands.md
- docs/cli/server-config.md
- docs/cli/sync.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/jj-integration.md
- spec/src/milestones.md
- spec/src/mutations.md
- spec/src/replication.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/src/sync_commands.rs#wait_timeout_parser_is_explicit_and_bounded
- crates/jj-store-rpc/tests/replication.rs#authenticated_rpc_applies_a_closure_and_checkpoints_only_after_ack
- crates/jj-store-rpc/tests/replication.rs#bounded_worker_retries_with_stable_identity_and_converges_both_directions
- crates/jj-store-rpc/tests/replication_control.rs#native_control_rpc_preserves_finite_frontiers_and_authorized_controls
- crates/jj-store-server/src/replication.rs#upstream_outage_keeps_local_authority_ready_and_controls_responsive
- crates/jj-store-server/tests/configuration.rs#replication_configuration_is_one_upstream_per_repository_bounded_and_secret_free
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M10-009 details 2](details-02.md)
- [E-M10-009 details 3](details-03.md)
