---
schema: 1
id: E-M17-032
---

# E-M17-032

<!-- evidence-field:id markdown -->
## Id

E-M17-032
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

- crates/jj-store-capture/src/agent_capture/budget.rs
- crates/jj-store-capture/src/agent_capture.rs
- crates/jj-store-capture/src/agent_publication.rs
- crates/jj-store-capture/src/local_agent_publication.rs
- crates/jj-store-capture/tests/codex_activity.rs
- crates/jj-store-capture/tests/codex_recovery.rs
- crates/jj-store-capture/tests/reader_progress/headroom.rs
- crates/jj-store-capture/tests/reader_progress.rs
- crates/jj-store-capture/tests/transcript_sanitization.rs
- crates/jj-store-server/tests/capture_parking.rs
- crates/jj-store-server/tests/capture_supervisor/backlog.rs
- crates/jj-store-server/tests/capture_supervisor/headroom.rs
- crates/jj-store-server/tests/capture_supervisor.rs
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/verification.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-capture/tests/reader_progress/headroom.rs#admitted_reader_reserves_maximum_native_preparation_across_restart_and_shared_writers
- crates/jj-store-capture/tests/reader_progress/headroom.rs#admitted_reader_reserves_local_payload_duplication_with_binding
- crates/jj-store-server/tests/capture_supervisor/headroom.rs#full_captured_queue_drains_ordered_native_heads_without_capacity_increase
- crates/jj-store-server/tests/capture_supervisor/backlog.rs#captured_preparation_growth_withholds_exact_intent_until_explicit_capacity_relief
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M17-032 details 2](details-02.md)
- [E-M17-032 details 3](details-03.md)
- [E-M17-032 details 4](details-04.md)
