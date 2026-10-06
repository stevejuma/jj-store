---
schema: 1
id: E-M21-024
---

# E-M21-024

<!-- evidence-field:id markdown -->
## Id

E-M21-024
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

sqrvnnlw
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Guarded unchanged-tree native checkout pointer completion
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; JJ 0.45.1; Git SHA-1/JJ 0.45.1 profile; macOS ARM64 disposable stock-JJ folders. CARGO_INCREMENTAL=0; installed services/executables unchanged.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Installation-private canonical CheckoutWitnessV1; native Checkout operation pointer only. Tree_state, manifest/WAL, companion, journal and combination formats unchanged; no wire/schema change.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-jj/src/local_execution/mod.rs
- crates/jj-store-jj/src/local_execution/recovery/checkout/staging.rs
- crates/jj-store-jj/src/local_execution/recovery/checkout/witness.rs
- crates/jj-store-jj/src/local_execution/recovery/checkout.rs
- crates/jj-store-jj/src/local_execution/recovery/publication/ancestry.rs
- crates/jj-store-jj/src/local_execution/recovery/publication.rs
- crates/jj-store-jj/src/local_execution/recovery/witness.rs
- crates/jj-store-jj/src/local_execution/recovery.rs
- crates/jj-store-jj/tests/stack_unchanged_checkout/lifecycle.rs
- crates/jj-store-jj/tests/stack_unchanged_checkout.rs
- crates/jj-store-service/src/reads.rs
- crates/jj-store-service/src/stack_workspace/selection.rs
- crates/jj-store-service/src/working_tree/guard.rs
- crates/jj-store-service/src/working_tree/mod.rs
- crates/jj-store-service/src/working_tree/observer.rs
- crates/jj-store-service/tests/working_tree_checkout.rs
- docs/implementation-plan.md
- docs/maintenance/coverage.json
- docs/site/technical/architecture.md
- docs/site/technical/local.md
- spec/src/architecture.md
- spec/src/documentation.md
- spec/src/jj-integration.md
- spec/src/local-jj.md
- spec/src/mutations.md
- spec/src/verification.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Two missing provider imports, one absent scanner-guard API and two inherited strict service lint diagnostics reproduced. Prior head publication could not finish native checkout pointer.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Original-request unchanged-tree pointer completion. Canonical bounded capture without writes, required caller persistence before selection, foreground/native head locks, complete source/extra/index and physical guards, deterministic prefix staging, synced atomic selection, exact interrupted retry. Real service scanner guards are distinct from known-file fixture guards; no runtime journal receipt or writable capability implied.
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-jj/tests/stack_unchanged_checkout.rs#exact_unchanged_checkout_updates_only_operation_pointer_and_retries_after_reopen
- crates/jj-store-jj/tests/stack_unchanged_checkout.rs#deterministic_staging_reuses_only_exact_prefix_and_rejects_unsafe_entries
- crates/jj-store-jj/tests/stack_unchanged_checkout.rs#changed_dirty_ignored_and_ignore_files_fail_before_pointer_update_and_preserve_new_bytes
- crates/jj-store-jj/tests/stack_unchanged_checkout.rs#later_cli_checkout_cannot_be_replaced_with_saved_candidate_pointer
- crates/jj-store-jj/tests/stack_unchanged_checkout.rs#scoped_witness_canonical_encoding_and_reuse_reject_tampering
- crates/jj-store-jj/tests/stack_unchanged_checkout.rs#replaced_or_unsafe_checkout_entries_fail_without_changing_targets
- crates/jj-store-jj/tests/stack_unchanged_checkout/lifecycle.rs#subprocess_loss_after_pointer_selection_reopens_original_saved_witness
- crates/jj-store-service/tests/working_tree_checkout.rs#complete_registered_inventory_guards_real_unchanged_checkout_and_new_untracked_files
- crates/jj-store-service/tests/working_tree_checkout.rs#full_guard_rejects_binding_revocation_modes_links_ignored_bytes_and_limits
- crates/jj-store-service/tests/working_tree_checkout.rs#full_guard_cancellation_retains_shared_admission_until_worker_exits
- crates/jj-store-cli/tests/native_lifecycle.rs#read_session_pins_history_and_reports_divergent_heads_without_publication
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M21-024 details 2](details-02.md)
- [E-M21-024 details 3](details-03.md)
