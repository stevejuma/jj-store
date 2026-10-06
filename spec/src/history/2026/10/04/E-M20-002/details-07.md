---
schema: 1
id: E-M20-002
---

# E-M20-002 details 7

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
node scripts/with-storybook-env.mjs playwright test -c playwright.workspace.config.ts -g 'centre drop shows the full overlay before moving a tab'
```

**Notes**

Output: /Users/stevejuma/code/lapismd/design-core/.local/diff-count-contrast/workspace-ready-isolated.log. Fresh isolated server had one 45-second readiness timeout with slow transforms and no network errors. Assertions and timeout unchanged.
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
node scripts/with-storybook-env.mjs playwright test -c playwright.workspace.config.ts -g 'centre drop shows the full overlay before moving a tab'
```

**Notes**

Output: /Users/stevejuma/code/lapismd/design-core/.local/diff-count-contrast/workspace-ready-isolated-retry.log. Exact isolated cold test passed unchanged; 35.8 seconds test time, 57.8 seconds command duration.
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
pnpm checks
```

**Notes**

Output: /Users/stevejuma/code/lapismd/design-core/.local/diff-count-contrast/checks-isolated-final.log. Source workspace pointer tests 14/14 passed; shell first-test total timeout failed once, with 10 shell passes. Assertions and default timeout unchanged.
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
node scripts/with-storybook-env.mjs playwright test -c playwright.shell.config.ts -g 'horizontal pans reveal and dismiss both edge lanes'
```

**Notes**

Output: /Users/stevejuma/code/lapismd/design-core/.local/diff-count-contrast/shell-mobile-exact.log. Exact shell regression passed cold, unchanged; 22.8 seconds test time and 30.9 seconds command duration.
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
STORYBOOK_PORT=9139 pnpm checks
```

**Notes**

Output: /Users/stevejuma/code/lapismd/design-core/.local/diff-count-contrast/checks-qualified.log. Complete required nonvisual Design Core qualification passed: 715 units/111 files, 652 stories/209 files, 14 workspace pointer, 11 shell, 3 AI and 36 shadcn browser tests; 1 platform-specific shadcn test skipped. Formatting, governance, types, static build and audit passed. Shared published tokens require the owning source checks under Design Core AGENTS.md. Existing ResizeObserver/proxy diagnostics appeared in source stories; no assertion or timeout was weakened. Committed zlyuvqyv/d7ebf3e1.
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
pnpm spec:validate
```

**Notes**

Output: .local/stack-header-drops-evidence/spec-scoped-final.log. 5 validators, 23 chapters and 173 requirements passed before final branch/footer follow-up.
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
pnpm spec:first
```

**Notes**

Output: .local/stack-header-drops-evidence/spec-first-scoped-final.log. 31 protected files and 4 canonical chapters passed before final branch/footer follow-up.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 8
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

Output: .local/stack-header-drops-evidence/progress-scoped-final.log. Eight bookkeeping failures: deleted Choice file listed as present and seven story names in the named-test-only field.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 8
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

Output: .local/stack-header-drops-evidence/progress-scoped-repaired.log. Same eight issues remained after converting CamelCase story names to display names. Fixed metadata by listing removed files and moving story names to story_scenarios.
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
pnpm check:progress
```

**Notes**

Output: .local/stack-header-drops-evidence/progress-scoped-qualified.log. 141 tasks passed; bookkeeping only.
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
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace catalog/components/community-projects/CommunityProjectsRevisionGraph.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLogFiles.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLogDiff.stories.svelte catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.stories.svelte catalog/components/store-application/StoreRepositoryPicker.stories.svelte catalog/components/store-application/StoreRepositoryNavigation.stories.svelte catalog/components/store-application/StoreRepositoryBranchContext.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte --reporter=verbose
```

**Notes**

Output: .local/stack-header-drops-evidence/all-stories-final-qualified.log. 107/107 interaction/accessibility scenarios passed after footer icon and Pencil changes, before subsequent branch/body-action/footer-tone/composer refinements.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 6
hasNote: true
noteEncoding: markdown

```

```shell
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceBranches.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceFooter.stories.svelte catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.stories.svelte -t 'AppliedBranchesHaveTheirOwnGroup|ApplyAndUnapplyActionsOccupyTheStackBody|WorkspaceTargetBranchAppearsAboveBodyTabs|RetainedDraftMatchesThe.*FooterChips|WorkspaceChipSitsAtTheTitleRowEdge' --reporter=verbose
```

**Notes**

Output: .local/stack-header-drops-evidence/branch-footer-before.log. Six requested branch/footer regressions failed before implementation. The target-row query initially used banner and was corrected to an explicit named group before implementation.
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
pnpm test:storybook catalog/components/store-stack-workspace/StoreStackWorkspaceBranches.stories.svelte -t WorkspaceTargetBranchAppearsAboveBodyTabs --reporter=verbose
```

**Notes**

Output: .local/stack-header-drops-evidence/branch-target-before-exact.log. Exact named-group regression reproduced the missing workspace target row above body tabs.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceBranches.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceFooter.stories.svelte catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.stories.svelte -t 'AppliedBranchesHaveTheirOwnGroup|ApplyAndUnapplyActionsOccupyTheStackBody|WorkspaceTargetBranchAppearsAboveBodyTabs|RetainedDraftMatchesThe.*FooterChips|WorkspaceChipSitsAtTheTitleRowEdge' --reporter=verbose
```

**Notes**

Output: .local/stack-header-drops-evidence/branch-footer-after.log. Five scenarios passed; group-search scenario held a stale group element with three initial options. Live UI search returned only feature/model. The test now resolves the current group on each wait; the one-option assertion remains unchanged.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-stack-workspace/StoreStackWorkspaceCommands.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceInline.stories.svelte -t InlineCommitWithStagedFilesAndHunks|IdleAndStagedCommitCards --reporter=verbose
```

**Notes**

Output: .local/stack-header-drops-evidence/composer-borders-before.log. The title-height regression failed before its change. The second filter name selected no test; the actual StartCommitBecomesAStagedStackCard regression was rerun separately.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-stack-workspace/StoreStackWorkspaceInline.stories.svelte -t StartCommitBecomesAStagedStackCard --reporter=verbose
```

**Notes**

Output: .local/stack-header-drops-evidence/drop-border-before-exact.log. Exact drop-target geometry failed: one-pixel border instead of requested two-pixel dashed border.
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
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceBranches.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceFooter.stories.svelte catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceCommands.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceInline.stories.svelte -t 'AppliedBranchesHaveTheirOwnGroup|ApplyAndUnapplyActionsOccupyTheStackBody|WorkspaceTargetBranchAppearsAboveBodyTabs|RetainedDraftMatchesThe.*FooterChips|WorkspaceChipSitsAtTheTitleRowEdge|InlineCommitWithStagedFilesAndHunks|StartCommitBecomesAStagedStackCard' --reporter=verbose
```

**Notes**

Output: .local/stack-header-drops-evidence/branch-footer-composer-exact.log. Seven scenarios passed; stale group reference still failed the search assertion. Fixed by resolving the current Applied group inside the unchanged one-option wait.
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
pnpm web:check
```

**Notes**

Output: .local/stack-header-drops-evidence/web-branch-footer-final.log. 0 errors and 0 warnings.
<!-- /run -->
<!-- /evidence-field -->

