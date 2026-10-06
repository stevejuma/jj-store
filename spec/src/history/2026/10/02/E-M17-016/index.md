---
schema: 1
id: E-M17-016
---

# E-M17-016

<!-- evidence-field:id markdown -->
## Id

E-M17-016
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

codex-active-only-eligibility-and-durable-offline-session-admission
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-capture/src/codex_activity.rs
- crates/jj-store-capture/src/codex_activity/scan.rs
- crates/jj-store-capture/src/codex_discovery.rs
- crates/jj-store-capture/src/codex_transcript.rs
- crates/jj-store-capture/src/codex_transcript/context.rs
- crates/jj-store-capture/src/codex_transcript/file.rs
- crates/jj-store-capture/src/lib.rs
- crates/jj-store-capture/src/source_registry.rs
- crates/jj-store-capture/src/source_registry/catalog.rs
- crates/jj-store-capture/src/source_registry/codex.rs
- crates/jj-store-capture/src/source_registry/files.rs
- crates/jj-store-capture/src/source_registry/files/contents.rs
- crates/jj-store-capture/tests/codex_activity.rs
- crates/jj-store-server/src/capture_admission.rs
- crates/jj-store-server/tests/capture_activity.rs
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/milestones.md
- spec/src/verification.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-capture/tests/codex_activity.rs#historical_sessions_and_growth_do_not_admit_or_allocate_source_state
- crates/jj-store-capture/tests/codex_activity.rs#incomplete_baseline_and_invalid_context_wait_for_a_new_verified_start
- crates/jj-store-capture/tests/codex_activity.rs#bounded_prefix_verification_detects_rewrites_before_reusing_activity
- crates/jj-store-capture/tests/codex_activity.rs#desktop_activity_partial_new_starts_and_append_races_are_independent
- crates/jj-store-capture/tests/codex_activity.rs#generic_registration_cannot_resume_history_and_new_sessions_need_new_activity
- crates/jj-store-capture/tests/codex_activity.rs#admitted_offline_tail_resumes_in_a_fresh_process_after_sanitized_progress
- crates/jj-store-capture/tests/codex_activity.rs#offline_admission_process_probe
- crates/jj-store-capture/tests/codex_activity.rs#read_only_registry_lookup_rejects_missing_corrupt_unsafe_or_foreign_state
- crates/jj-store-capture/tests/codex_activity.rs#oversized_records_foreign_headers_and_rebound_roots_withhold_activity
- crates/jj-store-capture/tests/codex_activity.rs#historical_record_budget_and_changes_during_arming_do_not_assert_activity
- crates/jj-store-capture/src/source_registry/catalog.rs#admission_budget_and_duplicate_digests_fail_closed_without_eviction
- crates/jj-store-server/tests/capture_activity.rs#unbound_activity_does_not_allocate_and_current_turn_directory_controls_admission
- crates/jj-store-cli/tests/workspace_management.rs#server_capture_admits_only_published_registry_bindings_and_revalidates_them
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M17-016 details 2](details-02.md)
- [E-M17-016 details 3](details-03.md)
