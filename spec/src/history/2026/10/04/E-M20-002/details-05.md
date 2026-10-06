---
schema: 1
id: E-M20-002
---

# E-M20-002 details 5

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
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceMenus.stories.svelte -t AssignSelectedChangesWithSettingsRowsAndBranchCommandPicker --reporter=verbose
```

**Notes**

Output: .local/stack-header-drops-evidence/assignment-rows-before.log. Test Files  1 failed (1) | Tests  1 failed | 6 skipped (7) Destination was a button instead of the Command View combobox.
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
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceMenus.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceRefinements.stories.svelte -t 'AssignSelectedChangesWithSettingsRowsAndBranchCommandPicker|CreateIndependentStack|CreateDependentStackWithCommandPicker|ContentPaddingSeparatesComponentsFromEdgeScrollbars' --reporter=verbose
```

**Notes**

Output: .local/stack-header-drops-evidence/padding-and-settings-rows-after.log. Test Files  2 passed (2) | Tests  4 passed | 10 skipped (14) 
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
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceRefinements.stories.svelte -t 'KebabMenusHaveDistinctHoverAndAlignedBranchControls|ContentPaddingSeparatesComponentsFromEdgeScrollbars' --reporter=verbose
```

**Notes**

Output: .local/stack-header-drops-evidence/balanced-padding-and-kebab-before.log. Test Files  1 failed (1) | Tests  2 failed | 6 skipped (8) Right card inset was 16px versus 8px left; branch kebab was offset 4px.
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
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceRefinements.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceMenus.stories.svelte -t 'KebabMenusHaveDistinctHoverAndAlignedBranchControls|ContentPaddingSeparatesComponentsFromEdgeScrollbars|AssignSelectedChangesWithSettingsRowsAndBranchCommandPicker|CreateIndependentStack|CreateDependentStackWithCommandPicker' --reporter=verbose
```

**Notes**

Output: .local/stack-header-drops-evidence/balanced-padding-kebab-form-after.log. Test Files  1 failed | 1 passed (2) | Tests  1 failed | 4 passed | 10 skipped (15) Alignment and forms passed; first hover token override gave only 7 RGB-channel contrast.
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
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceRefinements.stories.svelte -t KebabMenusHaveDistinctHoverAndAlignedBranchControls --reporter=verbose
```

**Notes**

Output: .local/stack-header-drops-evidence/kebab-hover-diagnostics.log. Test Files  1 failed (1) | Tests  1 failed | 7 skipped (8) Temporary diagnostics confirmed the public Button hover reads accent after muted. Diagnostics removed.
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
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceRefinements.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceForms.stories.svelte -t 'KebabMenusHaveDistinctHoverAndAlignedBranchControls|ContentPaddingSeparatesComponentsFromEdgeScrollbars|AssignSelectedChangesWithSettingsRowsAndBranchCommandPicker|CreateIndependentStack|CreateDependentStackWithCommandPicker' --reporter=verbose
```

**Notes**

Output: .local/stack-header-drops-evidence/layout-kebab-form-qualified.log. Test Files  2 passed (2) | Tests  5 passed | 6 skipped (11) 
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
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceRefinements.stories.svelte -t 'KebabMenusHaveDistinctHoverAndAlignedBranchControls|DarkKebabMenusHaveDistinctHoverAndAlignedBranchControls' --reporter=verbose
```

**Notes**

Output: .local/stack-header-drops-evidence/kebab-light-dark-qualified.log. Test Files  1 failed (1) | Tests  1 failed | 1 passed | 7 skipped (9) Light hover passed; dark hover assertions passed but axe found two shared File Change Stats tone violations. Fixed in Design Core owning source.
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

Output: .local/stack-header-drops-evidence/web-padding-form-check.log. svelte-check found 0 errors and 0 warnings 
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

Output: .local/stack-header-drops-evidence/catalog-padding-form-kebab.log. svelte-check found 0 errors and 0 warnings | catalog boundaries ok (211 modules) 
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
node scripts/web-architecture.mjs
```

**Notes**

Output: .local/stack-header-drops-evidence/architecture-padding-form-kebab.log.  Two line-budget violations (402-line Menus and 313-line CSS) fixed by separating owned Forms stories and layout CSS; limits unchanged.
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
node scripts/web-architecture.mjs
```

**Notes**

Output: .local/stack-header-drops-evidence/architecture-layout-form-final.log. web architecture ok (1014 modules) 
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
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceRefinements.stories.svelte -t 'Content padding|Stack columns omit' --reporter=verbose
```

**Notes**

Output: .local/stack-header-drops-evidence/workspace-scroll-scope-before.log. Test Files  1 skipped (1) | Tests  10 skipped (10) Human-spaced name filter selected no tests (10 skipped). Not acceptance evidence; rerun used actual CamelCase test names.
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
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceRefinements.stories.svelte -t 'ContentPadding|StackColumnsOmit' --reporter=verbose
```

**Notes**

Output: .local/stack-header-drops-evidence/workspace-scroll-scope-before-exact.log. Test Files  1 failed (1) | Tests  2 failed | 8 skipped (10) Two regressions failed: Branches viewport retained 8px extra reserve; redundant Applied metadata still rendered.
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
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceRefinements.stories.svelte -t 'ContentPadding|StackColumnsOmit' --reporter=verbose
```

**Notes**

Output: .local/stack-header-drops-evidence/workspace-scroll-scope-after.log. Test Files  1 passed (1) | Tests  2 passed | 8 skipped (10) 
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
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceRefinements.stories.svelte -t 'ContentPadding|StackColumnsOmit' --reporter=verbose
```

**Notes**

Output: .local/stack-header-drops-evidence/workspace-scroll-scope-all-columns.log. Test Files  1 passed (1) | Tests  2 passed | 8 skipped (10) Strengthened geometry includes an adjacent stack diff; Branches picker, selected stack and diff retain zero Workspace-only reserve.
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
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceRefinements.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceForms.stories.svelte -t 'ContentPadding|StackColumnsOmit|Kebab|Settings' --reporter=verbose
```

**Notes**

Output: .local/stack-header-drops-evidence/layout-source-contrast-final.log. Test Files  2 passed (2) | Tests  5 passed | 8 skipped (13) Five focused scenarios passed, including assignment Command View, padding, metadata removal and light/dark kebab contrast. The two other creation stories are covered by the complete affected lane.
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
svelte-check --tsconfig web/tsconfig.json
```

**Notes**

Output: .local/stack-header-drops-evidence/web-layout-final.log. svelte-check found 0 errors and 0 warnings 
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

Output: .local/stack-header-drops-evidence/catalog-layout-final.log. svelte-check found 0 errors and 0 warnings | catalog boundaries ok (212 modules) 
<!-- /run -->
<!-- /evidence-field -->

