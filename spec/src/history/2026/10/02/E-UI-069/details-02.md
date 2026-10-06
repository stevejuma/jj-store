---
schema: 1
id: E-UI-069
---

# E-UI-069 details 2

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
pnpm exec prettier --write web/src/lib/ui/forum-topic-list/ForumTopicList.svelte web/src/lib/ui/forum-topic-list/forum-topic-list.css web/src/lib/ui/store-work/StoreWorkTaskDetail.svelte spec/src/ui.md spec/src/projects.md
```

**Notes**

/tmp/jj-empty-task-actions-format.log.
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
pnpm web:check; pnpm check:ui-ownership; pnpm check:architecture
```

**Notes**

0 errors/warnings;1 ownership validator/21 chapters/147 requirements;744 architecture modules. /tmp/jj-empty-task-actions-{web,ownership,architecture}.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-threads/StoreThreads.stories.svelte catalog/components/store-work/TaskScroll.stories.svelte catalog/components/store-work/StoreWork.stories.svelte catalog/components/store-work/ProjectsHub.stories.svelte catalog/components/store-work/ProjectsActivity.stories.svelte
```

**Notes**

30 task tests passed in4 families; Threads suite failed before collecting tests when Vite/Tailwind loaded virtual Design Core CSS as Svelte source (Invalid declaration). /tmp/jj-empty-task-actions-stories.log. No product/assertion changes were made for this import failure.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-threads/StoreThreads.stories.svelte
```

**Notes**

Exact failed family retry on fresh Vitest browser/Vite process:7/7 pass with blocking accessibility. /tmp/jj-empty-task-actions-threads-retry.log. Final combined direct-consumer round follows.
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
cua_repl: open temporary empty Threads/task detail tabs, measure Empty header centering, inspect task action background/icons, capture screenshots
```

**Notes**

Before:empty width576 vs viewport1028, left-aligned in a355.98px list within623px viewport. After:header center differs by0.004px horizontally and0px vertically from the remaining Empty area; list fills891px viewport at818px width, no scroll overflow. Task buttons use secondary theme fill with repository/edit/subtask icons, aria-hidden, enabled and unhovered. No writes submitted. /tmp/jj-empty-threads-centered.png; /tmp/jj-task-actions-filled.png.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-threads/StoreThreads.stories.svelte catalog/components/store-work/TaskScroll.stories.svelte catalog/components/store-work/StoreWork.stories.svelte catalog/components/store-work/ProjectsHub.stories.svelte catalog/components/store-work/ProjectsActivity.stories.svelte
```

**Notes**

Final37/37 in5 direct-consumer families, fresh Vitest browser/Vite process with blocking accessibility. Thread creation/navigation/replies/reactions and task actions/desktop/narrow/read-only scrolling/drafts pass. /tmp/jj-empty-task-actions-stories-final.log. Initial virtual stylesheet import failure does not recur.
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

Initial gate identified missing mapped spec/src/api.md chapter for protected UI changes. Added explicit retained callback/access/list/create-contract wording; no API implementation change. /tmp/jj-empty-task-actions-spec-first-initial.log.
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
pnpm spec:validate; pnpm check:progress; pnpm exec prettier --check web/src/lib/ui/forum-topic-list/ForumTopicList.svelte web/src/lib/ui/forum-topic-list/forum-topic-list.css web/src/lib/ui/store-work/StoreWorkTaskDetail.svelte spec/src/ui.md spec/src/projects.md spec/src/milestones.md
```

**Notes**

Initial5 validators/21 chapters/147 requirements;120 tasks;6 changed files formatted. /tmp/jj-empty-task-actions-{spec-validate,progress,format-check}.log. Final gate repeats after mapped API wording and evidence update.
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
cua_repl: close temporary tabs14 and15
```

**Notes**

Both verification tabs closed; original user tab preserved. HMR/backend/fixture remain unchanged.
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
pnpm exec prettier --write web/src/lib/ui/store-work/StoreWorkTaskDetail.svelte web/src/lib/ui/store-work/task-detail-layout.css spec/src/projects.md
```

**Notes**

/tmp/jj-empty-task-actions-gap-format.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/TaskScroll.stories.svelte catalog/components/store-work/StoreWork.stories.svelte
```

**Notes**

12/12 after repository action spacing, fresh Vitest browser/Vite with blocking accessibility. /tmp/jj-empty-task-actions-gap-stories.log.
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
cua_repl: open temporary task tab; inspect repository button margin and adjacent text gap; capture screenshot; close tab
```

**Notes**

Actual margin-inline-end and adjacent name gap both8px; secondary background and repository icon retained. /tmp/jj-task-actions-filled.png updated. Temporary tab17 closed, user tab preserved.
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
pnpm exec prettier --write spec/src/milestones.md; pnpm exec prettier --write spec/src/api.md spec/src/milestones.md
```

**Notes**

Ledger/evidence and mapped API wording formatted. /tmp/jj-empty-task-actions-ledger-format.log; /tmp/jj-empty-task-actions-ledger-final-format.log.
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
pnpm web:check; pnpm spec:first; pnpm spec:validate; pnpm check:progress; pnpm exec prettier --check web/src/lib/ui/forum-topic-list/ForumTopicList.svelte web/src/lib/ui/forum-topic-list/forum-topic-list.css web/src/lib/ui/store-work/StoreWorkTaskDetail.svelte web/src/lib/ui/store-work/task-detail-layout.css spec/src/ui.md spec/src/projects.md spec/src/api.md spec/src/milestones.md
```

**Notes**

Final spacing source and corrected mapped chapters:0 errors/warnings;4 protected files/4 canonical chapters;5 validators/21 chapters/147 requirements;120 tasks;all8 changed files formatted. /tmp/jj-empty-task-actions-web-final.log and /tmp/jj-empty-task-actions-{spec-first,spec-validate,progress,format-check}.log. Evidence formatting and consuming governance checks repeated before commit.
<!-- /run -->
<!-- /evidence-field -->

