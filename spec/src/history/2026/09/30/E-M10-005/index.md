---
schema: 1
id: E-M10-005
---

# E-M10-005

<!-- evidence-field:id markdown -->
## Id

E-M10-005
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M10
- M10.3
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-30
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

authenticated-replication-transport-and-peer-cursors
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- Cargo.lock
- catalog/controller/in-memory-store-collaboration.svelte.ts
- catalog/fixtures/collaboration-review.ts
- crates/jj-store-replication/Cargo.toml
- crates/jj-store-replication/src/checkpoint.rs
- crates/jj-store-replication/src/lib.rs
- crates/jj-store-replication/src/protocol.rs
- crates/jj-store-replication/tests/checkpoint.rs
- crates/jj-store-rpc/Cargo.toml
- crates/jj-store-rpc/proto/native.proto
- crates/jj-store-rpc/src/client.rs
- crates/jj-store-rpc/src/lib.rs
- crates/jj-store-rpc/src/server.rs
- crates/jj-store-rpc/tests/replication.rs
- crates/jj-store-service/Cargo.toml
- crates/jj-store-service/src/guard.rs
- crates/jj-store-service/src/native.rs
- crates/jj-store-service/src/transport.rs
- crates/jj-store-wal/src/publication/replication_import.rs
- crates/jj-store-wal/src/publication/replication_protocol.rs
- crates/jj-store-wal/src/publication/replication_transfer.rs
- crates/jj-store-wal/src/records.rs
- spec/src/milestones.md
- spec/src/replication.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-replication/tests/checkpoint.rs#checkpoint_is_issuer_bound_monotonic_and_recovers_a_lost_ack
- crates/jj-store-rpc/tests/replication.rs#authenticated_rpc_applies_a_closure_and_checkpoints_only_after_ack
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M10-005 details 2](details-02.md)
- [E-M10-005 details 3](details-03.md)
