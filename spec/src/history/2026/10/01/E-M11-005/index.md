---
schema: 1
id: E-M11-005
---

# E-M11-005

<!-- evidence-field:id markdown -->
## Id

E-M11-005
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M11
- M11.4
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-01
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

native-workstore-projects-hub
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change nuwmlxxk
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; Rust 1.98.1; jj-cli/jj-lib 0.45.1; Git 2.55.0; Node 26.9.0; pnpm 10.32.1
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Existing native WorkStore/ReviewStore/WAL/manifest bytes unchanged. WorkBrowsePage v1 uses query-bound cursors; discovery RepositorySummary exposes reauthorized current PrincipalId. Optional ReviewThreadSummary.presentation derives creation actor/time, last accepted time and a nullable 240-Unicode-character excerpt from the selected root. Directory projections combine separately authorized code/work/review snapshots and bounded loaded counts; code 50, reviews 50, tasks 256 per home. Native IDs, self-only CSP and 8 MiB individual asset limit retained.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- .storybook/vite.config.ts
- catalog/components/store-application/StoreConnection.stories.svelte
- catalog/components/store-application/StoreRepositoryNavigation.stories.svelte
- catalog/components/store-application/StoreRepositoryPicker.stories.svelte
- catalog/components/store-work/ProjectsDirectory.stories.svelte
- catalog/components/store-work/ProjectsHub.stories.svelte
- catalog/components/store-work/StoreWork.stories.svelte
- catalog/fixtures/ProjectsHubFixture.svelte
- catalog/fixtures/StoreWorkFixture.svelte
- catalog/fixtures/collaboration-review.ts
- catalog/fixtures/projects-hub-fixture.ts
- catalog/fixtures/projects-hub-layout.ts
- crates/jj-store-cli/tests/native_lifecycle/http_collaboration_reviews.rs
- crates/jj-store-cli/tests/native_lifecycle/http_reads.rs
- crates/jj-store-cli/tests/native_lifecycle/http_work.rs
- crates/jj-store-http/src/dto.rs
- crates/jj-store-http/src/repositories.rs
- crates/jj-store-http/src/routes.rs
- crates/jj-store-http/src/work.rs
- crates/jj-store-service/src/collaboration/review_models.rs
- crates/jj-store-service/src/collaboration/review_presentation.rs
- crates/jj-store-service/src/collaboration/review_threads.rs
- crates/jj-store-service/src/collaboration.rs
- crates/jj-store-service/src/work/browse.rs
- crates/jj-store-service/src/work/browse_models.rs
- crates/jj-store-service/src/work/cursors.rs
- crates/jj-store-service/src/work/reads.rs
- crates/jj-store-service/src/work.rs
- crates/jj-store-service/tests/review_threads.rs
- crates/jj-store-service/tests/work_store.rs
- package.json
- schemas/openapi.json
- scripts/web-manifest.mjs
- scripts/web-manifest.test.mjs
- spec/src/api.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/compatibility.md
- spec/src/git-interop.md
- spec/src/milestones.md
- spec/src/mutations.md
- spec/src/projects.md
- spec/src/spec-governance.md
- spec/src/ui.md
- spec/src/verification.md
- web/src/App.svelte
- web/src/lib/api/generated.ts
- web/src/lib/controller/contracts.ts
- web/src/lib/controller/http-repository-directory.ts
- web/src/lib/controller/http-work-data-source.ts
- web/src/lib/controller/production-projects-hub.svelte.ts
- web/src/lib/controller/production-projects-integration.ts
- web/src/lib/controller/production-route-restorer.ts
- web/src/lib/controller/production-store-actions.ts
- web/src/lib/controller/production-store-connect.ts
- web/src/lib/controller/production-store-controller.svelte.ts
- web/src/lib/controller/production-store-lifecycle.svelte.ts
- web/src/lib/controller/production-store-view.ts
- web/src/lib/controller/production-work.svelte.ts
- web/src/lib/controller/projects-home-read.ts
- web/src/lib/controller/projects-hub-types.ts
- web/src/lib/controller/projects-navigation.svelte.ts
- web/src/lib/controller/projects-repositories.svelte.ts
- web/src/lib/controller/projects-repository-types.ts
- web/src/lib/controller/projects-reviews.svelte.ts
- web/src/lib/controller/projects-route-codec.ts
- web/src/lib/controller/projects-view-cache.ts
- web/src/lib/controller/projects-work-registry.svelte.ts
- web/src/lib/controller/store-route-codec.ts
- web/src/lib/controller/store-route-history.svelte.ts
- web/src/lib/controller/work-types.ts
- web/src/lib/ui/community-navigation/CommunityNavigation.css
- web/src/lib/ui/community-navigation/CommunityNavigation.svelte
- web/src/lib/ui/community-navigation/CommunityNavigationTree.css
- web/src/lib/ui/community-navigation/icons.ts
- web/src/lib/ui/community-navigation/model.ts
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryDocuments.svelte
- web/src/lib/ui/store-application/StoreApplication.svelte
- web/src/lib/ui/store-application/StoreConnection.svelte
- web/src/lib/ui/store-application/StoreRepositoryApplication.svelte
- web/src/lib/ui/store-application/StoreRepositoryPicker.svelte
- web/src/lib/ui/store-application/store-navigation.ts
- web/src/lib/ui/store-work/StoreProjects.svelte
- web/src/lib/ui/store-work/StoreProjectsBreadcrumb.svelte
- web/src/lib/ui/store-work/StoreProjectsDirectory.svelte
- web/src/lib/ui/store-work/StoreProjectsFilters.svelte
- web/src/lib/ui/store-work/StoreProjectsRail.svelte
- web/src/lib/ui/store-work/StoreProjectsRepositories.svelte
- web/src/lib/ui/store-work/StoreProjectsReviews.svelte
- web/src/lib/ui/store-work/StoreRepositoryActivity.svelte
- web/src/lib/ui/store-work/StoreRepositoryDirectoryRow.svelte
- web/src/lib/ui/store-work/StoreWork.svelte
- web/src/lib/ui/store-work/StoreWorkCollection.svelte
- web/src/lib/ui/store-work/StoreWorkComments.svelte
- web/src/lib/ui/store-work/StoreWorkDialog.svelte
- web/src/lib/ui/store-work/StoreWorkEmpty.svelte
- web/src/lib/ui/store-work/StoreWorkField.svelte
- web/src/lib/ui/store-work/StoreWorkLayoutControls.svelte
- web/src/lib/ui/store-work/StoreWorkMessageActions.svelte
- web/src/lib/ui/store-work/StoreWorkOwnershipGroups.svelte
- web/src/lib/ui/store-work/StoreWorkPicker.svelte
- web/src/lib/ui/store-work/StoreWorkProjectForm.svelte
- web/src/lib/ui/store-work/StoreWorkProjectRows.svelte
- web/src/lib/ui/store-work/StoreWorkRepositories.svelte
- web/src/lib/ui/store-work/StoreWorkReviewRow.svelte
- web/src/lib/ui/store-work/StoreWorkStateFilter.svelte
- web/src/lib/ui/store-work/StoreWorkStatus.svelte
- web/src/lib/ui/store-work/StoreWorkSubtasks.svelte
- web/src/lib/ui/store-work/StoreWorkTaskDetail.svelte
- web/src/lib/ui/store-work/StoreWorkTaskForm.svelte
- web/src/lib/ui/store-work/StoreWorkTaskRows.svelte
- web/src/lib/ui/store-work/StoreWorkTaskStateIcon.svelte
- web/src/lib/ui/store-work/community-work-breadcrumb.css
- web/src/lib/ui/store-work/community-work-cards.css
- web/src/lib/ui/store-work/community-work-collections.css
- web/src/lib/ui/store-work/community-work-directory.css
- web/src/lib/ui/store-work/community-work-disclosures.css
- web/src/lib/ui/store-work/community-work-forms.css
- web/src/lib/ui/store-work/community-work-layout.css
- web/src/lib/ui/store-work/community-work-reviews.css
- web/src/lib/ui/store-work/community-work-rows.css
- web/src/lib/ui/store-work/community-work-tasks.css
- web/src/lib/ui/store-work/community-work-thread.css
- web/src/lib/ui/store-work/projects-sidebar-model.ts
- web/src/lib/ui/store-work/repository-directory-presentation.ts
- web/src/lib/ui/store-work/store-projects.css
- web/src/lib/ui/store-work/store-work.css
- web/src/lib/ui/store-work/work-drafts.ts
- web/src/lib/ui/store-work/work-presentation.ts
- web/tests/real/work-store.spec.ts
- web/tests/store-route-codec.test.ts
- web/tests/vite/production-store-bootstrap.test.ts
- web/tests/vite/production-work.test.ts
- web/tests/vite/projects-code-navigation.test.ts
- web/tests/vite/projects-directories.test.ts
- web/tests/vite/projects-hub.test.ts
- web/tests/vite/projects-repositories.test.ts
- web/tests/vite/store-route-history.test.ts
- web/tests/work-presentation.test.ts
- web/vite.config.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-service/tests/work_store.rs#work_browse_filters_complete_roots_and_pins_summary_counts
- crates/jj-store-cli/tests/native_lifecycle/http_work.rs#http_workstore_pages_commands_permissions_and_contract_capture
- web/tests/vite/projects-hub.test.ts#hub reads homes independently and keeps
  failed homes visible
- web/tests/vite/projects-hub.test.ts#pending requests and drafts survive hub
  navigation but clear on session reset
- web/tests/vite/projects-hub.test.ts#late browse results cannot replace a newer
  filter
- web/tests/vite/projects-hub.test.ts#navigation restores pinned pages and
  continuation filters until explicit refresh
- web/tests/vite/projects-hub.test.ts#browse rejects a continuation from a
  different snapshot
- web/tests/vite/projects-hub.test.ts#denied browse clears home content and its
  unresolved command
- web/tests/real/work-store.spec.ts#native projects tasks and discussion persist
  on RustFS and refresh explicitly
- scripts/web-manifest.test.mjs#browser manifest retains dynamic chunks and
  fonts while excluding unreferenced workers
- scripts/web-manifest.test.mjs#browser manifest rejects missing dependencies
  and unsafe asset paths
- web/tests/vite/projects-hub.test.ts#refresh becomes busy before discovery and
  remains busy through reads
- web/tests/vite/projects-hub.test.ts#missing project never broadens reviews to
  unrelated repositories
- web/tests/vite/projects-directories.test.ts#sidebar uses independent
  unfiltered snapshots and active repository children
- web/tests/vite/projects-directories.test.ts#sidebar continuation rejects
  changed roots
- web/tests/vite/projects-directories.test.ts#review continuations stay pinned
  and permission removal drops cached repositories
- web/tests/vite/projects-code-navigation.test.ts#completed authorized review
  connection opens the full native thread
- web/tests/vite/projects-code-navigation.test.ts#late code reference reads
  cannot replace Projects restored with Back
- web/tests/vite/store-route-history.test.ts#connecting code from the
  installation Projects hub preserves one Back destination
- web/tests/vite/projects-code-navigation.test.ts#review navigation uses the
  target code operation rather than a home code pin
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M11-005 details 2](details-02.md)
- [E-M11-005 details 3](details-03.md)
- [E-M11-005 details 4](details-04.md)
- [E-M11-005 details 5](details-05.md)
- [E-M11-005 details 6](details-06.md)
- [E-M11-005 details 7](details-07.md)
- [E-M11-005 details 8](details-08.md)
- [E-M11-005 details 9](details-09.md)
- [E-M11-005 details 10](details-10.md)
- [E-M11-005 details 11](details-11.md)
- [E-M11-005 details 12](details-12.md)
- [E-M11-005 details 13](details-13.md)
