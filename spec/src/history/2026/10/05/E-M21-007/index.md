---
schema: 1
id: E-M21-007
---

# E-M21-007

<!-- evidence-field:id markdown -->
## Id

E-M21-007
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

mkpzlpsn
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Private bounded physical workspace observation foundation
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; custom JJ profile remains 0.45.1; parent fef62baa; macOS ARM64 focused service tests
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No persisted/HTTP/RPC/companion/native format change. Domain-separated private observation fingerprints use repository and binding generation, physical workspace identity, framed sorted directory/entry state and exact content digests. Typed service inventory is not serialized authority.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-service/src/lib.rs
- crates/jj-store-service/src/working_copy.rs
- crates/jj-store-service/src/working_tree/mod.rs
- crates/jj-store-service/src/working_tree/walk.rs
- crates/jj-store-service/tests/working_tree_scan.rs
- docs/implementation-plan.md
- docs/maintenance/coverage.json
- docs/site/technical/architecture.md
- spec/src/architecture.md
- spec/src/documentation.md
- spec/src/local-jj.md
- spec/src/mutations.md
- spec/src/verification.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

No whole-workspace physical fingerprint existed; individual text Save could not establish checkout-wide obstacles or content state.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Read-only descriptor-relative service scanner, shared private file stamp helper, no provider/wire/UI integration. Limits are 8192 cumulative directory entries, 16 MiB cumulative content across two passes, 2 MiB per file, 64 levels and two seconds. Both passes must agree; result is an observation, not a filesystem CAS.
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-service/tests/working_tree_scan.rs#physical_scan_is_read_only_sorted_and_binds_exact_binary_modes_and_names
- crates/jj-store-service/tests/working_tree_scan.rs#scan_includes_untracked_obstacles_but_never_follows_links_or_metadata
- crates/jj-store-service/tests/working_tree_scan.rs#physical_scan_limits_and_cancellation_return_no_partial_selection
- crates/jj-store-service/tests/working_tree_scan.rs#scan_rejects_hardlinks_special_files_unsafe_names_and_workspace_replacement
- crates/jj-store-service/tests/working_tree_scan.rs#later_file_edits_are_detected_by_the_second_bounded_pass
- crates/jj-store-service/tests/working_tree_scan.rs#physical_scan_enforces_depth_deadline_and_verify_cancellation
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M21-007 details 2](details-02.md)
- [E-M21-007 details 3](details-03.md)
