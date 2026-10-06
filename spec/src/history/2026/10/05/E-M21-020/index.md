---
schema: 1
id: E-M21-020
---

# E-M21-020

<!-- evidence-field:id markdown -->
## Id

E-M21-020
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

qtzxutkq
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Private durable combination metadata and registered selected-code guard
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; JJ 0.45.1; Git 2.55.0; macOS ARM64 real disposable stock-JJ folders and reserved code refs. Installed services/executables unchanged.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Private Combination catalog V1, Snapshot V1, scoped pending/selected/rejected records and RetainedCode anchors. No public HTTP/schema/client changes. Existing journal V1/V2, LocalExecutionV1, OperationArchiveV1, reserved ref V2, companion/native/WAL/manifest formats unchanged.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/stack_registered_preparation.rs
- crates/jj-store-server/src/lib.rs
- crates/jj-store-server/src/stack_combinations/authority.rs
- crates/jj-store-server/src/stack_combinations/catalog.rs
- crates/jj-store-server/src/stack_combinations/tests.rs
- crates/jj-store-server/src/stack_combinations/validation.rs
- crates/jj-store-server/src/stack_combinations.rs
- crates/jj-store-server/src/stack_command_journal.rs
- crates/jj-store-server/src/stack_preparation/registered.rs
- crates/jj-store-server/tests/stack_combinations/support.rs
- crates/jj-store-server/tests/stack_combinations.rs
- docs/implementation-plan.md
- docs/maintenance/coverage.json
- docs/site/technical/lifecycle.md
- docs/site/technical/local.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/documentation.md
- spec/src/local-jj.md
- spec/src/verification.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Exact new state-owner regression could not import the absent CombinationStore and initially had fixture path/native-ID-width errors (five diagnostics). First implementation run retained one wrong-width fixture diagnostic. Multi-combination stale fixture changed request ID without updating prepared identity; Invalid correctly preceded the expected Stale, and the fixture was repaired. A misplaced physical-state assertion caused one compile diagnostic, then moved into its owning test. Strict lint found two unnecessary unit expressions, removed without suppressions or semantic change.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

One private combination-state owner and direct registered preparation consumer. Bounded named target/merge/WIP/ordered stacks, exact retained code membership, generation/active-code guards, principal/physical scope, pending/selected separation, journal receipt linkage, private synced replacement and historical replay. No runtime setup, command transport, code publication or guarded checkout is enabled.
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-server/tests/stack_combinations.rs#reads_never_initialize_and_selected_combinations_survive_reopen
- crates/jj-store-server/tests/stack_combinations.rs#wrong_principal_scope_intent_and_unretained_code_fail_without_selection
- crates/jj-store-server/tests/stack_combinations.rs#rejected_update_retains_original_outcome_without_initializing_setup
- crates/jj-store-server/tests/stack_combinations.rs#selected_metadata_and_retained_code_reopen_after_source_loss_and_gc
- crates/jj-store-server/src/stack_combinations/tests.rs#acknowledgement_loss_and_replacement_recover_exact_original_pending_update
- crates/jj-store-server/src/stack_combinations/tests.rs#gate_stale_selection_and_malformed_generations_fail_before_replacement
- crates/jj-store-server/src/stack_combinations/tests.rs#process_loss_after_device_selection_reopens_exact_original_without_reexecution
- crates/jj-store-server/src/stack_combinations/tests.rs#multiple_combinations_preserve_anchors_and_historical_replay_never_rewinds
- crates/jj-store-server/src/stack_combinations/tests.rs#capacity_never_evicts_original_updates_or_selects_a_partial_record
- crates/jj-store-cli/tests/stack_registered_preparation.rs#caller_mode_missing_combination_and_immutable_ancestry_reject_before_selection
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M21-020 details 2](details-02.md)
- [E-M21-020 details 3](details-03.md)
