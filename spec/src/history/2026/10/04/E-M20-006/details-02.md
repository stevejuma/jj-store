---
schema: 1
id: E-M20-006
---

# E-M20-006 details 2

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
pnpm test:storybook catalog/components/store-stack-workspace/StoreStackWorkspaceTarget.stories.svelte
```

**Notes**

.local/target-branch-evidence/before.log: valid before regression, target branch option absent.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-stack-workspace/StoreStackWorkspaceTarget.stories.svelte
```

**Notes**

.local/target-branch-evidence/after-initial.log: interaction passed but accessibility found an option outside a listbox; fixed Command View composition.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:test web/tests/vite/stack-workspace-target-history.test.ts
```

**Notes**

.local/target-branch-evidence/target-unit.log: 6 passed, 1 test incorrectly loaded an extra page after refresh already loaded the initial page. Corrected test synchronization.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-stack-workspace/StoreStackWorkspaceTarget.stories.svelte
```

**Notes**

.local/target-branch-evidence/target-stories.log: one scroll assertion needed to reach the newly appended final page, and diff closure exposed one null prop runtime error; keyed history diff iteration retains its exact object until teardown.
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
pnpm catalog:test web/tests/vite/stack-workspace-target-history.test.ts
```

**Notes**

.local/target-branch-evidence/target-unit-after.log: 7/7 controller regressions.
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
pnpm test:storybook catalog/components/store-stack-workspace/StoreStackWorkspaceTarget.stories.svelte
```

**Notes**

.local/target-branch-evidence/target-stories-after.log: initial 2/2 interactions/accessibility.
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
pnpm test:storybook catalog/components/store-stack-workspace/StoreStackWorkspaceTarget.stories.svelte
```

**Notes**

.local/target-branch-evidence/target-keyboard.log: added custom-target keyboard regression retained a detached option after filter updates; re-query the live card.
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
pnpm test:storybook catalog/components/store-stack-workspace/StoreStackWorkspaceTarget.stories.svelte
```

**Notes**

.local/target-branch-evidence/target-keyboard-after.log: 3/3 interactions/accessibility, including origin/trunk keyboard selection.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-stack-workspace/StoreStackWorkspaceTarget.stories.svelte -t TargetHeaderUsesTheStackCardLayout
```

**Notes**

.local/target-branch-evidence/header-before.log: exact rendered regression confirms the missing graph inset gives a one-column header grid.
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
pnpm test:storybook catalog/components/store-stack-workspace/StoreStackWorkspaceTarget.stories.svelte
```

**Notes**

.local/target-branch-evidence/header-after.log: 3 passed, header measurement omitted its 1px bottom border. The final strengthened regression also bounds text inside the column. Manual DOM inspection exposed an oversized inset from oldest-first graph input; use the actual displayed newest-first revisions.
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
pnpm test:storybook catalog/components/store-stack-workspace/StoreStackWorkspaceTarget.stories.svelte
```

**Notes**

.local/target-branch-evidence/target-final.log: 4/4 target interactions/accessibility after restoring the 32px displayed graph inset, with exact title/icon/author/chip alignment and title clipping bounds.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-stack-workspace/*.stories.svelte catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLog*.stories.svelte
```

**Notes**

.local/target-branch-evidence/affected-stories.log: 102 passed, existing page-header regression found a duplicate Workspace target branch group name from the new card. The card group now uses Target branch. The process completed before an attempted interrupt; it was not cancelled.
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
pnpm test:storybook catalog/components/store-stack-workspace/StoreStackWorkspaceBranches.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceTarget.stories.svelte
```

**Notes**

.local/target-branch-evidence/branches-target-final.log: 7/7 target and existing Branches interactions/accessibility.
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
pnpm catalog:test web/tests/vite/stack-workspace*.test.ts
```

**Notes**

.local/target-branch-evidence/affected-units.log: 64/64 tests in 7 directly affected controller/drag/history files.
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
pnpm web:check
```

**Notes**

Initial web-check.log and web-check-final.log both report zero errors/warnings; final qualification is recorded below.
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

Initial catalog-check.log and catalog-check-final.log both report zero errors/warnings and 219 catalog modules; final qualification is recorded below.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:architecture
```

**Notes**

.local/target-branch-evidence/architecture.log: controller exceeded 300 counted lines. Extracted its existing exact selection retention into files.ts; no behavior change.
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

.local/target-branch-evidence/architecture-after.log: 1024 modules pass the unchanged architectural budget.
<!-- /run -->
<!-- /evidence-field -->

