---
schema: 1
id: E-M21-023
---

# E-M21-023

<!-- evidence-field:id markdown -->
## Id

E-M21-023
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

sokxvsru
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Exact retained head publication with concurrent CLI ancestry and interrupted retry
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; JJ 0.45.1; Git SHA-1/JJ 0.45.1 profile; macOS ARM64 real disposable stock-JJ folders. CARGO_INCREMENTAL=0; installed services/executables unchanged.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Pinned SimpleOpStore/native head markers and upstream Unix flock; witness V1, operation archive V1, reserved retention V2, journal/combination/companion/native manifest/WAL formats unchanged. No transport/schema/client change.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-jj/src/local_execution/mod.rs
- crates/jj-store-jj/src/local_execution/recovery.rs
- crates/jj-store-jj/src/local_execution/recovery/extras.rs
- crates/jj-store-jj/src/local_execution/recovery/indexes.rs
- crates/jj-store-jj/src/local_execution/recovery/publication.rs
- crates/jj-store-jj/src/local_execution/recovery/publication/ancestry.rs
- crates/jj-store-jj/src/local_execution/recovery/records.rs
- crates/jj-store-jj/src/local_execution/recovery/records/destination.rs
- crates/jj-store-jj/src/local_execution/recovery/records/synced.rs
- crates/jj-store-jj/src/local_execution/recovery/witness.rs
- crates/jj-store-jj/tests/stack_head_publication.rs
- crates/jj-store-jj/tests/stack_head_publication/adversarial.rs
- crates/jj-store-jj/tests/stack_head_publication/lifecycle.rs
- docs/implementation-plan.md
- docs/site/technical/local.md
- docs/maintenance/coverage.json
- spec/src/architecture.md
- spec/src/documentation.md
- spec/src/jj-integration.md
- spec/src/local-jj.md
- spec/src/verification.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Initial two-case target failed at three missing resume_publication calls. Ordinary saved recovery required unchanged original heads and offered no publication API. See runs for every intermediate failure and fixture correction.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

One private original-request code-publication primitive. Preserve strict ordinary restoration; separately prove bounded exact current ancestry, validate unchanged saved checkout/policy/binding and complete source/extra/index dependencies. Hold foreground and actual native head-store locks, sync create-only candidate before removing only original, preserve CLI siblings and acknowledge later-descendant retry without reannouncement. Provider-only proof binds retained key/witness, operation and view.
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-jj/tests/stack_head_publication.rs#exact_publication_replaces_only_original_head_without_checkout_and_retries_after_reopen
- crates/jj-store-jj/tests/stack_head_publication.rs#actual_cli_sibling_head_is_preserved_when_the_original_head_was_already_consumed
- crates/jj-store-jj/tests/stack_head_publication.rs#retry_after_real_cli_descendant_does_not_reannounce_or_rewind_candidate
- crates/jj-store-jj/tests/stack_head_publication.rs#multiple_real_cli_siblings_survive_exact_candidate_selection
- crates/jj-store-jj/tests/stack_head_publication.rs#ordinary_history_restoration_remains_strict_after_publication
- crates/jj-store-jj/tests/stack_head_publication/adversarial.rs#missing_extras_and_indexes_require_explicit_repair_before_head_selection
- crates/jj-store-jj/tests/stack_head_publication/adversarial.rs#missing_or_corrupt_archived_operation_and_view_fail_preflight_without_selection
- crates/jj-store-jj/tests/stack_head_publication/adversarial.rs#unrelated_missing_and_hash_substituted_current_heads_are_rejected
- crates/jj-store-jj/tests/stack_head_publication/adversarial.rs#native_head_lock_contention_is_nonblocking_and_retry_preserves_original
- crates/jj-store-jj/tests/stack_head_publication/adversarial.rs#unsafe_head_markers_coordination_files_and_arbitrary_staging_names_fail_closed
- crates/jj-store-jj/tests/stack_head_publication/adversarial.rs#safe_interrupted_head_staging_is_ignored_and_never_selected_as_a_head
- crates/jj-store-jj/tests/stack_head_publication/adversarial.rs#candidate_with_another_operation_parent_cannot_publish_under_saved_witness
- crates/jj-store-jj/tests/stack_head_publication/adversarial.rs#oversized_ancestry_and_excessive_head_enumeration_fail_before_selection
- crates/jj-store-jj/tests/stack_head_publication/adversarial.rs#actual_native_ancestry_count_and_parent_fanout_are_bounded
- crates/jj-store-jj/tests/stack_head_publication/lifecycle.rs#cancellation_after_selection_retains_admission_and_exact_retry_finishes_old_head_removal
- crates/jj-store-jj/tests/stack_head_publication/lifecycle.rs#revocation_and_replaced_head_lock_after_candidate_selection_preserve_exact_retry
- crates/jj-store-jj/tests/stack_head_publication/lifecycle.rs#changed_policy_checkout_and_replaced_head_directory_never_acknowledge_publication
- crates/jj-store-jj/tests/stack_head_publication/lifecycle.rs#subprocess_loss_after_candidate_selection_reopens_same_witness_without_checkout
- crates/jj-store-jj/tests/stack_head_publication/lifecycle.rs#cancellation_after_original_removal_retries_same_proof_without_reannouncing_heads
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M21-023 details 2](details-02.md)
- [E-M21-023 details 3](details-03.md)
