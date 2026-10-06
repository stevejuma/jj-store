---
schema: 1
id: E-M21-021
---

# E-M21-021

<!-- evidence-field:id markdown -->
## Id

E-M21-021
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

rykwtlvy
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Explicit retained GitBackend extra-metadata restoration and exact interrupted retry
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; JJ 0.45.1; Git 2.55.0; macOS ARM64 real disposable stock-JJ folders. Installed executables/services unchanged; CARGO_INCREMENTAL=0.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Pinned upstream GitBackend protobuf and stacked-table bytes. Existing journal V1/V2, combination V1, recovery witness V1, operation archive V1, reserved ref V2, companion/native/WAL/manifest formats unchanged. No public HTTP/schema/client change.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-jj/src/local_browser/git_read/extras.rs
- crates/jj-store-jj/src/local_browser/git_read.rs
- crates/jj-store-jj/src/local_execution/guard.rs
- crates/jj-store-jj/src/local_execution/recovery/extras/destination.rs
- crates/jj-store-jj/src/local_execution/recovery/extras/materialize.rs
- crates/jj-store-jj/src/local_execution/recovery/extras.rs
- crates/jj-store-jj/src/local_execution/recovery/records/destination.rs
- crates/jj-store-jj/src/local_execution/recovery/records/synced.rs
- crates/jj-store-jj/src/local_execution/recovery/records.rs
- crates/jj-store-jj/src/local_execution/recovery/witness.rs
- crates/jj-store-jj/src/local_execution/recovery.rs
- crates/jj-store-jj/tests/stack_source_recovery/extras.rs
- crates/jj-store-jj/tests/stack_source_recovery/extras_lifecycle.rs
- crates/jj-store-jj/tests/stack_source_recovery/extras_limits.rs
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

Two exact new integration cases failed with three absent RecoveredHistory::restore_extras diagnostics. Initial implementation passed both but emitted one unused Backend import warning, removed. Expanded lifecycle fixture used unwrap_err requiring Debug on an owned lease; one compile diagnostic corrected with a direct canceled-result match. Review fixed a large-entry batch zero-progress risk, accounted for unique combined source/candidate bounds and added final identity/snapshot revalidation without weakening assertions.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

One explicit private saved-history extra recovery step and shared nonblocking lock coordination. Bounded no-follow capture, existing native metadata comparison, private pinned-upstream canonical materialization, synced create-only missing segments/head markers, preserved source bytes and original request retry. All existing read paths remain non-repairing.
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-jj/tests/stack_source_recovery/extras.rs#exact_missing_extras_restore_after_source_loss_without_import_heads_or_checkout
- crates/jj-store-jj/tests/stack_source_recovery/extras.rs#conflicting_original_extra_record_fails_complete_preflight_without_table_selection
- crates/jj-store-jj/tests/stack_source_recovery/extras.rs#held_upstream_extra_lock_rejects_without_waiting_or_selecting
- crates/jj-store-jj/tests/stack_source_recovery/extras.rs#unsafe_extra_directories_segments_heads_and_locks_never_touch_targets
- crates/jj-store-jj/tests/stack_source_recovery/extras_lifecycle.rs#cancellation_after_segment_retains_admission_until_exit_and_retry_preserves_bytes
- crates/jj-store-jj/tests/stack_source_recovery/extras_lifecycle.rs#replaced_extra_lock_and_authorization_loss_leave_no_head_and_exact_retry_finishes
- crates/jj-store-jj/tests/stack_source_recovery/extras_lifecycle.rs#process_loss_after_extra_segment_reopens_saved_witness_and_exact_retry
- crates/jj-store-jj/tests/stack_source_recovery/extras_limits.rs#orphan_segment_conflict_fails_before_head_selection_and_exact_bytes_allow_retry
- crates/jj-store-jj/tests/stack_source_recovery/extras_limits.rs#oversized_segment_fails_without_selecting_or_truncating_source_bytes
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M21-021 details 2](details-02.md)
- [E-M21-021 details 3](details-03.md)
