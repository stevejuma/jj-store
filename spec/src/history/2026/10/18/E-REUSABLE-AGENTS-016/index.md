---
schema: 1
id: E-REUSABLE-AGENTS-016
---

# E-REUSABLE-AGENTS-016

<!-- evidence-field:id markdown -->
## Id

E-REUSABLE-AGENTS-016
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M33
- M33.3
- M33.4
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-ARCH-007
- JST-DOC-001
- JST-GOV-002
- JST-GOV-011
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-18
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- Cargo.lock
- crates/jj-agent/src/native_capture.rs
- crates/jj-store-conversations/Cargo.toml
- crates/jj-store-conversations/README.md
- crates/jj-store-conversations/src/lib.rs
- crates/jj-store-conversations/src/store_capture/candidate.rs
- crates/jj-store-conversations/src/store_capture/codec.rs
- crates/jj-store-conversations/src/store_capture/confirmation.rs
- crates/jj-store-conversations/src/store_capture/journal.rs
- crates/jj-store-conversations/src/store_capture/mod.rs
- crates/jj-store-conversations/src/store_capture/preparation.rs
- crates/jj-store-conversations/src/store_capture/sink.rs
- crates/jj-store-conversations/tests/store_capture.rs
- crates/jj-store-git/Cargo.toml
- crates/jj-store-git/tests/store_capture.rs
- crates/jj-store-git/tests/support/capture_journal.rs
- crates/jj-store-native/Cargo.toml
- crates/jj-store-native/tests/factories/store_capture.rs
- crates/jj-store-native/tests/factories.rs
- docs/maintenance/coverage.json
- docs/maintenance/reusable-agent-workflows-progress.md
- docs/site/technical/conversations.md
- scripts/check-boundaries.mjs
- scripts/conversations-features.test.mjs
- spec/src/current/contracts/architecture/25-injectable-collaboration-stores.md
- spec/src/current/contracts/architecture/26-reusable-agent-workflows.md
- spec/src/current/contracts/collaboration/18-agentstore-object-foundation.md
- spec/src/current/contracts/documentation/01-application-documentation.md
- spec/src/current/governance/SUMMARY.nav.md
- spec/src/current/governance/index.md
- spec/src/current/governance/reusable-capture-boundaries.md
- spec/src/current/progress/M33/history/M33/index.md
- spec/src/current/progress/M33/history/M33.3/index.md
- spec/src/current/progress/M33/history/M33.4/index.md
- spec/src/current/progress/M33/index.md
- spec/src/history/2026/10/18/E-REUSABLE-AGENTS-016/SUMMARY.nav.md
- spec/src/history/2026/10/18/E-REUSABLE-AGENTS-016/details-02.md
- spec/src/history/2026/10/18/E-REUSABLE-AGENTS-016/details-03.md
- spec/src/history/2026/10/18/E-REUSABLE-AGENTS-016/index.md
- spec/src/history/2026/10/18/SUMMARY.nav.md
- spec/src/history/2026/10/18/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-conversations/tests/store_capture.rs#private_native_preparation_and_original_receipts_roundtrip_without_number_or_byte_changes
- crates/jj-store-conversations/tests/store_capture.rs#private_records_reject_unknown_fields_tampering_and_mismatched_preparation_or_receipts
- crates/jj-store-conversations/tests/store_capture.rs#private_record_aggregate_and_provider_fields_are_bounded_before_host_io
- crates/jj-store-git/tests/store_capture.rs#production_sink_publishes_atomic_roots_and_durable_receipts_before_output
- crates/jj-store-git/tests/store_capture.rs#lost_commit_ack_and_journal_confirmation_recover_without_another_write
- crates/jj-store-git/tests/store_capture.rs#incomplete_and_not_known_entries_stay_unknown_without_preparation_or_execute
- crates/jj-store-git/tests/store_capture.rs#dropped_wait_fences_owner_and_recovery_only_reads_retained_intent
- crates/jj-store-git/tests/store_capture.rs#recovery_reauthorizes_and_rejects_unavailable_or_changed_original_receipts
- crates/jj-store-git/tests/store_capture.rs#changed_bytes_or_thread_under_same_ordinal_cannot_claim_another_request
- crates/jj-store-git/tests/store_capture.rs#exact_existing_batch_only_replays_and_incompatible_participants_fail_before_journal_io
- crates/jj-store-git/tests/store_capture.rs#stale_thread_after_preparation_does_not_publish_capture_or_retry
- crates/jj-store-git/tests/store_capture.rs#foreign_primary_assignment_and_wrong_repository_are_rejected
- crates/jj-store-native/tests/factories/store_capture.rs#native_sink_retains_exact_preparation_and_recovers_unavailable_original_receipts_read_only
<!-- /evidence-field -->

