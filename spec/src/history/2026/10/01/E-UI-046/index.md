---
schema: 1
id: E-UI-046
---

# E-UI-046

<!-- evidence-field:id markdown -->
## Id

E-UI-046
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M9.5
- M11.4
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-01
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

community-native-message-actions-and-copy-menus
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

Enclosing Jujutsu change soynnsrx.
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Main clean at c22556aa; settings workspace excluded. Exact edit tests fail twice for absent edit action. Mobile/header and Agent activity story files fail twice, with 18 other cases passing. Initial human-name -t filters skip all 20 stories and are not acceptance evidence; a list attempt overlaps a temporary component rename and fails to import. No persistence change or visual-baseline approval.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/NativeAgentMenus.stories.svelte
- catalog/components/store-application/NativeMessageActions.stories.svelte
- catalog/components/store-application/StoreApplication.stories.svelte
- catalog/components/store-application/StoreCollaboration.stories.svelte
- catalog/components/store-work/ProjectsHub.stories.svelte
- catalog/controller/fixture-review-edit.ts
- catalog/controller/in-memory-store-collaboration.svelte.ts
- catalog/fixtures/NativeReviewConversationFixture.svelte
- spec/src/api.md
- spec/src/collaboration.md
- spec/src/milestones.md
- spec/src/ui.md
- spec/src/verification.md
- web/src/lib/controller/contracts.ts
- web/src/lib/controller/production-review-actions.ts
- web/src/lib/controller/production-store-actions.ts
- web/src/lib/controller/production-store-collaboration.svelte.ts
- web/src/lib/controller/production-store-state.svelte.ts
- web/src/lib/controller/production-store-view.ts
- web/src/lib/controller/work-types.ts
- web/src/lib/ui/store-application/StoreApplication.svelte
- web/src/lib/ui/store-collaboration/StoreAgentContext.svelte
- web/src/lib/ui/store-collaboration/StoreAgentSessions.svelte
- web/src/lib/ui/store-collaboration/StoreAgentToolDetail.svelte
- web/src/lib/ui/store-collaboration/StoreAgentToolReview.svelte
- web/src/lib/ui/store-collaboration/StoreAgentTranscript.svelte
- web/src/lib/ui/store-collaboration/StoreMessageActions.svelte
- web/src/lib/ui/store-collaboration/StoreNativeAgentContext.svelte
- web/src/lib/ui/store-collaboration/StoreRecordMenu.svelte
- web/src/lib/ui/store-collaboration/StoreReviewComposer.svelte
- web/src/lib/ui/store-collaboration/StoreReviewConversation.svelte
- web/src/lib/ui/store-collaboration/review-drafts.svelte.ts
- web/src/lib/ui/store-work/StoreProjects.svelte
- web/src/lib/ui/store-work/StoreWork.svelte
- web/src/lib/ui/store-work/StoreWorkComments.svelte
- web/src/lib/ui/store-work/StoreWorkMessageActions.svelte
- web/src/lib/ui/store-work/StoreWorkRemoveComment.svelte
- web/src/lib/ui/store-work/StoreWorkTaskDetail.svelte
- web/tests/vite/production-store-collaboration.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/vite/production-store-collaboration.test.ts#review edits preserve
  the edit-start revision and native body contract
- web/tests/vite/production-store-collaboration.test.ts#unknown review edits
  block replacement submissions and retain their request
- web/tests/vite/production-store-collaboration.test.ts#late review receipts
  cannot restore a disconnected selection
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-UI-046 details 2](details-02.md)
- [E-UI-046 details 3](details-03.md)
