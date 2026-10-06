---
schema: 1
id: E-M17-021
---

# E-M17-021

<!-- evidence-field:id markdown -->
## Id

E-M17-021
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

exact-queued-codex-directory-proof-recovery-and-lease-preserving-cursor-resumption
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-capture/src/agent_publication/exact.rs
- crates/jj-store-capture/src/agent_publication.rs
- crates/jj-store-capture/src/codex_source.rs
- crates/jj-store-capture/src/codex_transcript/context.rs
- crates/jj-store-capture/src/codex_transcript/recovery/identity.rs
- crates/jj-store-capture/src/codex_transcript/recovery.rs
- crates/jj-store-capture/src/codex_transcript.rs
- crates/jj-store-capture/src/source_registry/codex.rs
- crates/jj-store-capture/tests/codex_recovery.rs
- crates/jj-store-cli/tests/agent_capture_publication.rs
- crates/jj-store-server/src/capture_source/acquire.rs
- crates/jj-store-server/src/capture_source/recovery.rs
- crates/jj-store-server/src/capture_source.rs
- crates/jj-store-server/tests/capture_recovery.rs
- crates/jj-store-server/tests/capture_source.rs
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/verification.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-capture/tests/codex_recovery.rs#exact_range_reconstructs_changed_context_and_rejects_mixed_or_changed_identity
- crates/jj-store-capture/tests/codex_recovery.rs#long_prefix_recovery_is_bounded_and_restarts_after_append_or_rewrite
- crates/jj-store-capture/tests/codex_recovery.rs#unknown_context_body_cannot_gain_an_omission_anchor
- crates/jj-store-capture/tests/codex_recovery.rs#fresh_process_recovers_exact_queued_range_from_retained_admitted_channel
- crates/jj-store-capture/tests/codex_recovery.rs#desktop_recovery_rejects_cli_profile_and_incomplete_ending
- crates/jj-store-server/tests/capture_recovery.rs#queued_other_repository_recovers_exact_context_after_restart
- crates/jj-store-server/tests/capture_recovery.rs#omitted_pending_range_requires_its_separately_authorized_prior_anchor
- crates/jj-store-server/tests/capture_recovery.rs#changed_pending_source_and_lost_native_write_preserve_exact_intent
- crates/jj-store-server/tests/capture_recovery.rs#multiple_queued_partitions_recover_in_source_order_before_new_reading
- crates/jj-store-cli/tests/agent_capture_publication.rs#exact_queued_entry_drain_preserves_other_batches_and_prepared_retry
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M17-021 details 2](details-02.md)
- [E-M17-021 details 3](details-03.md)
