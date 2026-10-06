---
schema: 1
id: E-UI-062
---

# E-UI-062

<!-- evidence-field:id markdown -->
## Id

E-UI-062
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M11.4
- M16
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-02
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

community-task-activity-comments-and-main-composer
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change mqppsqku
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Clean Appearance workspace mqppsqku above xvyrtsyt/7fa3f2a4. Task details use Focus/Split discussion side panel with an Open task discussion button; main Activity shows repository commits. User requests Community task comments under Activity and composer at the bottom of the main page, and explicitly removes the opener. Inspected CommunityProjects selectedTask Activity/comment/composer source at741b8285e2081f1c9af58afe5a9b72f538fad781 first, then Design Core public Collapsible. Owning UI/API/projects/governance and M11.4/M16 reviewed. Exact layout regression records the existing side panel failure.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- web/src/lib/ui/store-work/StoreWorkTaskDetail.svelte
- web/src/lib/ui/store-work/StoreWorkComments.svelte
- web/src/lib/ui/store-work/StoreProjects.svelte
- web/src/lib/ui/store-work/community-work-tasks.css
- web/src/lib/controller/production-projects-hub.svelte.ts
- web/src/lib/controller/projects-work-registry.svelte.ts
- catalog/components/store-work/StoreWork.stories.svelte
- catalog/components/store-work/ProjectsHub.stories.svelte
- catalog/fixtures/projects-hub-layout.ts
- web/tests/vite/projects-hub.test.ts
- web/tests/real/task-activity-layout.spec.ts
- docs/ui-provenance.json
- spec/src/ui.md
- spec/src/api.md
- spec/src/projects.md
- spec/src/spec-governance.md
- spec/src/verification.md
- spec/src/milestones.md
<!-- /evidence-field -->

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
jj --no-pager st; jj describe -m "Match Community task comments and composer layout"; scoped source/spec/reference inspection; jj --no-pager log -r @ --no-graph -T commit_id (Community); jj --no-pager diff --stat/git and final review
```

**Notes**

Reused Appearance workspace and existing Vite/backend. Main unrelated source work preserved. Native fixture read only; live user writes are independent. Exploratory wrong work/spec/helper paths were corrected from scoped file inventories. First guarded parent rewrite had a missing exact substring and wrote nothing; corrected rewrite succeeds.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/StoreWork.stories.svelte -t HierarchyAndDiscussion (before)
```

**Notes**

New exact regression finds the Task discussion side panel.1 failed,4 skipped. /tmp/jj-task-activity-before.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/StoreWork.stories.svelte -t HierarchyAndDiscussion (first after)
```

**Notes**

Layout assertions pass; immediate hidden-content assertion races Collapsible transition. Added waitFor for actual invisibility, preserving collapse/composer assertions. /tmp/jj-task-activity-regression.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/StoreWork.stories.svelte -t HierarchyAndDiscussion (corrected)
```

**Notes**

1 story passes,4 skipped: comments in main Activity, composer below, no side panel, reply context/editor instance/draft survive collapse. /tmp/jj-task-activity-regression-final.log.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_REAL_BASE_URL=http://127.0.0.1:7460 JJ_STORE_REAL_REPOSITORY=1bdb89c409c1b891ac7f180c01a265d5 JJ_STORE_REAL_PROJECT=38d1c167c245c42fd7d1818086a89ed1 JJ_STORE_REAL_TASK=c8a8573c0204be10644dcd77d16df2b8 pnpm exec playwright test --config web/playwright.real.config.ts web/tests/real/task-activity-layout.spec.ts (initial and final through final4)
```

**Notes**

Each of5 preliminary runs has1 failed browser test. Initial locator assumed a direct Collapsible content child; next used task-activity class shared by Subtasks. Scoped to Task activity region. Third reveals actual dropped commentId in Projects registry; exact unit added and production forwarding fixed. Fourth selected both comment wrapper and nested message; narrowed to role=listitem. Fifth passes functionality/focus but flags color contrast in the initially retained repository ID feed. Final composition matches Community task scope (comments only); repository commit feed remains in project Activity tab. No axe exclusion, ignored rule or token override. /tmp/jj-task-activity-browser{,-final,-final2,-final3,-final4}.log.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config web/vitest.config.ts web/tests/vite/projects-hub.test.ts -t "task comment links retain" (two attempts)
```

**Notes**

Two startup errors: nonexistent config. Read actual catalog config and reran the named regression under catalog-unit. No tests ran in these attempts. /tmp/jj-task-activity-focus-before{,2}.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/projects-hub.test.ts -t "task comment links retain" (before forwarding fix)
```

**Notes**

Exact regression receives null focusCommentId instead of the bound task comment.1 failed,10 skipped. /tmp/jj-task-activity-focus-before3.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/projects-hub.test.ts -t "task comment links retain"; pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/projects-hub.test.ts web/tests/vite/production-work.test.ts
```

**Notes**

Exact regression passes, then16 direct Projects hub/WorkStore controller units pass. Comment ID forwarding, unavailable identity, clearing focus, snapshots/caches, authorization, receipt/request and draft preservation remain covered. /tmp/jj-task-activity-focus-after.log and units-final.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/StoreWork.stories.svelte catalog/components/store-work/ProjectsHub.stories.svelte -t "HierarchyAndDiscussion|UnknownOutcome|NativeThreadedDiscussion|ProjectsRepositoriesAndTaskNavigation|ReadOnlyTask|TaskEditAndRetainedUnknownRequest|MobileTaskDetail" --reporter verbose (initial and final)
```

**Notes**

7 directly affected stories pass with interaction/accessibility/runtime guards;9 unrelated stories skipped. Fresh Vitest Vite/browser provider qualifies cold source mounting. Threaded/tombstoned/missing-parent comments, reply/edit, retained unknown request, hierarchy, read-only access and mobile detail preserved. Final rerun follows comment forwarding and Community Activity scope correction. /tmp/jj-task-activity-stories{,-final}.log.
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
JJ_STORE_REAL_BASE_URL=http://127.0.0.1:7460 JJ_STORE_REAL_REPOSITORY=1bdb89c409c1b891ac7f180c01a265d5 JJ_STORE_REAL_PROJECT=38d1c167c245c42fd7d1818086a89ed1 JJ_STORE_REAL_TASK=c8a8573c0204be10644dcd77d16df2b8 pnpm exec playwright test --config web/playwright.real.config.ts web/tests/real/task-activity-layout.spec.ts (final5)
```

**Notes**

1 live read-only browser test passes in12s:1351x988 light/Focus and390x844 dark/Split; main comments below Activity after Subtasks; no panel/opener; full-width bottom composer; keyboard disclosure collapse preserves draft; comment link reopens and focuses exact identity. Whole-page settled WCAG axe0 violations,0 runtime errors and no horizontal overflow. Desktop/mobile screenshots inspected. No native mutation or fixture reset. /tmp/jj-task-activity-browser-final5.log and desktop/mobile.png.
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
pnpm web:check (initial, final, final2); pnpm catalog:check (initial, final, final2); pnpm check:ui-ownership (initial and after provenance/source markers)
```

**Notes**

All web/catalog runs report0 errors/warnings;129 catalog modules. Ownership validates21 chapters/147 requirements. Three maintained task adaptations now carry enforced source markers and pinned Apache-2.0 provenance. /tmp/jj-task-activity-{web*,catalog*,ownership*}.log.
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
pnpm spec:first (initial)
```

**Notes**

Mapped spec-governance.md required for provenance manifest. Added actual task adaptation ownership contract; corrected gate passes. /tmp/jj-task-activity-spec-first.log.
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
pnpm exec prettier --write (changed source/docs files); pnpm spec:first (corrected)
```

**Notes**

Formatting commands pass; final spec-first passes12 protected files/4 mapped chapters. /tmp/jj-task-activity-format{,-final,-scope}.log and spec-first-final.log. Final ledger/document gates recorded after completion.
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
pnpm exec prettier --write spec/src/milestones.md spec/src/verification.md; pnpm exec prettier --check (all18 changed supported files); pnpm spec:validate; pnpm check:progress (initial and final ledger checks)
```

**Notes**

Formatting passes; canonical validation passes5 validators/21 chapters/147 requirements including UI ownership; progress passes120 tasks. Same-change ledger and verification map preserve M11.4 In progress and M16 Partial. /tmp/jj-task-activity-{format-ledger,format-check,spec-validate,progress}.log.
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

One UI task presentation and exact comment route handoff surface, including its direct Projects/WorkStore controller consumers. Tasks match Community same-task comment Activity and no longer embed a project repository commit feed; project Activity retains that feed unchanged. All functional/accessibility assertions on the final task surface pass. Initial repository-feed contrast observations belong to the unchanged project/repository feed and are not a qualification claim for that surface. Rust builds/tests, backend restarts, root aggregates, unrelated stories, native writes, fixture resets, public contract generation, package publication, production built-CSP/release qualification and visual baseline approval are out of scope. M11.4 remains In progress and M16 Partial; existing qualification gates remain separate.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Node26.9.0; pnpm10.32.1; Vitest4.1.11. Dependencies unchanged; public Design Core Collapsible and owned Community-derived composer/message rendering retained.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-UI-062 details 2](details-02.md)
