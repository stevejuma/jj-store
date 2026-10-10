---
schema: 1
id: E-UI-VIEW-READINESS-001
---

# E-UI-VIEW-READINESS-001 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 8
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-application/StoreFileEditor.stories.svelte catalog/components/store-application/AgentTranscriptSettings.stories.svelte catalog/components/store-application/AppearanceDiscussions.stories.svelte catalog/components/store-application/NativeReviewDetail.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceSettings.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceConflictVersions.stories.svelte catalog/components/store-mutations/StoreConflictEditor.stories.svelte
```

**Notes**

Fresh before run: 23 pass, 8 fail, 7 files. Seven premature view queries and one copy submenu transition. /tmp/readiness-before.log.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 16
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-application/AgentToolGroups.stories.svelte catalog/components/store-application/AgentLayout.stories.svelte catalog/components/store-application/ConversationCaret.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceLoading.stories.svelte catalog/components/store-application/RepositoryLayout.stories.svelte
```

**Notes**

Fresh before run: 11 pass, 16 fail, 5 files. Includes Classic-only grouped-call fixture expectation, reader editor queried before mount and prior Overview inset expectation. /tmp/readiness-second-before.log.
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
pnpm test:storybook catalog/components/store-application/StoreFileEditor.stories.svelte catalog/components/store-application/AgentTranscriptSettings.stories.svelte catalog/components/store-application/AppearanceDiscussions.stories.svelte catalog/components/store-application/NativeReviewDetail.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceSettings.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceConflictVersions.stories.svelte catalog/components/store-mutations/StoreConflictEditor.stories.svelte
```

**Notes**

30 pass, 1 fail: copy submenu keyboard opening still raced menu autofocus. /tmp/readiness-first-after.log.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 6
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-application/AgentToolGroups.stories.svelte catalog/components/store-application/AgentLayout.stories.svelte catalog/components/store-application/ConversationCaret.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceLoading.stories.svelte catalog/components/store-application/RepositoryLayout.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte
```

**Notes**

27 pass, 6 fail. Loading, Agent layout and caret families now pass; follow-on navigation readiness, collapse teardown and Overview contract expectations repaired. /tmp/readiness-second-after.log.
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
pnpm test:storybook catalog/components/store-application/StoreFileEditor.stories.svelte catalog/components/store-application/AgentTranscriptSettings.stories.svelte catalog/components/store-application/AppearanceDiscussions.stories.svelte catalog/components/store-application/NativeReviewDetail.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceSettings.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceConflictVersions.stories.svelte catalog/components/store-mutations/StoreConflictEditor.stories.svelte catalog/components/store-application/AgentToolGroups.stories.svelte catalog/components/store-application/AgentLayout.stories.svelte catalog/components/store-application/ConversationCaret.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceLoading.stories.svelte catalog/components/store-application/RepositoryLayout.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte
```

**Notes**

Fresh Chromium process: all 64 stories pass across 13 complete families, 100.47s. Includes unchanged focus, draft, geometry, loading phase, conflict and accessible copy controls. /tmp/readiness-combined-after.log.
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
pnpm catalog:check
```

**Notes**

Svelte diagnostics: 0 errors, 0 warnings; catalog boundaries: 370 modules. /tmp/readiness-catalog-check.log.
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
pnpm check:ui-ownership
```

**Notes**

Owned-source validator passes; no production or Community dependency added. /tmp/readiness-ownership.log.
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
pnpm exec playwright test --config .local/view-loading-proof/playwright.config.ts web/tests/browser/repository-navigation.spec.ts
```

**Notes**

2 pass, 1 fail. Corrected Overview contract passes, then old absent-Agents assertion conflicts with intentional installation-wide Agents/Threads navigation. Existing preview binary and built assets reused; no Rust build. /tmp/readiness-browser-after.log.
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
pnpm exec playwright test --config .local/view-loading-proof/playwright.config.ts web/tests/browser/repository-navigation.spec.ts
```

**Notes**

Cold isolated native-static fixture: all 3 flows pass in 5.7s; repository snapshots, responsive overflow/reload/keyboard, minimal rail and notifications retained. /tmp/readiness-browser-final.log.
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
pnpm spec:search -- "JST-UI-005"
```

**Notes**

Script forwards --, rejected by spec-validator. No repository change.
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
pnpm spec:search "JST-UI-005"
```

**Notes**

QMD native modules do not match Node ABI147. Used rg fallback and owning canonical requirement pages; no environment reinstall or indexed-source claim.
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
pnpm exec prettier --write $(jj diff --name-only | rg "\.(svelte|ts)$")
```

**Notes**

Local rg defaults prefix line numbers, producing invalid paths. No formatting occurred. Reran with Python subprocess passing explicit JJ path arguments.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 53
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook
```

**Notes**

Intermediate full Storybook audit: 704 pass, 53 fail, 159 families (135 pass/24 fail), 1041.76s. Escalated because shared view loading and story helpers span several product surfaces. Started before later readiness/style repairs; this is not a final current failure count or a controlled comparison with the earlier 639/118 run. No new shell-context exception reported; ResizeObserver diagnostics recur. /tmp/readiness-wide-after.log.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-appearance/StoreAppearance.stories.svelte catalog/components/store-application/StoreCaptureSettings.stories.svelte catalog/components/store-application/AgentSessionSettings.stories.svelte catalog/components/store-application/LocalCompute.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceSetup.stories.svelte
```

**Notes**

38 pass, 4 fail, 5 families. Corrected settings heading and shared header padding expectations later pass in affected acceptance. Native stack setup still queries its target control before mount and remains outside the repaired families. /tmp/readiness-settings-after.log.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-application/WorkspaceFiles.stories.svelte catalog/components/store-work/ProjectsFilters.stories.svelte
```

**Notes**

7 pass, 5 fail, 2 families. Two workspace startup queries race loading; toolbar divider is 0px where 1px is required, and opening Projects search shifts a header button by 0.796875px. Original geometry thresholds retained. /tmp/readiness-geometry-before.log.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-appearance/StoreAppearance.stories.svelte catalog/components/store-application/AgentLayout.stories.svelte catalog/components/store-application/AgentSessionSettings.stories.svelte catalog/components/store-application/AgentToolGroups.stories.svelte catalog/components/store-application/AgentTranscriptSettings.stories.svelte catalog/components/store-application/ConversationCaret.stories.svelte catalog/components/store-application/LocalCompute.stories.svelte catalog/components/store-application/NativeAgentMenus.stories.svelte catalog/components/store-application/NativeReviewDetail.stories.svelte catalog/components/store-application/RepositoryApplication.stories.svelte catalog/components/store-application/RepositoryLayout.stories.svelte catalog/components/store-application/RepositoryLoading.stories.svelte catalog/components/store-application/RepositoryNotices.stories.svelte catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-application/StoreCollaboration.stories.svelte catalog/components/store-application/StoreDirectoryNavigation.stories.svelte catalog/components/store-application/StoreEvolutionColumn.stories.svelte catalog/components/store-application/StoreFileEditor.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte catalog/components/store-application/StoreRepositoryBranchContext.stories.svelte catalog/components/store-application/StoreRepositoryFeedback.stories.svelte catalog/components/store-application/StoreRepositoryNavigation.stories.svelte catalog/components/store-application/StoreRepositorySearch.stories.svelte catalog/components/store-application/StoreWorkspaces.stories.svelte catalog/components/store-application/WorkspaceFiles.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceLoading.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceSettings.stories.svelte catalog/components/store-threads/StoreThreadAlignment.stories.svelte catalog/components/store-threads/StoreThreads.stories.svelte catalog/components/store-work/ProjectsActivity.stories.svelte catalog/components/store-work/ProjectsAgentActions.stories.svelte catalog/components/store-work/ProjectsFilters.stories.svelte catalog/components/store-work/ProjectsHub.stories.svelte catalog/components/store-work/ProjectsReviewDetail.stories.svelte catalog/components/store-application/AppearanceDiscussions.stories.svelte catalog/components/store-application/StoreCaptureSettings.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceConflictVersions.stories.svelte catalog/components/store-mutations/StoreConflictEditor.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceConflicts.stories.svelte
```

**Notes**

227 pass, 3 fail, 39 families, 197.12s. Remaining failures: NativeAgentMenus lazy header, StoreCollaboration transcript article mount, ThreadAlignment dismissal pointer lock. Those exact boundaries subsequently repaired; this run is retained rather than replaced by later results. /tmp/readiness-final-stories.log.
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
pnpm test:storybook catalog/components/store-application/NativeAgentMenus.stories.svelte catalog/components/store-application/StoreCollaboration.stories.svelte catalog/components/store-threads/StoreThreadAlignment.stories.svelte catalog/components/store-application/InboxControls.stories.svelte catalog/components/store-threads/StoreThreadSearch.stories.svelte
```

**Notes**

21 pass, 1 fail, 5 families. The sole remaining NativeAgentMenus failure moved from header readiness to its Classic tool-row assumption. Existing AgentLayoutFixture now selects Classic explicitly for that story; application default remains Document. /tmp/readiness-last-consumers.log.
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
pnpm test:storybook catalog/components/store-application/NativeAgentMenus.stories.svelte
```

**Notes**

Exact remaining NativeAgentMenus family: 1/1 passes, 19.48s. Clipboard identities, selection, native menu and hover geometry assertions retained. /tmp/readiness-native-menu-final.log.
<!-- /run -->
<!-- /evidence-field -->

