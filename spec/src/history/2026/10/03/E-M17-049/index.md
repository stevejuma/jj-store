---
schema: 1
id: E-M17-049
---

# E-M17-049

<!-- evidence-field:id markdown -->
## Id

E-M17-049
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-03
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

implementation
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M17
- M17.2
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-server/src/capture_control_state.rs
- crates/jj-store-server/src/capture_control_state/catalog.rs
- crates/jj-store-server/src/capture_control_state/files.rs
- crates/jj-store-server/src/capture_control_state/files/metadata.rs
- crates/jj-store-server/src/capture_control_state/files/publication.rs
- crates/jj-store-server/src/capture_control_state/tests.rs
- crates/jj-store-server/src/lib.rs
- crates/jj-store-server/tests/capture_control_state.rs
- crates/jj-store-server/tests/capture_control_state/capacity.rs
- crates/jj-store-server/tests/capture_control_state/safety.rs
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/milestones.md
- spec/src/verification.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-server/tests/capture_control_state.rs#pause_receipts_survive_reopen_and_old_replay_cannot_reverse_newer_resume
- crates/jj-store-server/tests/capture_control_state/safety.rs#an_open_handle_refuses_a_replaced_lock_or_directory
- crates/jj-store-server/tests/capture_control_state/capacity.rs#full_receipt_catalog_preserves_replay_and_refuses_new_requests_without_eviction
- crates/jj-store-server/src/capture_control_state/tests.rs#lost_coordination_after_replacement_is_unknown_and_exact_receipt_recovers
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M17-049 details 2](details-02.md)
- [E-M17-049 details 3](details-03.md)
