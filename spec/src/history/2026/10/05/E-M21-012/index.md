---
schema: 1
id: E-M21-012
---

# E-M21-012

<!-- evidence-field:id markdown -->
## Id

E-M21-012
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

mvvlrkuw
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Private exact stack preparation journal and monotonic historical receipt recovery
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Pinned Rust 1.98.1; JJ 0.45.1 contracts; macOS ARM64; pnpm 10.32.1. Tests use private disposable temporary state and development test outputs only.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

New bounded installation-private StackCommandJournal JSON version 1 outside checkouts: at most 128 retained records/8 MiB with 16 KiB receipt-growth reservation per record. Scope binds installation/repository/physical workspace/JJ workspace/profile; original request retains binding/combination generations. No native identity, manifest, WAL, companion or public HTTP/client format change.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-server/src/lib.rs
- crates/jj-store-server/src/stack_command_journal/catalog.rs
- crates/jj-store-server/src/stack_command_journal/tests.rs
- crates/jj-store-server/src/stack_command_journal/transition.rs
- crates/jj-store-server/src/stack_command_journal.rs
- crates/jj-store-server/tests/stack_command_journal/admission.rs
- crates/jj-store-server/tests/stack_command_journal/fixture.rs
- crates/jj-store-server/tests/stack_command_journal.rs
- docs/implementation-plan.md
- docs/maintenance/coverage.json
- docs/site/technical/lifecycle.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/documentation.md
- spec/src/local-jj.md
- spec/src/milestones.md
- spec/src/verification.md
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

The exact retained-preparation regression cannot import its missing server journal module.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Private leaf server journal over the unchanged shared physical file primitive and existing owned service contracts. Exact regression, complete new integration binary, focused private journal fault/process-loss tests, owning server all-target checks and no-deps strict lint, source boundaries, docs and mapped spec/progress checks. Shared file primitive and its existing consumers remain unchanged; no HTTP route, runtime executor or UI capability changes.
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-server/tests/stack_command_journal.rs#retained_preparation_survives_reopen_and_conflicting_request_reuse
- crates/jj-store-server/tests/stack_command_journal.rs#publication_checkout_and_retirement_are_monotonic_across_reopen
- crates/jj-store-server/tests/stack_command_journal.rs#pending_checkout_blocks_competing_publication_until_verified
- crates/jj-store-server/tests/stack_command_journal/admission.rs#near_full_admission_reserves_space_for_blocked_checkout_and_retirement
- crates/jj-store-server/src/stack_command_journal/tests.rs#lost_coordination_after_atomic_replacement_is_unknown_and_replay_syncs_original
- crates/jj-store-server/src/stack_command_journal/tests.rs#process_loss_after_replacement_recovers_exact_unresolved_receipt
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M21-012 details 2](details-02.md)
- [E-M21-012 details 3](details-03.md)
