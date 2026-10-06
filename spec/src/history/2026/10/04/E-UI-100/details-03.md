---
schema: 1
id: E-UI-100
---

# E-UI-100 details 3

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
pnpm dev:ui
```

**Notes**

dev-ui-final.log: persistent main fixture preserved; existing explicit executable reused; source Vite HMR serves 7460 with backend 7462 and search ready. No reset, installed command replacement or unrelated service restart.
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
CUA background browser check of sidebar and Projects toolbar repository add menus
```

**Notes**

Both menus render at 256px with Add Server Repository and Add Local Repository each on one line and without overflow. repository-menu-wide.png records final appearance; temporary verification tab closed and active user review tab preserved. No repository writes or conversion performed.
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
pnpm spec:validate (before evidence append)
```

**Notes**

spec-validate-initial.log: 5 validators, 22 chapters and 168 requirements.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/DesktopChrome.stories.svelte
```

**Notes**

desktop-chrome-retry.log: 2/2 pass in a fresh browser; the interrupted target has no assertion failure. Complete 74-file acceptance was repeated in isolated 10-file batches without changing assertions or catalog infrastructure.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
python3 (append evidence and assert verbatim historical prefix)
```

**Notes**

Initial append-only script assertion included the old array-separator newline. It stopped before writing the ledger. Corrected the separator check; all 426 historical objects are preserved verbatim and semantically identical.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress (before successful evidence append)
```

**Notes**

progress-evidence.log: same-change ledger update and new evidence were absent after the aborted append. Append completed; progress-evidence2.log then passed.
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
pnpm spec:validate; pnpm check:progress; pnpm spec:first (evidence preparation)
```

**Notes**

spec-validate-evidence.log validates 5 validators, 22 chapters and 168 requirements; progress-evidence2.log passes 137 tasks; spec-first-evidence.log passes mapped protected UI/API/governance chapters. Final checks will follow completed batch evidence.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
python3 .local/repository-ui-feedback/run-affected-batches.py 0; python3 .local/repository-ui-feedback/run-affected-batches.py 1 (independent lanes, each fresh browser per 10-file batch)
```

**Notes**

affected-batch-3.log: 76 pass / 1 fail; new HeaderSpacingAndBorderedRepositoryActionRow story queried the Repository button synchronously before Settings rendering completed. Wait for that accessible button with findByRole; retain all border/spacing/position assertions. Other batches remain independent.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/RepositoryEntryControls.stories.svelte -t HeaderSpacing
```

**Notes**

settings-story-timing-retry.log: exact repaired story passes; 1 other story intentionally skipped. Retry the complete original 10-file batch to prove the failure under its wider consumer scope.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/community-projects/CommunityProjectsRepositoryDirectory.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLogDiff.stories.svelte catalog/components/community-projects/CommunityProjectsRevisionGraph.stories.svelte catalog/components/community-projects/RepositoryDocuments.stories.svelte catalog/components/store-appearance/AppearanceComposer.stories.svelte catalog/components/store-appearance/AppearanceDialog.stories.svelte catalog/components/store-appearance/AppearanceShell.stories.svelte catalog/components/store-appearance/NativeGlass.stories.svelte catalog/components/store-appearance/StoreAppearance.stories.svelte
```

**Notes**

affected-batch-1.log: 10/10 files and 37/37 interaction/accessibility tests pass in a fresh browser. Independent direct consumer acceptance.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentComments.stories.svelte catalog/components/store-application/AgentLayout.stories.svelte catalog/components/store-application/AgentPromptMarkup.stories.svelte catalog/components/store-application/AgentToolGroups.stories.svelte catalog/components/store-application/AppearanceDiscussions.stories.svelte catalog/components/store-application/DesktopChrome.stories.svelte catalog/components/store-application/DiscussionSheets.stories.svelte catalog/components/store-application/NativeAgentMenus.stories.svelte catalog/components/store-application/NativeMessageActions.stories.svelte catalog/components/store-application/NativeReviewDetail.stories.svelte
```

**Notes**

affected-batch-2.log: 10/10 files and 41/41 interaction/accessibility tests pass in a fresh browser. Independent direct consumer acceptance.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/RepositoryApplication.stories.svelte catalog/components/store-application/RepositoryEntryControls.stories.svelte catalog/components/store-application/RepositoryLayout.stories.svelte catalog/components/store-application/RepositoryLoading.stories.svelte catalog/components/store-application/RepositoryReview.stories.svelte catalog/components/store-application/SettingsNavigation.stories.svelte catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-application/StoreCaptureSettings.stories.svelte catalog/components/store-application/StoreCaptureSources.stories.svelte catalog/components/store-application/StoreCollaboration.stories.svelte
```

**Notes**

affected-batch-3-after-timing.log: 10/10 files and 77/77 interaction/accessibility tests pass in a fresh browser. Complete original batch repeated after the asynchronous Settings test repair.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreComparisonParentPicker.stories.svelte catalog/components/store-application/StoreConnection.stories.svelte catalog/components/store-application/StoreEvolutionColumn.stories.svelte catalog/components/store-application/StoreFileEditor.stories.svelte catalog/components/store-application/StoreLocalFolderDialog.stories.svelte catalog/components/store-application/StoreMultiHarnessSettings.stories.svelte catalog/components/store-application/StoreNotifications.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte catalog/components/store-application/StoreRepositoryBranchContext.stories.svelte catalog/components/store-application/StoreRepositoryConversion.stories.svelte
```

**Notes**

affected-batch-4.log: 10/10 files and 30/30 interaction/accessibility tests pass in a fresh browser. Independent direct consumer acceptance.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreRepositoryNavigation.stories.svelte catalog/components/store-application/StoreRepositorySearch.stories.svelte catalog/components/store-application/StoreSearchSettings.stories.svelte catalog/components/store-application/StoreServerSettings.stories.svelte catalog/components/store-application/StoreStorageSettings.stories.svelte catalog/components/store-application/StoreSyncReconciliation.stories.svelte catalog/components/store-application/StoreSyncSettings.stories.svelte catalog/components/store-application/StoreWorkspaceSettings.stories.svelte catalog/components/store-application/StoreWorkspaces.stories.svelte catalog/components/store-history-query/StoreHistoryQuery.stories.svelte
```

**Notes**

affected-batch-5.log: 10/10 files and 43/43 interaction/accessibility tests pass in a fresh browser. Independent direct consumer acceptance.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-mutations/StoreConflictEditor.stories.svelte catalog/components/store-mutations/StoreRepositoryActions.stories.svelte catalog/components/store-search/StoreRecentSearch.stories.svelte catalog/components/store-search/StoreSearchCommand.stories.svelte catalog/components/store-terminal/Terminal.stories.svelte catalog/components/store-terminal/TerminalHeader.stories.svelte catalog/components/store-threads/StoreCapturedReactions.stories.svelte catalog/components/store-threads/StoreThreadAlignment.stories.svelte catalog/components/store-threads/StoreThreadNavigation.stories.svelte catalog/components/store-threads/StoreThreadSearch.stories.svelte
```

**Notes**

affected-batch-6.log: 10/10 files and 37/37 interaction/accessibility tests pass in a fresh browser. Independent direct consumer acceptance.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-threads/StoreThreads.stories.svelte catalog/components/store-work/DisclosurePersistence.stories.svelte catalog/components/store-work/NativeActivity.stories.svelte catalog/components/store-work/NavigationPersistence.stories.svelte catalog/components/store-work/ProjectsActivity.stories.svelte catalog/components/store-work/ProjectsActivityControls.stories.svelte catalog/components/store-work/ProjectsAgents.stories.svelte catalog/components/store-work/ProjectsDirectory.stories.svelte catalog/components/store-work/ProjectsFilters.stories.svelte catalog/components/store-work/ProjectsHub.stories.svelte
```

**Notes**

affected-batch-7.log: 10/10 files and 48/48 interaction/accessibility tests pass in a fresh browser. Independent direct consumer acceptance.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/ProjectsRepositoryActions.stories.svelte catalog/components/store-work/ProjectsReviewDetail.stories.svelte catalog/components/store-work/StoreWork.stories.svelte catalog/components/store-work/TaskScroll.stories.svelte
```

**Notes**

affected-batch-8.log: 4/4 files and 18/18 interaction/accessibility tests pass in a fresh browser. Independent direct consumer acceptance.
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
pnpm catalog:check; pnpm exec prettier --write catalog/components/store-application/RepositoryEntryControls.stories.svelte; pnpm exec prettier --check .local/repository-ui-feedback/evidence-draft.json
```

**Notes**

catalog-check-post-timing.log: zero errors/warnings and 190 catalog modules after the asynchronous story repair; story-timing-format.log and evidence-format.log pass.
<!-- /run -->
<!-- /evidence-field -->

