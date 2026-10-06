---
schema: 1
id: E-M17-058
---

# E-M17-058

<!-- evidence-field:id markdown -->
## Id

E-M17-058
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

bounded-read-only-codex-import-preparation
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-capture/src/codex_import/binding.rs
- crates/jj-store-capture/src/codex_import/scan.rs
- crates/jj-store-capture/src/codex_import.rs
- crates/jj-store-capture/src/lib.rs
- crates/jj-store-capture/tests/codex_import/boundaries.rs
- crates/jj-store-capture/tests/codex_import/identity.rs
- crates/jj-store-capture/tests/codex_import.rs
- spec/src/collaboration.md
- spec/src/verification.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-capture/tests/codex_import.rs#inactive_preparation_selects_complete_history_without_native_enrollment_or_raw_output
- crates/jj-store-capture/tests/codex_import.rs#partial_or_ambiguous_context_grants_no_current_directory
- crates/jj-store-capture/tests/codex_import/boundaries.rs#accepted_boundary_survives_append_with_current_context_and_excludes_new_tail
- crates/jj-store-capture/tests/codex_import/boundaries.rs#selected_prefix_rewrite_truncation_count_or_nonrecord_boundary_is_withheld
- crates/jj-store-capture/tests/codex_import/boundaries.rs#multi_step_snapshots_restart_preparation_but_preserve_accepted_prefix_on_reconcile
- crates/jj-store-capture/tests/codex_import/identity.rs#binding_rejects_byte_identical_physical_replacement_and_changed_header_or_profile
- crates/jj-store-capture/tests/codex_import/identity.rs#qualified_desktop_surface_has_the_same_complete_record_preparation_bounds
- crates/jj-store-capture/tests/codex_import/identity.rs#accepted_binding_reconciles_in_a_fresh_process_without_retargeting_appended_history
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M17-058 details 2](details-02.md)
- [E-M17-058 details 3](details-03.md)
