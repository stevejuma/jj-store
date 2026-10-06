---
schema: 1
id: E-READ-043
---

# E-READ-043

<!-- evidence-field:id markdown -->
## Id

E-READ-043
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M13.4
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-04
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change rlwzuolo
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Prepared local operation comparisons and selected change-group hydration through bounded immutable graph frontiers
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust1.98.1 (48a229cea 2026-09-01); jj-lib0.45.1; JJ0.45.1; macOS ARM64.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Disposable local-operations version2 uses JSTLOP2 NUL magic, selected64-byte operation, sorted128-byte records containing operation ID and canonical operation/view SHA256 fingerprints, and whole-file checksum. Resident256bytes/record plus64KiB admitted before allocation. Compact local decoder retains the already checksum-verified unchanged owned JJ format-6 segment with separate byte admission, validates fixed framing, sorted unique lookup, topological parents/generations and change offsets. Frontier/group/association memory is admitted before growth. Public schemas/routes/cursors, native/WAL/companion/receipt formats and ordinary limits unchanged.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/local_browser/preparation.rs
- crates/jj-store-cli/tests/local_browser/prepared_comparisons.rs
- crates/jj-store-cli/tests/local_browser/prepared_points_http.rs
- crates/jj-store-cli/tests/local_browser.rs
- crates/jj-store-jj/src/local_browser/dispatch.rs
- crates/jj-store-jj/src/local_browser/mod.rs
- crates/jj-store-jj/src/local_browser/operation_changes.rs
- crates/jj-store-jj/src/local_browser/operation_store.rs
- crates/jj-store-jj/src/local_browser/operations.rs
- crates/jj-store-jj/src/local_browser/preparation/comparison.rs
- crates/jj-store-jj/src/local_browser/preparation/comparison_tests.rs
- crates/jj-store-jj/src/local_browser/preparation/graph.rs
- crates/jj-store-jj/src/local_browser/preparation.rs
- crates/jj-store-jj/src/query/prepared/mod.rs
- crates/jj-store-jj/src/query/prepared/operations.rs
- docs/implementation-plan.md
- spec/src/api.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/compatibility.md
- spec/src/jj-integration.md
- spec/src/local-jj.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-LOCAL-004
- JST-JJ-003
- JST-API-001
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/local_browser/prepared_comparisons.rs#prepared_operation_comparison_skips_shared_history_and_git_objects
- crates/jj-store-cli/tests/local_browser/prepared_comparisons.rs#prepared_operation_change_pages_hydrate_only_selected_groups
- crates/jj-store-cli/tests/local_browser/prepared_points_http.rs#local_point_http_uses_ready_membership_and_only_falls_back_when_cold
- crates/jj-store-jj/src/local_browser/preparation/comparison_tests.rs#frontier_differences_match_exhaustive_ancestry_for_branches_merges_and_redundant_heads
- crates/jj-store-jj/src/local_browser/preparation/comparison_tests.rs#shared_frontiers_stop_early_and_visit_work_and_cancel_limits_fail_explicitly
- crates/jj-store-jj/src/local_browser/preparation/comparison_tests.rs#compact_graph_rejects_invalid_framing_lookup_parents_and_change_offsets
- crates/jj-store-cli/tests/native_lifecycle/graphs.rs#disk_preparation_matches_pinned_upstream_index_bytes
- crates/jj-store-cli/tests/local_browser.rs#local_code_reads_cover_graphs_diffs_roots_conflicts_and_multiple_heads
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-READ-043 details 2](details-02.md)
- [E-READ-043 details 3](details-03.md)
