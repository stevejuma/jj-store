---
schema: 1
id: E-M21-022
---

# E-M21-022

<!-- evidence-field:id markdown -->
## Id

E-M21-022
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

kpomxqxv
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Archive-only source/candidate index restoration with preserved graph visibility
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; JJ 0.45.1; Git 2.55.0; macOS ARM64 real disposable stock-JJ folders. Installed services/executables unchanged; CARGO_INCREMENTAL=0.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Existing pinned default commit-index format 6 and SegmentControl association protobuf. Journal V1/V2, combination V1, recovery witness V1, operation archive V1, reserved ref V2, companion/native/WAL/manifest formats unchanged. No public HTTP/schema/client change.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-jj/src/local_execution/recovery/indexes/archive.rs
- crates/jj-store-jj/src/local_execution/recovery/indexes/destination.rs
- crates/jj-store-jj/src/local_execution/recovery/indexes/materialize.rs
- crates/jj-store-jj/src/local_execution/recovery/indexes/verify.rs
- crates/jj-store-jj/src/local_execution/recovery/indexes.rs
- crates/jj-store-jj/src/local_execution/recovery.rs
- crates/jj-store-jj/tests/stack_source_recovery/indexes.rs
- crates/jj-store-jj/tests/stack_source_recovery/indexes_adversarial.rs
- crates/jj-store-jj/tests/stack_source_recovery/indexes_lifecycle.rs
- crates/jj-store-jj/tests/stack_source_recovery.rs
- docs/implementation-plan.md
- docs/maintenance/coverage.json
- docs/site/technical/local.md
- spec/src/architecture.md
- spec/src/documentation.md
- spec/src/jj-integration.md
- spec/src/local-jj.md
- spec/src/verification.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Initial exact two-case target failed with six diagnostics: three absent restore_indexes API uses, a wrong fixture StoreFactories constructor and two fixture ReadonlyIndex method receivers; one unused fixture import. Implementation runs exposed three missing ObjectId imports, three native Operation conversion inference diagnostics, two absent optional hex-crate references (replaced with bounded local encoding without dependency changes), and one macOS /var no-follow private-path failure (canonicalized only private scratch). Native semantic validation then used the expression builder API instead of the resolved enum, producing one diagnostic in each of two runs before repair. No assertions were weakened.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

One explicit private saved-history commit-index recovery step. Reuse the existing bounded disk-backed builder with an archive-only Source; private native decoding denies fallback traversal. Separate original/candidate association graphs, complete indexed change/parent identity verification, bounded no-follow original-byte capture, create-only segment-before-association selection and exact interrupted retry.
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-jj/tests/stack_source_recovery/indexes.rs#saved_original_and_candidate_indexes_restore_without_future_visibility_or_source_import
- crates/jj-store-jj/tests/stack_source_recovery/indexes.rs#conflicting_existing_index_association_fails_before_missing_segments_are_selected
- crates/jj-store-jj/tests/stack_source_recovery/indexes.rs#valid_existing_cli_index_bytes_are_preserved_while_missing_candidate_is_added
- crates/jj-store-jj/tests/stack_source_recovery/indexes.rs#valid_candidate_index_cannot_substitute_for_the_original_operation_graph
- crates/jj-store-jj/tests/stack_source_recovery/indexes_lifecycle.rs#cancellation_after_one_index_association_retains_admission_and_exact_retry
- crates/jj-store-jj/tests/stack_source_recovery/indexes_lifecycle.rs#process_loss_after_one_index_association_reopens_saved_request_and_preserves_segments
- crates/jj-store-jj/tests/stack_source_recovery/indexes_adversarial.rs#index_directory_replacement_and_authorization_loss_stop_before_association_selection
- crates/jj-store-jj/tests/stack_source_recovery/indexes_adversarial.rs#unsafe_index_directories_files_and_oversized_segments_fail_without_touching_targets
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M21-022 details 2](details-02.md)
- [E-M21-022 details 3](details-03.md)
