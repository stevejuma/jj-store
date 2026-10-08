---
schema: 1
id: E-CHAT-LIVE-003
---

# E-CHAT-LIVE-003

<!-- evidence-field:id markdown -->
## Id

E-CHAT-LIVE-003
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M24
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-COLLAB-034
- JST-COLLAB-035
- JST-COLLAB-036
- JST-API-001
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-09
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Frozen comment prompt context and exact durable agent answer links
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change kytwppxy
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Exact ThreadStore submissions existed, but prompts did not publish frozen batches or retain typed reply references; answers had no publication-validated destinations.
<!-- /evidence-field -->

<!-- evidence-field:findings markdown -->
## Findings

Added version-1 trusted reference instructions, bounded Markdown-aware parsing and exact answer-link validation shared by both publication providers. Managed and standalone prompts support comment-only batches and separate comment publication status. Sanitization preserves typed identifiers, freezes only authored/captured strings and retains the Sent wire context. Explicit batches merge eligible undelivered context. Answer requests retain immutable captures and exact revision identities through unknown outcomes; definitive stale recovery requires receipt evidence. Ordinary reply batches preserve their parent. Feedback exposes captured text parts and selected links. UI integration is a separate unfinished slice.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- Cargo.lock
- crates/jj-collab/Cargo.toml
- crates/jj-collab/src/comment_reply.rs
- crates/jj-collab/src/comment_source.rs
- crates/jj-collab/src/conversation_submission.rs
- crates/jj-collab/src/lib.rs
- crates/jj-collab/src/objects.rs
- crates/jj-collab/src/threads.rs
- crates/jj-collab/tests/comment_reply.rs
- crates/jj-collab/tests/conversation_submission.rs
- crates/jj-collab/tests/target_resolution.rs
- crates/jj-store-cli/tests/agent_execution/comment_answers.rs
- crates/jj-store-cli/tests/agent_execution/context_delivery.rs
- crates/jj-store-cli/tests/agent_execution/conversations.rs
- crates/jj-store-cli/tests/agent_execution/real.rs
- crates/jj-store-cli/tests/agent_execution.rs
- crates/jj-store-server/Cargo.toml
- crates/jj-store-server/src/agent_execution/commands.rs
- crates/jj-store-server/src/agent_execution/comment_answers.rs
- crates/jj-store-server/src/agent_execution/comment_submissions.rs
- crates/jj-store-server/src/agent_execution/configuration.rs
- crates/jj-store-server/src/agent_execution/conversation_origin.rs
- crates/jj-store-server/src/agent_execution/conversation_prompts.rs
- crates/jj-store-server/src/agent_execution/conversations.rs
- crates/jj-store-server/src/agent_execution/delivery.rs
- crates/jj-store-server/src/agent_execution/journal.rs
- crates/jj-store-server/src/agent_execution/mod.rs
- crates/jj-store-server/src/agent_execution/prompt_context/origin.rs
- crates/jj-store-server/src/agent_execution/prompt_context/tests.rs
- crates/jj-store-server/src/agent_execution/prompt_context.rs
- crates/jj-store-server/src/agent_execution/publication.rs
- crates/jj-store-service/src/agent_execution/context.rs
- crates/jj-store-service/src/agent_execution/context_submission.rs
- crates/jj-store-service/src/agent_execution/conversations.rs
- crates/jj-store-service/src/agent_execution.rs
- crates/jj-store-service/src/agent_feedback/source_parts.rs
- crates/jj-store-service/src/agent_feedback/tests.rs
- crates/jj-store-service/src/agent_feedback.rs
- crates/jj-store-service/src/collab_reads/agent_feedback.rs
- crates/jj-store-service/src/conversion/collab.rs
- crates/jj-store-service/src/local_repository/agent_feedback.rs
- crates/jj-store-service/src/local_repository/collab_commands.rs
- crates/jj-store-service/tests/local_collab.rs
- crates/jj-store-wal/src/publication/collab_commands.rs
- crates/jj-store-wal/tests/collab_commands.rs
- schemas/openapi.json
- spec/src/current/contracts/api/41-interactive-agent-execution.md
- spec/src/current/contracts/architecture/20-interactive-agent-execution.md
- spec/src/current/contracts/collaboration/41-conversation-comment-submissions.md
- spec/src/current/progress/M24/index.md
- spec/src/history/2026/10/09/E-CHAT-LIVE-003/SUMMARY.nav.md
- spec/src/history/2026/10/09/E-CHAT-LIVE-003/details-02.md
- spec/src/history/2026/10/09/E-CHAT-LIVE-003/index.md
- spec/src/history/2026/10/09/SUMMARY.nav.md
- spec/src/history/2026/10/09/index.md
- spec/src/history/2026/10/SUMMARY.nav.md
- spec/src/history/2026/10/index.md
- web/src/lib/api/generated.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-CHAT-LIVE-003 details 2](details-02.md)
