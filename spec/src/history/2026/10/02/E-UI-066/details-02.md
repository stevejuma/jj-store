---
schema: 1
id: E-UI-066
---

# E-UI-066 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 7
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/ProjectsActivity.stories.svelte catalog/components/store-work/ProjectsDirectory.stories.svelte catalog/components/store-application/RepositoryLoading.stories.svelte
```

**Notes**

Initial before run: dynamic Svelte CSF story name rejected by the story parser (one suite could not load) and six product regression failures in the other families. Replaced dynamic Story declarations with literal story names; no product edits. /tmp/jj-loading-before.log.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 13
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/ProjectsActivity.stories.svelte catalog/components/store-work/ProjectsDirectory.stories.svelte catalog/components/store-application/RepositoryLoading.stories.svelte
```

**Notes**

Corrected before reproduction:13 failures,14 passes/27; missing discovery/bootstrap/connection and task skeletons, loading banner during retained refresh, and varied project avatars. /tmp/jj-loading-before-corrected.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/ProjectsActivity.stories.svelte catalog/components/store-work/ProjectsDirectory.stories.svelte catalog/components/store-application/RepositoryLoading.stories.svelte
```

**Notes**

First repair:27/27. /tmp/jj-loading-after.log.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/RepositoryLoading.stories.svelte -t ReviewDetailBeforeListRead
```

**Notes**

Additional early selected-review stage reproduced an unavailable notice before the list read completes. /tmp/jj-loading-review-before.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/ProjectsActivity.stories.svelte catalog/components/store-work/ProjectsDirectory.stories.svelte catalog/components/store-application/RepositoryLoading.stories.svelte
```

**Notes**

Final strengthened regressions:29/29 across three families on a fresh Vitest browser/Vite process. Five main tabs retain loaded lists during deferred refresh; task detail keeps composer DOM/draft, repository tabs and selected reviews show skeletons during initial connection/list reads; avatars align at980/620/300px without overflow. /tmp/jj-loading-regressions-final.log.
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
pnpm web:check
```

**Notes**

Initial and final production checks:0 errors/warnings. /tmp/jj-loading-web-check.log; /tmp/jj-loading-web-check-final.log.
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
pnpm catalog:check
```

**Notes**

New avatar story incorrectly used the Playwright exact option on Testing Library getByRole. Removed unsupported option without changing the assertion. /tmp/jj-loading-catalog-check.log.
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
pnpm catalog:check
```

**Notes**

Corrected catalog:0 errors/warnings;129 modules. /tmp/jj-loading-catalog-check-final.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/ProjectsHub.stories.svelte catalog/components/store-work/ProjectsFilters.stories.svelte catalog/components/store-work/ProjectsActivityControls.stories.svelte catalog/components/store-work/StoreWork.stories.svelte catalog/components/store-application/NavigationPersistence.stories.svelte catalog/components/store-application/StoreCollaboration.stories.svelte catalog/components/store-application/NativeAgentMenus.stories.svelte catalog/components/store-application/NativeReviewDetail.stories.svelte catalog/components/store-application/RepositoryApplication.stories.svelte catalog/components/store-application/RepositoryLayout.stories.svelte catalog/components/store-application/RepositoryReview.stories.svelte catalog/components/store-application/StoreFileEditor.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte catalog/components/store-application/StoreRepositoryBranchContext.stories.svelte catalog/components/store-application/StoreRepositoryNavigation.stories.svelte catalog/components/store-application/StoreRepositorySearch.stories.svelte catalog/components/store-application/StoreWorkspaces.stories.svelte catalog/components/store-application/AppearanceDiscussions.stories.svelte
```

**Notes**

67/67 in17 direct repository/Projects consumer families. The navigation story path in this command did not exist and was subsequently corrected in the next command. /tmp/jj-loading-consumer-stories.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/NavigationPersistence.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryDirectory.stories.svelte
```

**Notes**

11/11 in three additional direct families:correct navigation persistence, graph and repository directory. /tmp/jj-loading-navigation-stories.log.
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
pnpm check:architecture
```

**Notes**

739 modules within limits, before and after stronger fixtures. /tmp/jj-loading-architecture-pre.log; /tmp/jj-loading-architecture.log.
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
pnpm spec:first
```

**Notes**

18 protected files/3 canonical chapters. /tmp/jj-loading-spec-first-pre.log.
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
cua_repl: fresh hidden Projects and task-detail pages; measure project avatar positions; capture Projects screenshot; await task content and close both temporary tabs
```

**Notes**

Production HMR cold pages expose Loading projects and Loading task details before accepted data; task settles to comments/composer. All three live project avatar left positions1109.40625px, each row scrollWidth=clientWidth=970px. /tmp/jj-project-avatars-final.png. User tab/fixture preserved; no publications or service restarts.
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
pnpm spec:first
```

**Notes**

Final ledger included:18 protected files/4 canonical chapters. /tmp/jj-loading-spec-first.log.
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
pnpm spec:validate
```

**Notes**

5 validators/21 chapters/147 requirements. /tmp/jj-loading-spec-validate.log.
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
pnpm check:progress
```

**Notes**

120 tasks; evidence bookkeeping only. /tmp/jj-loading-progress.log. Final evidence-only formatting/check repeat follows without milestone status changes.
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
pnpm check:ui-ownership
```

**Notes**

Owned presentation boundary passes. /tmp/jj-loading-ownership.log.
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
pnpm check:architecture
```

**Notes**

Final formatted739 modules pass. /tmp/jj-loading-architecture-final.log.
<!-- /run -->
<!-- /evidence-field -->

