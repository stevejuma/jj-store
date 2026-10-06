---
schema: 1
id: E-M17-027
---

# E-M17-027

<!-- evidence-field:id markdown -->
## Id

E-M17-027
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

fresh-observed-codex-start-current-context-admission-before-native-enrollment
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-capture/src/codex_activity.rs
- crates/jj-store-server/src/capture_source/resume.rs
- crates/jj-store-server/src/capture_source/resume/eligibility.rs
- crates/jj-store-server/tests/capture_activation.rs
- spec/src/collaboration.md
- spec/src/cli.md
- spec/src/verification.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-server/tests/capture_activation.rs#fresh_start_admits_current_directory_after_event_directory_revocation
- crates/jj-store-server/tests/capture_activation.rs#fresh_preparation_is_bounded_and_does_not_enroll_before_current_write
- crates/jj-store-server/tests/capture_activation.rs#changed_activity_snapshot_requires_renewal_from_the_original_probe
- crates/jj-store-server/tests/capture_activation.rs#fresh_start_cannot_enroll_an_unbound_directory_or_use_another_spool
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M17-027 details 2](details-02.md)
- [E-M17-027 details 3](details-03.md)
