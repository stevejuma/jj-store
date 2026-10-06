---
schema: 1
id: E-UI-074
---

# E-UI-074

<!-- evidence-field:id markdown -->
## Id

E-UI-074
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M11.4
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-02
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Projects tab-row search and trailing filters
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

Enclosing Jujutsu change yywsxtqq
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-015
- JST-WORK-008
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Visual approval remains separate. Rust, root aggregates and built production
  browser fixtures are out of scope for source UI composition.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Owned Projects navigation composes leading search, horizontally scrolling tabs and trailing supported filters. Open search replaces tabs with a full-row borderless labelled input. Enter submits existing512-byte-bounded route query; Close clears query and restores tabs/opener focus. Hover, input text or keyboard focus exposes Close. Background reads and filter popovers retain unsent drafts. Header settings/search trigger removed; Reviews retains repository filter without unsupported search; Repositories has no empty additional-filter popup. Community tabs and user screenshots inspected first, Design Core InputGroup/Button/Tabs/ScrollArea source and official docs second.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-work/ProjectsActivity.stories.svelte
- catalog/components/store-work/ProjectsActivityControls.stories.svelte
- catalog/components/store-work/ProjectsFilters.stories.svelte
- catalog/components/store-work/projects-layout-play.ts
- spec/src/api.md
- spec/src/milestones.md
- spec/src/ui.md
- web/src/lib/ui/store-work/StoreProjects.svelte
- web/src/lib/ui/store-work/StoreProjectsActions.svelte
- web/src/lib/ui/store-work/StoreProjectsFilterControls.svelte
- web/src/lib/ui/store-work/StoreProjectsFilters.svelte
- web/src/lib/ui/store-work/StoreProjectsNavigation.svelte
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Svelte5.57.1;Vitest4.1.11;Storybook10.5.7;released Design Core0.2.0.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No storage/object/API contract changes. Existing Projects hash q and UTF-8 query limit retained.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Apply standard Thread-reply sidebar to Agents and complete Community Focus structural sheets.
<!-- /evidence-field -->

<!-- evidence-field:artifacts markdown -->
## Artifacts

/tmp/jj-tab-search-*.log; /tmp/jj-tab-search.png.
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
pnpm test:storybook catalog/components/store-work/ProjectsFilters.stories.svelte
```

**Notes**

Before2/2 new desktop/mobile top-tab search contracts fail against old header/popover controls.
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
pnpm test:storybook catalog/components/store-work/ProjectsFilters.stories.svelte
```

**Notes**

After1/2 passes; noninteractive form keyboard warning moved onto input. Close visibility assertion ran before Svelte DOM flush; retained assertion waits for visible state.
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
pnpm test:storybook catalog/components/store-work/ProjectsFilters.stories.svelte catalog/components/store-work/ProjectsLayouts.stories.svelte catalog/components/store-work/ProjectsActivity.stories.svelte
```

**Notes**

8/9 selected stories pass; existing close visibility assertion repaired as above. ProjectsLayouts filename is absent; actual ownership-layout story resides in ProjectsHub and is covered by full direct store-work family.
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
pnpm test:storybook catalog/components/store-work/ProjectsFilters.stories.svelte
```

**Notes**

2/2 desktop/mobile top-row search, border/ring, tabs restoration, focus and additional filter checks pass.
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
pnpm web:check; pnpm catalog:check
```

**Notes**

Zero errors/warnings;134 catalog modules.
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
CUA read-only live search Enter/reload/Close on HMR7460
```

**Notes**

Recovery filters Projects and exact q restores on cold reload. Input computed border0px/shadownone. Close clears q, restores tabs and focuses Search projects. Screenshot /tmp/jj-tab-search.png.
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
pnpm test:storybook catalog/components/store-work
```

**Notes**

52/54 stories pass. Two Activity filter stories still asserted old section-header placement; updated their exact geometry parent to tab row.
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
pnpm test:storybook catalog/components/store-work/ProjectsActivityControls.stories.svelte
```

**Notes**

3/3 repaired direct Activity control stories pass; final combined direct suite follows.
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
pnpm spec:first
```

**Notes**

Generic web protected-path map requires API chapter. Added existing q submission/clear/admission semantics to api.md; no API/schema change.
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
pnpm test:storybook catalog/components/store-work
```

**Notes**

54/54 directly affected Projects story/interaction/accessibility scenarios pass after placement updates.
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
pnpm spec:first; pnpm spec:validate; pnpm check:architecture; pnpm check:progress
```

**Notes**

Corrected mapped API gate9 protected files/3 chapters;5 validators/21chapters/147requirements;761production modules;120task bookkeeping. Changed files formatted.
<!-- /run -->
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


