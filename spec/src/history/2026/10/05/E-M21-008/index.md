---
schema: 1
id: E-M21-008
---

# E-M21-008

<!-- evidence-field:id markdown -->
## Id

E-M21-008
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

slwqrnmn
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Private exact local WC baseline and physical ignore comparison
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; JJ library/installed workspace tool 0.45.1; pnpm 10.32.1; macOS ARM64; existing explicit development JJ executable, no rebuild/install/service replacement
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No HTTP/RPC/native/WAL/companion/persisted format change. Private typed baseline retains repository/generation, exact operation/view, workspace name, WC commit and tree terms. Raw content digest domain matches existing physical scan. Regular gitignore bytes remain private revalidated scan data.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-jj/src/local_browser/mod.rs
- crates/jj-store-jj/src/local_browser/working_tree/ignores.rs
- crates/jj-store-jj/src/local_browser/working_tree.rs
- crates/jj-store-jj/tests/working_tree_baseline.rs
- crates/jj-store-service/src/working_tree/mod.rs
- crates/jj-store-service/src/working_tree/projection.rs
- crates/jj-store-service/src/working_tree/walk.rs
- crates/jj-store-service/tests/working_tree_local.rs
- crates/jj-store-service/tests/working_tree_projection.rs
- docs/implementation-plan.md
- docs/maintenance/coverage.json
- docs/site/technical/architecture.md
- docs/site/technical/local.md
- spec/src/architecture.md
- spec/src/documentation.md
- spec/src/jj-integration.md
- spec/src/local-jj.md
- spec/src/mutations.md
- spec/src/ui.md
- spec/src/verification.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

No operation-pinned whole-WC raw baseline or JJ captured-rule comparison existed; physical inventory alone could not distinguish tracked changes and ignored candidate files.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Private read-only JJ provider and service comparison, complete-result bounds (4096 baseline files, 8 MiB raw content, 2 MiB/file) and upstream JJ ignore precedence. Historical WC baseline is explicitly distinct from active physical checkout/combination assignment.
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-jj/tests/working_tree_baseline.rs#baseline_reads_exact_workspace_at_selected_operation_without_publication
- crates/jj-store-jj/tests/working_tree_baseline.rs#baseline_limits_objects_and_rejects_submodules_without_partial_results
- crates/jj-store-jj/tests/working_tree_baseline.rs#ignore_rules_use_upstream_precedence_and_do_not_reinclude_ignored_directories
- crates/jj-store-jj/tests/working_tree_baseline.rs#ignore_capture_limits_duplicate_sources_and_cancellation_fail_closed
- crates/jj-store-service/tests/working_tree_projection.rs#projection_preserves_tracked_ignored_changes_and_all_untracked_obstacles
- crates/jj-store-service/tests/working_tree_projection.rs#projection_uses_content_and_mode_not_physical_timestamps_for_dirty_state
- crates/jj-store-service/tests/working_tree_projection.rs#projection_fails_closed_on_stale_binding_captured_rules_and_cancellation
- crates/jj-store-service/tests/working_tree_local.rs#real_folder_comparison_preserves_operations_metadata_and_unsnapshotted_files
- crates/jj-store-service/tests/working_tree_scan.rs#physical_scan_is_read_only_sorted_and_binds_exact_binary_modes_and_names
- crates/jj-store-service/tests/working_tree_scan.rs#scan_includes_untracked_obstacles_but_never_follows_links_or_metadata
- crates/jj-store-service/tests/working_tree_scan.rs#physical_scan_limits_and_cancellation_return_no_partial_selection
- crates/jj-store-service/tests/working_tree_scan.rs#scan_rejects_hardlinks_special_files_unsafe_names_and_workspace_replacement
- crates/jj-store-service/tests/working_tree_scan.rs#physical_scan_enforces_depth_deadline_and_verify_cancellation
- crates/jj-store-service/tests/working_tree_scan.rs#later_file_edits_are_detected_by_the_second_bounded_pass
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M21-008 details 2](details-02.md)
- [E-M21-008 details 3](details-03.md)
