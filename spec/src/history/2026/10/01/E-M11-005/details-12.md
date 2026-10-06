---
schema: 1
id: E-M11-005
---

# E-M11-005 details 12

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
cargo fmt --all -- --check
```

**Notes**

jsh-directory-rustformat2.log: Focused command completed.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

jsh-directory-progress1.log: Two ledger violations: unsupported skip outcomes and Done M11 parent with In progress M11.4 child. Zero-target commands remain failed acceptance probes; parent tracks the new child scope.
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
pnpm spec:validate
```

**Notes**

jsh-directory-spec1.log: Specification validated: 4 validators, 20 chapters, 125 requirements.
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
pnpm spec:first
```

**Notes**

jsh-directory-first1.log: Focused command completed.
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
pnpm check:boundaries
```

**Notes**

jsh-directory-boundaries1.log: package boundaries ok (15 packages)
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
pnpm test:governance
```

**Notes**

jsh-directory-governance1.log: ✔ rejects_unchanged_ledger_for_code (0.628042ms); ✔ allows_documentation_without_execution_ledger (0.079ms); ℹ pass 34
<!-- /run -->

<!-- run -->
### Run 7

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

jsh-directory-native1.log: Focused command completed.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

jsh-directory-progress2.log: Two Git prerequisite rows still name the reopened aggregate. Bind them explicitly to original qualified M11.1-M11.3; the new hub does not change native Git acceptance.
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
JJ_STORE_DEV_ROOT=/tmp/jsh-cold-ijPU4c JJ_STORE_DEV_PORT=7468 JJ_STORE_DEV_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs pnpm dev:restart
```

**Notes**

jsh-directory-cold4.log: Startup: 1246 ms cold, 3770 ms restart; Resident memory: 152320 KiB server plus storage
<!-- /run -->

<!-- run -->
### Run 10

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

jsh-directory-native1.log: Cold native 0/1: per-repository busy error flex item squeezes linked repository button to zero width; metadata errors must retain code navigation.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/ProjectsDirectory.stories.svelte
```

**Notes**

jsh-directory-error-before.log: 2/3: exact error-row story reproduces 0px repository target width in list layout.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/ProjectsDirectory.stories.svelte
```

**Notes**

jsh-directory-error-after.log: 3/3: read errors occupy their own row and full card width; repository targets stay visible and wider than 64px.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --check .storybook/vite.config.ts catalog/components/store-application/StoreConnection.stories.svelte catalog/components/store-application/StoreRepositoryNavigation.stories.svelte catalog/components/store-application/StoreRepositoryPicker.stories.svelte catalog/components/store-work/ProjectsDirectory.stories.svelte catalog/components/store-work/ProjectsHub.stories.svelte catalog/components/store-work/StoreWork.stories.svelte catalog/fixtures/ProjectsHubFixture.svelte catalog/fixtures/StoreWorkFixture.svelte catalog/fixtures/collaboration-review.ts catalog/fixtures/projects-hub-fixture.ts catalog/fixtures/projects-hub-layout.ts package.json schemas/openapi.json scripts/web-manifest.mjs scripts/web-manifest.test.mjs spec/src/api.md spec/src/architecture.md spec/src/cli.md spec/src/collaboration.md spec/src/compatibility.md spec/src/milestones.md spec/src/mutations.md spec/src/projects.md spec/src/spec-governance.md spec/src/ui.md spec/src/verification.md web/src/App.svelte web/src/lib/api/generated.ts web/src/lib/controller/contracts.ts web/src/lib/controller/http-repository-directory.ts web/src/lib/controller/http-work-data-source.ts web/src/lib/controller/production-projects-hub.svelte.ts web/src/lib/controller/production-projects-integration.ts web/src/lib/controller/production-route-restorer.ts web/src/lib/controller/production-store-actions.ts web/src/lib/controller/production-store-connect.ts web/src/lib/controller/production-store-controller.svelte.ts web/src/lib/controller/production-store-lifecycle.svelte.ts web/src/lib/controller/production-store-view.ts web/src/lib/controller/production-work.svelte.ts web/src/lib/controller/projects-home-read.ts web/src/lib/controller/projects-hub-types.ts web/src/lib/controller/projects-navigation.svelte.ts web/src/lib/controller/projects-repositories.svelte.ts web/src/lib/controller/projects-repository-types.ts web/src/lib/controller/projects-reviews.svelte.ts web/src/lib/controller/projects-route-codec.ts web/src/lib/controller/projects-view-cache.ts web/src/lib/controller/projects-work-registry.svelte.ts web/src/lib/controller/store-route-codec.ts web/src/lib/controller/store-route-history.svelte.ts web/src/lib/controller/work-types.ts web/src/lib/ui/community-navigation/CommunityNavigation.css web/src/lib/ui/community-navigation/CommunityNavigation.svelte web/src/lib/ui/community-navigation/CommunityNavigationTree.css web/src/lib/ui/community-navigation/icons.ts web/src/lib/ui/community-navigation/model.ts web/src/lib/ui/community-projects/CommunityProjectsRepositoryDocuments.svelte web/src/lib/ui/store-application/StoreApplication.svelte web/src/lib/ui/store-application/StoreConnection.svelte web/src/lib/ui/store-application/StoreRepositoryApplication.svelte web/src/lib/ui/store-application/StoreRepositoryPicker.svelte web/src/lib/ui/store-application/store-navigation.ts web/src/lib/ui/store-work/StoreProjects.svelte web/src/lib/ui/store-work/StoreProjectsBreadcrumb.svelte web/src/lib/ui/store-work/StoreProjectsDirectory.svelte web/src/lib/ui/store-work/StoreProjectsFilters.svelte web/src/lib/ui/store-work/StoreProjectsRail.svelte web/src/lib/ui/store-work/StoreProjectsRepositories.svelte web/src/lib/ui/store-work/StoreProjectsReviews.svelte web/src/lib/ui/store-work/StoreRepositoryActivity.svelte web/src/lib/ui/store-work/StoreRepositoryDirectoryRow.svelte web/src/lib/ui/store-work/StoreWork.svelte web/src/lib/ui/store-work/StoreWorkCollection.svelte web/src/lib/ui/store-work/StoreWorkComments.svelte web/src/lib/ui/store-work/StoreWorkDialog.svelte web/src/lib/ui/store-work/StoreWorkEmpty.svelte web/src/lib/ui/store-work/StoreWorkField.svelte web/src/lib/ui/store-work/StoreWorkLayoutControls.svelte web/src/lib/ui/store-work/StoreWorkMessageActions.svelte web/src/lib/ui/store-work/StoreWorkOwnershipGroups.svelte web/src/lib/ui/store-work/StoreWorkPicker.svelte web/src/lib/ui/store-work/StoreWorkProjectForm.svelte web/src/lib/ui/store-work/StoreWorkProjectRows.svelte web/src/lib/ui/store-work/StoreWorkRepositories.svelte web/src/lib/ui/store-work/StoreWorkReviewRow.svelte web/src/lib/ui/store-work/StoreWorkStateFilter.svelte web/src/lib/ui/store-work/StoreWorkStatus.svelte web/src/lib/ui/store-work/StoreWorkSubtasks.svelte web/src/lib/ui/store-work/StoreWorkTaskDetail.svelte web/src/lib/ui/store-work/StoreWorkTaskForm.svelte web/src/lib/ui/store-work/StoreWorkTaskRows.svelte web/src/lib/ui/store-work/StoreWorkTaskStateIcon.svelte web/src/lib/ui/store-work/community-work-breadcrumb.css web/src/lib/ui/store-work/community-work-cards.css web/src/lib/ui/store-work/community-work-collections.css web/src/lib/ui/store-work/community-work-directory.css web/src/lib/ui/store-work/community-work-disclosures.css web/src/lib/ui/store-work/community-work-forms.css web/src/lib/ui/store-work/community-work-layout.css web/src/lib/ui/store-work/community-work-reviews.css web/src/lib/ui/store-work/community-work-rows.css web/src/lib/ui/store-work/community-work-tasks.css web/src/lib/ui/store-work/community-work-thread.css web/src/lib/ui/store-work/projects-sidebar-model.ts web/src/lib/ui/store-work/repository-directory-presentation.ts web/src/lib/ui/store-work/store-projects.css web/src/lib/ui/store-work/store-work.css web/src/lib/ui/store-work/work-drafts.ts web/src/lib/ui/store-work/work-presentation.ts web/tests/real/work-store.spec.ts web/tests/store-route-codec.test.ts web/tests/vite/production-store-bootstrap.test.ts web/tests/vite/production-work.test.ts web/tests/vite/projects-code-navigation.test.ts web/tests/vite/projects-directories.test.ts web/tests/vite/projects-hub.test.ts web/tests/vite/projects-repositories.test.ts web/tests/vite/store-route-history.test.ts web/tests/work-presentation.test.ts web/vite.config.ts
```

**Notes**

jsh-directory-formatcheck1.log: ledger formatting corrected before final changed-path check.
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
pnpm exec prettier --write web/src/lib/ui/store-work/community-work-directory.css catalog/components/store-work/ProjectsDirectory.stories.svelte
```

**Notes**

jsh-directory-format10.log: changed-path formatting applied.
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
pnpm exec prettier --write spec/src/milestones.md spec/src/git-interop.md
```

**Notes**

jsh-directory-format9.log: changed-path formatting applied.
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
pnpm catalog:check
```

**Notes**

jsh-directory-catalog4.log: svelte-check found 0 errors and 0 warnings; catalog boundaries ok (87 modules)
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:architecture
```

**Notes**

jsh-directory-arch4.log: web architecture ok (377 modules)
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/ProjectsDirectory.stories.svelte catalog/components/store-work/ProjectsHub.stories.svelte catalog/components/store-work/StoreWork.stories.svelte
```

**Notes**

jsh-directory-stories5.log: 17/17 final direct directory/hub/task stories after the error-row navigation fix; prior nine direct families pass 57/57.
<!-- /run -->
<!-- /evidence-field -->

