---
schema: 1
id: E-UI-068
---

# E-UI-068

<!-- evidence-field:id markdown -->
## Id

E-UI-068
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4.9
- M9.5
- M11.4
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-02
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

focused full-page review navigation, representative loading and aligned message actions
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

Enclosing Jujutsu change pprvmqwv
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-005
- JST-UI-015
- JST-COLLAB-005
- JST-API-005
- JST-WORK-008
- JST-ARCH-004
- JST-GOV-007
- JST-UI-017
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Functional checks pass. Visual baseline approval and existing
  released-registry Design Core qualification remain separate gates.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

UI-only review composition in the existing web package. Community review composition inspected first and Design Core public ScrollArea/Skeleton second. Repository, Projects directory and selected-project review selection replace the list with the same full page, independent of thread Focus/Split preferences. Header and writable comment composer remain fixed around one scrolling content body; comments stay under Activity, draft/reply/edit context survives Activity collapse and returning to the list, and default comments select no implicit parent. Initial connection/list/detail reads use the same representative full-page skeleton. Header/composer wrapper separators are removed to match task detail. Focused direct consumer stories, production navigation unit tests and cold live HMR verification cover this surface. Rust builds/tests, root aggregate spec:check/check/test/build, unrelated Storybook families, built-bundle release qualification, visual baseline updates, main landing, package publication, fixture reset/reseed and backend restart are out of scope under the targeted UI policy. Additional user steering aligns Add reaction with hover/focus message actions for native reviews and captured agent messages; task/channel/reply messages already use that placement. Existing counts remain below content, with target-keyed feedback and native pending/permission gating retained. Resolve/Reopen now use the public secondary button variant with leading check/rotate icons, preserving status commands and disabled behavior; the previous ghost button had no visible background or icon at rest. Final user steering selects the public compact xs size: status-button labels decrease from14px to12px with coordinated icon and spacing, without custom typography overrides.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Node26.9.0; pnpm10.32.1; JJ0.45.1; Svelte5.57.1; Storybook10.5.7; Vitest4.1.11; Playwright1.63.0. Existing private Design Core source hook and tracked registry range^0.2.0 retained.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No API, anchor, operation pin, ReviewStore identity, WAL, manifest, cursor, command, version or permission changes. The existing draft owner and native commands retain explicit reply/edit IDs and unknown requests. Inline diff/tool discussions retain their composer placement and Focus/Split behavior. Review detail skeleton placeholders are hidden and the decorative scroll region is inert; read-only access has no composer placeholder.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Review the committed slice through Vite HMR7460 in the Appearance workspace; visual baselines and released-registry qualification remain separate.
<!-- /evidence-field -->

<!-- evidence-field:artifacts markdown -->
## Artifacts

/tmp/jj-review-page-*.log; /tmp/jj-review-page-final.png; /tmp/jj-review-page-skeleton.png. Main persistent fixture, backend7462 and HMR7460 preserved.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/NativeReviewDetail.stories.svelte
- catalog/components/store-application/RepositoryLoading.stories.svelte
- catalog/components/store-application/StoreCollaboration.stories.svelte
- catalog/components/store-threads/StoreCapturedReactions.stories.svelte
- catalog/components/store-work/ProjectsReviewDetail.stories.svelte
- catalog/fixtures/ProjectsHubFixture.svelte
- catalog/fixtures/StoreCapturedReactionsFixture.svelte
- spec/src/api.md
- spec/src/collaboration.md
- spec/src/milestones.md
- spec/src/projects.md
- spec/src/ui.md
- web/src/lib/ui/store-collaboration/StoreAgentTranscript.svelte
- web/src/lib/ui/store-collaboration/StoreReviewComposer.svelte
- web/src/lib/ui/store-collaboration/StoreReviewConversation.svelte
- web/src/lib/ui/store-collaboration/StoreReviewDetail.svelte
- web/src/lib/ui/store-collaboration/StoreReviewDetailSkeleton.svelte
- web/src/lib/ui/store-collaboration/StoreReviewThreads.svelte
- web/src/lib/ui/store-collaboration/community-native-review.css
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-UI-068 details 2](details-02.md)
- [E-UI-068 details 3](details-03.md)
- [E-UI-068 details 4](details-04.md)
