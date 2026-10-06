---
schema: 1
id: E-M11-008
---

# E-M11-008

<!-- evidence-field:id markdown -->
## Id

E-M11-008
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M11
- M11.4
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-01
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

projects-page-header-filters
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-work/ProjectsActivity.stories.svelte
- catalog/components/store-work/ProjectsActivityControls.stories.svelte
- catalog/components/store-work/ProjectsFilters.stories.svelte
- catalog/components/store-work/ProjectsHub.stories.svelte
- catalog/components/store-work/projects-layout-play.ts
- spec/src/api.md
- spec/src/milestones.md
- spec/src/ui.md
- web/src/lib/ui/store-work/StoreProjects.svelte
- web/src/lib/ui/store-work/StoreProjectsFilterControls.svelte
- web/src/lib/ui/store-work/StoreProjectsFilters.svelte
- web/src/lib/ui/store-work/StoreProjectsRail.svelte
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
<!-- /evidence-field -->

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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/ProjectsFilters.stories.svelte
```

**Notes**

Before implementation, desktop and mobile stories cannot find page-header filter triggers; filters live in the details rail.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/ProjectsFilters.stories.svelte
```

**Notes**

First implementation run hit one suite import failure during Vite optimization of the new SlidersHorizontal icon; no tests executed. The log retains the optimizer-triggered reload; rerun from a fresh runner.
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
pnpm web:check; pnpm catalog:check
```

**Notes**

Initial header integration passes with zero errors/warnings; 109 catalog modules.
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
pnpm check:architecture && pnpm check:boundaries
```

**Notes**

New assertions grew ProjectsHub story to 409 counted lines over 400 and StoreProjects to 302 over 300. Extracted the complete layout play function and grouped activity/general filter controls in their owning component; no budget changes.
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
pnpm check:architecture && pnpm check:boundaries
```

**Notes**

After extraction: 443 modules and 15 package boundaries.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/ProjectsFilters.stories.svelte
```

**Notes**

Fresh rerun executed both stories: mobile passed, desktop observed a closed popover still present during its exit transition. Wait for eventual unmount after Escape while retaining the focus and draft assertions; no product relaxation.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/ProjectsFilters.stories.svelte
```

**Notes**

Both exact desktop/mobile scenarios pass across all five headers: icon-only triggers, no filter rail, search draft retention, Escape and focus restoration.
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
pnpm web:check; pnpm catalog:check
```

**Notes**

Final extracted controls and stories pass with zero errors/warnings; 110 catalog modules.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/ProjectsFilters.stories.svelte catalog/components/store-work/ProjectsHub.stories.svelte catalog/components/store-work/ProjectsActivity.stories.svelte catalog/components/store-work/ProjectsActivityControls.stories.svelte catalog/components/store-work/ProjectsDirectory.stories.svelte
```

**Notes**

All 19 directly affected interaction/accessibility stories pass from a fresh runner. Existing search, task-state, repository-review, Activity category filters, project-detail scope, layout, navigation and read-only scenarios retained.
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
pnpm web:build
```

**Notes**

Fresh production bundle built; existing dependency CSS and bundle-size warnings remain in ignored log.
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
pnpm spec:first && pnpm spec:validate && pnpm check:progress
```

**Notes**

Spec-first required the mapped api.md chapter as well as ui.md for the protected application presentation. Added explicit existing controller-action/route/read boundary; later commands did not run.
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
pnpm dev:restart
```

**Notes**

Owned demo on 7460 rebuilt/restarted with retained repo, identity and RustFS volume; Search ready. Development helper builds binaries, without a Rust test lane.
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
cua_repl: cold demo reload; Tasks filters; 390x844 viewport; Activity header; reset viewport
```

**Notes**

Live Tasks shows icon-only Filters beside grid/list and no right-rail Filters section. Existing fields render in popover; 390px phone view has document width 390px and all fields visible. Activity has its category and general filter icons in header. Viewport reset; no filter values or native data changed. Screenshots saved in ignored .scratch.
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
pnpm spec:first && pnpm spec:validate && pnpm check:progress
```

**Notes**

Final canonical gates pass: 9 protected files, 3 changed canonical chapters, 132 requirements and 111 tracked tasks.
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
pnpm exec prettier --write <changed Svelte/TypeScript/Markdown files>; pnpm exec prettier --check <all changed files>
```

**Notes**

All changed presentation, stories, play helper and canonical documentation formatted; final check passes.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

[]
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

UI-only Projects header/filter presentation and its five directly affected fixture story families. Fresh Storybook runners, web/catalog checks and cold rebuilt owned demo review. No controller, wire or persistence change; Rust tests, root aggregates and unrelated stories are out of scope. Live demo reloaded the new built assets after restart; desktop and 390x844 mobile geometry verified.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M11-008 details 2](details-02.md)
