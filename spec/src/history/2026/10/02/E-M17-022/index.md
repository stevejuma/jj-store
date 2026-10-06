---
schema: 1
id: E-M17-022
---

# E-M17-022

<!-- evidence-field:id markdown -->
## Id

E-M17-022
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

bounded-current-directory-native-session-resume-preparation-before-reader-leases
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-capture/src/codex_resume.rs
- crates/jj-store-capture/src/codex_transcript/context.rs
- crates/jj-store-capture/src/lib.rs
- crates/jj-store-capture/src/source_registry/catalog.rs
- crates/jj-store-capture/src/source_registry.rs
- crates/jj-store-capture/tests/codex_resume.rs
- crates/jj-store-server/src/capture_source/acquire.rs
- crates/jj-store-server/src/capture_source/resume.rs
- crates/jj-store-server/src/capture_source.rs
- crates/jj-store-server/tests/capture_resume.rs
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/verification.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-server/tests/capture_resume.rs#admitted_resume_uses_current_repository_when_original_header_is_revoked
- crates/jj-store-server/tests/capture_resume.rs#resume_preparation_is_bounded_read_only_and_partial_context_cannot_rebind
- crates/jj-store-server/tests/capture_resume.rs#resume_cannot_enroll_rotation_without_write_or_import_unadmitted_history
- crates/jj-store-server/tests/capture_resume.rs#file_changes_during_native_admission_withhold_before_source_acquisition
- crates/jj-store-server/tests/capture_resume.rs#current_binding_and_private_spool_are_revalidated_at_each_preparation_step
- crates/jj-store-capture/tests/codex_resume.rs#complete_context_only_and_sticky_unknown_history_have_separate_prior_anchors
- crates/jj-store-capture/tests/codex_resume.rs#bounded_snapshot_restarts_on_append_rewrite_and_rejects_replacement
- crates/jj-store-capture/tests/codex_resume.rs#lookup_is_read_only_and_only_retained_native_session_markers_resume
- crates/jj-store-capture/tests/codex_resume.rs#desktop_snapshot_qualifies_independently_from_cli
- crates/jj-store-capture/tests/codex_resume.rs#fresh_process_reconstructs_current_complete_context_from_read_only_admitted_lookup
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M17-022 details 2](details-02.md)
- [E-M17-022 details 3](details-03.md)
