---
schema: 1
id: E-CHAT-WORKSPACE-003
---

# E-CHAT-WORKSPACE-003

<!-- evidence-field:id markdown -->
## Id

E-CHAT-WORKSPACE-003
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M24
- M13.3
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-COLLAB-001
- JST-COLLAB-034
- JST-COLLAB-035
- JST-COLLAB-036
- JST-LOCAL-001
- JST-LOCAL-002
- JST-LOCAL-003
- JST-PUB-001
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

- crates/jj-collab/src/comment_reply.rs
- crates/jj-collab/src/conversation_submission.rs
- crates/jj-collab/src/resolver.rs
- crates/jj-collab/src/targets.rs
- crates/jj-collab/tests/target_resolution.rs
- crates/jj-store-cli/tests/local_review_files.rs
- crates/jj-store-server/src/agent_execution/comment_answers.rs
- crates/jj-store-server/src/agent_execution/comment_submissions.rs
- crates/jj-store-server/src/agent_execution/mod.rs
- crates/jj-store-server/src/agent_execution/prompt_context/origin.rs
- crates/jj-store-server/src/agent_execution/prompt_context/tests.rs
- crates/jj-store-server/src/agent_execution/prompt_context.rs
- crates/jj-store-server/src/agent_execution/qualified_reviews.rs
- crates/jj-store-server/src/agent_execution/workspace_inspection/source.rs
- crates/jj-store-service/src/agent_execution/context_submission.rs
- crates/jj-store-service/src/collab_mutations.rs
- crates/jj-store-service/src/collab_reads/agent_feedback.rs
- crates/jj-store-service/src/conversion/collab.rs
- crates/jj-store-service/src/lib.rs
- crates/jj-store-service/src/local_repository/agent_feedback.rs
- crates/jj-store-service/src/local_repository/collab_commands.rs
- crates/jj-store-service/src/qualified_review.rs
- crates/jj-store-wal/src/publication/collab_commands.rs
- crates/jj-store-wal/src/publication/collab_targets.rs
- crates/jj-store-wal/tests/collab_commands.rs
- schemas/openapi.json
- spec/src/current/contracts/api/41-interactive-agent-execution.md
- spec/src/current/contracts/cli/35-interactive-agent-execution.md
- spec/src/current/contracts/collaboration/41-conversation-comment-submissions.md
- spec/src/current/contracts/compatibility/13-interactive-agent-execution.md
- spec/src/current/contracts/mutations/22-interactive-agent-execution.md
- spec/src/current/contracts/publication/10-m15-additive-collaboration-publication.md
- spec/src/current/progress/M13/history/M13.3/index.md
- spec/src/current/progress/M13/index.md
- spec/src/current/progress/M24/history/M24/index.md
- spec/src/current/progress/M24/index.md
- web/src/lib/api/generated.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-collab/tests/target_resolution.rs#qualified_review_requires_explicit_authority_and_keeps_repository_tokens_distinct
- crates/jj-collab/tests/target_resolution.rs#comment_answers_require_delivered_revision_and_exact_controller_capture
- crates/jj-store-wal/tests/collab_commands.rs#qualified_reviews_require_an_explicit_reader_without_foreign_dependencies
- crates/jj-store-cli/tests/local_review_files.rs#portable_file_reviews_replay_unknown_publication_and_retain_original_after_cleanup_and_gc
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-CHAT-WORKSPACE-003 details 2](details-02.md)
- [E-CHAT-WORKSPACE-003 details 3](details-03.md)
