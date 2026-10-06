---
schema: 1
id: E-M21-031
---

# E-M21-031

<!-- evidence-field:id markdown -->
## Id

E-M21-031
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

nnrxmokm
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Durable complete original-inventory selection and exact checkout command linkage
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; pinned JJ 0.45.1/Git SHA-1 profile; macOS ARM64. Isolated real-native candidate/scanner and metadata-only fixtures; CARGO_INCREMENTAL=0. Installed binaries/services unchanged.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Private plan catalog version 2 adds optional SavedPhysicalInventoryV1 beside an immutable exact plan; inventory-free canonical version 1 bytes unchanged. Private journal version 5 adds inventory digest/encoded length/plan digest and requires the existing tree-plan reference; versions 1–4 retain absent-field serialization/read behavior. Existing 128-record/64-MiB plan budget includes inventory bodies; each inventory remains 8 MiB. Journal budget/reservations and public/native/companion/WAL formats unchanged.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/stack_checkout_plans/inventory.rs
- crates/jj-store-cli/tests/stack_checkout_plans/inventory_process.rs
- crates/jj-store-cli/tests/stack_checkout_plans/linkage.rs
- crates/jj-store-cli/tests/stack_checkout_plans.rs
- crates/jj-store-jj/src/local_execution/mod.rs
- crates/jj-store-server/src/stack_checkout_plans/catalog.rs
- crates/jj-store-server/src/stack_checkout_plans/inventory.rs
- crates/jj-store-server/src/stack_checkout_plans/tests/fixture.rs
- crates/jj-store-server/src/stack_checkout_plans/tests.rs
- crates/jj-store-server/src/stack_checkout_plans.rs
- crates/jj-store-server/src/stack_command_journal/catalog.rs
- crates/jj-store-server/src/stack_command_journal/execution.rs
- crates/jj-store-server/src/stack_command_journal/inventory.rs
- crates/jj-store-server/src/stack_command_journal/transition.rs
- crates/jj-store-server/src/stack_command_journal/tree_checkout.rs
- crates/jj-store-server/src/stack_command_journal.rs
- docs/implementation-plan.md
- docs/maintenance/coverage.json
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
- JST-JJ-004
- JST-GOV-002
- JST-GOV-003
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Exact new target fails with two missing API diagnostics. The first implementation/queued expanded compile fails because the owning inventory bound was not exported. Exporting that constant repairs both attempts; all final focused runs pass. Existing changed-tree metadata admission assertion is strengthened to require complete inventory as well as the exact plan.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Private original input only. Authenticate principal/request/intent/prepared selection and complete recovery binding before catalog access, validate exact saved plan linkage, sync immutable inventory before journal reference under journal-then-plan lock order. Immutable reopen/replay, orphan-body recovery, missing-body refusal, terminal non-upgrade, legacy bytes, contention and process loss grant no current physical validity, publication proof or checkout completion. New changed-tree publication admission requires both references.
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/stack_checkout_plans/inventory.rs#saved_complete_inventory_is_required_for_changed_tree_publication
- crates/jj-store-cli/tests/stack_checkout_plans/inventory.rs#immutable_inventory_reopen_replays_without_scanning_new_editor_state
- crates/jj-store-cli/tests/stack_checkout_plans/inventory.rs#inventory_binding_and_original_authentication_precede_store_access
- crates/jj-store-cli/tests/stack_checkout_plans/inventory.rs#missing_inventory_and_tampered_reference_are_never_repaired_from_input
- crates/jj-store-cli/tests/stack_checkout_plans/inventory.rs#terminal_legacy_request_never_acquires_inventory_or_changes_bytes
- crates/jj-store-cli/tests/stack_checkout_plans/inventory.rs#inventory_contention_releases_journal_and_rejects_wrong_actor_first
- crates/jj-store-cli/tests/stack_checkout_plans/inventory_process.rs#process_loss_on_each_side_of_inventory_reference_retains_exact_original
- crates/jj-store-cli/tests/stack_checkout_plans/inventory_process.rs#exit_after_inventory_selection
- crates/jj-store-server/src/stack_checkout_plans/tests.rs#replaced_store_or_lock_is_unavailable_and_original_selection_is_unchanged
- crates/jj-store-server/src/stack_checkout_plans/tests.rs#unknown_plan_acknowledgement_keeps_exact_input_and_replay_syncs_it
- crates/jj-store-server/src/stack_checkout_plans/tests.rs#aggregate_capacity_fails_before_selection_without_evicting_original
- crates/jj-store-server/src/stack_checkout_plans/tests.rs#unknown_inventory_acknowledgement_reopens_exact_body_without_clearing_plan
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M21-031 details 2](details-02.md)
- [E-M21-031 details 3](details-03.md)
