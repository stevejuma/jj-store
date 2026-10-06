---
schema: 1
id: E-M21-035
---

# E-M21-035

<!-- evidence-field:id markdown -->
## Id

E-M21-035
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

lvuxnmqs
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Explicit real initial merge/WIP setup with registered original execution and bound combination selection
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; JJ 0.45.1/Git SHA-1; macOS ARM64; CARGO_INCREMENTAL=0. Disposable real colocated JJ folders, actual registry/physical scanner, stock-JJ readers and reserved archive retention.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No public wire, native, companion, WAL or private catalog format changes. Journal versions 1–6, execution version 1, combination snapshot/catalog version 1 and retained operation archive version 1 keep their interpretations. Existing sticky/fixed UI and package manifests are untouched.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/stack_registered_preparation/setup/recovery.rs
- crates/jj-store-cli/tests/stack_registered_preparation/setup.rs
- crates/jj-store-cli/tests/stack_registered_preparation.rs
- crates/jj-store-jj/src/local_evidence/operation_archive.rs
- crates/jj-store-jj/src/local_execution/mod.rs
- crates/jj-store-jj/src/local_execution/recovery.rs
- crates/jj-store-jj/src/local_execution/reword.rs
- crates/jj-store-jj/src/local_execution/setup/report.rs
- crates/jj-store-jj/src/local_execution/setup.rs
- crates/jj-store-jj/tests/stack_setup_preparation/lifecycle.rs
- crates/jj-store-jj/tests/stack_setup_preparation/policy.rs
- crates/jj-store-jj/tests/stack_setup_preparation.rs
- crates/jj-store-server/src/stack_combinations.rs
- crates/jj-store-server/src/stack_command_journal/checkout.rs
- crates/jj-store-server/src/stack_command_journal/transition.rs
- crates/jj-store-server/src/stack_preparation/registered.rs
- crates/jj-store-server/src/stack_preparation/setup/apply.rs
- crates/jj-store-server/src/stack_preparation/setup/report.rs
- crates/jj-store-server/src/stack_preparation/setup.rs
- crates/jj-store-server/src/stack_preparation.rs
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

Real reword preparation baseline passed, while first-use setup had no provider-derived generation or registered coordinated execution. Initial new provider tests exposed a stale fixture pointer and incorrect resolved-label predicate; provider visibility, fixture scope and two transient existing journal errors are explicitly recorded.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Explicit first-use Setup only. Exact existing local/known-remote target, unchanged native baseline, unpublished merge/WIP generation, original history/refs/other workspace preservation, complete physical guards, once-assigned server combination identity, journal-before-retention recovery, provider-derived after-state, bound publication admission and unchanged native checkout before combination sync/final acknowledgement. Original replay never prepares a replacement, rewinds current state or recreates missing staged metadata.
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/stack_registered_preparation/setup.rs#registered_setup_creates_real_merge_wip_and_selects_combination_only_after_native_checkout
- crates/jj-store-cli/tests/stack_registered_preparation/setup.rs#registered_setup_does_not_recreate_missing_staged_state
- crates/jj-store-cli/tests/stack_registered_preparation/setup.rs#registered_setup_rejects_stale_physical_selection_without_publication
- crates/jj-store-cli/tests/stack_registered_preparation/setup/recovery.rs#setup_original_apply_recovers_retained_source_after_metadata_loss_and_git_gc
- crates/jj-store-cli/tests/stack_registered_preparation/setup/recovery.rs#setup_original_retention_retry_keeps_saved_candidate_and_combination_identity
- crates/jj-store-cli/tests/stack_registered_preparation/setup/recovery.rs#setup_replays_original_head_receipt_and_pointer_interruptions_without_repreparing
- crates/jj-store-cli/tests/stack_registered_preparation/setup/recovery.rs#setup_revoked_registration_prevents_original_preparation_and_apply_recovery
- crates/jj-store-jj/tests/stack_setup_preparation.rs#actual_setup_retains_original_history_refs_and_dirty_bytes_with_unpublished_merge_wip
- crates/jj-store-jj/tests/stack_setup_preparation.rs#setup_requires_exact_existing_target_and_complete_physical_guard
- crates/jj-store-jj/tests/stack_setup_preparation/lifecycle.rs#authorization_loss_after_candidate_write_exposes_no_preparation_or_publication
- crates/jj-store-jj/tests/stack_setup_preparation/lifecycle.rs#cancellation_after_candidate_write_releases_lease_only_when_worker_exits
- crates/jj-store-jj/tests/stack_setup_preparation/policy.rs#distinct_target_tree_and_other_workspace_selection_survive_setup
- crates/jj-store-jj/tests/stack_setup_preparation/policy.rs#setup_resolves_only_an_exact_already_known_remote_bookmark
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M21-035 details 2](details-02.md)
- [E-M21-035 details 3](details-03.md)
- [E-M21-035 details 4](details-04.md)
