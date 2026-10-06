---
schema: 1
id: E-M21-017
---

# E-M21-017

<!-- evidence-field:id markdown -->
## Id

E-M21-017
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

puspqzpm
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Explicit exact source operation-history recovery under foreground coordination
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; JJ 0.45.1; Git 2.55.0; macOS ARM64 isolated real stock-JJ disposable fixtures. Installed executables/services unchanged.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

New private bounded RecoveryWitnessV1 with canonical encoding, 64-KiB cap and domain-separated digest; captures original authorized repository/binding, checkout/operation/view/commit, policy/head and physical Git-directory fingerprints. RecoveredHistory owns the foreground lock/admission. Existing OperationArchiveV1, reserved retention versions 1/2, original companion/native/WAL/manifest, server journal and public transport formats unchanged. Restores exact native operation/view records and original fixed op-store profile marker only; no head or checkout selection.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-jj/src/local_browser/mod.rs
- crates/jj-store-jj/src/local_browser/working_tree/current.rs
- crates/jj-store-jj/src/local_browser/working_tree/current/capture.rs
- crates/jj-store-jj/src/local_evidence/mod.rs
- crates/jj-store-jj/src/local_evidence/operation_archive/materialize.rs
- crates/jj-store-jj/src/local_execution/mod.rs
- crates/jj-store-jj/src/local_execution/recovery.rs
- crates/jj-store-jj/src/local_execution/recovery/records.rs
- crates/jj-store-jj/src/local_execution/recovery/records/destination.rs
- crates/jj-store-jj/src/local_execution/recovery/records/synced.rs
- crates/jj-store-jj/src/local_execution/recovery/witness.rs
- crates/jj-store-jj/tests/stack_source_recovery.rs
- crates/jj-store-jj/tests/stack_source_recovery/adversarial.rs
- crates/jj-store-jj/tests/stack_source_recovery/lifecycle.rs
- docs/implementation-plan.md
- docs/maintenance/coverage.json
- docs/site/technical/local.md
- spec/src/architecture.md
- spec/src/documentation.md
- spec/src/jj-integration.md
- spec/src/local-jj.md
- spec/src/milestones.md
- spec/src/verification.md
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Seven missing API diagnostics establish that saved witness capture and explicit retained source restoration were absent. Ordinary foreground acquisition cannot validate after source operation-store/index loss.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

One local-JJ source-history recovery boundary. Saved versioned witness, original exact reserved ref lookup, metadata/policy/device revalidation without operation/index loader, nonblocking foreground lock, full native conflict/link preflight, synced no-replace restoration and exact retry. Shared current fingerprint/policy helpers retain ordinary reader behavior, so complete direct local-browser consumers run. No server runtime/transport/UI mutation enabled.
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-jj/tests/stack_source_recovery.rs#restart_restores_exact_retained_history_without_heads_checkout_or_index_changes
- crates/jj-store-jj/tests/stack_source_recovery.rs#stale_checkout_or_original_key_cannot_restore_missing_state
- crates/jj-store-jj/tests/stack_source_recovery.rs#conflicting_existing_record_fails_before_any_missing_record_is_restored
- crates/jj-store-jj/tests/stack_source_recovery/adversarial.rs#recovery_codec_and_complete_key_reject_substitution
- crates/jj-store-jj/tests/stack_source_recovery/adversarial.rs#unsafe_record_and_directory_links_never_touch_targets
- crates/jj-store-jj/tests/stack_source_recovery/adversarial.rs#changed_heads_ignore_policy_and_source_binding_fail_without_initialization
- crates/jj-store-jj/tests/stack_source_recovery/adversarial.rs#recovered_lease_keeps_foreground_exclusive_and_preserves_editor_bytes
- crates/jj-store-jj/tests/stack_source_recovery/lifecycle.rs#canceled_recovery_keeps_admission_until_worker_exits_and_exact_retry_finishes
- crates/jj-store-jj/tests/stack_source_recovery/lifecycle.rs#process_loss_reopens_original_witness_and_retries_partial_immutable_restoration
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M21-017 details 2](details-02.md)
- [E-M21-017 details 3](details-03.md)
