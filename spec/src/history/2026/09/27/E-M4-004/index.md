---
schema: 1
id: E-M4-004
---

# E-M4-004

<!-- evidence-field:id markdown -->
## Id

E-M4-004
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

native-object-tree-and-verified-content-reads
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/native_lifecycle/content.rs
- crates/jj-store-cli/tests/native_lifecycle.rs
- crates/jj-store-jj/src/codec.rs
- crates/jj-store-jj/src/lib.rs
- crates/jj-store-jj/src/native_dto.rs
- crates/jj-store-jj/src/query/content/trees.rs
- crates/jj-store-jj/src/query/content.rs
- crates/jj-store-jj/src/query/mod.rs
- crates/jj-store-object/src/staging.rs
- crates/jj-store-object/tests/contract.rs
- crates/jj-store-service/src/reads.rs
- spec/src/api.md
- spec/src/cli.md
- spec/src/identity.md
- spec/src/jj-integration.md
- spec/src/mutations.md
- spec/src/object-store.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/native_lifecycle/content.rs#content_read_preserves_every_native_object_field_and_virtual_roots
- crates/jj-store-cli/tests/native_lifecycle/content.rs#content_read_lists_native_directory_entries_with_paths_and_pagination
- crates/jj-store-cli/tests/native_lifecycle/content.rs#content_read_verifies_binary_ranges_before_streaming_and_retains_permits
- crates/jj-store-cli/tests/native_lifecycle/content.rs#content_read_rechecks_revocation_and_limits_and_rejects_staged_objects
- crates/jj-store-cli/tests/native_lifecycle/content.rs#content_read_rejects_corruption_outside_requested_range_and_missing_committed_objects
- crates/jj-store-cli/tests/native_lifecycle/content.rs#content_read_stream_deadline_and_merged_tree_conflicts_preserve_native_terms
- crates/jj-store-cli/tests/native_lifecycle/content.rs#content_read_canceled_chunk_cannot_resume_with_skipped_bytes
- crates/jj-store-object/tests/contract.rs#private_spool_ranges_seek_and_end_at_exact_boundaries
- crates/jj-store-jj/tests/native_codec.rs#all_native_fields_roundtrip_without_identity_loss
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M4-004 details 2](details-02.md)
