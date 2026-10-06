---
schema: 1
id: E-M20-001
---

# E-M20-001 details 2

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
jj --no-pager st; jj workspace add /Users/stevejuma/code/lapismd/jj-store-stack-workspace-ui --name stack-workspace-ui -r vrsptlyk -m "Build local stack workspace UI with mocked controller"
```

**Notes**

Clean base vrsptlyk/1528d2f1; isolated new UI workspace; main and other workspaces preserved. Before-state recorded in ignored .local/stack-workspace-ui-evidence/before-state.txt.
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
pnpm web:check (before implementation)
```

**Notes**

Baseline zero diagnostics.
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
pnpm web:check (first implementation)
```

**Notes**

One type error (MergeResolvedChange.resolved) and one draggable group accessibility warning, both repaired.
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
pnpm catalog:test web/tests/vite/stack-workspace.test.ts (initial)
```

**Notes**

17 controller/mock tests; final additions are qualified separately.
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
pnpm test:storybook catalog/components/store-stack-workspace/StoreStackWorkspace.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceRecovery.stories.svelte (initial shared node_modules link)
```

**Notes**

Two setup suites failed to fetch dependencies outside workspace; no stories executed. Link removed and workspace-local installation used.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm install --offline --frozen-lockfile; pnpm install --offline --lockfile=false; pnpm exec prettier --write feature files (before installation finished)
```

**Notes**

Frozen checksum drift, unavailable offline lucide tarball and unavailable formatter. No manifest or lockfile edits; succeeded with pnpm install --lockfile=false and subsequent feature formatting.
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
pnpm install --lockfile=false
```

**Notes**

Workspace-local dependency tree with existing repository Design Core file hook; no manifests or lockfile changed. Local validation only, not registry-only release qualification.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 15
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-stack-workspace/StoreStackWorkspace.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceRecovery.stories.svelte (local dependencies)
```

**Notes**

4/19 passed; dynamic columns rendered before ensureColumn registration. Repaired registration/render ordering using the public controller API.
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
pnpm test:storybook catalog/components/store-stack-workspace/StoreStackWorkspace.stories.svelte -t 'Combined working files'
```

**Notes**

All 14 skipped due to transformed names; not acceptance evidence. Reran -t Combined and exercised one passing story.
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
pnpm test:storybook catalog/components/store-stack-workspace/StoreStackWorkspace.stories.svelte -t Combined
```

**Notes**

Exact column-registration regression: one exercised story passed, 13 excluded.
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
pnpm catalog:test web/tests/vite/stack-workspace.test.ts (second); pnpm web:check (second)
```

**Notes**

19 tests passed and zero web diagnostics after original-request ownership and lost-acknowledgement recovery additions.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 47
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:check (first)
```

**Notes**

Unsupported exact property on ByRoleOptions in new stories/helpers; removed it because string role names match exactly by default.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-stack-workspace (third)
```

**Notes**

18/23 passed. Four stories expected combobox instead of the public Select button role; one generic synthetic drag event did not carry DataTransfer. ResizeObserver diagnostics also recorded; no suppression or weakened runtime guard.
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
pnpm test:storybook catalog/components/store-stack-workspace -t 'Named|CreateWorkspace|Amend|Split|Drag'
```

**Notes**

Four corrected workflows passed; drag failed. Narrowed to exact Drag test and added payload assertion.
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
pnpm test:storybook catalog/components/store-stack-workspace/StoreStackWorkspaceCommands.stories.svelte -t Drag (first payload assertion)
```

**Notes**

Verified generic event supplied no drag payload; switched story to native DragEvent with DataTransfer, retaining the source-payload and destination-effect assertions.
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
pnpm test:storybook catalog/components/store-stack-workspace/StoreStackWorkspaceCommands.stories.svelte -t Drag (native event)
```

**Notes**

Exact drag workflow passed, 3 excluded; selection payload, preview, apply and retained unselected hunk asserted.
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
pnpm catalog:check (second); pnpm check:architecture; pnpm check:ui-ownership
```

**Notes**

Zero catalog diagnostics; catalog boundary, architecture and ownership checks passed. Final checks after subsequent refinements are recorded below.
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
pnpm exec storybook dev --host 127.0.0.1 --port 6057 --no-open --disable-telemetry; GET /index.json; browser preview inspection
```

**Notes**

Isolated fresh workspace preview; new 23 stories indexed, production root rendered and browser tab opened. Existing unrelated ports/services unchanged.
<!-- /run -->
<!-- /evidence-field -->

