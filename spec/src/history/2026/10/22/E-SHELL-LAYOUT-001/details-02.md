---
schema: 1
id: E-SHELL-LAYOUT-001
---

# E-SHELL-LAYOUT-001 details 2

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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/store-shell-layout.test.ts --reporter verbose
```

**Notes**

Registry baseline and archive rerun each pass 1/1: independent persisted Thread pane and repository rail widths. /tmp/jst-shell-layout-baseline-unit.log; /tmp/jst-shell-layout-unit-final.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/ApplicationMenu.stories.svelte -t NativeChangeDraftSurvivesMobileToDesktopRelocation --reporter verbose
```

**Notes**

Before adoption: one failure from menu node replacement across the breakpoint; /tmp/jst-shell-layout-menu-before.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/ApplicationMenu.stories.svelte -t NativeChangeDraftSurvivesMobileToDesktopRelocation --reporter verbose
```

**Notes**

After: 1/1 passes with the same menu node, draft, pending owner and return focus; /tmp/jst-shell-layout-menu-after.log.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/ApplicationMenu.stories.svelte catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-application/DesktopChrome.stories.svelte catalog/components/store-application/DiscussionSheets.stories.svelte catalog/components/store-application/ShellOverlay.stories.svelte --reporter verbose
```

**Notes**

First five-family run: 17/20 pass. The details action incorrectly collapsed rather than closed the outer rail; restored application-owned open/close. Reload bounds asserted before the modal outro released its pointer lease; wait for the established lease cleanup while retaining the width threshold. New attached-pane resize case entered constrained mode; exercise inline adjacency at explicit 1600x900. /tmp/jst-shell-layout-stories.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/DiscussionSheets.stories.svelte -t AttachedPaneRestoresItsWidthBesideAnIndependentOuterRail --reporter verbose
```

**Notes**

Exact attached-panel resize/close/reopen/independent outer-rail target passes 1/1. Captured Close element avoids querying after its own synchronous removal. /tmp/jst-shell-layout-attached-final.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/ApplicationMenu.stories.svelte catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-application/DesktopChrome.stories.svelte catalog/components/store-application/DiscussionSheets.stories.svelte catalog/components/store-application/ShellOverlay.stories.svelte --reporter verbose
```

**Notes**

Cold run: 20/20 pass in all five families. Includes native/narrow header geometry, mobile header placement, menu identity, saved-settings reload, overlay scrolling/footer/draft, and independent attached pane. /tmp/jst-shell-layout-stories-final.log.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 12
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-appearance/AppearanceShell.stories.svelte catalog/components/store-appearance/NativeGlass.stories.svelte catalog/components/store-application/RepositoryEntryControls.stories.svelte catalog/components/store-application/SettingsShell.stories.svelte catalog/components/store-application/AgentLayout.stories.svelte catalog/components/store-application/AgentChatLayout.stories.svelte catalog/components/store-application/WorkspaceFiles.stories.svelte catalog/components/store-application/WorkspaceInspector.stories.svelte catalog/components/store-mutations/StoreConflictEditor.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceConflictLayout.stories.svelte catalog/components/store-work/ProjectsHub.stories.svelte catalog/components/store-workflows/StoreWorkflows.stories.svelte --reporter verbose
```

**Notes**

Complete directly affected consumer group: 52/64 pass. Twelve old file-path selectors fail under the archive's intentional DC-DIFF-002 native naming contract for custom row snippets. Remaining 50 cases in the other eight families pass. /tmp/jst-shell-layout-consumers.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/WorkspaceFiles.stories.svelte catalog/components/store-mutations/StoreConflictEditor.stories.svelte --reporter verbose
```

**Notes**

Unmodified parent f58b9363 in disposable /tmp/jst-shell-layout-parent with its own registry install: 11/11 pass, confirming selectors predate the archive naming contract. /tmp/jst-shell-layout-parent-stories.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/WorkspaceFiles.stories.svelte catalog/components/store-application/WorkspaceInspector.stories.svelte catalog/components/store-mutations/StoreConflictEditor.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceConflictLayout.stories.svelte --reporter verbose
```

**Notes**

After exact accessible-name selector updates: 14/14 pass across all four affected families; retain strict behavior, draft, scroll, icon, width and geometry assertions, add path identity assertions. /tmp/jst-shell-layout-file-contract.log. Combined final coverage: 84 distinct scenarios across 17 files; zero unresolved interaction or accessibility failures.
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
pnpm web:check
```

**Notes**

Zero errors/warnings. /tmp/jst-shell-layout-types-final.log.
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
pnpm catalog:check
```

**Notes**

Zero errors/warnings and catalog ownership passes after removing noninteractive fixture tabindex. /tmp/jst-shell-layout-catalog-types-final.log.
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
pnpm web:build
```

**Notes**

Vite bundle and manifest complete, then documentation staging invokes managed Rust command references and is blocked by disk preflight: 22.3 GiB free below 30 GiB minimum; another agent owns the active primary build target. No Rust build, target cleanup or service interruption performed. /tmp/jst-shell-layout-web-build.log.
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
pnpm exec vite build --config web/vite.config.ts
```

**Notes**

Exact UI-only production build passes; advisory ineffective dynamic import and chunk-size warnings. /tmp/jst-shell-layout-vite-build.log.
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
pnpm catalog:check
```

**Notes**

Final selector updates: zero errors/warnings; catalog boundaries pass (473 modules). /tmp/jst-shell-layout-catalog-final.log.
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

180 tasks and 949 evidence records checked before final boundary-confirmation record. /tmp/jst-shell-layout-progress.log.
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
pnpm check:ui-ownership
```

**Notes**

One validator passes, retaining production/catalog/Community provenance boundaries. /tmp/jst-shell-layout-ownership.log.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:architecture
```

**Notes**

Two unchanged stack-workspace file-size failures: StoreStackWorkspaceApplication.stories.svelte 401/400; StoreStackWorkspace.svelte 310/300. Parent reproduction has those same two plus StoreApplicationMain.svelte 313/300, which this change reduces below the limit. /tmp/jst-shell-layout-architecture.log.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:architecture (parent f58b9363, disposable workspace)
```

**Notes**

Exact parent command confirms inherited diagnostics. /tmp/jst-shell-layout-parent-architecture.log. No thresholds/baselines changed.
<!-- /run -->
<!-- /evidence-field -->

