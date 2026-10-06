---
schema: 1
id: E-M21-029
---

# E-M21-029

<!-- evidence-field:id markdown -->
## Id

E-M21-029
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

syqlmuxn
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Durable original tree-checkout plan selection and journal references
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; JJ 0.45.1; Git 2.55.0; pinned Git SHA-1 profile; macOS ARM64 isolated real-native and metadata-only fixtures; CARGO_INCREMENTAL=0. Installed services/binaries unchanged.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Separate private canonical plan catalog version 1: 128 immutable records, 32 MiB per plan and 64 MiB aggregate. Private command catalogs use version 4 only when TreeCheckoutReferenceV1 digest/encoded-length links exist; versions 1/2/3 retain absent-field serialization. Existing 8 MiB journal and growth reservations, public transport, native/companion/WAL formats unchanged.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/stack_checkout_plans/candidate.rs
- crates/jj-store-cli/tests/stack_checkout_plans/linkage.rs
- crates/jj-store-cli/tests/stack_checkout_plans/process.rs
- crates/jj-store-cli/tests/stack_checkout_plans/storage.rs
- crates/jj-store-cli/tests/stack_checkout_plans.rs
- crates/jj-store-jj/src/local_execution/recovery/tree_checkout/plan.rs
- crates/jj-store-server/src/lib.rs
- crates/jj-store-server/src/stack_checkout_plans/catalog.rs
- crates/jj-store-server/src/stack_checkout_plans/tests/fixture.rs
- crates/jj-store-server/src/stack_checkout_plans/tests.rs
- crates/jj-store-server/src/stack_checkout_plans.rs
- crates/jj-store-server/src/stack_command_journal/catalog.rs
- crates/jj-store-server/src/stack_command_journal/execution.rs
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
- JST-GOV-003
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

The exact new target initially cannot compile because plan storage does not exist. One existing reword test later reports Journal(Unavailable) during fixture setup; the exact rerun and complete final family pass unchanged. No check was relaxed or assertion removed.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Private durable input only. Authenticate and bind original scope/request/intent/prepared key, recovery, retained commit/archive, candidate, physical selection, before/after trees and ordered affected paths before plan storage. Journal-then-plan locks select/sync canonical input before the digest/length reference. Immutable replay, terminal refusal, saved-body lookup, contention, capacity and unknown acknowledgements grant no publication/installation/completion proof or mutation transport.
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/stack_checkout_plans/linkage.rs#publication_requires_saved_original_plan_and_reopen_resolves_it
- crates/jj-store-cli/tests/stack_checkout_plans/linkage.rs#original_auth_and_linkage_precede_any_store_access
- crates/jj-store-cli/tests/stack_checkout_plans/linkage.rs#terminal_request_never_acquires_new_plan_and_legacy_bytes_stay_unchanged
- crates/jj-store-cli/tests/stack_checkout_plans/linkage.rs#saved_input_is_provider_revalidated_under_original_lease
- crates/jj-store-cli/tests/stack_checkout_plans/linkage.rs#malformed_reference_and_missing_body_never_repair_from_supplied_plan
- crates/jj-store-cli/tests/stack_checkout_plans/linkage.rs#acknowledged_retention_replay_preserves_saved_plan_reference
- crates/jj-store-cli/tests/stack_checkout_plans/linkage.rs#plan_contention_releases_journal_and_cannot_bypass_original_authentication
- crates/jj-store-cli/tests/stack_checkout_plans/process.rs#process_loss_after_plan_before_reference_reuses_original_on_reopen
- crates/jj-store-cli/tests/stack_checkout_plans/process.rs#exit_after_plan
- crates/jj-store-cli/tests/stack_checkout_plans/process.rs#process_loss_after_reference_selection_preserves_original_receipt
- crates/jj-store-cli/tests/stack_checkout_plans/process.rs#exit_after_reference
- crates/jj-store-cli/tests/stack_checkout_plans/storage.rs#open_lookup_and_duplicate_are_immutable_and_never_touch_checkout
- crates/jj-store-cli/tests/stack_checkout_plans/storage.rs#scope_actor_intent_and_conflicting_request_reuse_fail_without_selection
- crates/jj-store-cli/tests/stack_checkout_plans/storage.rs#malformed_noncanonical_and_unsafe_store_state_is_not_repaired
- crates/jj-store-cli/tests/stack_checkout_plans/storage.rs#bounded_record_capacity_preserves_all_original_plans
- crates/jj-store-server/src/stack_checkout_plans/tests.rs#replaced_store_or_lock_is_unavailable_and_original_selection_is_unchanged
- crates/jj-store-server/src/stack_checkout_plans/tests.rs#unknown_plan_acknowledgement_keeps_exact_input_and_replay_syncs_it
- crates/jj-store-server/src/stack_checkout_plans/tests.rs#aggregate_capacity_fails_before_selection_without_evicting_original
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M21-029 details 2](details-02.md)
- [E-M21-029 details 3](details-03.md)
