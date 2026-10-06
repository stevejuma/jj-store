---
schema: 1
id: E-M4-007
---

# E-M4-007

<!-- evidence-field:id markdown -->
## Id

E-M4-007
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4
- M4.1
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-27
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

native-content-diffs-and-conflict-descriptors
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/native_lifecycle/conflicts.rs
- crates/jj-store-cli/tests/native_lifecycle/content.rs
- crates/jj-store-cli/tests/native_lifecycle/diff.rs
- crates/jj-store-jj/src/copy_query.rs
- crates/jj-store-jj/src/query/content/conflicts.rs
- crates/jj-store-jj/src/query/content/diff.rs
- crates/jj-store-jj/src/query/content/diff_copies.rs
- crates/jj-store-jj/src/query/content/native.rs
- crates/jj-store-jj/src/query/content/trees.rs
- crates/jj-store-jj/src/query/content.rs
- crates/jj-store-jj/tests/native_backend.rs
- crates/jj-store-model/src/ids.rs
- crates/jj-store-service/src/reads.rs
- spec/src/api.md
- spec/src/cli.md
- spec/src/identity.md
- spec/src/jj-integration.md
- spec/src/mutations.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/native_lifecycle/diff.rs#content_diff_native_lines_modes_binary_symlinks_submodules_and_prefixes
- crates/jj-store-cli/tests/native_lifecycle/diff.rs#content_diff_copy_and_rename_sources_match_pinned_native_stream
- crates/jj-store-cli/tests/native_lifecycle/diff.rs#content_diff_bounds_stale_membership_corruption_and_revocation
- crates/jj-store-cli/tests/native_lifecycle/diff.rs#content_diff_multiple_copy_lineages_are_explicit_without_guessing_one_source
- crates/jj-store-cli/tests/native_lifecycle/diff.rs#content_diff_canceled_copy_terms_cannot_reappear_as_sources
- crates/jj-store-cli/tests/native_lifecycle/conflicts.rs#conflict_descriptor_preserves_native_terms_labels_paths_and_editor_capabilities
- crates/jj-store-cli/tests/native_lifecycle/conflicts.rs#conflict_descriptor_multisided_and_fingerprints_bind_every_input
- crates/jj-store-cli/tests/native_lifecycle/conflicts.rs#conflict_descriptor_limits_revocation_and_verified_content_fail_closed
- crates/jj-store-jj/src/query/content/conflicts.rs#conflict_fingerprint_v1_matches_independent_canonical_vector
- crates/jj-store-jj/tests/native_backend.rs#copy_placeholder_parents_are_boundaries_not_stored_history
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M4-007 details 2](details-02.md)
- [E-M4-007 details 3](details-03.md)
