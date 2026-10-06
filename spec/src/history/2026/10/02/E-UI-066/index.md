---
schema: 1
id: E-UI-066
---

# E-UI-066

<!-- evidence-field:id markdown -->
## Id

E-UI-066
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4.9
- M11.4
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-02
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

focused initial loading skeleton and project avatar regression
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

Enclosing Jujutsu change upskknvy
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-003
- JST-UI-005
- JST-UI-015
- JST-API-005
- JST-WORK-008
- JST-ARCH-004
- JST-GOV-007
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

UI-only Projects and repository initial-load presentation within the existing web package. Inspect Community first and Design Core second; retain public Skeleton primitive and owned Community copies. Focused interaction/accessibility stories cover early bootstrap/discovery, all five Projects tabs, repository Overview/Branches/JJ Log/Op Log/Agents/Reviews, selected task/review details, retained refresh lists/drafts, varied counts/dates and responsive avatar columns. Repository Projects uses the same initial gate. Production async reads, API contracts, snapshots and mutation policy are unchanged. Rust builds/tests, root aggregate spec:check/check/test/build, unrelated full Storybook suite, built-bundle/CSP release qualification, visual baseline updates, main landing, package publication, fixture reset/reseed and backend restart are out of scope under the targeted UI policy.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Node26.9.0; pnpm10.32.1; JJ0.45.1; Svelte5.57.1; Storybook10.5.7; Vitest4.1.11; Playwright1.63.0. Existing private Design Core source hook and tracked registry range^0.2.0 retained.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No API, operation pin, WorkStore, native identity, WAL, manifest, cursor or permission changes. Skeletons include initial connection/discovery; background reads retain accepted content. Avatars reserve a metadata column independent of counts and relative-date width.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Review the committed slice through existing Vite HMR7460 in the Appearance workspace; visual baselines and registry qualification remain separate.
<!-- /evidence-field -->

<!-- evidence-field:artifacts markdown -->
## Artifacts

/tmp/jj-loading-*.log; /tmp/jj-project-avatars-final.png. Main persistent fixture and HMR/backend preserved.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/RepositoryLoading.stories.svelte
- catalog/components/store-work/ProjectsActivity.stories.svelte
- catalog/components/store-work/ProjectsDirectory.stories.svelte
- catalog/controller/loading-store-controller.ts
- catalog/fixtures/ProjectsHubFixture.svelte
- catalog/fixtures/StoreRepositoryApplicationFixture.svelte
- spec/src/api.md
- spec/src/projects.md
- spec/src/ui.md
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryFiles.svelte
- web/src/lib/ui/store-application/StoreRepositoryApplication.svelte
- web/src/lib/ui/store-collaboration/StoreAgentSessions.svelte
- web/src/lib/ui/store-collaboration/StoreCollaborationSkeleton.svelte
- web/src/lib/ui/store-collaboration/StoreReviewThreads.svelte
- web/src/lib/ui/store-work/StoreProjects.svelte
- web/src/lib/ui/store-work/StoreProjectsDirectory.svelte
- web/src/lib/ui/store-work/StoreProjectsLoading.svelte
- web/src/lib/ui/store-work/StoreRepositoryProjects.svelte
- web/src/lib/ui/store-work/StoreWorkProjectRows.svelte
- web/src/lib/ui/store-work/community-work-rows.css
- web/src/lib/ui/store-work/projects-initial-loading.ts
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-UI-066 details 2](details-02.md)
- [E-UI-066 details 3](details-03.md)
