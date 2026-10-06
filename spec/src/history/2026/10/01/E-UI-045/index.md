---
schema: 1
id: E-UI-045
---

# E-UI-045

<!-- evidence-field:id markdown -->
## Id

E-UI-045
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

community-native-review-and-shared-project-activity
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

Enclosing Jujutsu change nqpoyvww.
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Main clean at 541edf40. Native Reviews used the two-column discussion canvas, Projects had four tabs and no shared commit/agent feed, task triggers had 0.75rem leading padding and Subtasks/Activity opened by default. Initial display-name test filters skipped all tests and are not regression evidence. Exact new review story then fails once for absent Review details; ProjectsHub has two failures for the missing Activity tab and newly required collapsed defaults. No milestone or visual-baseline status changes. Final live visual inspection also finds absent line counts because the path-only native response intentionally omits diff content. Read counts from the exact pinned file diff and change the regression path fixture to not_requested so path-index zeroes cannot satisfy it. Subsequent Activity regression proves ordinary hash links cleared app Back history (one failing test); append encountered destinations while preserving known route restoration. Intermediate fixtures assumed a noncanonical state=active hash and eager context reads; align expectations with route serialization and deferred disclosure reads. User first requested distinct page/column review modes, then superseded that scope with a full-width Community review list and no sidebar view buttons. The final list story caught a 17.6px shrinking ScrollArea host; declare the consumer flex width and import the existing Community row styles. Do not change the shared component or weaken the width assertion.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/NativeReviewDetail.stories.svelte
- catalog/components/store-application/StoreCollaboration.stories.svelte
- catalog/components/store-work/NativeActivity.stories.svelte
- catalog/components/store-work/ProjectsActivity.stories.svelte
- catalog/components/store-work/ProjectsHub.stories.svelte
- catalog/components/store-work/StoreWork.stories.svelte
- catalog/controller/in-memory-store-collaboration.svelte.ts
- catalog/fixtures/NativeActivityFixture.svelte
- catalog/fixtures/ProjectsHubFixture.svelte
- catalog/fixtures/native-activity.ts
- catalog/fixtures/projects-hub-fixture.ts
- catalog/fixtures/projects-hub-layout.ts
- spec/src/api.md
- spec/src/collaboration.md
- spec/src/milestones.md
- spec/src/ui.md
- spec/src/verification.md
- web/src/lib/controller/contracts.ts
- web/src/lib/controller/http-store-data-source.ts
- web/src/lib/controller/http-work-data-source.ts
- web/src/lib/controller/native-activity-reader.ts
- web/src/lib/controller/native-activity-types.ts
- web/src/lib/controller/native-agent-context.ts
- web/src/lib/controller/native-review-context.ts
- web/src/lib/controller/production-projects-hub.svelte.ts
- web/src/lib/controller/production-store-collaboration.svelte.ts
- web/src/lib/controller/production-store-lifecycle.svelte.ts
- web/src/lib/controller/production-store-state.svelte.ts
- web/src/lib/controller/production-store-view.ts
- web/src/lib/controller/projects-activity-scope.ts
- web/src/lib/controller/projects-activity.svelte.ts
- web/src/lib/controller/projects-hub-types.ts
- web/src/lib/controller/projects-route-actions.ts
- web/src/lib/controller/projects-route-codec.ts
- web/src/lib/controller/store-route-codec.ts
- web/src/lib/controller/store-route-history.svelte.ts
- web/src/lib/ui/store-application/StoreApplication.svelte
- web/src/lib/ui/store-application/StoreWorkspaceSectionHeader.svelte
- web/src/lib/ui/store-collaboration/StoreAgentToolReview.svelte
- web/src/lib/ui/store-collaboration/StoreAgentTranscript.svelte
- web/src/lib/ui/store-collaboration/StoreNativeAgentContext.svelte
- web/src/lib/ui/store-collaboration/StoreRepositoryActivity.svelte
- web/src/lib/ui/store-collaboration/StoreReviewCommits.svelte
- web/src/lib/ui/store-collaboration/StoreReviewDetail.svelte
- web/src/lib/ui/store-collaboration/StoreReviewFiles.svelte
- web/src/lib/ui/store-collaboration/StoreReviewThreads.svelte
- web/src/lib/ui/store-collaboration/community-native-activity.css
- web/src/lib/ui/store-collaboration/community-native-diff.css
- web/src/lib/ui/store-collaboration/community-native-review.css
- web/src/lib/ui/store-work/StoreProjects.svelte
- web/src/lib/ui/store-work/StoreProjectsActivity.svelte
- web/src/lib/ui/store-work/StoreProjectsDirectory.svelte
- web/src/lib/ui/store-work/StoreProjectsRail.svelte
- web/src/lib/ui/store-work/StoreProjectsRepositories.svelte
- web/src/lib/ui/store-work/StoreProjectsReviews.svelte
- web/src/lib/ui/store-work/StoreWork.svelte
- web/src/lib/ui/store-work/StoreWorkComments.svelte
- web/src/lib/ui/store-work/StoreWorkEmpty.svelte
- web/src/lib/ui/store-work/StoreWorkReviewRow.svelte
- web/src/lib/ui/store-work/StoreWorkSubtasks.svelte
- web/src/lib/ui/store-work/StoreWorkTaskDetail.svelte
- web/src/lib/ui/store-work/community-work-disclosures.css
- web/src/lib/ui/store-work/store-projects.css
- web/tests/real/projects-activity.spec.ts
- web/tests/real/work-seed.spec.ts
- web/tests/real/work-store.spec.ts
- web/tests/vite/native-activity.test.ts
- web/tests/vite/projects-code-navigation.test.ts
- web/tests/vite/store-route-history.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/vite/native-activity.test.ts#revision review files and commit
  activity retain the exact anchor comparison
- web/tests/vite/native-activity.test.ts#anchored agent activity never
  substitutes a newer session snapshot
- web/tests/vite/native-activity.test.ts#activity continuation rejects another
  snapshot and preserves displayed rows
- web/tests/vite/native-activity.test.ts#an explicit home operation replaces a
  cached activity capture
- web/tests/vite/projects-code-navigation.test.ts#native activity links
  authorize a code connection when Projects has no active source
- web/tests/real/projects-activity.spec.ts#native project task and review pages
  share pinned commit activity
- web/tests/vite/store-route-history.test.ts#ordinary activity commit links
  retain the exact Projects Back and Forward destinations
- web/tests/vite/store-route-history.test.ts#retired review column links restore
  the Community list or selected detail
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-UI-045 details 2](details-02.md)
- [E-UI-045 details 3](details-03.md)
- [E-UI-045 details 4](details-04.md)
- [E-UI-045 details 5](details-05.md)
