---
schema: 1
id: E-M13-026
---

# E-M13-026

<!-- evidence-field:id markdown -->
## Id

E-M13-026
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M13.5
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-04
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

implementation
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

zzskmqvv
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Creation slice normzvzq/c21a0e4e committed. M13.5 has no independent code/history translation. Conversion API/UI capability remains disabled.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; pinned JJ 0.45.1; SHA-1 Git profile; locked Cargo dependencies; own workspace target directory.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Owned bounded CodeArchiveV1, RepositoryArchiveV1 and explicit WorkSnapshotArchiveV1. Native/companion records retain existing codecs and identities. Archive is provenance; settings, credentials, grants and execution journals are excluded.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-jj/src/conversion.rs
- crates/jj-store-jj/src/lib.rs
- crates/jj-store-jj/src/local_evidence/capture.rs
- crates/jj-store-jj/src/local_evidence/mod.rs
- crates/jj-store-jj/tests/conversion.rs
- crates/jj-store-service/src/conversion/agent.rs
- crates/jj-store-service/src/conversion/collab.rs
- crates/jj-store-service/src/conversion/history.rs
- crates/jj-store-service/src/conversion/review.rs
- crates/jj-store-service/src/conversion.rs
- crates/jj-store-service/src/lib.rs
- crates/jj-store-service/src/local_repository/conversion.rs
- crates/jj-store-service/src/local_repository.rs
- crates/jj-store-service/tests/conversion.rs
- crates/jj-store-wal/src/publication/native_copy.rs
- crates/jj-store-wal/src/publication/review_catalog_engine.rs
- crates/jj-store-wal/src/publication/work.rs
- crates/jj-store-wal/src/publication.rs
- spec/src/collaboration.md
- spec/src/jj-integration.md
- spec/src/local-jj.md
- spec/src/projects.md
- spec/src/publication.md
- spec/src/spec-governance.md
- spec/src/milestones.md
- spec/src/mutations.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-jj/tests/conversion.rs#bidirectional_code_conversion_preserves_evolution_view_and_dirty_source
- crates/jj-store-service/tests/conversion.rs#local_history_archive_retains_every_selected_work_version_and_exact_git_evidence
- crates/jj-store-service/tests/conversion.rs#native_history_archive_contains_selected_evidence_and_excludes_configuration
- crates/jj-store-jj/tests/local_evidence.rs#capsule_retains_full_source_context_after_op_store_loss_and_git_gc
- crates/jj-store-wal/tests/native_copy.rs#native_copy_restores_code_agent_review_and_installs_manifest_last
- crates/jj-store-wal/tests/work_store.rs#linked_tasks_comments_pins_and_native_copy
- crates/jj-store-wal/tests/review_catalog.rs#review_root_rejects_snapshot_state_not_reconstructed_by_events
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M13-026 details 2](details-02.md)
- [E-M13-026 details 3](details-03.md)
