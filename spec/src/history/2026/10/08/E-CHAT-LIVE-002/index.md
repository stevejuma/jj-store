---
schema: 1
id: E-CHAT-LIVE-002
---

# E-CHAT-LIVE-002

<!-- evidence-field:id markdown -->
## Id

E-CHAT-LIVE-002
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

2026-10-08
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Exact conversation sources and atomic comment submissions
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change uxkolyts
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

ThreadStore could target only whole agent messages; no exact captured part/range or atomic conversation comment batch was available.
<!-- /evidence-field -->

<!-- evidence-field:findings markdown -->
## Findings

Optional exact source selectors preserve old canonical encodings. Bounded Submit projects discussions, version-checked comments, optional ordinary text and a CommentBatch on a private graph, with distinct deterministic event requests. Corrected the preceding just-created contract slice status to pending before further delivery; it contained future live gates and was not a complete qualification. Native and local providers validate every source/reference on pinned selections before existing receipt-backed publication. Conversion retains typed references through its existing explicit maps.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-collab/src/commands.rs
- crates/jj-collab/src/comment_source.rs
- crates/jj-collab/src/conversation_submission.rs
- crates/jj-collab/src/graph.rs
- crates/jj-collab/src/lib.rs
- crates/jj-collab/src/objects.rs
- crates/jj-collab/src/threads.rs
- crates/jj-collab/tests/agent_discussion.rs
- crates/jj-collab/tests/conversation_submission.rs
- crates/jj-collab/tests/target_resolution.rs
- crates/jj-store-service/src/conversion/collab.rs
- crates/jj-store-service/src/local_repository/collab_commands.rs
- crates/jj-store-service/tests/local_collab.rs
- crates/jj-store-wal/src/publication/collab_commands.rs
- crates/jj-store-wal/tests/collab_commands.rs
- schemas/openapi.json
- spec/src/current/contracts/api/41-interactive-agent-execution.md
- spec/src/current/contracts/collaboration/41-conversation-comment-submissions.md
- spec/src/current/progress/M24/index.md
- spec/src/history/2026/10/08/E-CHAT-LIVE-002/SUMMARY.nav.md
- spec/src/history/2026/10/08/E-CHAT-LIVE-002/index.md
- spec/src/history/2026/10/08/SUMMARY.nav.md
- spec/src/history/2026/10/08/index.md
- web/src/lib/api/generated.ts
- web/src/lib/ui/store-threads/thread-context.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-CHAT-LIVE-002 details 2](details-02.md)
- [E-CHAT-LIVE-002 details 3](details-03.md)
