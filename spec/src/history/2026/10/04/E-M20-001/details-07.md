---
schema: 1
id: E-M20-001
---

# E-M20-001 details 7

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
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace -t 'IconToolbar|UnstagedHeader|LinkedBookmarks|BookmarkActionBars|WorkspaceDraggable'
```

**Notes**

Output: .local/stack-workspace-ui-evidence/latest-stories-third.log. Command completed. Interrupted to incorporate the requested body-header actions and sticky/resizable columns; no acceptance claim.
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
vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/stack-workspace.test.ts web/tests/vite/store-route-history.test.ts
```

**Notes**

Output: .local/stack-workspace-ui-evidence/latest-unit.log. Test Files  2 passed (2) | Tests  38 passed (38) 
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
svelte-check --tsconfig web/tsconfig.json
```

**Notes**

Output: .local/stack-workspace-ui-evidence/latest-web-first.log. svelte-check found 0 errors and 0 warnings 
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
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspace.stories.svelte -t 'Immutable|Narrow|Initial' --reporter=verbose
```

**Notes**

Output: .local/stack-workspace-ui-evidence/layout-diagnostic.log. Test Files  1 passed (1) | Tests  3 passed | 11 skipped (14) 
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
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspace.stories.svelte -t Narrow --reporter=verbose
```

**Notes**

Output: .local/stack-workspace-ui-evidence/narrow-diagnostic.log. Test Files  1 passed (1) | Tests  1 passed | 13 skipped (14) 
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
spec-validator validate --only ui-ownership
```

**Notes**

Output: .local/stack-workspace-ui-evidence/ownership-final-second.log. Specification validated: 1 validators, 23 chapters, 173 requirements. 
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
spec-validator validate --only ui-ownership
```

**Notes**

Output: .local/stack-workspace-ui-evidence/ownership-final.log. Specification validated: 1 validators, 23 chapters, 173 requirements. 
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
spec-validator validate --only ui-ownership
```

**Notes**

Output: .local/stack-workspace-ui-evidence/ownership.log. Specification validated: 1 validators, 23 chapters, 173 requirements. 
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
node scripts/check-progress.mjs
```

**Notes**

Output: .local/stack-workspace-ui-evidence/progress-first.log. Command completed. 
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
spec-validator validate
```

**Notes**

Output: .local/stack-workspace-ui-evidence/spec-final-second.log. Command completed. 
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
spec-validator validate
```

**Notes**

Output: .local/stack-workspace-ui-evidence/spec-final-third.log. Specification validated: 5 validators, 23 chapters, 173 requirements. 
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
spec-validator validate
```

**Notes**

Output: .local/stack-workspace-ui-evidence/spec-final.log. Command completed. 
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
spec-validator first
```

**Notes**

Output: .local/stack-workspace-ui-evidence/spec-first-final.log. Spec-first gate passed: 45 protected file(s), 4 canonical chapter(s). 
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
spec-validator first
```

**Notes**

Output: .local/stack-workspace-ui-evidence/spec-first.log. Spec-first gate passed: 28 protected file(s), 4 canonical chapter(s). 
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
spec-validator validate
```

**Notes**

Output: .local/stack-workspace-ui-evidence/spec-validate-first.log. Command completed. 
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
spec-validator validate
```

**Notes**

Output: .local/stack-workspace-ui-evidence/spec-validate-second.log. Command completed. 
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace
```

**Notes**

Output: .local/stack-workspace-ui-evidence/stories-body-first.log. ⎯⎯⎯⎯⎯⎯⎯ Failed Tests 5 ⎯⎯⎯⎯⎯⎯⎯ | Test Files  3 failed | 1 passed (4) | Tests  5 failed | 23 passed (28) 
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
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace -t 'BranchesApply|LocalFolder|SelectBranch|ExistingBranches|SplitReorder'
```

**Notes**

Output: .local/stack-workspace-ui-evidence/stories-body-targeted.log. Test Files  3 passed | 1 skipped (4) | Tests  5 passed | 23 skipped (28) 
<!-- /run -->
<!-- /evidence-field -->

