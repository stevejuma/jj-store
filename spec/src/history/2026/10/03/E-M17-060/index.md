---
schema: 1
id: E-M17-060
---

# E-M17-060

<!-- evidence-field:id markdown -->
## Id

E-M17-060
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M17
- M17.2
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-03
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

exact-explicit-history-registry-admission
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-capture/src/codex_import.rs
- crates/jj-store-capture/src/source_registry.rs
- crates/jj-store-capture/src/source_registry/acquisition.rs
- crates/jj-store-capture/src/source_registry/catalog.rs
- crates/jj-store-capture/src/source_registry/parking/pinned.rs
- crates/jj-store-capture/tests/source_registry_import.rs
- crates/jj-store-capture/tests/source_registry_import/leases.rs
- crates/jj-store-capture/tests/source_registry_import/loss.rs
- spec/src/collaboration.md
- spec/src/verification.md
- spec/src/milestones.md
- crates/jj-store-capture/src/source_registry/import.rs
- crates/jj-store-capture/tests/source_registry_import/restart.rs
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-capture/tests/source_registry_import.rs#import_requires_completed_exact_preparation_and_current_consent_before_allocation
- crates/jj-store-capture/tests/source_registry_import.rs#explicit_enrollment_preserves_registered_identity_and_exact_replay
- crates/jj-store-capture/tests/source_registry_import.rs#appended_snapshot_requires_reconciliation_without_extending_accepted_history
- crates/jj-store-capture/tests/source_registry_import.rs#replacement_or_changed_header_cannot_inherit_import_only_consent
- crates/jj-store-capture/tests/source_registry_import/loss.rs#preselection_authority_or_source_loss_withholds_catalog_selection
- crates/jj-store-capture/tests/source_registry_import/loss.rs#postselection_loss_keeps_marker_and_exact_retry_does_not_duplicate_identity
- crates/jj-store-capture/tests/source_registry_import/loss.rs#retained_registry_gate_excludes_other_acquisition_through_selection
- crates/jj-store-capture/tests/source_registry_import/leases.rs#aliases_restart_and_capacity_use_existing_identity_and_exclusive_leases
- crates/jj-store-capture/tests/source_registry_import/leases.rs#imported_raw_frame_reserves_and_resumes_common_channel_without_reallocation
- crates/jj-store-capture/tests/source_registry_import/restart.rs#import_only_marker_never_promotes_unverified_prefix_into_ordinary_resume
- crates/jj-store-capture/tests/source_registry_import/restart.rs#fresh_process_reconciles_retained_import_marker_without_retargeting_replacement
- crates/jj-store-capture/tests/source_registry_import/restart.rs#import_registry_process_probe
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M17-060 details 2](details-02.md)
- [E-M17-060 details 3](details-03.md)
