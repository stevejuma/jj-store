---
schema: 1
id: E-M17-072
---

# E-M17-072

<!-- evidence-field:id markdown -->
## Id

E-M17-072
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

opencode-admitted-snapshot-ingestion-publication
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change psrtrwwm
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-capture/src/lib.rs
- crates/jj-store-capture/src/opencode_ingress.rs
- crates/jj-store-capture/src/source_registry/opencode.rs
- crates/jj-store-capture/src/source_registry.rs
- crates/jj-store-capture/src/transcript_sanitization.rs
- crates/jj-store-capture/tests/opencode_ingress.rs
- crates/jj-store-server/src/lib.rs
- crates/jj-store-server/src/opencode_read_api.rs
- crates/jj-store-server/src/opencode_source/ingress.rs
- crates/jj-store-server/src/opencode_source/publication.rs
- crates/jj-store-server/src/opencode_source/snapshot.rs
- crates/jj-store-server/src/opencode_source.rs
- crates/jj-store-server/tests/opencode_source/fixture.rs
- crates/jj-store-server/tests/opencode_source.rs
- spec/src/collaboration.md
- spec/src/verification.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-capture/tests/opencode_ingress.rs#api_source_identity_is_private_exclusive_installation_bound_and_resumable
- crates/jj-store-capture/tests/opencode_ingress.rs#sanitized_api_records_are_durable_before_ack_and_keep_exact_retry_after_restart
- crates/jj-store-capture/tests/opencode_ingress.rs#missing_context_keeps_only_safe_omission_and_does_not_skip_later_records
- crates/jj-store-server/tests/opencode_source.rs#api_ingestion_partitions_sanitizes_publishes_and_resumes_exact_offline_history
- crates/jj-store-server/tests/opencode_source.rs#inactive_history_revocation_and_spool_pressure_never_grant_or_advance_admission
- crates/jj-store-server/tests/opencode_source.rs#rewritten_snapshots_restart_a_generation_and_withhold_old_pending_intents
- crates/jj-store-server/tests/opencode_source.rs#a_changed_queued_prefix_starts_reconstruction_without_skipping_to_the_last_record
- crates/jj-store-server/tests/opencode_source.rs#native_write_and_changed_api_binding_withhold_allocation_and_publication
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M17-072 details 2](details-02.md)
- [E-M17-072 details 3](details-03.md)
