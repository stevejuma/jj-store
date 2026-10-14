---
schema: 1
id: E-OVERVIEW-PRESENTATION-001
---

# E-OVERVIEW-PRESENTATION-001 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/RepositoryLayout.stories.svelte catalog/components/store-application/RepositoryLoading.stories.svelte catalog/components/store-workflows/WorkflowHistory.stories.svelte -t 'Overview.*BodyInset|FileDetail$|ASavedRunLink|RunReloadSkeleton'
```

**Notes**

Before: two Overview divider cases, file viewer frame and two workflow metadata-to-console gap cases failed; 24 unrelated cases skipped.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/RepositoryLayout.stories.svelte catalog/components/store-application/RepositoryLoading.stories.svelte catalog/components/store-workflows/WorkflowHistory.stories.svelte -t 'Overview.*BodyInset|FileDetail$|ASavedRunLink|RunReloadSkeleton'
```

**Notes**

After: all five exact presentation regressions passed; 24 unrelated cases skipped.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/RepositoryFileSkeleton.stories.svelte
```

**Notes**

Six initial/module/detail Markdown/source and narrow-frame interaction/Axe cases passed. Intermediate module cases found duplicate status ownership, then an isolated fixture missing its application stylesheet; corrected both. No assertion weakened.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/RepositoryLayout.stories.svelte catalog/components/store-application/RepositoryFileSkeleton.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte
```

**Notes**

14 navigation/loading/layout cases passed after the additional requested four-pixel breadcrumb indentation and updating the persistent-layout divider expectation.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/RepositoryLayout.stories.svelte catalog/components/store-application/RepositoryLoading.stories.svelte catalog/components/store-application/RepositoryFileSkeleton.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte catalog/components/store-application/StoreFileEditor.stories.svelte catalog/components/store-workflows/WorkflowHistory.stories.svelte catalog/components/store-workflows/WorkflowConsole.stories.svelte
```

**Notes**

Final wider surface: 52/54 passed; ReturnToRootClearsFileEditor and EditAndRename failed. Initial wider run had four failures, including module-fixture stylesheet and obsolete persistent-layout border expectation, both repaired. The remaining editor cases concern navigation/save flows, not presentation.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.parent.config.ts --project storybook catalog/components/store-application/StoreFileEditor.stories.svelte -t 'ReturnToRoot|EditAndRename'
```

**Notes**

Both failures reproduced on parent 2b258c71 in isolated JJ workspace overview-parent-check. Initial parent attempt lacked pnpm binaries, then shared dependency imports required explicit Vite fs allowance in a temporary parent-only config. The qualified parent run reproduced both original failures. Workspace forgotten and removed after completion; other workspaces preserved.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreFileEditor.stories.svelte -t 'ReturnToRoot|EditAndRename'
```

**Notes**

Exploratory route-effect and test-flow changes did not repair these inherited cases. All speculative changes, diagnostics and editor/fixture edits were restored; file-editing behavior remains unchanged. Existing generic editing navigation guard prevents root navigation during editing; catalog mutation routing prefers rewrite replacement before created file context. No correction to those policies is included.
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
pnpm web:check
```

**Notes**

0 errors and warnings.
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
pnpm catalog:check
```

**Notes**

0 errors and warnings; 442-module catalog boundary passed.
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
pnpm check:architecture
```

**Notes**

1880 modules passed, budgets unchanged.
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
pnpm check:ui-ownership
```

**Notes**

Owned Community-derived presentation and released Design Core primitives retained.
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
node scripts/docs.mjs review application "Reviewed borderless Overview workspace-to-body spacing, one-rem breadcrumb inset, shared metadata/statistics/viewer loading frames and matching workflow console spacing; updated browsing guidance."
```

**Notes**

Application coverage fingerprint refreshed; browsing guide and provenance updated.
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
pnpm docs:check
```

**Notes**

143-file documentation closure, links and anchors passed.
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
pnpm exec prettier --write changed UI/catalog/docs/spec paths
```

**Notes**

Named change paths only.
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

Fresh evidence and unchanged milestone reviews validated.
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
pnpm spec:validate
```

**Notes**

Canonical document validators passed.
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
CUA: preserved live Overview and pinned workflow cf179354 at publication 400
```

**Notes**

Overview body divider 0px; final breadcrumb inset 16px and leading content x65 matches workspace content x65. Workflow metadata-to-console gap 20px. Screenshots: .local/overview-layout-final.png and .local/workflow-detail-spacing.png. User tab/data preserved; temporary agent tab closed. No restart, save or reload required.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Two pre-existing file-editor failures remain unresolved and reproduced on
  parent 2b258c71. After these failures were reported, the user explicitly
  requested committing all pending main-workspace changes on 2026-10-14;
  this accepts the focused commit with those failures recorded.
- Visual baseline approval, release/platform and M28 provider/protocol gates
  remain separate.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; JJ 0.45.1; Node 24.10.0; pnpm 10.32.1; Svelte 5.57.1; Design Core 0.3.1; Vitest 4.1.11; Storybook 10.5.7.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Presentation-only Overview/selected-file loading/workflow spacing. Native storage, WorkflowStore evidence, public API, CLI, editing/command behavior and exact publication pins remain unchanged.
<!-- /evidence-field -->
