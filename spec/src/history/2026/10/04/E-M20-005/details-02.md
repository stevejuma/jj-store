---
schema: 1
id: E-M20-005
---

# E-M20-005 details 2

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
pnpm test:storybook src/shared/shadcn/column-canvas/ColumnCanvasToggle.stories.svelte
```

**Notes**

Design Core .local/panel-toggle/before.log: Invalid preflight: inline example string contained a script closing tag; moved to external example sources.
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
pnpm test:storybook src/shared/shadcn/column-canvas/ColumnCanvasToggle.stories.svelte
```

**Notes**

Design Core .local/panel-toggle/before-exact.log: Valid before regression: shared icon absent.
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
pnpm test:storybook src/shared/shadcn/column-canvas/ColumnCanvasToggle.stories.svelte
```

**Notes**

Design Core .local/panel-toggle/after.log: Exact initial geometry and collapse/expand regression passed 1/1.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 6
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook src/shared/shadcn/panel-toggle/PanelToggle.stories.svelte src/shared/shadcn/column-canvas/ColumnCanvasToggle.stories.svelte src/shared/shadcn/column-canvas/ColumnCanvas.stories.svelte
```

**Notes**

Design Core .local/panel-toggle/stories.log: Redundant output status role caused compilation errors and six Vite overlay accessibility failures; removed redundant role.
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
pnpm test:storybook src/shared/shadcn/panel-toggle/PanelToggle.stories.svelte src/shared/shadcn/column-canvas/ColumnCanvasToggle.stories.svelte src/shared/shadcn/column-canvas/ColumnCanvas.stories.svelte
```

**Notes**

Design Core .local/panel-toggle/stories-retry.log: Disabled userEvent click rejected pointer-events:none; native disabled behavior retained and verified through browser mouse input.
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
pnpm test:storybook src/shared/shadcn/panel-toggle/PanelToggle.stories.svelte src/shared/shadcn/column-canvas/ColumnCanvasToggle.stories.svelte src/shared/shadcn/column-canvas/ColumnCanvas.stories.svelte
```

**Notes**

Design Core .local/panel-toggle/stories-final.log: 19/19 stories before custom icon regression addition.
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
pnpm test:storybook src/shared/shadcn/panel-toggle/PanelToggle.stories.svelte src/shared/shadcn/column-canvas/ColumnCanvasToggle.stories.svelte src/shared/shadcn/column-canvas/ColumnCanvas.stories.svelte
```

**Notes**

Design Core .local/panel-toggle/stories-custom.log: Custom example duplicated default and explicit headers; use separate columnTitle accessible metadata.
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
pnpm test:storybook src/shared/shadcn/panel-toggle/PanelToggle.stories.svelte src/shared/shadcn/column-canvas/ColumnCanvasToggle.stories.svelte src/shared/shadcn/column-canvas/ColumnCanvas.stories.svelte
```

**Notes**

Design Core .local/panel-toggle/stories-verified.log: 20/20 including custom icon override, separate accessible title, preview and rail coverage.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
STORYBOOK_PORT=9139 pnpm test:shadcn:pointer tests/shadcn/panel-toggle-pointer.spec.ts
```

**Notes**

Design Core .local/panel-toggle/pointer.log: Four readiness timeouts after invalid story compilation.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
STORYBOOK_PORT=9139 pnpm test:shadcn:pointer tests/shadcn/panel-toggle-pointer.spec.ts
```

**Notes**

Design Core .local/panel-toggle/pointer-retry.log: One pass; three focus-dependent expectations failed because story play retained focus. Browser setup now clicks empty body to reset real focus before hover/Tab, with assertions/timeouts unchanged.
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
STORYBOOK_PORT=9139 pnpm test:shadcn:pointer tests/shadcn/panel-toggle-pointer.spec.ts
```

**Notes**

Design Core .local/panel-toggle/pointer-final.log: 4/4 real pointer/keyboard, geometry, reversed states, disabled, right orientation, reduced-motion and forced-color cases.
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
pnpm check:types
```

**Notes**

Design Core .local/panel-toggle/types.log: One wrong Root prop (sizing) and three redundant output roles; corrected to public displayMode and native output semantics.
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
pnpm check:types
```

**Notes**

Design Core .local/panel-toggle/types-retry.log: All four source type lanes found zero errors/warnings.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:check
```

**Notes**

Design Core .local/panel-toggle/spec.log: New verification row appended outside the table; moved into canonical matrix.
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
pnpm spec:check
```

**Notes**

Design Core .local/panel-toggle/spec-retry.log: Validation, mdbook and spec-first passed.
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
pnpm install --lockfile=false --ignore-scripts
```

**Notes**

.local/panel-toggle-evidence/dependency-refresh.log: Existing ignored local foundation snapshot refreshed; manifest and lockfile unchanged.
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
pnpm test:storybook catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte -t WorkspaceInAppShell
```

**Notes**

.local/panel-toggle-evidence/exact.log: Geometry passed; accessible rail used internal working key. Added independent columnTitle metadata instead of default title header.
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
pnpm test:storybook catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte -t WorkspaceInAppShell
```

**Notes**

.local/panel-toggle-evidence/exact-retry.log: 1/1 passed with seven unrelated stories skipped; later header composition fixed to use columnTitle.
<!-- /run -->
<!-- /evidence-field -->

