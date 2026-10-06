---
schema: 1
id: E-UI-067
---

# E-UI-067 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/TaskScroll.stories.svelte
```

**Notes**

Before regression:3/3 fail because the original task detail has no separate Task content scroll region. /tmp/jj-task-scroll-before.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/TaskScroll.stories.svelte
```

**Notes**

Initial repair:3/3 desktop950px, narrow380px and read-only380px. Fixed title/composer measurements, Activity collapse and draft retention. /tmp/jj-task-scroll-after.log.
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
pnpm exec prettier --write catalog/components/store-work/TaskScroll.stories.svelte catalog/fixtures/ProjectsHubFixture.svelte spec/src/api.md spec/src/projects.md spec/src/ui.md web/src/lib/ui/store-work/StoreProjects.svelte web/src/lib/ui/store-work/StoreWork.svelte web/src/lib/ui/store-work/StoreWorkComments.svelte web/src/lib/ui/store-work/StoreWorkTaskDetail.svelte web/src/lib/ui/store-work/task-detail-layout.css
```

**Notes**

Initial10 changed source, fixture, story and canonical files formatted.
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
pnpm web:check
```

**Notes**

Initial and final checks:0 errors/warnings. /tmp/jj-task-scroll-web-check.log; /tmp/jj-task-scroll-web-check-final.log.
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
pnpm catalog:check
```

**Notes**

Initial and final checks:0 errors/warnings;130 catalog modules. /tmp/jj-task-scroll-catalog-check.log; /tmp/jj-task-scroll-catalog-check-final.log.
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
pnpm check:architecture
```

**Notes**

Initial formatted StoreProjects orchestration exceeded300 counted lines at305. Extracted the conditional page-scroll wrapper to StoreProjectsViewport; no limit weakened.
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
pnpm exec prettier --write web/src/lib/ui/store-work/StoreProjects.svelte web/src/lib/ui/store-work/StoreProjectsViewport.svelte; pnpm check:architecture
```

**Notes**

Extracted wrapper formatted;742 modules within architecture limits.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/TaskScroll.stories.svelte catalog/components/store-work/ProjectsHub.stories.svelte catalog/components/store-work/ProjectsActivity.stories.svelte catalog/components/store-work/StoreWork.stories.svelte catalog/components/store-work/NavigationPersistence.stories.svelte catalog/components/store-work/ProjectsDirectory.stories.svelte catalog/components/store-work/ProjectsFilters.stories.svelte catalog/components/store-work/ProjectsActivityControls.stories.svelte
```

**Notes**

39 pass/40. Existing read-only title-row check caught refresh-button alignment versus newly padded header bounds. Moved spacing to header margins to preserve its content edge; assertion retained. /tmp/jj-task-scroll-consumers.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/TaskScroll.stories.svelte catalog/components/store-work/ProjectsHub.stories.svelte catalog/components/store-work/ProjectsActivity.stories.svelte catalog/components/store-work/StoreWork.stories.svelte catalog/components/store-work/NavigationPersistence.stories.svelte catalog/components/store-work/ProjectsDirectory.stories.svelte catalog/components/store-work/ProjectsFilters.stories.svelte catalog/components/store-work/ProjectsActivityControls.stories.svelte
```

**Notes**

Pre-restoration40/40 in8 direct families on a fresh Vitest browser/Vite process, with blocking accessibility checks. Fixed desktop/narrow/read-only scroll, title-row alignment, reply/edit/reactions/unknown outcomes, retained refresh drafts and navigation. /tmp/jj-task-scroll-consumers-final.log.
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
jj --version; node --version; pnpm --version
```

**Notes**

JJ0.45.1;Node26.9.0;pnpm10.32.1.
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
cua_repl: fresh hidden task detail; inspect and measure fixed rows; scroll content; capture screenshot; close temporary tab
```

**Notes**

Live HMR cold page settles from task skeleton. Content scrollTop0 to588.5px (scrollHeight1033/clientHeight445); header top67.3984375/bottom111.7890625 and footer top570/bottom711 unchanged. Task width=scrollWidth=1079px. AX scroll target was covered by a button; retargeted to the visible content at900,450. Read-only UI verification; no fixture publications. /tmp/jj-task-scroll-final.png.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/TaskScroll.stories.svelte -t TaskContentScrollSurvivesReturning
```

**Notes**

Additional navigation regression:restored task content scroll0px versus saved400px after moving the scroller. Bound the task viewport to the existing Projects scroll owner and captured the route key per listener. /tmp/jj-task-scroll-return-before.log.
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
pnpm exec prettier --write catalog/components/store-work/TaskScroll.stories.svelte web/src/lib/ui/store-work/StoreProjects.svelte web/src/lib/ui/store-work/StoreWorkTaskDetail.svelte; pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/TaskScroll.stories.svelte
```

**Notes**

Final4/4 task-scroll regressions include400px restored content scroll and retained draft after navigating to the task list and returning. /tmp/jj-task-scroll-return-after.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/TaskScroll.stories.svelte catalog/components/store-work/ProjectsHub.stories.svelte catalog/components/store-work/ProjectsActivity.stories.svelte catalog/components/store-work/StoreWork.stories.svelte catalog/components/store-work/NavigationPersistence.stories.svelte catalog/components/store-work/ProjectsDirectory.stories.svelte catalog/components/store-work/ProjectsFilters.stories.svelte catalog/components/store-work/ProjectsActivityControls.stories.svelte
```

**Notes**

Final41/41 in8 direct families after restored-scroll binding, with blocking accessibility checks. /tmp/jj-task-scroll-stories-final.log. Final web/catalog/architecture repeats also pass with0 errors/warnings and742 production/catalog modules.
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
cua_repl: fresh hidden final task page; measure and scroll content; Back to tasks; reopen selected task; capture screenshot; close temporary tab
```

**Notes**

Final live HMR page:header top67.3984375/bottom111.7890625 and footer top570/bottom711 unchanged; scrollTop0 to588.5 and restored588.5 after navigating away and back. Task width=scrollWidth=1028px. Final screenshot /tmp/jj-task-scroll-final.png. User tab and fixture preserved; no publications.
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
pnpm spec:first; pnpm spec:validate; pnpm check:progress; pnpm check:ui-ownership; pnpm check:architecture
```

**Notes**

8 protected files/4 canonical chapters;5 validators/21 chapters/147 requirements;120 tasks;ownership1 validator;742 modules. /tmp/jj-task-scroll-{spec-first,spec-validate,progress,ownership,architecture-final}.log. Repeated after final binding and ledger update without wider lanes.
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
pnpm exec prettier --write spec/src/milestones.md
```

**Notes**

E-UI-067 ledger formatted; repeated for final evidence-only update.
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
pnpm exec prettier --check catalog/components/store-work/TaskScroll.stories.svelte catalog/fixtures/ProjectsHubFixture.svelte spec/src/api.md spec/src/projects.md spec/src/ui.md web/src/lib/ui/store-work/StoreProjects.svelte web/src/lib/ui/store-work/StoreProjectsViewport.svelte web/src/lib/ui/store-work/StoreWork.svelte web/src/lib/ui/store-work/StoreWorkComments.svelte web/src/lib/ui/store-work/StoreWorkTaskDetail.svelte web/src/lib/ui/store-work/task-detail-layout.css spec/src/milestones.md
```

**Notes**

All12 changed files formatted. /tmp/jj-task-scroll-format-check.log. Repeated with pnpm exec prettier --write spec/src/ui.md spec/src/milestones.md and pnpm check:progress after final evidence update.
<!-- /run -->
<!-- /evidence-field -->

