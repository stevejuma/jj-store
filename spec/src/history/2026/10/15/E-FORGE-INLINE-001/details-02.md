---
schema: 1
id: E-FORGE-INLINE-001
---

# E-FORGE-INLINE-001 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-forge/StoreForgeInlineWorkspace.stories.svelte
```

**Notes**

Initial creation/floating and expandable-summary regressions: 2 failed before implementation. Log .local/forge-inline-before.log. The summary placement was subsequently corrected following the user’s Op Log instruction; this initial target does not qualify the final placement.
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

Interim StackLane exceeded the 300 counted production-line limit (305). Extracted its existing menu and the bookmark header into owned components without changing their action contracts. The complete final architecture check passes.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-forge/StoreForgeInlineWorkspace.stories.svelte
```

**Notes**

Initial implementation: 2 failing stories and 2 unhandled optimizer/runtime errors. Restored the still-needed Trash import after extracting the menu and reran in fresh processes; no shared Vite/test configuration or dependencies were patched. Log .local/forge-inline-after.log.
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
pnpm catalog:check
```

**Notes**

Initial missing Trash import in the retained lane action; restored locally. Log .local/forge-inline-types.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-forge/StoreForgeInlineWorkspace.stories.svelte
```

**Notes**

Initial inline creation and same-editor float/redock passed, 2 stories. Log .local/forge-inline-after2.log. This predates the corrected PR+diff placement.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-forge/StoreForgeWorkspace.stories.svelte catalog/components/store-forge/StoreForgeInlineWorkspace.stories.svelte catalog/components/store-forge/StoreForge.stories.svelte
```

**Notes**

Intermediate direct consumers: 17 stories passed before the user’s final placement correction and additional failure/lifecycle cases. Log .local/forge-inline-consumers.log.
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
pnpm catalog:check
```

**Notes**

Unsupported Testing Library ByRoleOptions exact property in the new story; removed the unsupported option while retaining exact string role names. Log .local/forge-inline-types2.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/forge.test.ts
```

**Notes**

Initial creation/draft/unknown-outcome controller additions: 12 tests passed. Log .local/forge-inline-controller.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-forge/StoreForgeInlineWorkspace.stories.svelte -t PrSummaryStaysAboveChanges
```

**Notes**

Selector mismatch: all 4 stories skipped, zero assertions. This is not acceptance evidence. Log .local/forge-details-before.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-forge/StoreForgeInlineWorkspace.stories.svelte -t 'PR summary stays above changes'
```

**Notes**

Display-name selector also matched no generated export: all 4 stories skipped, zero assertions. Replaced with the actual story-file run below. Log .local/forge-details-before2.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-forge/StoreForgeInlineWorkspace.stories.svelte
```

**Notes**

Corrected placement regression before moving production UI: 1 failed and 3 passed. PR summary was incorrectly inside the stack lane instead of the same column as the selected file diff. Log .local/forge-details-before3.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-forge/StoreForgeInlineWorkspace.stories.svelte catalog/components/store-forge/StoreForgeWorkspace.stories.svelte
```

**Notes**

After moving PR details into the shared per-origin details column: all 10 stories passed. Log .local/forge-details-after.log.
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
pnpm catalog:check
```

**Notes**

TypeScript retained an obsolete undefined-plan narrowing across async preparation. Moved explicit plan clearing into resetPlan, keeping failed previews unpublishable and reading the newly prepared plan after success. Log .local/forge-details-types.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-forge/StoreForgeInlineWorkspace.stories.svelte catalog/components/store-forge/StoreForgeFloating.stories.svelte
```

**Notes**

6 stories passed, including actual native pointer move/resize, viewport clamp, minimize/restore, opaque floating surface, nested-menu Escape, focused Escape/redock, return focus and identical retained Markdown editor. Log .local/forge-details-lifecycle.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/forge.test.ts web/tests/vite/forge-details.test.ts
```

**Notes**

Intermediate 16 tests passed for exact submission pins/drafts, branch-owned PR/diff resolution and independent origins. Log .local/forge-details-controller.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-forge/StoreForgeWorkspace.stories.svelte catalog/components/store-forge/StoreForgeInlineWorkspace.stories.svelte catalog/components/store-forge/StoreForgeFloating.stories.svelte catalog/components/store-forge/StoreForge.stories.svelte
```

**Notes**

Final cold Chromium/Vitest run: all 22 directly affected forge stories passed, with interaction/accessibility coverage. Includes Op Log-style PR above file diff, PR-only opening, automatic matching PR on file selection, one column identity while switching files, collapse/file-close isolation, explicit CI inspection, inline branch-default creation, top-only controls, light/dark buttons, menu submission, failure toasts and explicit observation of unknown submission without replay. Log .local/forge-details-final-stories.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceRefinements.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceLazy.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceTarget.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceSelection.stories.svelte -t '[Mm]ultiple.*[Dd]iffs|[Rr]ead.*[Bb]rowsing.*[Dd]iffs|[Ss]elected.*[Pp]atch.*[Rr]etry|[Cc]hanged.*[Oo]peration.*[Rr]eloads|[Tt]arget.*[Bb]ranch.*[Oo]pens|[Ss]elected.*[Cc]ommits.*[Hh]eaders|[Bb]ranch.*[Cc]hips|[Bb]ranch.*[Aa]ctions.*[Dd]iffer|[Kk]ebab.*[Mm]enus'
```

**Notes**

11 directly affected no-forge Stack/diff scenarios passed; 17 unrelated scenarios intentionally skipped. Covers independent adjacent diffs and hunk/view-mode state, selected patch retry and operation refresh, target history, retained selections, extracted header/menu action geometry and chip hover. Log .local/forge-details-diff-consumers.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/forge.test.ts web/tests/vite/forge-details.test.ts
```

**Notes**

Final 17 tests passed. Includes explicit observation completing an unknown inline creation without a second submit, retained legacy submission drafts and original plan guards, precise branch/commit PR resolution, PR-only/file origin reuse, retained PR after closing the file and pending-publication protection during file browsing. Log .local/forge-details-controller-final.log.
<!-- /run -->
<!-- /evidence-field -->

