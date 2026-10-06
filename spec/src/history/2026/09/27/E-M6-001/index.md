---
schema: 1
id: E-M6-001
---

# E-M6-001

<!-- evidence-field:id markdown -->
## Id

E-M6-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M6
- M6.1
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-27
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

adapter-delivery-checkpoints
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- Cargo.lock
- Cargo.toml
- crates/jj-store-adapters/Cargo.toml
- crates/jj-store-adapters/src/context.rs
- crates/jj-store-adapters/src/lib.rs
- crates/jj-store-adapters/src/progress.rs
- crates/jj-store-adapters/src/runner.rs
- crates/jj-store-adapters/src/source.rs
- crates/jj-store-adapters/tests/delivery.rs
- crates/jj-store-adapters/tests/delivery/support.rs
- crates/jj-store-cli/Cargo.toml
- crates/jj-store-cli/tests/native_lifecycle.rs
- crates/jj-store-cli/tests/native_lifecycle/adapters.rs
- crates/jj-store-service/Cargo.toml
- crates/jj-store-service/src/adapter.rs
- crates/jj-store-service/src/lib.rs
- crates/jj-store-wal/src/publication.rs
- crates/jj-store-wal/src/publication/reader.rs
- crates/jj-store-wal/tests/publication.rs
- docs/evidence/m6-adapter-delivery.md
- scripts/check-boundaries.mjs
- spec/src/adapters.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/compatibility.md
- spec/src/milestones.md
- spec/src/mutations.md
- spec/src/publication.md
- spec/src/security.md
- spec/src/spec-governance.md
- crates/jj-store-cli/tests/native_rpc.rs
- spec/src/verification.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-adapters/tests/delivery.rs#adapter_replays_only_selected_records_and_bootstraps_after_artifact_loss
- crates/jj-store-adapters/tests/delivery.rs#adapter_effect_crashes_and_poison_never_advance_or_skip_source
- crates/jj-store-adapters/tests/delivery.rs#adapter_cursor_crashes_reconcile_durable_artifact_after_restart
- crates/jj-store-adapters/tests/delivery.rs#adapter_restored_older_artifact_catches_up_without_remote_regression_and_config_rebuilds
- crates/jj-store-adapters/tests/delivery.rs#adapter_denial_limits_and_corrupt_selected_data_fail_without_advancing
- crates/jj-store-adapters/tests/delivery.rs#adapter_timeout_keeps_writer_and_admission_until_background_work_exits
- crates/jj-store-adapters/tests/delivery.rs#adapter_polling_retries_poison_with_health_and_stops_on_cancellation
- crates/jj-store-adapters/tests/delivery.rs#adapter_corrupt_native_payload_and_noncanonical_or_foreign_cursor_are_rejected
- crates/jj-store-adapters/tests/delivery.rs#adapter_bootstrap_rejects_corrupt_selected_tail_before_destination_effect
- crates/jj-store-cli/tests/native_lifecycle/adapters.rs#native_adapter_source_verifies_committed_objects_and_rechecks_revocation
- crates/jj-store-wal/tests/publication.rs#selected_reader_keeps_exact_committed_refs_and_inventory_across_checkpoint
- crates/jj-store-cli/tests/native_rpc.rs#rpc_stream_protocol_deadlines_and_permits_are_bounded
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M6-001 details 2](details-02.md)
- [E-M6-001 details 3](details-03.md)
