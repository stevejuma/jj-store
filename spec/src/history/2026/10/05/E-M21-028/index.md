---
schema: 1
id: E-M21-028
---

# E-M21-028

<!-- evidence-field:id markdown -->
## Id

E-M21-028
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

olqwlryy
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Exact retained tree-changing checkout plan and physical preflight
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; JJ 0.45.1; Git 2.55.0; pinned Git SHA-1 profile; macOS ARM64 disposable stock-JJ fixtures; CARGO_INCREMENTAL=0. Installed services and binaries unchanged.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

New bounded canonical installation-private TreeCheckoutPlanV1 data only. Maximum 32 MiB encoding; native checkout 64 KiB and tree-state 4 MiB; tree scans 4096 entries, 2 MiB per blob and 8 MiB file payload per source/candidate tree. Existing journal, receipts, native/companion/WAL and public transport formats unchanged; no writable capability.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-jj/src/local_browser/working_tree/current/decode.rs
- crates/jj-store-jj/src/local_browser/working_tree/current.rs
- crates/jj-store-jj/src/local_browser/working_tree.rs
- crates/jj-store-jj/src/local_execution/mod.rs
- crates/jj-store-jj/src/local_execution/recovery/tree_checkout/effects.rs
- crates/jj-store-jj/src/local_execution/recovery/tree_checkout/observation.rs
- crates/jj-store-jj/src/local_execution/recovery/tree_checkout/plan.rs
- crates/jj-store-jj/src/local_execution/recovery/tree_checkout.rs
- crates/jj-store-jj/src/local_execution/recovery.rs
- crates/jj-store-jj/tests/stack_tree_checkout/adversarial.rs
- crates/jj-store-jj/tests/stack_tree_checkout/lifecycle.rs
- crates/jj-store-jj/tests/stack_tree_checkout/process.rs
- crates/jj-store-jj/tests/stack_tree_checkout.rs
- docs/implementation-plan.md
- docs/maintenance/coverage.json
- docs/site/technical/local.md
- spec/src/architecture.md
- spec/src/documentation.md
- spec/src/jj-integration.md
- spec/src/local-jj.md
- spec/src/verification.md
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-LOCAL-005
- JST-JJ-004
- JST-JJ-002
- JST-GOV-003
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Existing unchanged-tree baseline passes 10 cases. New target initially fails compilation because there is no changed-tree checkout-plan API. Initial implementation had three compile errors, a fixture missing native descendant rebase and an incorrect resolved-label expectation; cancellation fixture then had one Debug-bound compile error. All corrected before final 18-parent pass. Strict lint then found one comparison-to-empty diagnostic and the final review acknowledgement needed coverage.json formatting; both corrected with no suppressed checks.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Private provider preparation/revalidation only. Exact retained archive and verified raw reachable Git objects determine the named workspace before/after trees and file effects. Original foreground ownership, complete trusted guard bracketing, canonical native metadata, per-file stamps and ancestor-directory identities remain pinned. Dirty affected sources, untracked/ignored addition targets, unsafe links/modes, sparse scope, ASCII case aliases, unsupported Unicode and file/directory transitions fail without writes. No physical installer, journal plan storage, transport or runtime command is enabled.
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-jj/tests/stack_tree_checkout/adversarial.rs#dirty_missing_and_wrong_mode_source_files_are_preserved_and_rejected
- crates/jj-store-jj/tests/stack_tree_checkout/adversarial.rs#untracked_and_ignored_addition_targets_are_not_adopted_even_with_matching_bytes
- crates/jj-store-jj/tests/stack_tree_checkout/adversarial.rs#safe_deletion_keeps_unrelated_dirty_and_ignored_files_untouched
- crates/jj-store-jj/tests/stack_tree_checkout/adversarial.rs#unsafe_files_and_parent_links_cannot_redirect_checkout_preflight
- crates/jj-store-jj/tests/stack_tree_checkout/adversarial.rs#replaced_files_and_directories_invalidate_saved_plan_even_when_bytes_match
- crates/jj-store-jj/tests/stack_tree_checkout/adversarial.rs#aliases_and_file_directory_transitions_are_rejected_without_changes
- crates/jj-store-jj/tests/stack_tree_checkout/adversarial.rs#oversized_blobs_are_rejected_before_physical_capture
- crates/jj-store-jj/tests/stack_tree_checkout/adversarial.rs#plan_decode_rejects_wrong_digest_noncanonical_and_unsupported_fields
- crates/jj-store-jj/tests/stack_tree_checkout/adversarial.rs#symlink_candidate_is_readable_but_cannot_gain_checkout_plan_support
- crates/jj-store-jj/tests/stack_tree_checkout/adversarial.rs#recomputed_plan_digest_cannot_reauthorize_a_tampered_retained_request
- crates/jj-store-jj/tests/stack_tree_checkout/adversarial.rs#creating_a_previously_missing_parent_invalidates_saved_absence
- crates/jj-store-jj/tests/stack_tree_checkout/adversarial.rs#native_sparse_checkout_is_unsupported_without_expanding_its_disk_scope
- crates/jj-store-jj/tests/stack_tree_checkout/lifecycle.rs#saved_original_plan_revalidates_after_reopen_without_new_preparation
- crates/jj-store-jj/tests/stack_tree_checkout/lifecycle.rs#changing_physical_guard_cannot_return_a_plan_or_touch_checkout_metadata
- crates/jj-store-jj/tests/stack_tree_checkout/lifecycle.rs#cancellation_retains_actual_foreground_lock_until_blocked_worker_exits
- crates/jj-store-jj/tests/stack_tree_checkout/lifecycle.rs#denied_complete_guard_leaves_source_and_physical_state_intact
- crates/jj-store-jj/tests/stack_tree_checkout/process.rs#worker
- crates/jj-store-jj/tests/stack_tree_checkout/process.rs#killed_worker_saved_plan_reopens_against_original_retained_tree_and_files
- crates/jj-store-jj/tests/stack_tree_checkout.rs#retained
- crates/jj-store-jj/tests/stack_tree_checkout.rs#retained_changed_tree_can_be_planned_without_checkout_or_publication
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M21-028 details 2](details-02.md)
- [E-M21-028 details 3](details-03.md)
- [E-M21-028 details 4](details-04.md)
