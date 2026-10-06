---
schema: 1
id: E-M20-001
---

# E-M20-001 details 14

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
vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/stack-workspace-branches.test.ts
```

**Notes**

Output: .local/stack-workspace-ui-evidence/inline-branches-unit-before.log. ⎯⎯⎯⎯⎯⎯⎯ Failed Tests 1 ⎯⎯⎯⎯⎯⎯⎯ | Test Files  1 failed (1) | Tests  1 failed (1) One failing regression reproduced the all-stack aggregate source instead of the selected branch segment.
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
pnpm exec prettier --write web/src/lib/controller/stack-workspace/commit-drafts.svelte.ts web/src/lib/controller/stack-workspace/controller.svelte.ts web/src/lib/controller/stack-workspace/drag-session.svelte.ts web/src/lib/controller/stack-workspace/drag.ts web/src/lib/controller/stack-workspace/file-drag.ts web/src/lib/controller/stack-workspace/files.ts web/src/lib/controller/stack-workspace/presentation.ts web/src/lib/controller/stack-workspace/types.ts web/src/lib/ui/store-stack-workspace/stack-workspace.css web/src/lib/ui/store-stack-workspace/StoreStackBookmarkActions.svelte web/src/lib/ui/store-stack-workspace/StoreStackBookmarkCard.svelte web/src/lib/ui/store-stack-workspace/StoreStackCommitActions.svelte web/src/lib/ui/store-stack-workspace/StoreStackCommitComposer.svelte web/src/lib/ui/store-stack-workspace/StoreStackCopyMenu.svelte web/src/lib/ui/store-stack-workspace/StoreStackDiffHunk.svelte web/src/lib/ui/store-stack-workspace/StoreStackDropGuide.svelte web/src/lib/ui/store-stack-workspace/StoreStackRelativeActions.svelte web/src/lib/ui/store-stack-workspace/StoreStackStagedFiles.svelte web/src/lib/ui/store-stack-workspace/StoreStackUnstaged.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspace.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceActions.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceCanvas.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceChoice.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceConflict.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceDiff.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceForm.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspacePicker.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspacePreview.svelte web/tests/vite/stack-workspace-branches.test.ts web/tests/vite/stack-workspace.test.ts spec/src/ui.md
```

**Notes**

Output: .local/stack-workspace-ui-evidence/inline-format.log. Command completed; formatter output or installation completion retained.
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
vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/stack-workspace-branches.test.ts web/tests/vite/stack-workspace-menus.test.ts web/tests/vite/stack-workspace.test.ts web/tests/vite/store-route-history.test.ts
```

**Notes**

Output: .local/stack-workspace-ui-evidence/inline-unit-final.log. Test Files  4 passed (4) | Tests  57 passed (57) 57/57 controller, menu, branch-source and history tests passed across four files.
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
pnpm exec prettier --write catalog/components/store-stack-workspace/StoreStackWorkspaceInline.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspacePresentation.stories.svelte docs/ui-provenance.json
```

**Notes**

Output: .local/stack-workspace-ui-evidence/inline-format2.log. Command completed; formatter output or installation completion retained.
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
pnpm exec prettier --write catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspacePresentation.stories.svelte docs/ui-provenance.json
```

**Notes**

Output: .local/stack-workspace-ui-evidence/inline-format3.log. Command completed; formatter output or installation completion retained.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 8
hasNote: true
noteEncoding: markdown

```

```shell
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace --testNamePattern 'BranchFilesStayInsideTheirOwnCard|StartCommitBecomesAStagedStackCard|WholeStackHeadersAndSelectedCommits|WholeStackFilesAndContinuousGraph|InlineCommitWithStagedFilesAndHunks|EditCommitInInlineComposer|FolderDrops' --reporter=verbose
```

**Notes**

Output: .local/stack-workspace-ui-evidence/inline-stories-first.log. Test Files  8 failed (8) | Tests  no tests Eight test-file import failures and no executed tests from a temporarily malformed Lane edit. Reconstructed the Lane; exact seven-story rerun passed.
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
pnpm exec prettier --write web/src/lib/ui/store-stack-workspace/StoreStackLane.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspacePresentation.stories.svelte docs/ui-provenance.json
```

**Notes**

Output: .local/stack-workspace-ui-evidence/inline-format4.log. Command completed; formatter output or installation completion retained.
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
node scripts/web-architecture.mjs
```

**Notes**

Output: .local/stack-workspace-ui-evidence/inline-architecture-final.log. Command completed; formatter output or installation completion retained. CSS exceeded 300 counted lines at 303; consolidated duplicate rules.
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
svelte-check --tsconfig web/tsconfig.json
```

**Notes**

Output: .local/stack-workspace-ui-evidence/inline-web-final.log. svelte-check found 0 errors and 0 warnings
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
svelte-check --tsconfig catalog/tsconfig.json --fail-on-warnings && node scripts/check-catalog.mjs
```

**Notes**

Output: .local/stack-workspace-ui-evidence/inline-catalog-final.log. svelte-check found 0 errors and 0 warnings | catalog boundaries ok (209 modules)
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
pnpm exec prettier --write web/src/lib/ui/store-stack-workspace/stack-workspace.css
```

**Notes**

Output: .local/stack-workspace-ui-evidence/inline-css-format.log. Command completed; formatter output or installation completion retained.
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
node scripts/web-architecture.mjs
```

**Notes**

Output: .local/stack-workspace-ui-evidence/inline-architecture-pass.log. Command completed; formatter output or installation completion retained. CSS still exceeded the same budget at 301; removed an unused rule. inline-architecture-last.log passed at the unchanged budget.
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
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace --testNamePattern 'BranchFilesStayInsideTheirOwnCard|StartCommitBecomesAStagedStackCard|WholeStackHeadersAndSelectedCommits|WholeStackFilesAndContinuousGraph|InlineCommitWithStagedFilesAndHunks|EditCommitInInlineComposer|FolderDrops' --reporter=verbose
```

**Notes**

Output: .local/stack-workspace-ui-evidence/inline-stories-second.log. Test Files  6 passed | 2 skipped (8) | Tests  7 passed | 43 skipped (50) Seven exact branch-inline/composer/edit/file-drop scenarios passed; measured two idle sections, staged header/list/form order and unchanged Unstaged files before apply.
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
pnpm exec prettier --write web/src/lib/ui/store-stack-workspace/stack-workspace.css
```

**Notes**

Output: .local/stack-workspace-ui-evidence/inline-css-format2.log. Command completed; formatter output or installation completion retained.
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
spec-validator validate --only ui-ownership
```

**Notes**

Output: .local/stack-workspace-ui-evidence/inline-ownership-final.log. Specification validated: 1 validators, 23 chapters, 173 requirements.
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
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace catalog/components/community-projects/CommunityProjectsRevisionGraph.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLogFiles.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLogDiff.stories.svelte catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.stories.svelte catalog/components/store-application/StoreRepositoryPicker.stories.svelte catalog/components/store-application/StoreRepositoryNavigation.stories.svelte catalog/components/store-application/StoreRepositoryBranchContext.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte --reporter=verbose
```

**Notes**

Output: .local/stack-workspace-ui-evidence/inline-direct-consumers-final.log. Test Files  17 passed (17) | Tests  90 passed (90) All 90 directly affected interaction/accessibility stories passed across 17 files in a fresh Vitest browser process. No ResizeObserver notice in this final consumer log.
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
pnpm checks (Design Core)
```

**Notes**

Output: .local/stack-workspace-ui-evidence/source/design-core-stack-drag-checks-final.log. 650/651 stories passed; unchanged Complete CV InteractiveSampleCV timed out at 30s. Source exact unchanged rerun passed before the complete final run.
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
pnpm test:storybook src/shared/forms/complete-cv-form/CompleteCvForm.stories.svelte --testNamePattern InteractiveSampleCV --reporter=verbose (Design Core)
```

**Notes**

Output: .local/stack-workspace-ui-evidence/source/design-core-stack-cv-exact.log. One unchanged exact CV scenario passed in 12.4s; original source timeout is recorded separately.
<!-- /run -->
<!-- /evidence-field -->

