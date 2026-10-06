---
schema: 1
id: E-UI-080
---

# E-UI-080

<!-- evidence-field:id markdown -->
## Id

E-UI-080
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M9.5
- M11.4
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-02
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Agent layout, inline threaded tool comments and session comment footer
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

Enclosing Jujutsu change lkytrwtt
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-002
- JST-UI-005
- JST-UI-006
- JST-UI-015
- JST-UI-018
- JST-WORK-008
- JST-COLLAB-003
- JST-COLLAB-004
- JST-COLLAB-005
- JST-COLLAB-007
- JST-API-006
- JST-PUB-007
- JST-PUB-009
- JST-GOV-002
- JST-GOV-003
- JST-GOV-007
- JST-GOV-008
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Visual baselines remain visual-pending. Filesystem-local agent review/session
  comments remain deferred. Root aggregates, real-RustFS fault qualification,
  production built-CSP qualification, publication, main landing and fixture
  reset are out of scope.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Appearance workspace native/UI slice. Community inspected first, Design Core sheet/sidebar and footer contracts second. Directory rows retain 4px separation; repository, selected-project and hub responses honor Focus sheets or Split body sidebars, with retained selections and opener focus. A mounted controlled sheet clears mobile teleported content. Independently read tool comments and full-row reply composers stay below each exact call, avoiding nested Focus layers. Tool row hover/focus excludes comment descendants. Agent tools/sessions are conversation roots: top comments depth1, replies depth2 with the existing Thread 2.25rem indentation, capped depth and curved connectors. Reply/Edit activates the exact retained review. Session comments use a fixed footer outside the transcript scroll body, an exact native session snapshot anchor and existing ReviewStore publication. Drafts and exact unknown requests survive Close/Settings/layout; all inline composers offer receipt-status recovery. Late reads are target-guarded. The shared composer stops async binding reads after successful submission removes it. Owned loader/reader, reply-tree, fixture mutation and story helpers satisfy module limits without raising architecture budgets. Shared shell/conversation/composer/fixture consumers justify store-work, store-application, store-threads and AppearanceComposer; the additive native anchor also requires focused model/WAL/HTTP/generated-contract lanes. No root aggregate qualification.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/AgentComments.stories.svelte
- catalog/components/store-application/AgentLayout.stories.svelte
- catalog/components/store-application/AgentToolGroups.stories.svelte
- catalog/components/store-application/NativeAgentMenus.stories.svelte
- catalog/components/store-application/agent-comment-replies-play.ts
- catalog/components/store-application/appearance-tool-discussion-play.ts
- catalog/controller/in-memory-review-comments.ts
- catalog/controller/in-memory-store-collaboration.svelte.ts
- catalog/fixtures/AgentLayoutFixture.svelte
- catalog/fixtures/StoreRepositoryApplicationFixture.svelte
- crates/jj-review/src/anchors.rs
- crates/jj-review/src/dependencies.rs
- crates/jj-review/src/validation.rs
- crates/jj-review/tests/anchors.rs
- crates/jj-store-cli/tests/native_lifecycle/http_collaboration_reviews.rs
- crates/jj-store-wal/src/publication/review.rs
- crates/jj-store-wal/tests/review_commands.rs
- docs/ui-provenance.json
- schemas/openapi.json
- spec/src/api.md
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/compatibility.md
- spec/src/milestones.md
- spec/src/projects.md
- spec/src/publication.md
- spec/src/spec-governance.md
- spec/src/ui.md
- web/src/lib/api/generated.ts
- web/src/lib/appearance/drafts.svelte.ts
- web/src/lib/collaboration/presentation.ts
- web/src/lib/collaboration/review-comment-rows.ts
- web/src/lib/collaboration/revision-collaboration.ts
- web/src/lib/controller/agent-review-reader.ts
- web/src/lib/controller/contracts.ts
- web/src/lib/controller/native-agent-context.ts
- web/src/lib/controller/native-review-context.ts
- web/src/lib/controller/production-agent-loaders.ts
- web/src/lib/controller/production-inline-review-actions.ts
- web/src/lib/controller/production-store-actions.ts
- web/src/lib/controller/production-store-collaboration.svelte.ts
- web/src/lib/controller/project-agent-response.svelte.ts
- web/src/lib/controller/review-comment-anchor.ts
- web/src/lib/ui/community-chat-composer/CommunityChatComposer.svelte
- web/src/lib/ui/community-chat-composer/composer-autocomplete-apply.ts
- web/src/lib/ui/store-application/StoreApplicationRightPanels.svelte
- web/src/lib/ui/store-collaboration/StoreAgentComments.svelte
- web/src/lib/ui/store-collaboration/StoreAgentSessionRow.svelte
- web/src/lib/ui/store-collaboration/StoreAgentSessions.svelte
- web/src/lib/ui/store-collaboration/StoreAgentSidebar.svelte
- web/src/lib/ui/store-collaboration/StoreAgentToolDiscussion.svelte
- web/src/lib/ui/store-collaboration/StoreAgentToolReview.svelte
- web/src/lib/ui/store-collaboration/StoreAgentTools.svelte
- web/src/lib/ui/store-collaboration/StoreReviewComposer.svelte
- web/src/lib/ui/store-collaboration/StoreReviewConversation.svelte
- web/src/lib/ui/store-discussion/StoreDiscussionSheet.svelte
- web/src/lib/ui/store-work/StoreProjectsAgentDirectory.svelte
- web/tests/vite/production-store-collaboration.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-review/tests/anchors.rs#session_anchor_retains_only_the_exact_session_snapshot
- crates/jj-store-wal/tests/review_commands.rs#session_comments_retain_the_snapshot_and_reject_identity_mismatch_after_cold_open
- crates/jj-store-cli/tests/native_lifecycle/http_collaboration_reviews.rs#local_trusted_http_pages_root_pinned_review_threads_and_resolves_detail
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Svelte5.57.1;Vitest4.1.11;Storybook10.5.7;pnpm10.32.1;Rust1.98.1;existing governed local Design Core source (InputGroup fix zukowoxl/4922feea);portable dependency ranges unchanged.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive ArtifactTargetV1 agent_session JSON variant with stable session_id and exact session_snapshot reference. Existing V1 anchor bytes/hashes, manifest/WAL/request/receipt formats and publication authority unchanged. Kind/presence/identity checked during staging and cold reads; complete AgentStore dependency closure retained. OpenAPI and TypeScript regenerated. Filesystem-local agent review targets remain unsupported.
<!-- /evidence-field -->

<!-- evidence-field:artifacts markdown -->
## Artifacts

/tmp/jj-agent-layout-*.log; /tmp/jj-agent-comments-*.log; /tmp/jj-agent-session-*.log; /tmp/jj-agent-comment-*.log; /tmp/jj-agent-comments-live.png
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Retain visual-pending; filesystem-local agent review/session comments require a separate governed slice. Main landing and publication remain outside this slice.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-UI-080 details 2](details-02.md)
- [E-UI-080 details 3](details-03.md)
- [E-UI-080 details 4](details-04.md)
- [E-UI-080 details 5](details-05.md)
