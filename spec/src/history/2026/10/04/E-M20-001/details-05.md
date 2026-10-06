---
schema: 1
id: E-M20-001
---

# E-M20-001 details 5

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
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceCommands.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte -t 'InlineCommit|MultipleAdjacent'
```

**Notes**

Output: .local/stack-workspace-ui-evidence/composer-stories-exact.log. Test Files  2 passed (2) | Tests  2 passed | 12 skipped (14) 
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceCommands.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspacePresentation.stories.svelte
```

**Notes**

Output: .local/stack-workspace-ui-evidence/composer-stories-first.log. ⎯⎯⎯⎯⎯⎯⎯ Failed Tests 4 ⎯⎯⎯⎯⎯⎯⎯ | Test Files  2 failed (2) | Tests  4 failed | 6 passed (10) 
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceCommands.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspacePresentation.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte
```

**Notes**

Output: .local/stack-workspace-ui-evidence/composer-stories-second.log. ⎯⎯⎯⎯⎯⎯⎯ Failed Tests 2 ⎯⎯⎯⎯⎯⎯⎯ | Test Files  2 failed | 1 passed (3) | Tests  2 failed | 16 passed (18) 
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
vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/stack-workspace.test.ts web/tests/vite/store-route-history.test.ts
```

**Notes**

Output: .local/stack-workspace-ui-evidence/composer-unit.log. Test Files  2 passed (2) | Tests  43 passed (43) 
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
vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/stack-workspace.test.ts
```

**Notes**

Output: .local/stack-workspace-ui-evidence/controller-final.log. Test Files  1 passed (1) | Tests  20 passed (20) 
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
vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/stack-workspace.test.ts
```

**Notes**

Output: .local/stack-workspace-ui-evidence/controller-second.log. Test Files  1 passed (1) | Tests  19 passed (19) 
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
vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/stack-workspace.test.ts -t 'independent diff|obsolete diffs|available selected hunks'
```

**Notes**

Output: .local/stack-workspace-ui-evidence/diffs-after-second.log. Test Files  1 passed (1) | Tests  3 passed | 20 skipped (23) 
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
vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/stack-workspace.test.ts -t 'independent diff|obsolete diffs|available selected hunks'
```

**Notes**

Output: .local/stack-workspace-ui-evidence/diffs-after.log. ⎯⎯⎯⎯⎯⎯⎯ Failed Tests 2 ⎯⎯⎯⎯⎯⎯⎯ | Test Files  1 failed (1) | Tests  2 failed | 1 passed | 20 skipped (23) Two remaining origin/hunk reconciliation failures, repaired before the exact rerun.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/stack-workspace.test.ts -t 'independent diff|obsolete diffs|available selected hunks'
```

**Notes**

Output: .local/stack-workspace-ui-evidence/diffs-before.log. ⎯⎯⎯⎯⎯⎯⎯ Failed Tests 3 ⎯⎯⎯⎯⎯⎯⎯ | Test Files  1 failed (1) | Tests  3 failed | 20 skipped (23) Three failing regressions reproduced replacement of independent diff origins.
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
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte -t 'MultipleAdjacent|StackHeader|WholeStack'
```

**Notes**

Output: .local/stack-workspace-ui-evidence/diffs-stories-fifth.log. Test Files  1 passed (1) | Tests  3 passed | 5 skipped (8) 
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
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte -t 'MultipleAdjacent|WorkspaceInAppShell'
```

**Notes**

Output: .local/stack-workspace-ui-evidence/diffs-stories-first.log. ⎯⎯⎯⎯⎯⎯⎯ Failed Tests 1 ⎯⎯⎯⎯⎯⎯⎯ | Test Files  1 failed (1) | Tests  1 failed | 1 passed | 4 skipped (6) 
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte -t 'MultipleAdjacent|StackHeader|WholeStack'
```

**Notes**

Output: .local/stack-workspace-ui-evidence/diffs-stories-fourth.log. ⎯⎯⎯⎯⎯⎯⎯ Failed Tests 2 ⎯⎯⎯⎯⎯⎯⎯ | Test Files  1 failed (1) | Tests  2 failed | 1 passed | 5 skipped (8) 
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
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte -t 'MultipleAdjacent|WorkspaceInAppShell|StackHeader'
```

**Notes**

Output: .local/stack-workspace-ui-evidence/diffs-stories-second.log. ⎯⎯⎯⎯⎯⎯⎯ Failed Tests 2 ⎯⎯⎯⎯⎯⎯⎯ | Test Files  1 failed (1) | Tests  2 failed | 1 passed | 4 skipped (7) 
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte -t 'MultipleAdjacent|StackHeader|WholeStack'
```

**Notes**

Output: .local/stack-workspace-ui-evidence/diffs-stories-third.log. ⎯⎯⎯⎯⎯⎯⎯ Failed Tests 3 ⎯⎯⎯⎯⎯⎯⎯ | Test Files  1 failed (1) | Tests  3 failed | 5 skipped (8) 
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
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceCommands.stories.svelte -t Drag
```

**Notes**

Output: .local/stack-workspace-ui-evidence/drag-first.log. ⎯⎯⎯⎯⎯⎯⎯ Failed Tests 1 ⎯⎯⎯⎯⎯⎯⎯ | Test Files  1 failed (1) | Tests  1 failed | 3 skipped (4) 
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
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceCommands.stories.svelte -t Drag
```

**Notes**

Output: .local/stack-workspace-ui-evidence/drag-second.log. Test Files  1 passed (1) | Tests  1 passed | 3 skipped (4) 
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
node scripts/web-architecture.mjs
```

**Notes**

Output: .local/stack-workspace-ui-evidence/final-architecture.log. web architecture ok (996 modules) 
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
svelte-check --tsconfig catalog/tsconfig.json --fail-on-warnings && node scripts/check-catalog.mjs
```

**Notes**

Output: .local/stack-workspace-ui-evidence/final-catalog.log. svelte-check found 0 errors and 0 warnings | catalog boundaries ok (204 modules) 
<!-- /run -->
<!-- /evidence-field -->

