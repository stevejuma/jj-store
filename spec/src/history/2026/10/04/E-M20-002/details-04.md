---
schema: 1
id: E-M20-002
---

# E-M20-002 details 4

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
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceHeaderDrops.stories.svelte --reporter=verbose
```

**Notes**

Output: .local/stack-header-drops-evidence/stories-exact-final.log. Test Files  1 passed (1) | Tests  6 passed (6)
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/stack-workspace-header-drops.test.ts web/tests/vite/stack-workspace-sorting.test.ts
```

**Notes**

Output: .local/stack-header-drops-evidence/unit-after.log. Test Files  2 failed (2) | Tests  2 failed | 4 passed (6) Two immutable fixture assertions initially changed only the controller snapshot; corrected to seed the adapter authority before construction. Rejections and unchanged history assertions retained.
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
vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/stack-workspace-header-drops.test.ts web/tests/vite/stack-workspace-sorting.test.ts
```

**Notes**

Output: .local/stack-header-drops-evidence/unit-ancestor-final.log. Test Files  2 passed (2) | Tests  8 passed (8)
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/stack-workspace-header-drops.test.ts
```

**Notes**

Output: .local/stack-header-drops-evidence/unit-before.log. Test Files  1 failed (1) | Tests  4 failed (4)
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
vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/stack-workspace.test.ts web/tests/vite/stack-workspace-menus.test.ts web/tests/vite/stack-workspace-branches.test.ts web/tests/vite/store-route-history.test.ts web/tests/vite/stack-workspace-header-drops.test.ts web/tests/vite/stack-workspace-sorting.test.ts
```

**Notes**

Output: .local/stack-header-drops-evidence/unit-consumers-final.log. Test Files  6 passed (6) | Tests  65 passed (65)
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
vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/stack-workspace-header-drops.test.ts web/tests/vite/stack-workspace-sorting.test.ts
```

**Notes**

Output: .local/stack-header-drops-evidence/unit-exact-final.log. Test Files  2 passed (2) | Tests  7 passed (7)
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
svelte-check --tsconfig web/tsconfig.json
```

**Notes**

Output: .local/stack-header-drops-evidence/web-all-final-check.log. svelte-check found 0 errors and 0 warnings
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

Output: .local/stack-header-drops-evidence/web-check.log. svelte-check found 0 errors and 0 warnings
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

Output: .local/stack-header-drops-evidence/web-final-check.log. svelte-check found 0 errors and 0 warnings
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
pnpm test:storybook catalog/components/store-stack-workspace catalog/components/community-projects/CommunityProjectsRevisionGraph.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLogFiles.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLogDiff.stories.svelte catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.stories.svelte catalog/components/store-application/StoreRepositoryPicker.stories.svelte catalog/components/store-application/StoreRepositoryNavigation.stories.svelte catalog/components/store-application/StoreRepositoryBranchContext.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte --reporter=verbose
```

**Notes**

Output: .local/stack-header-drops-evidence/all-consumer-stories-qualified.log. 103/103 stories in 19 directly affected files passed in a fresh test process, with interaction/accessibility and runtime guards. No ResizeObserver or unhandled runtime diagnostic. This is the complete affected lane, not the repository suite.
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
pnpm exec prettier --check catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceHeaderDrops.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceInline.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspacePresentation.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceRefinements.stories.svelte catalog/fixtures/stack-workspace/preview.ts catalog/fixtures/stack-workspace/reducer.ts catalog/fixtures/stack-workspace/validation.ts docs/ui-provenance.json spec/src/api.md spec/src/ui.md web/src/lib/controller/stack-workspace/commit-placement.ts web/src/lib/controller/stack-workspace/drag-session.svelte.ts web/src/lib/controller/stack-workspace/drag.ts web/src/lib/controller/stack-workspace/types.ts web/src/lib/ui/community-projects/CommunityProjectsRepositoryLogFiles.svelte web/src/lib/ui/community-projects/CommunityProjectsRevisionGraph.svelte web/src/lib/ui/store-stack-workspace/StoreStackBookmarkCard.svelte web/src/lib/ui/store-stack-workspace/StoreStackLane.svelte web/src/lib/ui/store-stack-workspace/StoreStackStagedFiles.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspace.svelte web/src/lib/ui/store-stack-workspace/stack-workspace-drag.css web/src/lib/ui/store-stack-workspace/stack-workspace.css web/tests/vite/stack-workspace-header-drops.test.ts web/tests/vite/stack-workspace-sorting.test.ts
```

**Notes**

Output: .local/stack-header-drops-evidence/format-qualified.log. All 25 implementation, story, test and canonical/provenance files at that point use Prettier style.
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
pnpm spec:validate
```

**Notes**

Output: .local/stack-header-drops-evidence/spec-validate.log. Five validators, 23 chapters and 173 requirements.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first
```

**Notes**

Output: .local/stack-header-drops-evidence/spec-first.log. Provenance edits also map to spec-governance.md. Added the owning follow-up governance paragraph rather than weakening the change map.
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
pnpm spec:first
```

**Notes**

Output: .local/stack-header-drops-evidence/spec-first-repaired.log. Mapped canonical chapter now accompanies the protected changes.
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
pnpm check:progress
```

**Notes**

Output: .local/stack-header-drops-evidence/progress.log. 141 tasks; evidence bookkeeping only.
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
CUA app-shell branch selection, DOM observation and screenshot on Storybook 6057
```

**Notes**

A fresh preview tab recovered from two stale-tab focus timeouts. Verified the visible Changed files heading, branch-specific accessible name, absence of Selected chips, left marker, available tag row and scrollbar-side gutter. Screenshot: .local/stack-header-drops-evidence/workspace-refinements.jpg. No screenshot comparison or baseline update.
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
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceRefinements.stories.svelte -t ContentPaddingSeparatesComponentsFromEdgeScrollbars --reporter=verbose
```

**Notes**

Output: .local/stack-header-drops-evidence/content-padding-before.log. Test Files  1 failed (1) | Tests  1 failed | 6 skipped (7) Rail was 8px away from column edge.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceRefinements.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceMenus.stories.svelte -t 'ContentPaddingSeparatesComponentsFromEdgeScrollbars|CreateIndependentStack|CreateDependentStackWithCommandPicker' --reporter=verbose
```

**Notes**

Output: .local/stack-header-drops-evidence/padding-after-and-create-rows-before.log. Test Files  1 failed | 1 passed (2) | Tests  2 failed | 1 passed | 10 skipped (13) Padding passed; two creation stories exposed non-Settings form rows.
<!-- /run -->
<!-- /evidence-field -->

