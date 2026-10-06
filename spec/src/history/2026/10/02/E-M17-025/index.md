---
schema: 1
id: E-M17-025
---

# E-M17-025

<!-- evidence-field:id markdown -->
## Id

E-M17-025
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

retained-owned-off-runtime-pipeline-step-execution-deadline-pause-drain-and-shutdown
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-server/src/capture_worker/executor/tests.rs
- crates/jj-store-server/src/capture_worker/executor.rs
- crates/jj-store-server/src/capture_worker.rs
- crates/jj-store-server/src/lib.rs
- crates/jj-store-server/tests/capture_pipeline_worker.rs
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/verification.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-server/tests/capture_pipeline_worker.rs#pipeline_work_runs_with_owned_resources_and_clean_shutdown
- crates/jj-store-server/tests/capture_pipeline_worker.rs#owned_step_failure_preserves_state_and_cannot_override_native_authority
- crates/jj-store-server/src/capture_worker/executor/tests.rs#blocking_io_deadline_retains_owner_and_keeps_runtime_responsive
- crates/jj-store-server/src/capture_worker/executor/tests.rs#pause_resume_cannot_replace_an_undrained_owner
- crates/jj-store-server/src/capture_worker/executor/tests.rs#expired_queued_work_never_starts_its_source_step
- crates/jj-store-server/src/capture_worker/executor/tests.rs#shutdown_timeout_retains_owner_until_settlement
- crates/jj-store-server/src/capture_worker/executor/tests.rs#completed_late_result_remains_known
- crates/jj-store-server/src/capture_worker/executor/tests.rs#task_loss_closes_the_owner_without_replacement
- crates/jj-store-server/src/capture_worker/executor/tests.rs#dropped_controller_keeps_running_resources_until_settlement
- crates/jj-store-server/src/capture_worker/executor/tests.rs#invalid_deadline_fails_before_dispatch
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M17-025 details 2](details-02.md)
- [E-M17-025 details 3](details-03.md)
