---
schema: 1
id: E-M21-027
---

# E-M21-027

<!-- evidence-field:id markdown -->
## Id

E-M21-027
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

mrpxsmqs
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Current retained reword policy and acknowledgement-only historical publication recovery
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; JJ 0.45.1; Git 2.55.0; macOS ARM64 disposable stock-JJ registered folders; CARGO_INCREMENTAL=0. Installed services and executables unchanged.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No serialized/public wire or persisted format change. Owned provider methods add bounded archived policy validation, informative publication selection and acknowledgement-only proofs. Original journal V1/V2/V3, archive, receipt/witness, combination, companion, native manifest and WAL formats unchanged.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/stack_registered_preparation/apply.rs
- crates/jj-store-cli/tests/stack_registered_preparation/policy.rs
- crates/jj-store-cli/tests/stack_registered_preparation.rs
- crates/jj-store-jj/src/local_execution/recovery/policy.rs
- crates/jj-store-jj/src/local_execution/recovery/publication.rs
- crates/jj-store-jj/src/local_execution/recovery.rs
- crates/jj-store-jj/tests/stack_source_recovery/policy.rs
- crates/jj-store-jj/tests/stack_source_recovery/policy_lifecycle.rs
- crates/jj-store-jj/tests/stack_source_recovery.rs
- crates/jj-store-server/src/stack_preparation/apply.rs
- docs/implementation-plan.md
- docs/maintenance/coverage.json
- docs/site/reference/configuration.md
- docs/site/technical/local.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/documentation.md
- spec/src/jj-integration.md
- spec/src/local-jj.md
- spec/src/verification.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-LOCAL-005
- JST-JJ-002
- JST-CLI-001
- JST-UI-020
- JST-GOV-003
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

New registered immutable-policy regression failed once because current protection was not enforced at apply. Added lifecycle fixture initially failed compilation on an unnecessary RecoveredHistory Debug bound; changed only the cancellation assertion to pattern matching. Later source review exposed another recovery edge: a typed Published receipt before witness save reannounced a removed candidate after source-head restoration and returned Verified. Its exact negative failed once, then passed with mandatory acknowledgement-only recovery.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

One original-request private reword policy/recovery boundary. Unpublished Prepared and Unknown requests revalidate current registered combination/code/physical policy and trusted immutable visible-parent ancestry over the retained source. Current selection or a typed Published receipt requires acknowledgement-only provider recovery; disappeared selection cannot create/reannounce a candidate. Bounded work/parent-edge charging, original lease/admission, authorization/cancellation and historical terminal semantics remain exact. No production transport or capability enabled.
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-jj/tests/stack_source_recovery/policy.rs#retained_source_policy_rejects_protected_hidden_duplicate_and_oversized_inputs_without_writes
- crates/jj-store-jj/tests/stack_source_recovery/policy.rs#current_protected_descendant_blocks_rewriting_its_retained_ancestor
- crates/jj-store-jj/tests/stack_source_recovery/policy.rs#acknowledgement_only_rechecks_publication_and_cannot_reannounce_a_lost_candidate
- crates/jj-store-jj/tests/stack_source_recovery/policy_lifecycle.rs#current_policy_authorization_loss_releases_original_admission_without_writes
- crates/jj-store-jj/tests/stack_source_recovery/policy_lifecycle.rs#canceled_policy_worker_retains_original_lock_and_admission_until_exit
- crates/jj-store-cli/tests/stack_registered_preparation/apply.rs#newly_immutable_commit_rejects_original_prepared_apply_without_publication
- crates/jj-store-cli/tests/stack_registered_preparation/policy.rs#unpublished_unknown_retry_revalidates_current_immutable_policy_before_head_selection
- crates/jj-store-cli/tests/stack_registered_preparation/policy.rs#already_published_original_retry_completes_checkout_after_live_policy_changes
- crates/jj-store-cli/tests/stack_registered_preparation/policy.rs#typed_published_receipt_cannot_reannounce_lost_selection_before_witness_save
- crates/jj-store-server/tests/stack_local_execution/lifecycle.rs#process_loss_at_publication_and_checkout_boundaries_resumes_saved_journal
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M21-027 details 2](details-02.md)
- [E-M21-027 details 3](details-03.md)
- [E-M21-027 details 4](details-04.md)
