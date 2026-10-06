---
schema: 1
id: E-M21-057
---

# E-M21-057 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: pass
failures: 0
detail: "No tests executed: human-readable filter did not match generated
  PascalCase story names (15 skipped); reran the exact file.
  /tmp/merge-header-before.log records the actual regression run."
hasNote: false
noteEncoding: markdown

```

```shell
Design Core: pnpm test:storybook src/shared/diff/merge-editor/MergeEditor.stories.svelte -t "Centers host controls"
```
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 1
detail: "Before centered slot: 1 failed/14 passed. The header control did not
  render. /tmp/merge-header-before.log."
hasNote: false
noteEncoding: markdown

```

```shell
Design Core: pnpm test:storybook src/shared/diff/merge-editor/MergeEditor.stories.svelte
```
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: pass
failures: 0
detail: "Centered slot: 15 passed, measuring 720/480/320px center, non-overlap,
  narrow second row, retained focus and pager. /tmp/merge-header-after.log."
hasNote: false
noteEncoding: markdown

```

```shell
Design Core: pnpm test:storybook src/shared/diff/merge-editor/MergeEditor.stories.svelte
```
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 1
detail: Initial governance stopped the aggregate on a multi-sentence acceptance
  bullet; corrected the normative wording. Other two lanes were cancelled.
  /tmp/merge-header-checks.log.
hasNote: false
noteEncoding: markdown

```

```shell
Design Core: pnpm checks
```
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 2
detail: "Before gutter/default-sync fix: 2 failed/15 passed.
  Editable/current-hunk text painted over gutter and horizontal synchronization
  defaulted off. /tmp/merge-settings-before.log."
hasNote: false
noteEncoding: markdown

```

```shell
Design Core: pnpm test:storybook src/shared/diff/merge-editor/MergeEditor.stories.svelte
```
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: pass
failures: 0
detail: "After source fix: 17 passed; hit-tested scrolled text/current
  highlights beneath gutters and default horizontal synchronization.
  /tmp/merge-settings-after.log."
hasNote: false
noteEncoding: markdown

```

```shell
Design Core: pnpm test:storybook src/shared/diff/merge-editor/MergeEditor.stories.svelte
```
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 1
detail: "Before picker relocation: 1 failed/2 passed; trigger outside Merge
  Editor toolbar. /tmp/stack-header-before.log."
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceConflictVersions.stories.svelte
```
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 1
detail: "Initial settings integration: 1 failed/3 passed. Preventing selection
  also prevented CheckboxItem state updates. Replaced with public
  closeOnSelect=false. /tmp/stack-settings-before.log."
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceConflictVersions.stories.svelte
```
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: pass
failures: 0
detail: "Fresh browser process: 15 passed, including centered 24px picker,
  persisted settings after editor remount, retained draft, Escape/trigger focus
  and native/workspace conflict flows. /tmp/stack-settings-stories.log."
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceConflictVersions.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceConflicts.stories.svelte catalog/components/store-mutations/StoreConflictEditor.stories.svelte
```
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: pass
failures: 0
detail: "3 passed: malformed/foreign storage, exact preference-only
  serialization and storage-denial fallback. /tmp/stack-settings-unit.log."
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec tsx --test web/tests/merge-editor-settings.test.ts
```
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: pass
failures: 0
detail: "Both: 0 errors/0 warnings; 242 catalog modules.
  /tmp/stack-settings-web.log and /tmp/stack-settings-catalog.log."
hasNote: false
noteEncoding: markdown

```

```shell
pnpm web:check; pnpm catalog:check
```
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: pass
failures: 0
detail: Ownership validates 1 checker/23 chapters/173 requirements; architecture
  validates 1136 modules. /tmp/stack-settings-ownership.log and
  /tmp/stack-settings-architecture.log.
hasNote: false
noteEncoding: markdown

```

```shell
pnpm check:ui-ownership; pnpm check:architecture
```
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: pass
failures: 0
detail: Refreshed the existing supported temporary validation dependency; exact
  runtime source matches. Portable package manifest and registry lockfile
  unchanged. /tmp/stack-header-dependency.log.
hasNote: false
noteEncoding: markdown

```

```shell
pnpm install --offline --force --ignore-scripts; jj restore --from @- pnpm-lock.yaml; cmp <installed/source MergeEditor.svelte and MergeEditor.css>
```
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: pass
failures: 0
detail: Fresh UI process on 7476 and owned Storybook on 6057; backend PID 42570
  and .local/dev-demo preserved. No Rust builds, backend restart or reset.
hasNote: false
noteEncoding: markdown

```

```shell
Owned Vite-only restart and pnpm storybook --port 6057 --ci
```
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: pass
failures: 0
detail: "Live picker height 24px, center error 0.00390625px; settings restored
  after reload and sync defaults on. Center pane scrollLeft=468.5px and gutter
  hit-test stays above text. Menu width 224px/item height 32px; live merge and
  Workspace Settings item typography both Inter 14px/400/20px. Escape retains
  editor and focuses trigger. Initial busy read recovered via Refresh. Focused
  Op Log has the same four options, preserves typed resolution and restores both
  comparison options after reload; filters reset off afterward. Tab-local 4000ms
  CDP latency exposed the actual loading skeleton, then latency reset to zero,
  Network disabled and loaded editor confirmed. Temporary tab closed; no
  resolution applied. Proof: .local/stacks-merge-header/live-oplog-settings.png
  and live-loading-skeleton.png."
hasNote: false
noteEncoding: markdown

```

```shell
CUA live picker geometry, settings reload, native horizontal scroll, menu sizing and Escape
```
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: pass
failures: 0
detail: Validate, mdbook and specification-first lanes pass.
  /tmp/merge-settings-spec-final.log.
hasNote: false
noteEncoding: markdown

```

```shell
Design Core: pnpm spec:check
```
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: pass
failures: 0
detail: "Initial header/gutter slice: 722 units, 670 stories, 15 workspace, 11
  shell, 3 AI and 48 shadcn browser tests plus one governed skip;
  types/static/audit/governance passed. Committed rmlqkuql/5c8be23b.
  /tmp/merge-settings-checks.log."
hasNote: false
noteEncoding: markdown

```

```shell
Design Core: pnpm checks
```
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: fail
failures: 1
detail: "Comparison settings discarded edited content: 1 failed/17 passed.
  /tmp/merge-comparison-before.log."
hasNote: false
noteEncoding: markdown

```

```shell
Design Core: pnpm test:storybook src/shared/diff/merge-editor/MergeEditor.stories.svelte
```
<!-- /run -->

<!-- run -->
### Run 19

```yaml
result: pass
failures: 0
detail: 18 passed after separating content identity from comparison-model
  identity; edited and accepted three-way content survives both filters.
  /tmp/merge-comparison-after.log.
hasNote: false
noteEncoding: markdown

```

```shell
Design Core: pnpm test:storybook src/shared/diff/merge-editor/MergeEditor.stories.svelte
```
<!-- /run -->
<!-- /evidence-field -->

