---
schema: 1
id: E-STORY-PARITY-001
---

# E-STORY-PARITY-001 details 4

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
node --test scripts/validation.test.mjs scripts/storybook-parity.test.mjs
```

**Notes**

16 tests passed, including real JJ object transfer and preserved dirty-source parents.
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
JJ_STORE_VALIDATION_RUNTIME=native pnpm catalog:check
```

**Notes**

0 Svelte errors/warnings; catalog boundary and full story parity validation passed.
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
JJ_STORE_VALIDATION_RUNTIME=native pnpm web:check
```

**Notes**

0 errors/warnings.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 28
hasNote: true
noteEncoding: markdown

```

```shell
node scripts/validation.mjs run test:governance
```

**Notes**

First remote run exposed missing Cargo/lsof in the selected toolchain and missing registry packages for offline clean-install tests. Full image selection and registry fetch are being corrected; requalification pending.
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
pnpm test:governance
```

**Notes**

Docker run e975b9f9-c18e-407a-a0ff-003650f5a564: 115/115 passed after pinned-source transfer and Cargo/pnpm offline cache preparation fixes.
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
node --test scripts/validation.test.mjs scripts/validation-pins.test.mjs scripts/storybook-parity.test.mjs
```

**Notes**

21 tests passed, including dirty sibling exclusion, tamper rejection and exact-versus-unfiltered browser invocation selection.
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
JJ_STORE_VALIDATION_RUNTIME=native pnpm catalog:check
```

**Notes**

0 errors and warnings; 487 catalog modules; storybook-parity validator passed.
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
JJ_STORE_VALIDATION_RUNTIME=native pnpm web:check
```

**Notes**

0 errors and warnings.
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
JJ_STORE_VALIDATION_RUNTIME=native pnpm catalog:test
```

**Notes**

189 files; 1206 tests passed and one existing skipped test.
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
JJ_STORE_VALIDATION_RUNTIME=native pnpm web:test
```

**Notes**

180 tests passed.
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
node scripts/validation.mjs run test:storybook catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte catalog/components/store-application/AgentChatBody.stories.svelte catalog/components/store-application/AgentChatRuntime.stories.svelte catalog/components/store-application/AgentFleet.stories.svelte catalog/components/store-application/AgentHarnessModels.stories.svelte catalog/components/store-application/AgentInboxNavigation.stories.svelte catalog/components/store-application/AgentLayout.stories.svelte catalog/components/store-application/AgentSessionSettings.stories.svelte catalog/components/store-application/AgentSessions.stories.svelte catalog/components/store-application/AgentToolGroups.stories.svelte catalog/components/store-application/ConversationContinuity.stories.svelte catalog/components/store-application/DesktopChrome.stories.svelte catalog/components/store-application/RepositoryReview.stories.svelte catalog/components/store-application/SettingsShell.stories.svelte catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-application/StoreCollaboration.stories.svelte catalog/components/store-application/StoreFileEditor.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte catalog/components/store-application/UnavailableContent.stories.svelte catalog/components/store-application/WorkspaceFilePresentation.stories.svelte catalog/components/store-application/WorkspaceFilesPerformance.stories.svelte catalog/components/store-forge/StoreForgeInlineWorkspace.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceConflicts.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceDrag.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceInline.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceLoading.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspacePinning.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspacePresentation.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceRefinements.stories.svelte catalog/components/store-stack-workspace/StoreStacksNavigation.stories.svelte catalog/components/store-work/DisclosurePersistence.stories.svelte catalog/components/store-work/ProjectsActivity.stories.svelte catalog/components/store-work/ProjectsFilters.stories.svelte catalog/components/store-work/ProjectsRepositoryActions.stories.svelte catalog/components/store-workflows/WorkflowHistoryNavigation.stories.svelte catalog/components/store-workflows/WorkflowSidebarActions.stories.svelte
```

**Notes**

Docker bc0bff3c-ed42-45e0-9b58-2f1f5ecea36c: 212 passed /1 failed, down from 73 failures in the first unfiltered run. Remaining refresh case captured rows before viewport-driven initial repository summaries loaded.
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
JJ_STORE_VALIDATION_RUNTIME=native pnpm test:storybook catalog/components/store-work/ProjectsActivity.stories.svelte -t LoadedProjectsSurviveRefresh
```

**Notes**

Exact remaining regression passed; 7 untargeted tests skipped. Awaited initial summaries before testing retention; row identity/visibility assertions remain.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook
```

**Notes**

Docker 711b30cd-61ed-4c89-bc00-b2bc6b176e43 executed all 216 files and 1048 tests in four fresh sequential browsers: 1044 passed /4 failed. No browser disconnect or unexecuted files. The new findings concern remount readiness, directory names with counts, native-versus-synthetic hover semantics and mounted hidden workflow output.
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
JJ_STORE_VALIDATION_RUNTIME=native pnpm test:storybook catalog/components/store-work/NavigationPersistence.stories.svelte
```

**Notes**

Exact remount regression passed after awaiting the remounted button, retaining both expanded/collapsed persistence assertions.
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
JJ_STORE_VALIDATION_RUNTIME=native pnpm test:storybook catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLogFiles.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceDrag.stories.svelte
```

**Notes**

All 11 cases passed. Directory locators require exact data-directory-path plus visible name; branch hover now uses a real pointer, retains keyboard selection and verifies hover/restoration CSS surfaces.
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
node catalog/fixtures/delta-chat/browser.acceptance.mjs
```

**Notes**

Complete cold macOS native pointer/keyboard acceptance passed: immutable captures, queue editing, focus/caret, drafts, scroll, resize, teardown, mobile read panes, gutters and footer menus. Fixture launchers are hidden only for this product-hit-target acceptance; menus select the current open surface during animation.
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
JJ_STORE_VALIDATION_RUNTIME=native pnpm check:architecture
```

**Notes**

2076 modules passed after extracting the footer, story helper and menu stylesheet to retain source-size bounds.
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
pnpm test:storybook catalog/components/store-work/NavigationPersistence.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLogFiles.stories.svelte catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.stories.svelte catalog/components/store-workflows/WorkflowHistory.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceDrag.stories.svelte
```

**Notes**

Docker 368787e1-2812-4d8f-8e59-2bada3bfd68f: 5 files /21 tests passed. Combined with the four-shard run, every one of the 1048 stories has passing current-scope evidence. No remaining interaction/accessibility failure or lost file.
<!-- /run -->
<!-- /evidence-field -->

