---
schema: 1
id: E-UI-101
---

# E-UI-101 details 2

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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/community-projects/CommunityProjectsRevisionGraph.stories.svelte -t "Wide graph keeps row content readable"
```

**Notes**

Initial display-name filter skipped 5 tests; corrected to export-name regex. graph-before.log; not regression evidence.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/community-projects/CommunityProjectsRevisionGraph.stories.svelte -t "Wide|wide" (before)
```

**Notes**

graph-before 2.log: avatar spread 0, expected &gt;10.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --project storybook src/shared/shadcn/column-canvas/ColumnCanvasScroll.stories.svelte (Design Core before/initial attempts)
```

**Notes**

Initial CSF namespace metadata parser failure; corrected direct Root import. Second fixture lacked template snippet and produced 2 failures/2 errors; corrected fixture before acceptance.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --project storybook src/shared/shadcn/column-canvas/ColumnCanvasScroll.stories.svelte (Design Core corrected before)
```

**Notes**

/tmp/design-core-column-body-before 3.log: both strategies force vertical orientation.
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
pnpm exec vitest run --project storybook src/shared/shadcn/column-canvas/ColumnCanvasScroll.stories.svelte (Design Core initial after)
```

**Notes**

Synthetic Storybook ArrowRight does not perform native scrolling; corrected new harness to assert viewport focus and programmatic movement. Real wheel gestures verified separately.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/community-projects/CommunityProjectsRevisionGraph.stories.svelte -t "Wide|wide" (initial after)
```

**Notes**

graph-after.log: test measured intrinsic 145px button width; corrected to measure available message width and assert title is untruncated.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/community-projects/CommunityProjectsRevisionGraph.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte catalog/components/store-application/RepositoryReview.stories.svelte catalog/components/store-application/StoreEvolutionColumn.stories.svelte
```

**Notes**

final-stories.log:21 focused cold production-component stories pass, including accessibility/runtime guards. Direct component consumers only.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/community-projects/CommunityProjectsRevisionGraph.stories.svelte -t "Wide|wide" (final)
```

**Notes**

graph-final.log:1 passes/4 deliberately skipped after typed query-option correction.
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
pnpm web:check
```

**Notes**

web-check.log:0 errors/0 warnings.
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
pnpm catalog:check (initial)
```

**Notes**

catalog-check.log: unsupported exact option in Storybook typed getByRole; removed redundant option, preserving default exact name matching.
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
pnpm catalog:check (final)
```

**Notes**

catalog-check-final.log:0 errors/0 warnings;192 catalog modules.
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
pnpm exec vitest run --project storybook src/shared/shadcn/column-canvas/ColumnCanvasScroll.stories.svelte src/shared/shadcn/column-canvas/ColumnCanvas.stories.svelte (Design Core)
```

**Notes**

/tmp/design-core-column-body-final-stories.log:16 passes; final same-family metadata rerun 2 passes in final-exact.log.
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
pnpm exec vitest run --project unit src/shared/shadcn/column-canvas/column-canvas-docs.spec.ts (Design Core)
```

**Notes**

/tmp/design-core-column-body-units.log:4 passes.
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
pnpm check:types (Design Core)
```

**Notes**

/tmp/design-core-column-body-types.log:source/shell/filter/source-consumer checks each have 0 errors/warnings.
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
pnpm spec:validate (Design Core initial)
```

**Notes**

New story title lacked mapping and appended acceptance exceeded word/sentence limits; kept existing family title and condensed same requirement without widening scope.
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
pnpm spec:validate; pnpm spec:first (Design Core final)
```

**Notes**

Final source-spec logs:8 validators/30 chapters/258 requirements;4 protected files/2 canonical chapters. Preserve prior verification table bytes outside one row.
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
pnpm exec prettier --write (changed Svelte/UI/canonical source files); pnpm exec prettier --check (Design Core changed code/docs/layout chapter)
```

**Notes**

Formatting completed. Source verification retains one changed row rather than whole-table alignment churn.
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
pnpm check:ui-ownership
```

**Notes**

ownership.log:1 validator; no new Community dependency or copied component.
<!-- /run -->
<!-- /evidence-field -->

