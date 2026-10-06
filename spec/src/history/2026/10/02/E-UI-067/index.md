---
schema: 1
id: E-UI-067
---

# E-UI-067

<!-- evidence-field:id markdown -->
## Id

E-UI-067
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4.9
- M11.4
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-02
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

focused task detail fixed header and composer regression
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

Enclosing Jujutsu change kzzwpwsz
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-005
- JST-UI-015
- JST-API-005
- JST-WORK-008
- JST-ARCH-004
- JST-GOV-007
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Functional checks pass. Visual baseline approval and existing
  released-registry Design Core qualification remain separate gates.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

UI-only task-detail layout within the existing web package. Community task detail inspected first, Design Core public ScrollArea second. Keep the title/header and writable composer in fixed grid rows, with one intervening ScrollArea for metadata, description, subtasks and Activity/comments. StoreWorkComments retains draft/reply/edit state while supplying activity/composer snippets to the page layout. The Projects hub omits its outer page scroller for accepted task detail and binds the content viewport to the existing per-route scroll cache; the legacy shared WorkStore host uses the same fixed rows. Focused direct interaction/accessibility stories and a fresh live HMR task page verify desktop/narrow/read-only behavior and retained comments/drafts. Rust builds/tests, root aggregate spec:check/check/test/build, unrelated Storybook families, built-bundle release qualification, visual baseline updates, main landing, package publication, fixture reset/reseed and backend restart are out of scope under the targeted UI policy.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Node26.9.0; pnpm10.32.1; JJ0.45.1; Svelte5.57.1; Storybook10.5.7; Vitest4.1.11; Playwright1.63.0. Existing private Design Core source hook and tracked registry range^0.2.0 retained.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No API, operation pin, WorkStore identity, WAL, manifest, cursor, command, version or permission changes. Comment ownership, reply/edit context and drafts remain in the same native adapter. Read-only task detail has a fixed header and content scroll without a composer. Task navigation restores content scroll from the existing bounded in-session map; no browser-storage persistence is added.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Review the committed slice through existing Vite HMR7460 in the Appearance workspace; visual baselines and registry qualification remain separate.
<!-- /evidence-field -->

<!-- evidence-field:artifacts markdown -->
## Artifacts

/tmp/jj-task-scroll-*.log; /tmp/jj-task-scroll-final.png. Main persistent fixture and HMR/backend preserved.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-work/TaskScroll.stories.svelte
- catalog/fixtures/ProjectsHubFixture.svelte
- spec/src/api.md
- spec/src/projects.md
- spec/src/ui.md
- web/src/lib/ui/store-work/StoreProjects.svelte
- web/src/lib/ui/store-work/StoreProjectsViewport.svelte
- web/src/lib/ui/store-work/StoreWork.svelte
- web/src/lib/ui/store-work/StoreWorkComments.svelte
- web/src/lib/ui/store-work/StoreWorkTaskDetail.svelte
- web/src/lib/ui/store-work/task-detail-layout.css
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-UI-067 details 2](details-02.md)
