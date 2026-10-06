---
schema: 1
id: E-M17-035
---

# E-M17-035

<!-- evidence-field:id markdown -->
## Id

E-M17-035
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

- crates/jj-store-capture/src/codex_discovery.rs
- crates/jj-store-capture/src/source_registry/codex.rs
- crates/jj-store-capture/tests/source_registry.rs
- crates/jj-store-server/src/capture_supervisor/scheduling.rs
- crates/jj-store-server/src/capture_supervisor/source.rs
- crates/jj-store-server/tests/capture_supervisor.rs
- crates/jj-store-server/tests/capture_supervisor/rotation.rs
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/milestones.md
- spec/src/verification.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-capture/src/codex_discovery.rs#current_binding_distinguishes_absence_from_unknown_io_outcomes
- crates/jj-store-capture/tests/source_registry.rs#current_binding_inspection_preserves_read_only_state_and_rejects_replacement_headers
- crates/jj-store-server/tests/capture_supervisor/rotation.rs#compact_replaced_source_releases_tracking_without_admitting_historical_starts
- crates/jj-store-server/tests/capture_supervisor/rotation.rs#replacement_cannot_evict_an_exact_unacknowledged_noncompact_frame
- crates/jj-store-server/tests/capture_supervisor/rotation.rs#compact_retirement_preserves_exact_a_queue_while_fresh_replacement_b_delivers
- crates/jj-store-server/tests/capture_supervisor/rotation.rs#compact_header_inspection_error_retains_owner_until_exact_binding_recovers
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M17-035 details 2](details-02.md)
- [E-M17-035 details 3](details-03.md)
