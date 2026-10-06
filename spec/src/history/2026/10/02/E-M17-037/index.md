---
schema: 1
id: E-M17-037
---

# E-M17-037

<!-- evidence-field:id markdown -->
## Id

E-M17-037
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

- crates/jj-store-capture/src/agent_capture.rs
- crates/jj-store-capture/src/agent_capture/budget.rs
- crates/jj-store-capture/tests/reader_progress/headroom.rs
- crates/jj-store-server/src/capture_source/ingress.rs
- crates/jj-store-server/src/capture_source/recovery.rs
- crates/jj-store-server/src/capture_supervisor.rs
- crates/jj-store-server/src/capture_supervisor/delivery.rs
- crates/jj-store-server/src/capture_supervisor/scheduling.rs
- crates/jj-store-server/src/capture_supervisor/source.rs
- crates/jj-store-server/src/lib.rs
- crates/jj-store-server/tests/capture_supervisor.rs
- crates/jj-store-server/tests/capture_supervisor/backlog.rs
- crates/jj-store-server/tests/capture_supervisor/capacity.rs
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/milestones.md
- spec/src/verification.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-capture/tests/reader_progress/headroom.rs#preparation_capacity_refusal_reports_exact_relief_without_mutating_intent
- crates/jj-store-server/tests/capture_supervisor/capacity.rs#ingress_capacity_status_preserves_cursor_and_clears_after_explicit_relief
- crates/jj-store-server/tests/capture_supervisor/capacity.rs#compact_preparation_status_reports_exact_relief_without_rewriting_queue
- crates/jj-store-server/tests/capture_supervisor/backlog.rs#captured_preparation_growth_withholds_exact_intent_until_explicit_capacity_relief
- crates/jj-store-server/tests/capture_supervisor.rs#activity_without_current_native_write_cannot_enroll_or_acknowledge
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M17-037 details 2](details-02.md)
