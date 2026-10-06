---
schema: 1
id: E-UI-071
---

# E-UI-071

<!-- evidence-field:id markdown -->
## Id

E-UI-071
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M11.4
- M9.5
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-02
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

focused mandatory layout selection and aligned project/agent rows
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

Enclosing Jujutsu change nyovpkun
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-015
- JST-UI-006
- JST-COLLAB-005
- JST-WORK-008
- JST-API-005
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Visual baseline approval remains separate. Root aggregate, Rust and unrelated
  UI lanes are out of scope under targeted verification policy.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

UI-only existing presentation. Shared card/list function binding rejects empty selection. Stable project copy/count columns preserve avatar and responsive alignment. Agent transcript/tool counts and duration align at the trailing second row with duration last. Tool actions remain within the tool row for visibility and continuous pointer access. Community and public Design Core sources inspected. No backend build, restart, fixture writes, main landing, release or publication.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Node26.9.0;pnpm10.32.1;JJ0.45.1;existing Svelte5.57.1,Storybook10.5.7,Vitest4.1.11 retained.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No API, native record, permission, route or persistence format changes.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Continue authorized search, structural focus sheet and scoped Projects Agents tabs.
<!-- /evidence-field -->

<!-- evidence-field:artifacts markdown -->
## Artifacts

/tmp/jj-layout-toggle-{before,after}.log; /tmp/jj-row-*.log
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/NativeAgentMenus.stories.svelte
- catalog/components/store-work/LayoutControls.stories.svelte
- catalog/components/store-work/ProjectsDirectory.stories.svelte
- spec/src/api.md
- spec/src/collaboration.md
- spec/src/milestones.md
- spec/src/projects.md
- spec/src/ui.md
- web/src/lib/ui/store-collaboration/StoreAgentSessions.svelte
- web/src/lib/ui/store-collaboration/StoreAgentToolReview.svelte
- web/src/lib/ui/store-collaboration/store-collaboration.css
- web/src/lib/ui/store-work/StoreWorkLayoutControls.svelte
- web/src/lib/ui/store-work/StoreWorkProjectRows.svelte
- web/src/lib/ui/store-work/community-work-rows.css
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/LayoutControls.stories.svelte
```

**Notes**

Before:2/2 fail when selected layout clears. Function binding fixes pointer checks; initial keyboard token {Space} was corrected to the actual space key without weakening selection assertions.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/ProjectsDirectory.stories.svelte catalog/components/store-application/NativeAgentMenus.stories.svelte
```

**Notes**

Before:project aligned description span absent and agent duration is not trailing.4 other stories pass.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/LayoutControls.stories.svelte catalog/components/store-work/ProjectsDirectory.stories.svelte catalog/components/store-application/NativeAgentMenus.stories.svelte
```

**Notes**

8/8 focused stories including repeated pointer/keyboard selection, responsive column coordinates and clickable tool copy menu.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work catalog/components/store-application/NativeAgentMenus.stories.svelte catalog/components/store-application/StoreCollaboration.stories.svelte catalog/components/store-application/AppearanceDiscussions.stories.svelte
```

**Notes**

59/60 pass. TaskScroll returning-scroll assertion sampled0 instead of400 in this batch; exact unmodified TaskScroll family rerun passes4/4. No assertion or product change was needed.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/TaskScroll.stories.svelte
```

**Notes**

4/4 unmodified TaskScroll stories pass in fresh process, including retained400px scroll and draft.
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
pnpm web:check; pnpm catalog:check; pnpm check:architecture; pnpm check:ui-ownership
```

**Notes**

0 errors/warnings;749 production modules;133 catalog modules;ownership validator passes.
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
pnpm spec:first
```

**Notes**

Mapped API chapter initially missing. Added presentation-only contract note; final gate rerun follows.
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
pnpm spec:first; pnpm spec:validate; pnpm check:progress; pnpm exec prettier --write changed source, stories and canonical chapters
```

**Notes**

Final mapped gate10 protected files/5 chapters;5 validators/21 chapters/147 requirements;120 tasks;changed source/stories/spec formatted. All reads/checks and exact focused results recorded; broader lanes are out of scope.
<!-- /run -->
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


