---
schema: 1
id: E-CHAT-WORKSPACE-002
---

# E-CHAT-WORKSPACE-002

<!-- evidence-field:id markdown -->
## Id

E-CHAT-WORKSPACE-002
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M24
- M13.3
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-LOCAL-001
- JST-LOCAL-002
- JST-LOCAL-003
- JST-COLLAB-001
- JST-COLLAB-034
- JST-COLLAB-035
- JST-COLLAB-036
- JST-PUB-001
- JST-JJ-003
- JST-MUT-001
- JST-API-001
- JST-CLI-004
- JST-COMPAT-001
- JST-COMPAT-003
- JST-COMPAT-005
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-09
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-review/src/anchors.rs
- crates/jj-review/src/dependencies.rs
- crates/jj-review/src/objects.rs
- crates/jj-review/src/validation.rs
- crates/jj-review/tests/portable.rs
- crates/jj-store-cli/tests/local_review_files.rs
- crates/jj-store-jj/src/local_metadata/evidence_files.rs
- crates/jj-store-jj/src/local_metadata/mod.rs
- crates/jj-store-service/src/local_repository/agent_evidence.rs
- crates/jj-store-service/src/local_repository/collab_external.rs
- crates/jj-store-service/src/local_repository/conversion_import.rs
- crates/jj-store-service/src/local_repository/review_commands.rs
- crates/jj-store-service/src/local_repository/review_io.rs
- crates/jj-store-service/src/local_repository/review_store.rs
- crates/jj-store-service/src/local_repository/review_types.rs
- crates/jj-store-service/tests/local_review.rs
- crates/jj-store-wal/src/publication/review.rs
- crates/jj-store-wal/src/publication/review_projector.rs
- crates/jj-store-wal/tests/review_commands.rs
- schemas/openapi.json
- spec/src/current/contracts/api/41-interactive-agent-execution.md
- spec/src/current/contracts/cli/35-interactive-agent-execution.md
- spec/src/current/contracts/collaboration/23-exact-review-anchors.md
- spec/src/current/contracts/compatibility/13-interactive-agent-execution.md
- spec/src/current/contracts/jj-integration/32-interactive-agent-workspaces.md
- spec/src/current/contracts/local-jj/04-delivery-gates.md
- spec/src/current/contracts/mutations/16-local-browser-composition.md
- spec/src/current/contracts/publication/10-m15-additive-collaboration-publication.md
- spec/src/current/progress/M13/history/M13.3/index.md
- spec/src/current/progress/M13/index.md
- spec/src/current/progress/M24/history/M24/index.md
- spec/src/current/progress/M24/index.md
- web/src/lib/api/generated.ts
- web/src/lib/controller/native-review-context.ts
- web/tests/vite/native-activity.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/local_review_files.rs#portable_file_reviews_replay_unknown_publication_and_retain_original_after_cleanup_and_gc
- crates/jj-store-cli/tests/local_review_files.rs#portable_review_diff_sides_and_whole_files_require_exact_capsules_paths_and_blobs
- crates/jj-review/tests/portable.rs#shared_review_envelopes_keep_native_bytes_and_enumerate_portable_diff_evidence
- crates/jj-store-wal/tests/review_commands.rs#native_review_refuses_portable_code_without_qualified_carrier
- crates/jj-store-service/tests/local_review.rs#local_review_rejects_foreign_and_unqualified_anchors_without_publication
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-CHAT-WORKSPACE-002 details 2](details-02.md)
- [E-CHAT-WORKSPACE-002 details 3](details-03.md)
