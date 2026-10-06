---
schema: 1
id: E-M20-001
---

# E-M20-001 details 3

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

Output: .local/stack-workspace-ui-evidence/architecture-final-second.log. web architecture ok (992 modules) 
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
node scripts/web-architecture.mjs
```

**Notes**

Output: .local/stack-workspace-ui-evidence/architecture-final.log. Command completed. Owned revision graph exceeded 300-line budget; extracted its SVG composition locally with matching provenance.
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
node scripts/web-architecture.mjs
```

**Notes**

Output: .local/stack-workspace-ui-evidence/architecture-shell-first.log. web architecture ok (990 modules) 
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
node scripts/web-architecture.mjs
```

**Notes**

Output: .local/stack-workspace-ui-evidence/architecture.log. web architecture ok (984 modules) 
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
svelte-check --tsconfig web/tsconfig.json
```

**Notes**

Output: .local/stack-workspace-ui-evidence/before-web-check.log. svelte-check found 0 errors and 0 warnings 
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
svelte-check --tsconfig catalog/tsconfig.json --fail-on-warnings && node scripts/check-catalog.mjs
```

**Notes**

Output: .local/stack-workspace-ui-evidence/catalog-final.log. svelte-check found 0 errors and 0 warnings | catalog boundaries ok (203 modules) 
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 47
hasNote: true
noteEncoding: markdown

```

```shell
svelte-check --tsconfig catalog/tsconfig.json --fail-on-warnings && node scripts/check-catalog.mjs
```

**Notes**

Output: .local/stack-workspace-ui-evidence/catalog-first.log. svelte-check found 47 errors and 0 warnings in 4 files 
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
svelte-check --tsconfig catalog/tsconfig.json --fail-on-warnings && node scripts/check-catalog.mjs
```

**Notes**

Output: .local/stack-workspace-ui-evidence/catalog-second.log. svelte-check found 0 errors and 0 warnings | catalog boundaries ok (200 modules) 
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
svelte-check --tsconfig catalog/tsconfig.json --fail-on-warnings && node scripts/check-catalog.mjs
```

**Notes**

Output: .local/stack-workspace-ui-evidence/catalog-shell-first.log. Command completed. Wrong fixture import, repaired before app-shell acceptance.
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

Output: .local/stack-workspace-ui-evidence/catalog-shell-second.log. svelte-check found 0 errors and 0 warnings | catalog boundaries ok (203 modules) 
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
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspace.stories.svelte -t 'Combined working files'
```

**Notes**

Output: .local/stack-workspace-ui-evidence/combined-first.log. Test Files  1 skipped (1) | Tests  14 skipped (14) 
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
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspace.stories.svelte -t Combined
```

**Notes**

Output: .local/stack-workspace-ui-evidence/combined-second.log. Test Files  1 passed (1) | Tests  1 passed | 13 skipped (14) 
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
node scripts/web-architecture.mjs
```

**Notes**

Output: .local/stack-workspace-ui-evidence/composer-architecture.log. web architecture ok (998 modules) 
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
vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/stack-workspace.test.ts
```

**Notes**

Output: .local/stack-workspace-ui-evidence/composer-before-unit.log. Test Files  1 passed (1) | Tests  26 passed (26) 
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
svelte-check --tsconfig web/tsconfig.json
```

**Notes**

Output: .local/stack-workspace-ui-evidence/composer-before-web.log. svelte-check found 0 errors and 0 warnings 
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
svelte-check --tsconfig catalog/tsconfig.json --fail-on-warnings && node scripts/check-catalog.mjs
```

**Notes**

Output: .local/stack-workspace-ui-evidence/composer-catalog.log. svelte-check found 0 errors and 0 warnings | catalog boundaries ok (204 modules) 
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
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace catalog/components/community-projects/CommunityProjectsRevisionGraph.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLogFiles.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLogDiff.stories.svelte catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.stories.svelte catalog/components/store-application/StoreRepositoryPicker.stories.svelte catalog/components/store-application/StoreRepositoryNavigation.stories.svelte catalog/components/store-application/StoreRepositoryBranchContext.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte
```

**Notes**

Output: .local/stack-workspace-ui-evidence/composer-direct-consumers-final.log. Test Files  14 passed (14) | Tests  77 passed (77) 77/77 stories passed; one ResizeObserver loop diagnostic remained visible. Follow-up verbose run isolates this diagnostic; no suppression.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace catalog/components/community-projects/CommunityProjectsRevisionGraph.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLogFiles.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLogDiff.stories.svelte catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.stories.svelte catalog/components/store-application/StoreRepositoryPicker.stories.svelte catalog/components/store-application/StoreRepositoryNavigation.stories.svelte catalog/components/store-application/StoreRepositoryBranchContext.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte
```

**Notes**

Output: .local/stack-workspace-ui-evidence/composer-direct-consumers.log. ⎯⎯⎯⎯⎯⎯⎯ Failed Tests 4 ⎯⎯⎯⎯⎯⎯⎯ | Test Files  4 failed | 10 passed (14) | Tests  4 failed | 73 passed (77) 73/77 passed; repaired two compact-list virtualization regressions and two obsolete composer assertions. Exact repaired regressions passed separately.
<!-- /run -->
<!-- /evidence-field -->

