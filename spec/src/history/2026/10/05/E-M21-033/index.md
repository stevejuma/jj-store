---
schema: 1
id: E-M21-033
---

# E-M21-033

<!-- evidence-field:id markdown -->
## Id

E-M21-033
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

nzzlvwkw
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Actual native tree-state and checkout completion with exact provider-owned recovery
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; pinned JJ 0.45.1/Git SHA-1 profile; macOS ARM64 and Rustix 1.1.5. Real disposable native/scanner and stock-JJ fixtures, CARGO_INCREMENTAL=0.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

New provider-owned canonical native completion frame version 1, 64-KiB bound, saved plan/inventory/installed fingerprint and private/prepared inode stamps. Native metadata remains the pinned upstream protobufs. Existing physical progress version 1, plan catalog version 2, journal version 5, combination snapshot version 1 and public wire/companion/WAL formats unchanged.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/stack_checkout_plans/completion.rs
- crates/jj-store-cli/tests/stack_checkout_plans.rs
- crates/jj-store-jj/src/local_execution/mod.rs
- crates/jj-store-jj/src/local_execution/recovery/checkout.rs
- crates/jj-store-jj/src/local_execution/recovery/tree_checkout/completion/native.rs
- crates/jj-store-jj/src/local_execution/recovery/tree_checkout/completion/storage.rs
- crates/jj-store-jj/src/local_execution/recovery/tree_checkout/completion.rs
- crates/jj-store-jj/src/local_execution/recovery/tree_checkout/installation/files.rs
- crates/jj-store-jj/src/local_execution/recovery/tree_checkout/installation/progress.rs
- crates/jj-store-jj/src/local_execution/recovery/tree_checkout/installation/storage.rs
- crates/jj-store-jj/src/local_execution/recovery/tree_checkout/installation/validation.rs
- crates/jj-store-jj/src/local_execution/recovery/tree_checkout/installation.rs
- crates/jj-store-jj/src/local_execution/recovery/tree_checkout/plan.rs
- crates/jj-store-jj/src/local_execution/recovery/tree_checkout.rs
- crates/jj-store-jj/src/local_execution/recovery/witness.rs
- crates/jj-store-jj/src/local_execution/recovery.rs
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

Physical-only baseline passes with original native metadata. Newly added completion initially encountered internal visibility/mode, private upstream test-field, enum-size and parser diagnostics; all repaired. Exact completion, final affected families and strict lint pass with no product test failures.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Complete exact native metadata from sealed installed files under original foreground admission. Preserve unaffected file states/sparse input, native path ordering and signed mtime; clear stale Watchman clock. Stage both native files and durably acknowledge the exact frame before ordered replacements. Resume only existing original plan/inventory/frame inputs; admit only original fingerprints or exact intended prepared inodes/bytes. Full fresh physical namespace, original backups, source policy/access and held identities guard selections and final proof. Canceled workers retain lease/admission until exit. No application command acknowledgement, selected combination after-state or writable transport is inferred.
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/stack_checkout_plans/completion.rs#native_tree_completion_and_exact_replay_preserve_installed_inodes
- crates/jj-store-cli/tests/stack_checkout_plans/completion.rs#native_file_states_use_component_order_and_preserve_unaffected_records
- crates/jj-store-cli/tests/stack_checkout_plans/completion.rs#changed_physical_file_blocks_before_native_frame_or_selection
- crates/jj-store-cli/tests/stack_checkout_plans/completion.rs#checkout_selected_before_tree_and_missing_completion_evidence_are_rejected
- crates/jj-store-cli/tests/stack_checkout_plans/completion.rs#process_loss_before_frame_and_after_each_native_selection_resumes_exact_input
- crates/jj-store-cli/tests/stack_checkout_plans/completion.rs#partial_initial_frame_accepts_only_exact_retained_prefix
- crates/jj-store-cli/tests/stack_checkout_plans/completion.rs#native_replacement_policy_and_backup_changes_block_remaining_checkout
- crates/jj-store-cli/tests/stack_checkout_plans/completion.rs#exit_at_native_boundary
- crates/jj-store-cli/tests/stack_checkout_plans/completion.rs#authorization_loss_after_tree_selection_retains_original_checkout_and_resumes
- crates/jj-store-cli/tests/stack_checkout_plans/completion.rs#canceled_native_worker_retains_admission_and_foreground_until_exit
- crates/jj-store-jj/src/local_execution/recovery/tree_checkout/completion/native.rs#signed_native_milliseconds_match_upstream_duration_truncation
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M21-033 details 2](details-02.md)
- [E-M21-033 details 3](details-03.md)
