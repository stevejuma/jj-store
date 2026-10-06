---
schema: 1
id: E-M4-015
---

# E-M4-015

<!-- evidence-field:id markdown -->
## Id

E-M4-015
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4
- M4.2
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-27
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

committed-event-replay-and-authenticated-sse
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/native_lifecycle/events.rs
- crates/jj-store-cli/tests/native_lifecycle/http_events.rs
- crates/jj-store-cli/tests/native_lifecycle.rs
- crates/jj-store-cli/tests/server_executable.rs
- crates/jj-store-http/Cargo.toml
- crates/jj-store-http/src/error.rs
- crates/jj-store-http/src/lib.rs
- crates/jj-store-http/src/routes.rs
- crates/jj-store-service/src/administration.rs
- crates/jj-store-service/src/events.rs
- crates/jj-store-service/src/lib.rs
- crates/jj-store-service/src/native.rs
- crates/jj-store-service/src/transport.rs
- docs/evidence/m4-committed-events.md
- spec/src/api.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/compatibility.md
- spec/src/milestones.md
- spec/src/mutations.md
- spec/src/security.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/native_lifecycle/events.rs#events_replay_only_committed_positions_across_restart_and_checkpoint
- crates/jj-store-cli/tests/native_lifecycle/events.rs#events_reject_foreign_cursors_and_reset_expired_future_and_replaced_epochs
- crates/jj-store-cli/tests/native_lifecycle/events.rs#events_reauthorize_queued_records_after_revocation
- crates/jj-store-cli/tests/native_lifecycle/events.rs#events_local_publication_wakes_shared_host_without_waiting_for_poll
- crates/jj-store-cli/tests/native_lifecycle/events.rs#events_corrupt_selected_record_fails_without_skipping_or_resetting
- crates/jj-store-cli/tests/native_lifecycle/events.rs#events_epoch_replacement_resets_before_queued_delivery
- crates/jj-store-cli/tests/native_lifecycle/events.rs#events_storage_deadlines_close_without_advancing_resume_position
- crates/jj-store-cli/tests/native_lifecycle/events.rs#events_lagged_wake_hints_replay_every_selected_record_in_bounded_pages
- crates/jj-store-cli/tests/native_lifecycle/http_events.rs#http_events_resume_restart_and_reset_use_selected_publications
- crates/jj-store-cli/tests/native_lifecycle/http_events.rs#http_events_have_separate_admission_and_expire_unpolled_streams
- crates/jj-store-cli/tests/native_lifecycle/http_events.rs#http_events_validate_resume_headers_and_reset_without_replaying_ahead
- crates/jj-store-cli/tests/native_lifecycle/http_events.rs#http_events_revocation_terminates_queued_delivery
- crates/jj-store-cli/tests/server_executable.rs#real_rustfs_server_cli_restart_and_revocation
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M4-015 details 2](details-02.md)
- [E-M4-015 details 3](details-03.md)
