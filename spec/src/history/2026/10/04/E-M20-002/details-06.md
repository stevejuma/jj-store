---
schema: 1
id: E-M20-002
---

# E-M20-002 details 6

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
node scripts/web-architecture.mjs
```

**Notes**

Output: .local/stack-header-drops-evidence/architecture-layout-scoped.log. web architecture ok (1014 modules) 
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
spec-validator validate --only ui-ownership
```

**Notes**

Output: .local/stack-header-drops-evidence/ownership-scoped-final.log. Specification validated: 1 validators, 23 chapters, 173 requirements. 
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
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace catalog/components/community-projects/CommunityProjectsRevisionGraph.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLogFiles.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLogDiff.stories.svelte catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.stories.svelte catalog/components/store-application/StoreRepositoryPicker.stories.svelte catalog/components/store-application/StoreRepositoryNavigation.stories.svelte catalog/components/store-application/StoreRepositoryBranchContext.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte --reporter=verbose
```

**Notes**

Output: .local/stack-header-drops-evidence/all-stories-layout-qualified.log. Test Files  20 passed (20) | Tests  107 passed (107) Complete directly affected interaction/accessibility lane after the shared token fix and fresh Storybook restart, not the full repository suite.
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
pnpm exec prettier --check affected changed files
```

**Notes**

Output: .local/stack-header-drops-evidence/new-files-format-check.log. All matched files use Prettier code style! 
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
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte -t WorkspaceInAppShell --reporter=verbose
```

**Notes**

Output: .local/stack-header-drops-evidence/footer-before.log. Test Files  1 failed (1) | Tests  1 failed | 7 skipped (8) The app-shell regression found the old Folder:/Base: prefixes and combined workspace text.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte -t WorkspaceInAppShell --reporter=verbose
```

**Notes**

Output: .local/stack-header-drops-evidence/footer-after.log. Test Files  1 failed (1) | Tests  no tests A newly imported pair of count icons triggered a mid-test Vite optimizer reload and one failed import suite. Reused existing GitBranchPlus/File icon paths already present in the app and its optimizer list; no test or runtime configuration change.
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
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte -t WorkspaceInAppShell --reporter=verbose
```

**Notes**

Output: .local/stack-header-drops-evidence/footer-qualified.log. Test Files  1 passed (1) | Tests  1 passed | 7 skipped (8) App-shell footer interaction/accessibility passes using the existing subtle CommunityChip with Folder, Project, Git branch and count icons; both redundant prefixes removed.
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
svelte-check --tsconfig web/tsconfig.json
```

**Notes**

Output: .local/stack-header-drops-evidence/web-footer-final.log. svelte-check found 0 errors and 0 warnings 
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
svelte-check --tsconfig catalog/tsconfig.json --fail-on-warnings && node scripts/check-catalog.mjs
```

**Notes**

Output: .local/stack-header-drops-evidence/catalog-footer-final.log. svelte-check found 0 errors and 0 warnings | catalog boundaries ok (212 modules) 
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
node scripts/web-architecture.mjs
```

**Notes**

Output: .local/stack-header-drops-evidence/architecture-footer-final.log. web architecture ok (1014 modules) 
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
pnpm exec prettier --check affected changed files
```

**Notes**

Output: .local/stack-header-drops-evidence/format-all-ui-final.log. All matched files use Prettier code style! 
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
pnpm test:storybook src/shared/diff/file-change-stats/FileChangeStats.stories.svelte -t DarkCountsRetainReadableContrast --reporter=verbose
```

**Notes**

Output: /Users/stevejuma/code/lapismd/design-core/.local/diff-count-contrast/before.log. One story failed with two insufficient-contrast count tones before the owning source token correction.
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
pnpm test:storybook src/shared/diff/file-change-stats/FileChangeStats.stories.svelte -t DarkCountsRetainReadableContrast --reporter=verbose
```

**Notes**

Output: /Users/stevejuma/code/lapismd/design-core/.local/diff-count-contrast/after.log. Exact regression passed: 1 story, 2 skipped.
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
pnpm test:storybook src/shared/diff/file-change-stats/FileChangeStats.stories.svelte --reporter=verbose
```

**Notes**

Output: /Users/stevejuma/code/lapismd/design-core/.local/diff-count-contrast/all-count-stories.log. All 3 owning File Change Stats scenarios passed.
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
pnpm spec:first
```

**Notes**

Output: /Users/stevejuma/code/lapismd/design-core/.local/diff-count-contrast/spec-first.log. 2 protected files and 2 canonical chapters passed in Design Core.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: fail
failures: 47
hasNote: true
noteEncoding: markdown

```

```shell
pnpm checks
```

**Notes**

Output: /Users/stevejuma/code/lapismd/design-core/.local/diff-count-contrast/checks.log. Source units 715/715 passed; 47 Storybook import suites failed with a setup SyntaxError Unexpected token /; 492 tests passed. Fresh retry cleared the setup failure.
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
pnpm checks
```

**Notes**

Output: /Users/stevejuma/code/lapismd/design-core/.local/diff-count-contrast/checks-retry.log. Source units 715 and stories 652 passed; workspace pointer lane had one cold readiness timeout and 13 passes.
<!-- /run -->

<!-- run -->
### Run 18

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

Output: /Users/stevejuma/code/lapismd/design-core/.local/diff-count-contrast/workspace-ready-exact.log. Started while the original runner server was shutting down; exact cold test failed readiness with connection loss/incomplete resources.
<!-- /run -->
<!-- /evidence-field -->

