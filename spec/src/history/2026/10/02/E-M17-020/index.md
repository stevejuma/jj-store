---
schema: 1
id: E-M17-020
---

# E-M17-020

<!-- evidence-field:id markdown -->
## Id

E-M17-020
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

admitted-codex-source-sanitizer-spool-and-guarded-native-publication-data-flow
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-capture/src/agent_capture_binding.rs
- crates/jj-store-capture/src/codex_activity.rs
- crates/jj-store-capture/src/lib.rs
- crates/jj-store-capture/src/source_registry/codex.rs
- crates/jj-store-server/src/capture_source.rs
- crates/jj-store-server/src/capture_source/acquire.rs
- crates/jj-store-server/src/lib.rs
- crates/jj-store-server/tests/capture_source.rs
- crates/jj-store-server/tests/capture_source/fixture.rs
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/milestones.md
- spec/src/verification.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-server/tests/capture_source.rs#admitted_codex_source_sanitizes_partitions_safe_gaps_and_resumes_offline_tail
- crates/jj-store-server/tests/capture_source.rs#spool_refusal_does_not_acknowledge_and_unadmitted_history_does_not_acquire
- crates/jj-store-server/tests/capture_source.rs#source_enrollment_requires_native_write_authority_and_pending_restart_requires_proof_recovery
- crates/jj-store-server/tests/capture_source.rs#changed_spool_binding_never_clears_a_retained_source_intent
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M17-020 details 2](details-02.md)
- [E-M17-020 details 3](details-03.md)
- [E-M17-020 details 4](details-04.md)
