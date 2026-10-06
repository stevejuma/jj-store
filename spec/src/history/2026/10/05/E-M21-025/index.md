---
schema: 1
id: E-M21-025
---

# E-M21-025

<!-- evidence-field:id markdown -->
## Id

E-M21-025
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

tlykvpus
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Original-request retained reword publication and durable unchanged-tree checkout receipt join
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; JJ 0.45.1; Git SHA-1/JJ 0.45.1 profile; macOS ARM64 disposable stock-JJ folders. CARGO_INCREMENTAL=0; installed services/executables unchanged.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Private journal catalog V3 only when saved checkout exists; V1/V2 absent-field bytes stay readable and status does not upgrade. SavedCheckoutV1 links canonical CheckoutWitnessV1. 8-KiB saved witness bound; 32-KiB new local execution/16-KiB metadata growth reservation. Native pointer only; tree_state, companion, manifest/WAL, public contracts and combination formats unchanged.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/stack_registered_preparation.rs
- crates/jj-store-jj/src/local_execution/mod.rs
- crates/jj-store-jj/src/local_execution/recovery/checkout/witness.rs
- crates/jj-store-jj/src/local_execution/recovery.rs
- crates/jj-store-server/src/stack_command_journal/catalog.rs
- crates/jj-store-server/src/stack_command_journal/checkout.rs
- crates/jj-store-server/src/stack_command_journal/execution.rs
- crates/jj-store-server/src/stack_command_journal/transition.rs
- crates/jj-store-server/src/stack_command_journal.rs
- crates/jj-store-server/src/stack_preparation/apply.rs
- crates/jj-store-server/src/stack_preparation.rs
- crates/jj-store-server/tests/stack_combinations/support.rs
- crates/jj-store-server/tests/stack_local_execution/lifecycle.rs
- crates/jj-store-server/tests/stack_local_execution.rs
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

<!-- evidence-field:before markdown -->
## Before

The initial execution/journal target failed with three missing API/field diagnostics. A stricter actual-proof gate then exposed two metadata fixture failures and lint one collapsible conditional; owner/fixture corrections are recorded without weakening current proof requirements.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

One installation-private retained reword code/checkout pipeline. Current registered/combination/physical first-apply guards; original identity lookup and terminal history; Unknown before head selection, exact retained commit/archive/candidate match, typed publication receipt, canonical witness selection before pointer completion, typed verification and pending discovery. Real physical/registered and killed-process evidence stays distinct from isolated combination authority and simulated historical retirement/metadata fixtures. No transport or writable UI capability.
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-server/tests/stack_local_execution.rs#real_retained_reword_execution_saves_checkout_witness_and_replays_receipt_without_writes
- crates/jj-store-server/tests/stack_local_execution.rs#interrupted_durable_boundaries_reopen_and_finish_the_same_original_candidate
- crates/jj-store-server/tests/stack_local_execution.rs#physical_edits_reject_prepublication_and_keep_saved_checkout_pending_after_publication
- crates/jj-store-server/tests/stack_local_execution.rs#foreign_apply_identities_and_unproved_execution_receipts_cannot_publish
- crates/jj-store-server/tests/stack_local_execution.rs#retired_receipt_and_saved_witness_are_historical_after_later_real_cli_work
- crates/jj-store-server/tests/stack_local_execution.rs#saved_checkout_is_immutable_and_malformed_linkage_cannot_reopen
- crates/jj-store-server/tests/stack_local_execution.rs#saved_pending_checkout_repairs_lost_source_dependencies_without_repreparing
- crates/jj-store-server/tests/stack_local_execution.rs#journal_retention_mismatch_is_rejected_before_publication
- crates/jj-store-server/tests/stack_local_execution/lifecycle.rs#process_loss_at_publication_and_checkout_boundaries_resumes_saved_journal
- crates/jj-store-cli/tests/stack_registered_preparation.rs#registered_authority_executes_retained_reword_with_real_physical_guard_and_historical_retry
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M21-025 details 2](details-02.md)
- [E-M21-025 details 3](details-03.md)
- [E-M21-025 details 4](details-04.md)
