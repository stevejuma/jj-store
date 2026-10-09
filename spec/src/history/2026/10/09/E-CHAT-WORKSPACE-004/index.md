---
schema: 1
id: E-CHAT-WORKSPACE-004
---

# E-CHAT-WORKSPACE-004

<!-- evidence-field:id markdown -->
## Id

E-CHAT-WORKSPACE-004
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M24
- M13.3
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-JJ-003
- JST-JJ-004
- JST-UI-001
- JST-UI-015
- JST-UI-018
- JST-COLLAB-034
- JST-COLLAB-035
- JST-COLLAB-036
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

- catalog/components/store-application/WorkspaceFiles.stories.svelte
- catalog/fixtures/WorkspaceFilesFixture.svelte
- crates/jj-collab/src/conversation_submission.rs
- crates/jj-collab/src/lib.rs
- crates/jj-collab/src/workspace_comments.rs
- crates/jj-collab/tests/conversation_submission.rs
- crates/jj-store-cli/tests/agent_execution/comment_answers.rs
- crates/jj-store-cli/tests/agent_execution/workspace_comments.rs
- crates/jj-store-cli/tests/agent_execution.rs
- crates/jj-store-cli/tests/local_review_files.rs
- crates/jj-store-jj/src/agent_workspaces/inspection.rs
- crates/jj-store-server/src/agent_execution/commands.rs
- crates/jj-store-server/src/agent_execution/comment_submissions.rs
- crates/jj-store-server/src/agent_execution/conversations.rs
- crates/jj-store-server/src/agent_execution/delivery.rs
- crates/jj-store-server/src/agent_execution/fleet.rs
- crates/jj-store-server/src/agent_execution/journal.rs
- crates/jj-store-server/src/agent_execution/mod.rs
- crates/jj-store-server/src/agent_execution/qualified_reviews.rs
- crates/jj-store-server/src/agent_execution/runtime.rs
- crates/jj-store-server/src/agent_execution/workspace_deferred.rs
- crates/jj-store-server/src/agent_execution/workspace_freeze.rs
- crates/jj-store-server/src/agent_execution/workspace_inspection.rs
- crates/jj-store-server/src/agent_execution/workspace_publication.rs
- crates/jj-store-server/src/agent_execution/workspace_sources.rs
- crates/jj-store-server/src/agent_execution/workspace_turns.rs
- crates/jj-store-service/src/qualified_review.rs
- crates/jj-store-service/tests/local_collab.rs
- crates/jj-store-wal/tests/collab_commands.rs
- schemas/openapi.json
- spec/src/current/contracts/api/41-interactive-agent-execution.md
- spec/src/current/contracts/cli/35-interactive-agent-execution.md
- spec/src/current/contracts/collaboration/41-conversation-comment-submissions.md
- spec/src/current/contracts/compatibility/13-interactive-agent-execution.md
- spec/src/current/contracts/jj-integration/32-interactive-agent-workspaces.md
- spec/src/current/contracts/local-jj/04-delivery-gates.md
- spec/src/current/contracts/mutations/22-interactive-agent-execution.md
- spec/src/current/contracts/publication/10-m15-additive-collaboration-publication.md
- spec/src/current/contracts/ui/37-document-conversations.md
- spec/src/current/progress/M13/history/M13.3/index.md
- spec/src/current/progress/M13/index.md
- spec/src/current/progress/M24/history/M24/index.md
- spec/src/current/progress/M24/index.md
- web/src/lib/api/generated.ts
- web/src/lib/conversations/inspection.svelte.ts
- web/src/lib/conversations/session.svelte.ts
- web/src/lib/conversations/submission-result.ts
- web/src/lib/conversations/workspace-browser.svelte.ts
- web/src/lib/conversations/workspace-types.ts
- web/src/lib/ui/community-app-shell/right-sidebar.css
- web/src/lib/ui/store-agent-chat/StoreAgentChatSidebar.svelte
- web/src/lib/ui/store-agent-chat-body/ChatMessageFooter.svelte
- web/src/lib/ui/store-agent-chat-body/comments.svelte.ts
- web/src/lib/ui/store-agent-chat-body/model.ts
- web/src/lib/ui/store-agent-execution/StoreAgentConversation.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentDetailsSidebar.svelte
- web/src/lib/ui/store-agent-workspace/StoreWorkspaceButton.svelte
- web/src/lib/ui/store-agent-workspace/StoreWorkspaceFiles.svelte
- web/src/lib/ui/store-agent-workspace/StoreWorkspaceInspector.svelte
- web/src/lib/ui/store-application/StoreApplication.svelte
- web/src/lib/ui/store-application/StoreApplicationRightPanels.svelte
- web/tests/vite/workspace-browser.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/agent_execution/workspace_comments.rs#workspace_comments_freeze_group_publish_and_recover_without_duplicate_reviews
- crates/jj-collab/src/workspace_comments.rs#draft_paths_revisions_and_diff_sides_are_explicit
- crates/jj-store-server/src/agent_execution/workspace_sources.rs#captured_bytes_require_exact_unicode_boundaries_and_full_source_hash
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-CHAT-WORKSPACE-004 details 2](details-02.md)
- [E-CHAT-WORKSPACE-004 details 3](details-03.md)
