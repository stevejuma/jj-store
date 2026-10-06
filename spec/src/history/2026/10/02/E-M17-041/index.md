---
schema: 1
id: E-M17-041
---

# E-M17-041

<!-- evidence-field:id markdown -->
## Id

E-M17-041
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M17
- M17.2
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-02
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

implementation
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-server/src/capture_runtime.rs
- crates/jj-store-server/src/capture_supervisor.rs
- crates/jj-store-server/src/capture_supervisor/lifetime.rs
- crates/jj-store-server/src/capture_supervisor/scheduling.rs
- crates/jj-store-server/src/capture_worker/executor.rs
- crates/jj-store-server/src/storage.rs
- crates/jj-store-server/src/storage/lifetime.rs
- crates/jj-store-server/src/storage/lifetime/tests.rs
- crates/jj-store-server/tests/capture_runtime.rs
- crates/jj-store-server/tests/capture_runtime/draining.rs
- crates/jj-store-server/tests/capture_runtime/storage.rs
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/verification.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-server/tests/capture_runtime/storage.rs#cold_storage_lifetime_follows_capture_drain_drop_and_cancelled_shutdown
- crates/jj-store-server/src/storage/lifetime/tests.rs#storage_waits_for_last_capture_owner_and_holds_lock_through_child_shutdown
- crates/jj-store-server/src/storage/lifetime/tests.rs#dropped_storage_or_cancelled_wait_cannot_stop_a_retained_child
- crates/jj-store-server/src/storage/lifetime/tests.rs#detached_blocking_capture_task_retains_storage_on_completion_and_loss
- crates/jj-store-server/src/storage/lifetime/tests.rs#unconsumed_completion_retains_storage_until_source_state_is_dropped
- crates/jj-store-server/src/storage/lifetime/tests.rs#final_release_and_shutdown_registration_cannot_lose_a_wakeup
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M17-041 details 2](details-02.md)
- [E-M17-041 details 3](details-03.md)
