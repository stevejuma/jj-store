---
schema: 1
id: E-M9-036
---

# E-M9-036

<!-- evidence-field:id markdown -->
## Id

E-M9-036
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M9
- M9.5
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-29
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

collaboration-layout-refinement
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- .storybook/main.ts
- catalog/components/store-application/RepositoryReview.stories.svelte
- catalog/components/store-application/StoreCollaboration.stories.svelte
- catalog/components/store-application/StoreRepositoryApplication.stories.svelte
- catalog/controller/in-memory-store-collaboration.svelte.ts
- catalog/controller/in-memory-store-controller.svelte.ts
- catalog/controller/store-controller.test.ts
- catalog/fixtures/collaboration-review.ts
- catalog/fixtures/collaboration.ts
- package.json
- pnpm-lock.yaml
- scripts/catalog-browser.test.mjs
- spec/src/api.md
- spec/src/architecture.md
- spec/src/collaboration.md
- spec/src/milestones.md
- spec/src/spec-governance.md
- spec/src/ui.md
- web/public/agent-icons/SOURCES.md
- web/public/agent-icons/codex.svg
- web/public/agent-icons/cursor.svg
- web/public/agent-icons/opencode.svg
- web/src/lib/collaboration/agent-avatar.ts
- web/src/lib/collaboration/collaboration-canvas.ts
- web/src/lib/collaboration/presentation.ts
- web/src/lib/collaboration/revision-collaboration.ts
- web/src/lib/community/avatar-content.ts
- web/src/lib/controller/contracts.ts
- web/src/lib/controller/http-collaboration-data-source.ts
- web/src/lib/controller/http-store-data-source.ts
- web/src/lib/controller/native-data-source.ts
- web/src/lib/controller/production-collaboration-context.ts
- web/src/lib/controller/production-review-actions.ts
- web/src/lib/controller/production-store-actions.ts
- web/src/lib/controller/production-store-collaboration.svelte.ts
- web/src/lib/controller/production-store-lifecycle.svelte.ts
- web/src/lib/controller/production-store-state.svelte.ts
- web/src/lib/controller/production-store-view.ts
- web/src/lib/controller/review-anchor-patch.ts
- web/src/lib/projects/repository-log-author.ts
- web/src/lib/ui/community-member-avatar-group/CommunityMemberAvatarGroup.svelte
- web/src/lib/ui/community-projects/CommunityProjectsGitAuthorAvatar.svelte
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryDirectory.svelte
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryLog.svelte
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryLogDiff.svelte
- web/src/lib/ui/community-projects/CommunityProjectsRevisionCollaboration.svelte
- web/src/lib/ui/community-projects/CommunityProjectsRevisionGraph.svelte
- web/src/lib/ui/community-projects/CommunityProjectsRevisionGraphCopy.svelte
- web/src/lib/ui/store-application/StoreRepositoryApplication.svelte
- web/src/lib/ui/store-application/StoreRepositoryDetails.svelte
- web/src/lib/ui/store-collaboration/StoreAgentContext.svelte
- web/src/lib/ui/store-collaboration/StoreAgentSessions.svelte
- web/src/lib/ui/store-collaboration/StoreAgentTranscript.svelte
- web/src/lib/ui/store-collaboration/StoreReviewConversation.svelte
- web/src/lib/ui/store-collaboration/StoreReviewDiscussion.svelte
- web/src/lib/ui/store-collaboration/StoreReviewThreads.svelte
- web/src/lib/ui/store-collaboration/store-collaboration.css
- web/src/lib/ui/store-mutations/StoreRepositoryActions.svelte
- web/tests/collaboration-presentation.test.ts
- web/tests/repository-log-author.test.ts
- web/tests/vite/production-store-collaboration.test.ts
- web/tests/vite/production-store-data-source.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- catalog/controller/store-controller.test.ts#operation selection and file
  drill-down retain exact collaboration
- web/tests/collaboration-presentation.test.ts#supported agents use bundled
  artwork and agent avatar shape metadata
- web/tests/collaboration-presentation.test.ts#review replies follow Community
  branch order and bounded indentation
- web/tests/collaboration-presentation.test.ts#operation collaboration includes
  explicit code comparison context
- web/tests/repository-log-author.test.ts#Git author email produces
  Community-compatible Gravatar URLs
- web/tests/vite/production-store-collaboration.test.ts#operation context
  follows associations and every exact operation anchor
- web/tests/vite/production-store-collaboration.test.ts#operation path context
  keeps the exact anchored review selected
- web/tests/vite/production-store-collaboration.test.ts#code text review loads
  its exact pinned comparison pair
- web/tests/vite/production-store-collaboration.test.ts#a review patch failure
  preserves the loaded conversation
- web/tests/vite/production-store-collaboration.test.ts#a stale review patch
  failure cannot clear newer state
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M9-036 details 2](details-02.md)
- [E-M9-036 details 3](details-03.md)
