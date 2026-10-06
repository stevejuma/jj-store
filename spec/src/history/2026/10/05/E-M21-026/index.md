---
schema: 1
id: E-M21-026
---

# E-M21-026

<!-- evidence-field:id markdown -->
## Id

E-M21-026
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M21
- M21.2
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-05
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

wknlomtl
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Registered retained Prepared first apply after original source operation/index loss
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; JJ 0.45.1; Git 2.55.0; macOS ARM64 disposable registered stock-JJ folders; CARGO_INCREMENTAL=0. Installed binaries/services unchanged.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No public or persisted format change. Existing original-request journal V1/V2/V3, retained source archive, saved checkout witness, combination metadata, companion, manifest and WAL formats unchanged.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/stack_registered_preparation/apply.rs
- crates/jj-store-cli/tests/stack_registered_preparation/recovery.rs
- crates/jj-store-cli/tests/stack_registered_preparation.rs
- crates/jj-store-server/src/stack_preparation/apply.rs
- docs/implementation-plan.md
- docs/maintenance/coverage.json
- docs/site/reference/configuration.md
- docs/site/technical/local.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/documentation.md
- spec/src/local-jj.md
- spec/src/verification.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

The exact new registered first-apply regression failed with Provider(Unavailable) after operation-store/index loss and Git GC. After ordering repair it passed. Broader new guard test then exposed one fixture assumption: rewriting original file bytes is still Stale because the physical guard includes identity and timestamps. Separate combination-success and editor/reverted-byte rejection cases preserve the exact physical contract.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

One private original retained reword first-apply ordering boundary: registration before source reads, exact original recovery under the foreground lock, checked retained selection and explicit extra/index reconstruction, then source-backed current combination/code/physical validation before Unknown/publication. No re-preparation or production mutation enablement.
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-LOCAL-005
- JST-CLI-001
- JST-UI-020
- JST-GOV-003
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/stack_registered_preparation/apply.rs#registered_first_apply_recovers_original_prepared_source_after_loss_and_gc
- crates/jj-store-cli/tests/stack_registered_preparation/apply.rs#registered_recovered_first_apply_rechecks_combination_before_publication
- crates/jj-store-cli/tests/stack_registered_preparation/apply.rs#registered_recovered_first_apply_preserves_editor_changes_and_exact_physical_guard
- crates/jj-store-cli/tests/stack_registered_preparation/apply.rs#revoked_registration_cannot_restore_prepared_source_or_publish
- crates/jj-store-server/tests/stack_local_execution/lifecycle.rs#process_loss_at_publication_and_checkout_boundaries_resumes_saved_journal
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M21-026 details 2](details-02.md)
- [E-M21-026 details 3](details-03.md)
