---
schema: 1
id: E-M21-019
---

# E-M21-019

<!-- evidence-field:id markdown -->
## Id

E-M21-019
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

rknmpkzm
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Registered LocalTrusted preparation context and immutable ancestry policy
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; JJ 0.45.1; Git 2.55.0; macOS ARM64 isolated real stock-JJ folders with actual CLI LocalFolders and DeviceRepositories. Installed binaries/services unchanged.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Owned RegisteredAuthority, ActiveCombination and CombinationAuthority interfaces; trusted Reword input adds immutable_heads. No persisted or public transport format change. Existing journal catalog V1/V2, LocalExecutionV1, OperationArchiveV1, reserved retention, companion/native/WAL/manifest formats remain unchanged.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/local_browser/bounded_reads.rs
- crates/jj-store-cli/tests/stack_registered_preparation/recovery.rs
- crates/jj-store-cli/tests/stack_registered_preparation.rs
- crates/jj-store-jj/src/local_execution/reword/policy.rs
- crates/jj-store-jj/src/local_execution/reword.rs
- crates/jj-store-jj/tests/stack_operation_archive.rs
- crates/jj-store-jj/tests/stack_reword_preparation/lifecycle.rs
- crates/jj-store-jj/tests/stack_reword_preparation/policy.rs
- crates/jj-store-jj/tests/stack_reword_preparation.rs
- crates/jj-store-jj/tests/stack_source_recovery.rs
- crates/jj-store-server/src/stack_preparation/registered.rs
- crates/jj-store-server/src/stack_preparation.rs
- crates/jj-store-server/tests/stack_local_preparation/lifecycle.rs
- crates/jj-store-server/tests/stack_local_preparation.rs
- docs/implementation-plan.md
- docs/maintenance/coverage.json
- docs/site/technical/lifecycle.md
- docs/site/technical/local.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/documentation.md
- spec/src/jj-integration.md
- spec/src/local-jj.md
- spec/src/milestones.md
- spec/src/verification.md
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

stack-registered-preparation-before.log: one import diagnostic names three absent authority types. Initial real registration/scanner cases pass after implementation. Immutable-ancestor expansion initially names the pre-snapshot commit, which becomes hidden when stock JJ creates its child; fixture now selects the actual parent identity before checking immutable closure. A stale-generation assertion was initially placed after removal/retained request reuse; moved to a fresh still-registered request, preserving the expected Stale contract.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

One registered preparation-context and policy boundary, with configured LocalTrusted Admin principal, current device folder/generation reauthorization, existing bounded physical observer, trusted combination-owner seam and exact immutable ancestry before candidate writes. Direct JJ, server preparation/journal and CLI registration/read consumers run. No transport, setup, operation publication or checkout enabled.
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/stack_registered_preparation.rs#registered_folder_and_real_observation_gate_preparation_and_original_replay
- crates/jj-store-cli/tests/stack_registered_preparation.rs#caller_mode_missing_combination_and_immutable_ancestry_reject_before_selection
- crates/jj-store-cli/tests/stack_registered_preparation/recovery.rs#registered_pending_recovery_resumes_saved_archive_after_operation_loss_without_scanning
- crates/jj-store-jj/tests/stack_reword_preparation/policy.rs#immutable_head_ancestry_unrelated_heads_and_duplicate_policy_fail_before_candidate_writes
- crates/jj-store-server/tests/stack_local_preparation.rs#real_preparation_joins_exact_preview_retention_and_private_recovery_witness
- crates/jj-store-cli/tests/local_browser.rs#registration_replays_across_restart_and_removal_preserves_metadata
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M21-019 details 2](details-02.md)
- [E-M21-019 details 3](details-03.md)
- [E-M21-019 details 4](details-04.md)
