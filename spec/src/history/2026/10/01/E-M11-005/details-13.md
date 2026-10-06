---
schema: 1
id: E-M11-005
---

# E-M11-005 details 13

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_REAL_BASE_URL=http://127.0.0.1:7468 JJ_STORE_REAL_REPOSITORY=5ae8240e6d77fe8d0f6dd0b13f3d3874 JJ_STORE_REAL_RUNTIME=/tmp/jsh-cold-ijPU4c JJ_STORE_REAL_ROOT=/Users/stevejuma/code/lapismd/jj-store-projects-hub pnpm exec playwright test --config web/playwright.real.config.ts work-store.spec.ts
```

**Notes**

jsh-directory-native2.log: Cold native 0/1: new assertion scopes its has locator from the surrounding region, so it cannot match inside an article. Actual snapshot includes review metadata; use a relative role locator within the article.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --write spec/src/milestones.md
```

**Notes**

jsh-directory-format11.log: changed-path formatting applied.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --write web/tests/real/work-store.spec.ts
```

**Notes**

jsh-directory-format12.log: changed-path formatting applied.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:validate
```

**Notes**

jsh-directory-spec2.log: Specification validated: 4 validators, 20 chapters, 125 requirements.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first
```

**Notes**

jsh-directory-first2.log: Focused command completed.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:boundaries
```

**Notes**

jsh-directory-boundaries2.log: package boundaries ok (15 packages)
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --write spec/src/milestones.md
```

**Notes**

jsh-directory-format13.log: changed-path formatting applied.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_DEV_ROOT=/tmp/jsh-cold-ijPU4c pnpm dev:stop
```

**Notes**

jsh-directory-stop1.log: Focused command completed.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --write spec/src/milestones.md
```

**Notes**

jsh-directory-format14.log: changed-path formatting applied.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --check .storybook/vite.config.ts catalog/components/store-application/StoreConnection.stories.svelte catalog/components/store-application/StoreRepositoryNavigation.stories.svelte catalog/components/store-application/StoreRepositoryPicker.stories.svelte catalog/components/store-work/ProjectsDirectory.stories.svelte catalog/components/store-work/ProjectsHub.stories.svelte catalog/components/store-work/StoreWork.stories.svelte catalog/fixtures/ProjectsHubFixture.svelte catalog/fixtures/StoreWorkFixture.svelte catalog/fixtures/collaboration-review.ts catalog/fixtures/projects-hub-fixture.ts catalog/fixtures/projects-hub-layout.ts package.json schemas/openapi.json scripts/web-manifest.mjs scripts/web-manifest.test.mjs spec/src/api.md spec/src/architecture.md spec/src/cli.md spec/src/collaboration.md spec/src/compatibility.md spec/src/git-interop.md spec/src/milestones.md spec/src/mutations.md spec/src/projects.md spec/src/spec-governance.md spec/src/ui.md spec/src/verification.md web/src/App.svelte web/src/lib/api/generated.ts web/src/lib/controller/contracts.ts web/src/lib/controller/http-repository-directory.ts web/src/lib/controller/http-work-data-source.ts web/src/lib/controller/production-projects-hub.svelte.ts web/src/lib/controller/production-projects-integration.ts web/src/lib/controller/production-route-restorer.ts web/src/lib/controller/production-store-actions.ts web/src/lib/controller/production-store-connect.ts web/src/lib/controller/production-store-controller.svelte.ts web/src/lib/controller/production-store-lifecycle.svelte.ts web/src/lib/controller/production-store-view.ts web/src/lib/controller/production-work.svelte.ts web/src/lib/controller/projects-home-read.ts web/src/lib/controller/projects-hub-types.ts web/src/lib/controller/projects-navigation.svelte.ts web/src/lib/controller/projects-repositories.svelte.ts web/src/lib/controller/projects-repository-types.ts web/src/lib/controller/projects-reviews.svelte.ts web/src/lib/controller/projects-route-codec.ts web/src/lib/controller/projects-view-cache.ts web/src/lib/controller/projects-work-registry.svelte.ts web/src/lib/controller/store-route-codec.ts web/src/lib/controller/store-route-history.svelte.ts web/src/lib/controller/work-types.ts web/src/lib/ui/community-navigation/CommunityNavigation.css web/src/lib/ui/community-navigation/CommunityNavigation.svelte web/src/lib/ui/community-navigation/CommunityNavigationTree.css web/src/lib/ui/community-navigation/icons.ts web/src/lib/ui/community-navigation/model.ts web/src/lib/ui/community-projects/CommunityProjectsRepositoryDocuments.svelte web/src/lib/ui/store-application/StoreApplication.svelte web/src/lib/ui/store-application/StoreConnection.svelte web/src/lib/ui/store-application/StoreRepositoryApplication.svelte web/src/lib/ui/store-application/StoreRepositoryPicker.svelte web/src/lib/ui/store-application/store-navigation.ts web/src/lib/ui/store-work/StoreProjects.svelte web/src/lib/ui/store-work/StoreProjectsBreadcrumb.svelte web/src/lib/ui/store-work/StoreProjectsDirectory.svelte web/src/lib/ui/store-work/StoreProjectsFilters.svelte web/src/lib/ui/store-work/StoreProjectsRail.svelte web/src/lib/ui/store-work/StoreProjectsRepositories.svelte web/src/lib/ui/store-work/StoreProjectsReviews.svelte web/src/lib/ui/store-work/StoreRepositoryActivity.svelte web/src/lib/ui/store-work/StoreRepositoryDirectoryRow.svelte web/src/lib/ui/store-work/StoreWork.svelte web/src/lib/ui/store-work/StoreWorkCollection.svelte web/src/lib/ui/store-work/StoreWorkComments.svelte web/src/lib/ui/store-work/StoreWorkDialog.svelte web/src/lib/ui/store-work/StoreWorkEmpty.svelte web/src/lib/ui/store-work/StoreWorkField.svelte web/src/lib/ui/store-work/StoreWorkLayoutControls.svelte web/src/lib/ui/store-work/StoreWorkMessageActions.svelte web/src/lib/ui/store-work/StoreWorkOwnershipGroups.svelte web/src/lib/ui/store-work/StoreWorkPicker.svelte web/src/lib/ui/store-work/StoreWorkProjectForm.svelte web/src/lib/ui/store-work/StoreWorkProjectRows.svelte web/src/lib/ui/store-work/StoreWorkRepositories.svelte web/src/lib/ui/store-work/StoreWorkReviewRow.svelte web/src/lib/ui/store-work/StoreWorkStateFilter.svelte web/src/lib/ui/store-work/StoreWorkStatus.svelte web/src/lib/ui/store-work/StoreWorkSubtasks.svelte web/src/lib/ui/store-work/StoreWorkTaskDetail.svelte web/src/lib/ui/store-work/StoreWorkTaskForm.svelte web/src/lib/ui/store-work/StoreWorkTaskRows.svelte web/src/lib/ui/store-work/StoreWorkTaskStateIcon.svelte web/src/lib/ui/store-work/community-work-breadcrumb.css web/src/lib/ui/store-work/community-work-cards.css web/src/lib/ui/store-work/community-work-collections.css web/src/lib/ui/store-work/community-work-directory.css web/src/lib/ui/store-work/community-work-disclosures.css web/src/lib/ui/store-work/community-work-forms.css web/src/lib/ui/store-work/community-work-layout.css web/src/lib/ui/store-work/community-work-reviews.css web/src/lib/ui/store-work/community-work-rows.css web/src/lib/ui/store-work/community-work-tasks.css web/src/lib/ui/store-work/community-work-thread.css web/src/lib/ui/store-work/projects-sidebar-model.ts web/src/lib/ui/store-work/repository-directory-presentation.ts web/src/lib/ui/store-work/store-projects.css web/src/lib/ui/store-work/store-work.css web/src/lib/ui/store-work/work-drafts.ts web/src/lib/ui/store-work/work-presentation.ts web/tests/real/work-store.spec.ts web/tests/store-route-codec.test.ts web/tests/vite/production-store-bootstrap.test.ts web/tests/vite/production-work.test.ts web/tests/vite/projects-code-navigation.test.ts web/tests/vite/projects-directories.test.ts web/tests/vite/projects-hub.test.ts web/tests/vite/projects-repositories.test.ts web/tests/vite/store-route-history.test.ts web/tests/work-presentation.test.ts web/vite.config.ts
```

**Notes**

jsh-directory-formatcheck2.log: all supported changed paths pass formatting.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --write spec/src/milestones.md
```

**Notes**

jsh-directory-format15.log: changed-path formatting applied.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_REAL_BASE_URL=http://127.0.0.1:7468 JJ_STORE_REAL_REPOSITORY=5ae8240e6d77fe8d0f6dd0b13f3d3874 JJ_STORE_REAL_RUNTIME=/tmp/jsh-cold-ijPU4c JJ_STORE_REAL_ROOT=/Users/stevejuma/code/lapismd/jj-store-projects-hub pnpm exec playwright test --config web/playwright.real.config.ts work-store.spec.ts
```

**Notes**

jsh-directory-native3.log: Cold built native WorkStore acceptance, including repository error navigation and richer repository/review list/cards; exact runtime-error assertion retained.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --write spec/src/milestones.md
```

**Notes**

jsh-directory-format16.log: changed-path formatting applied.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

jsh-directory-progress3.log: Focused command completed.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

jsh-directory-progress4.log: Focused command completed.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --write spec/src/milestones.md
```

**Notes**

jsh-directory-format17.log: changed-path formatting applied.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Final focused UI, controller, service/HTTP, generated-contract, governance and
  cold built native WorkStore gates pass. Disposable 7468 service stopped;
  Storybook 6046 retained. Shared-package release qualification remains pending.
- "Released-registry qualification for the Community composer fix remains
  pending: unchanged source controller architecture/type failures, wider
  Storybook failures and Docker materializer connectivity failure are recorded,
  not waived."
- Human visual review and baseline approval remain pending.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Community Projects/Tasks/Reviews/Repositories landing and project directories, centered shell breadcrumb, sidebar project/repository hierarchy with Community FolderGit2 icon and restored section spacing, card/list Mine/Other groups (repository Mine = administer access), Settings forms with full-width prose fields, task/project/repository Command View selectors, right-rail invisible search and state-icon radio dropdown, Community task disclosure headings and adjacent chevrons, native task header/subtasks/review alignment with full-row seven-field geometry and threaded discussion with immutable edit preview. Independent bounded snapshots, native authorization, exact retained requests, explicit refresh, local-service CLI and WorkStore origin freeze preserved. Repository directory actions size to their own native fields; document-tab measurements stop after navigation removes their nodes. Repository and linked-repository directories use latest selected change summary, contributor, activity bar and date in Community list/card anatomy. Review list/cards show selected excerpt, native identity, exact artifact, count, creation actor and date; right-aligned final field and unclipped card count. Repository read errors occupy a separate row and retain their navigation target. Failed/partial snapshots stay explicit, authorization loss clears old protected facts and reads never reconcile heads.
<!-- /evidence-field -->

<!-- evidence-field:verification_scope markdown -->
## Verification scope

Smallest complete direct impact surface: focused service/HTTP and executed contract gates, direct UI controller and story families, web/catalog checks, bounded architecture, governance/document consumers and cold built real-RustFS WorkStore/browser acceptance. Root spec:check/check/test/build, full Rust workspace and unrelated UI suites out of scope under targeted policy. Source Community wider qualification recorded separately.
<!-- /evidence-field -->

<!-- evidence-field:visual_status markdown -->
## Visual status

visual-pending; no baselines approved or changed
<!-- /evidence-field -->

<!-- evidence-field:dependency_status markdown -->
## Dependency status

Public Community/Design Core APIs through documented ignored temporary local snapshot hooks; publishable manifest and lockfile semver ranges retained. Community edit preview committed locally as oowutlvo / 5041b4c9; focused source acceptance passes, broader source and released-registry qualification remain pending. No package publication or consumer package patch.
<!-- /evidence-field -->

<!-- evidence-field:diagnostic_logs markdown -->
## Diagnostic logs

/tmp/jj-store-projects-*.log, /tmp/jsh-final-*.log, /tmp/jsh-latest-*.log and /tmp/jsh-preview-source-*.log are owned local diagnostics; structured outcomes retained before disposable service cleanup. Latest metadata diagnostics: /tmp/jsh-directory-*.log.
<!-- /evidence-field -->

