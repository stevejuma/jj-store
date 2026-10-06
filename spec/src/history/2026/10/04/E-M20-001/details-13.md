---
schema: 1
id: E-M20-001
---

# E-M20-001 details 13

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
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspace.stories.svelte -t 'NativeCheckout|ImmutableCommit'
```

**Notes**

Output: .local/stack-workspace-ui-evidence/drag-fixture-regressions-final.log. Test Files  1 passed (1) | Tests  2 passed | 12 skipped (14)
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 9
hasNote: true
noteEncoding: markdown

```

```shell
vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/stack-workspace-menus.test.ts
```

**Notes**

Output: .local/stack-workspace-ui-evidence/menus-unit-before.log. ⎯⎯⎯⎯⎯⎯⎯ Failed Tests 9 ⎯⎯⎯⎯⎯⎯⎯ | Test Files  1 failed (1) | Tests  9 failed (9) All nine new creation/positional/edit/delete contracts failed before implementation.
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
pnpm exec prettier --write web/src/lib/controller/stack-workspace/commit-drafts.svelte.ts web/src/lib/controller/stack-workspace/controller.svelte.ts web/src/lib/controller/stack-workspace/drag-session.svelte.ts web/src/lib/controller/stack-workspace/drag.ts web/src/lib/controller/stack-workspace/file-drag.ts web/src/lib/controller/stack-workspace/files.ts web/src/lib/controller/stack-workspace/presentation.ts web/src/lib/controller/stack-workspace/types.ts web/src/lib/ui/store-stack-workspace/stack-workspace.css web/src/lib/ui/store-stack-workspace/StoreStackBookmarkActions.svelte web/src/lib/ui/store-stack-workspace/StoreStackBookmarkCard.svelte web/src/lib/ui/store-stack-workspace/StoreStackCommitActions.svelte web/src/lib/ui/store-stack-workspace/StoreStackCommitComposer.svelte web/src/lib/ui/store-stack-workspace/StoreStackCopyMenu.svelte web/src/lib/ui/store-stack-workspace/StoreStackDiffHunk.svelte web/src/lib/ui/store-stack-workspace/StoreStackDropGuide.svelte web/src/lib/ui/store-stack-workspace/StoreStackLane.svelte web/src/lib/ui/store-stack-workspace/StoreStackRelativeActions.svelte web/src/lib/ui/store-stack-workspace/StoreStackUnstaged.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspace.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceActions.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceCanvas.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceChoice.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceConflict.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceDiff.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceForm.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspacePicker.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspacePreview.svelte catalog/fixtures/stack-workspace/adapter.ts catalog/fixtures/stack-workspace/changes.ts catalog/fixtures/stack-workspace/data.ts catalog/fixtures/stack-workspace/menu-reducer.ts catalog/fixtures/stack-workspace/menu-validation.ts catalog/fixtures/stack-workspace/preview.ts catalog/fixtures/stack-workspace/reducer.ts catalog/fixtures/stack-workspace/story-actions.ts catalog/fixtures/stack-workspace/validation.ts web/tests/vite/stack-workspace-menus.test.ts spec/src/ui.md spec/src/api.md
```

**Notes**

Output: .local/stack-workspace-ui-evidence/menus-format.log. Command completed; formatter output or installation completion retained.
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
node scripts/web-architecture.mjs
```

**Notes**

Output: .local/stack-workspace-ui-evidence/menus-architecture-first.log. Command completed; formatter output or installation completion retained. Controller exceeded 300 lines at 308; moved draft cleanup into the existing draft owner.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/stack-workspace-menus.test.ts web/tests/vite/stack-workspace.test.ts web/tests/vite/store-route-history.test.ts
```

**Notes**

Output: .local/stack-workspace-ui-evidence/menus-unit-after.log. ⎯⎯⎯⎯⎯⎯⎯ Failed Tests 3 ⎯⎯⎯⎯⎯⎯⎯ | Test Files  1 failed | 2 passed (3) | Tests  3 failed | 53 passed (56) 53/56 passed; three branch-boundary regressions exposed equal-head history duplication. Fixed grouping and deletion retargeting; 56/56 exact rerun passed.
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
pnpm exec prettier --write catalog/components/store-stack-workspace/StoreStackWorkspaceMenus.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspace.stories.svelte web/src/lib/controller/stack-workspace/commit-drafts.svelte.ts web/src/lib/controller/stack-workspace/controller.svelte.ts web/src/lib/controller/stack-workspace/drag-session.svelte.ts web/src/lib/controller/stack-workspace/drag.ts web/src/lib/controller/stack-workspace/file-drag.ts web/src/lib/controller/stack-workspace/files.ts web/src/lib/controller/stack-workspace/presentation.ts web/src/lib/controller/stack-workspace/types.ts
```

**Notes**

Output: .local/stack-workspace-ui-evidence/menus-format2.log. Command completed; formatter output or installation completion retained.
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
svelte-check --tsconfig web/tsconfig.json
```

**Notes**

Output: .local/stack-workspace-ui-evidence/menus-web-first.log. Command completed; formatter output or installation completion retained. One TypeScript narrowing error in edit-draft lookup, repaired before zero-diagnostic web checks.
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
pnpm exec prettier --write web/src/lib/controller/stack-workspace/presentation.ts web/src/lib/ui/store-stack-workspace/StoreStackCommitComposer.svelte web/src/lib/ui/store-stack-workspace/StoreStackRelativeActions.svelte web/src/lib/ui/store-stack-workspace/StoreStackBookmarkActions.svelte web/src/lib/ui/store-stack-workspace/StoreStackBookmarkCard.svelte
```

**Notes**

Output: .local/stack-workspace-ui-evidence/menus-format3.log. Command completed; formatter output or installation completion retained.
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
vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/stack-workspace-menus.test.ts web/tests/vite/stack-workspace.test.ts web/tests/vite/store-route-history.test.ts
```

**Notes**

Output: .local/stack-workspace-ui-evidence/menus-unit-final.log. Test Files  3 passed (3) | Tests  56 passed (56)
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

Output: .local/stack-workspace-ui-evidence/menus-architecture-final.log. web architecture ok (1007 modules)
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
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceMenus.stories.svelte --reporter=verbose
```

**Notes**

Output: .local/stack-workspace-ui-evidence/menus-stories-first.log. ⎯⎯⎯⎯⎯⎯⎯ Failed Tests 1 ⎯⎯⎯⎯⎯⎯⎯ | Test Files  1 failed (1) | Tests  1 failed | 5 passed (6) 5/6 passed; edit story queried the composer beneath its own root. Corrected to assert the actual composer class and retained content assertions.
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
svelte-check --tsconfig web/tsconfig.json
```

**Notes**

Output: .local/stack-workspace-ui-evidence/menus-web-final.log. svelte-check found 0 errors and 0 warnings
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
svelte-check --tsconfig catalog/tsconfig.json --fail-on-warnings && node scripts/check-catalog.mjs
```

**Notes**

Output: .local/stack-workspace-ui-evidence/menus-catalog-final.log. svelte-check found 0 errors and 0 warnings | catalog boundaries ok (208 modules)
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
pnpm exec prettier --write catalog/components/store-stack-workspace/StoreStackWorkspaceMenus.stories.svelte
```

**Notes**

Output: .local/stack-workspace-ui-evidence/menus-format4.log. Command completed; formatter output or installation completion retained.
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
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceMenus.stories.svelte --testNamePattern EditCommitInInlineComposer --reporter=verbose
```

**Notes**

Output: .local/stack-workspace-ui-evidence/menus-edit-exact.log. Test Files  1 passed (1) | Tests  1 passed | 5 skipped (6) Exact edit composer regression passed without changing its behavior contract.
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
pnpm exec prettier --write web/src/lib/ui/store-stack-workspace/StoreStackCopyMenu.svelte docs/ui-provenance.json
```

**Notes**

Output: .local/stack-workspace-ui-evidence/menus-format5.log. Command completed; formatter output or installation completion retained.
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
pnpm exec prettier --write docs/ui-provenance.json catalog/fixtures/stack-workspace/reducer.ts web/src/lib/controller/stack-workspace/presentation.ts catalog/components/store-stack-workspace/StoreStackWorkspaceMenus.stories.svelte
```

**Notes**

Output: .local/stack-workspace-ui-evidence/menus-format6.log. Command completed; formatter output or installation completion retained.
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
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceMenus.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspace.stories.svelte --testNamePattern 'CommitCopyAndPositionalSubmenus|ImmutableCommit|CreateIndependentStack' --reporter=verbose
```

**Notes**

Output: .local/stack-workspace-ui-evidence/menus-exact-final.log. Test Files  2 passed (2) | Tests  3 passed | 17 skipped (20) Three exact menu/creation cases passed, including copy-error toast retention and immutable copy availability.
<!-- /run -->
<!-- /evidence-field -->

