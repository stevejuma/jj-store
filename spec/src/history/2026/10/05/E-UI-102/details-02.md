---
schema: 1
id: E-UI-102
---

# E-UI-102 details 2

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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreRepositoryFeedback.stories.svelte (before)
```

**Notes**

before.log: exact preparation regression finds the inline Preparing the graph index paragraph.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreRepositoryFeedback.stories.svelte (initial after)
```

**Notes**

after.log: preparation, skeleton and original-task cancellation/retry flow passes.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreRepositoryFeedback.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryDirectory.stories.svelte catalog/components/community-projects/CommunityProjectsRevisionGraph.stories.svelte catalog/components/store-application/StoreEvolutionColumn.stories.svelte catalog/components/store-application/StoreKindPreparation.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-history-query/StoreHistoryQuery.stories.svelte
```

**Notes**

focused-stories.log: 47 pass, 3 fail. Two isolated composites owned separate toast hosts, causing duplicate notification landmarks; share one fallback owner. The screen-reader-only paging label requires clipping/class assertions rather than toBeVisible, which ignores clipping.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte catalog/components/store-application/StoreKindPreparation.stories.svelte catalog/components/community-projects/CommunityProjectsRevisionGraph.stories.svelte catalog/components/store-application/StoreRepositoryFeedback.stories.svelte
```

**Notes**

corrected-stories.log: all 22 targeted stories pass, including the three corrected failures and dismissal across page changes.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreEvolutionColumn.stories.svelte catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-application/StoreNotifications.stories.svelte catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.stories.svelte catalog/components/store-application/AgentLayout.stories.svelte catalog/components/store-application/AgentComments.stories.svelte catalog/components/store-application/NativeReviewDetail.stories.svelte catalog/components/store-work/NativeActivity.stories.svelte catalog/components/store-work/ProjectsAgents.stories.svelte
```

**Notes**

direct-consumers.log: 44 pass. Shared VirtualRows loading presentation also affects bookmark, agent, review, activity and project-agent surfaces; these direct interaction/accessibility consumers justify this focused expansion.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreRepositoryFeedback.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryDirectory.stories.svelte catalog/components/store-history-query/StoreHistoryQuery.stories.svelte
```

**Notes**

final-pages.log: 14 pass, including metadata/directory-page skeletons. Navigation with an already selected comparison now skeletonizes missing detail instead of flashing binary-unavailable content.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreRepositoryFeedback.stories.svelte catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-application/StoreKindPreparation.stories.svelte
```

**Notes**

final-lifetime.log: 20 pass after application-level ownership. Preparation remains present and singular in Settings and Advanced; cancellation retry uses the original task. Completion is persistent until dismissal.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte (final)
```

**Notes**

op-count-final.log: 9 pass after suppressing an operation-head notice while no heads are loaded.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/notification-service.test.ts
```

**Notes**

notification-units.log: 6 pass; existing notification service policy is unchanged.
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
pnpm web:check (initial and final)
```

**Notes**

web-check.log, web-check-final.log and web-final.log: each has 0 errors/warnings.
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
pnpm catalog:check (initial)
```

**Notes**

catalog-check.log: new fixture getter widened status literals; explicitly typed its StoreController state return.
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
pnpm catalog:check (intermediate)
```

**Notes**

catalog-check-final.log and catalog-check-final2.log: 0 errors/warnings; 194 modules.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:check (new Settings regression)
```

**Notes**

catalog-final.log: two unsupported exact query options in the new story; removed redundant options without changing exact-name intent.
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
pnpm catalog:check (final)
```

**Notes**

catalog-last.log: 0 errors/warnings, 194 modules.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:architecture (initial)
```

**Notes**

architecture.log: log component and one existing story exceeded their line ceilings. Grouped log notices in an owned leaf and kept the story within its existing ceiling; no policy/allowlist change.
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
pnpm check:architecture (final)
```

**Notes**

architecture-final.log and architecture-last.log: 997 modules.
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
pnpm check:ui-ownership (initial and final)
```

**Notes**

ownership.log and ownership-final.log: no external Community imports or new copied sources.
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
pnpm exec prettier --write (changed source, stories and canonical files)
```

**Notes**

format.log, format-final.log, last-format.log and global-format.log preserve focused formatting.
<!-- /run -->
<!-- /evidence-field -->

