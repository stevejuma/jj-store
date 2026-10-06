---
schema: 1
id: E-UI-042
---

# E-UI-042

<!-- evidence-field:id markdown -->
## Id

E-UI-042
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4.29
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-01
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

served-editor-and-composer-focus-treatment
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

Enclosing Jujutsu change uonqxxvs.
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Clean working change after E-UI-041. Live focused editable .cm-content has native auto 1px outline while the outer .cm-editor already has no outline/shadow. Initial 9 strengthened stories pass because Storybook admits injected CodeMirror styles. Exact cold browser regression with production asset CSP fails both editor and prose composer cases: expected outline none, received auto.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/StoreCollaboration.stories.svelte
- catalog/components/store-application/StoreFileEditor.stories.svelte
- catalog/components/store-application/collaboration-story-assertions.ts
- spec/src/api.md
- spec/src/milestones.md
- spec/src/ui.md
- spec/src/verification.md
- web/src/lib/ui/community-projects/StoreFileEditor.svelte
- web/src/lib/ui/editor-focus.css
- web/src/lib/ui/store-collaboration/StoreAgentToolReview.svelte
- web/src/lib/ui/store-collaboration/StoreCommentableCommitDiff.svelte
- web/src/lib/ui/store-collaboration/StoreReviewConversation.svelte
- web/src/lib/ui/store-mutations/StoreMutationFields.svelte
- web/tests/browser/editor-focus.spec.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-UI-042 details 2](details-02.md)
- [E-UI-042 details 3](details-03.md)
