---
schema: 1
id: E-M17-040
---

# E-M17-040

<!-- evidence-field:id markdown -->
## Id

E-M17-040
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
- crates/jj-store-server/src/lib.rs
- crates/jj-store-server/tests/capture_runtime/draining.rs
- crates/jj-store-server/tests/capture_runtime.rs
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/verification.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-server/tests/capture_runtime.rs#owned_runtime_drives_discovery_active_capture_pause_and_terminal_shutdown
- crates/jj-store-server/tests/capture_runtime.rs#runtime_shutdown_does_not_acknowledge_a_backpressured_raw_frame
- crates/jj-store-server/tests/capture_runtime/draining.rs#shutdown_timeout_retains_join_and_raw_ownership_until_a_later_wait
- crates/jj-store-server/tests/capture_runtime/draining.rs#dropping_runtime_requests_stop_while_blocking_owner_retains_raw_channel
- crates/jj-store-server/tests/capture_runtime.rs#failed_driver_marks_controls_terminal_and_never_repolls_a_completed_join
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M17-040 details 2](details-02.md)
- [E-M17-040 details 3](details-03.md)
