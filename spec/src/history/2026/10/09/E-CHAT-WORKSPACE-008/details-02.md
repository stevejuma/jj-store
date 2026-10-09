---
schema: 1
id: E-CHAT-WORKSPACE-008
---

# E-CHAT-WORKSPACE-008 details 2

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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/WorkspaceFiles.stories.svelte -t 'WorkspaceFooter|WorkspaceMenu'
```

**Notes**

Final focused regression: two passed, eight filtered. Native mouse and keyboard exercise exact clipboard payloads, focus return, leading-edge alignment, hover background and multi-binding selection. Log workspace-008-target3.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/WorkspaceFiles.stories.svelte
```

**Notes**

Final cold full affected family: all ten interaction/accessibility stories pass. Copy paths/names, keyboard submenus, file/diff opening, retained rail ownership, loading, source comments and responsive controls remain covered. ResizeObserver warning still occurs during existing resize scenarios. Log workspace-008-final-stories.log.
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
pnpm web:check
```

**Notes**

Final source check: zero errors and warnings. Log workspace-008-final-web.log.
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
pnpm catalog:check
```

**Notes**

Final story/fixture checks: zero errors and warnings;351 modules. Log workspace-008-final-catalog.log.
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
pnpm check:progress
```

**Notes**

146 tasks and706 evidence records.
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
pnpm spec:first
```

**Notes**

Three protected files and two canonical chapters accepted.
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
pnpm spec:validate
```

**Notes**

All seven document validators pass.
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
pnpm evidence:new -- --id E-CHAT-WORKSPACE-008 --task M24 --requirement JST-UI-001 --requirement JST-UI-015 --requirement JST-UI-018 --requirement JST-API-001; node .local/workspace-evidence8.mjs; pnpm progress:history
```

**Notes**

Created fresh evidence, updated M24 history and generated navigation. Restored unrelated Bootstrap/M25 generated pages.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/WorkspaceFiles.stories.svelte -t 'WorkspaceFooter'
```

**Notes**

Before implementation: new regression confirms existing 14px trigger is not reduced. Source inspection confirms centered action buttons, visible filesystem location and separate copy actions. Log workspace-008-before.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/WorkspaceFiles.stories.svelte -t 'WorkspaceFooter'
```

**Notes**

First implementation: synthetic Testing Library hover did not activate the native CSS pseudo-class. Use Vitest browser native pointer hover for computed color acceptance. Log workspace-008-target.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/WorkspaceFiles.stories.svelte -t 'WorkspaceFooter'
```

**Notes**

After native hover, the test reopened the menu during its closing pointer/focus transition. Added explicit dismissal/pointer-restoration waits. Log workspace-008-target2.log.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/WorkspaceFiles.stories.svelte
```

**Notes**

Six passed. Three existing stories clicked newly opened panels before the menu completed closing; the copy keyboard path raced submenu autofocus and selected the first item. Logs workspace-008-stories.log.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/WorkspaceFiles.stories.svelte
```

**Notes**

Six passed. A trial modal=false root prop did not affect closing behavior and was rejected by type checking; it was removed. Keyboard assertions now await submenu focus. Log workspace-008-stories2.log.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/WorkspaceFiles.stories.svelte
```

**Notes**

Eight passed. The two remaining old stories had not yet adopted the close-transition helper due to different formatting; applied the same awaited visible state to both. Log workspace-008-stories3.log.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/WorkspaceFiles.stories.svelte
```

**Notes**

Eight passed. Mixing native hover with synthetic keyboard events made the new submenu focus checks unreliable; the failed test left an open menu affecting the next test. Converted both new interaction stories to native browser pointer/keyboard input without changing assertions. Log workspace-008-stories4.log.
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
pnpm web:check
```

**Notes**

Initial menu composition passed with zero errors and warnings. Log workspace-008-web.log.
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
pnpm web:check
```

**Notes**

Trial modal root prop is not part of the released Bits/Design Core API. Removed it; final uses the standard supported menu lifecycle. Log workspace-008-web2.log.
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
pnpm catalog:check
```

**Notes**

Initial story/component types passed with zero errors and warnings;351 catalog modules. Log workspace-008-catalog.log.
<!-- /run -->
<!-- /evidence-field -->

