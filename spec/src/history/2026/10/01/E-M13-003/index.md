---
schema: 1
id: E-M13-003
---

# E-M13-003

<!-- evidence-field:id markdown -->
## Id

E-M13-003
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M13
- M13.3
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-01
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

filesystem-git
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

tysxnlom
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- Cargo.lock
- crates/jj-review/src/codec.rs
- crates/jj-review/src/lib.rs
- crates/jj-review/src/portable.rs
- crates/jj-review/tests/portable.rs
- crates/jj-store-jj/Cargo.toml
- crates/jj-store-jj/src/lib.rs
- crates/jj-store-jj/src/local_evidence/capture.rs
- crates/jj-store-jj/src/local_evidence/commits.rs
- crates/jj-store-jj/src/local_evidence/mod.rs
- crates/jj-store-jj/src/local_evidence/operations.rs
- crates/jj-store-jj/src/local_evidence/views.rs
- crates/jj-store-jj/src/local_evidence/wire.rs
- crates/jj-store-jj/src/local_metadata/blobs.rs
- crates/jj-store-jj/src/local_metadata/mod.rs
- crates/jj-store-jj/tests/local_evidence.rs
- crates/jj-store-model/src/ids.rs
- crates/jj-store-model/src/lib.rs
- crates/jj-store-model/src/portable.rs
- spec/src/architecture.md
- spec/src/collaboration.md
- spec/src/identity.md
- spec/src/jj-integration.md
- spec/src/local-jj.md
- spec/src/milestones.md
- spec/src/verification.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-jj/tests/local_evidence.rs#capsule_retains_full_source_context_after_op_store_loss_and_git_gc
- crates/jj-store-jj/tests/local_evidence.rs#capsule_codec_rejects_wrong_profiles_tampering_and_missing_extra_records
- crates/jj-store-jj/tests/local_evidence.rs#root_capsule_preserves_upstream_sentinels_without_git_null_edges
- crates/jj-store-jj/tests/local_evidence.rs#source_codec_keeps_conflicted_refs_remote_tags_and_all_operation_fields
- crates/jj-store-jj/tests/local_evidence.rs#missing_evidence_dependencies_and_blob_limits_never_select_partial_state
- crates/jj-review/tests/portable.rs#portable_anchors_bind_source_profiles_and_exact_utf8_selections
- crates/jj-review/tests/portable.rs#mixed_profile_and_invalid_artifact_records_fail_closed
- crates/jj-review/tests/portable.rs#portable_diff_selection_requires_the_exact_declared_side
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M13-003 details 2](details-02.md)
- [E-M13-003 details 3](details-03.md)
