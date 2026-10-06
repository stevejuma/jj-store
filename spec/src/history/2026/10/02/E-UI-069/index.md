---
schema: 1
id: E-UI-069
---

# E-UI-069

<!-- evidence-field:id markdown -->
## Id

E-UI-069
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M11.4
- M15.5
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-02
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

focused empty Threads alignment and task action presentation
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

Enclosing Jujutsu change qkppqmpx
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-017
- JST-UI-015
- JST-WORK-008
- JST-ARCH-004
- JST-GOV-007
- JST-API-005
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Functional checks pass. Visual baseline approval and existing
  released-registry qualification remain separate.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

UI-only changes in the existing web package. Inspected Community forum empty layout and task presentation first, Design Core Empty/Button and ScrollArea second. Empty Threads now fills the content viewport and uses the public Empty centering below the new-post control; populated forum rows remain unchanged. Task Open target repository/Edit task/Add subtask use public secondary buttons and decorative leading icons. Permissions, callbacks, disabled behavior, drafts and native IDs are retained. Existing focused Threads and task direct-consumer stories plus live HMR geometry checks cover the change. No new mirror tests for this reversible styling slice. Rust builds/tests, aggregate root lanes, unrelated Storybook families, release/built-asset qualification, visual baseline updates, fixture reset/reseed, backend restart, main landing and publication are out of scope under the targeted UI policy. Final user steering adds0.5rem inline-end margin to the repository action before its name.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Node26.9.0; pnpm10.32.1; JJ0.45.1; existing Svelte5.57.1, Storybook10.5.7, Vitest4.1.11 and Design Core public Empty/Button/ScrollArea retained.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No API, native identity, command, permission or persistence format changes. Empty-only layout modifier; public secondary sm buttons with aria-hidden icons and data-icon inline-start.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Review the committed slice through the preserved Vite HMR7460 workspace; visual baseline approval and released-registry qualification remain separate.
<!-- /evidence-field -->

<!-- evidence-field:artifacts markdown -->
## Artifacts

/tmp/jj-empty-task-actions-*.log; /tmp/jj-empty-threads-centered.png; /tmp/jj-task-actions-filled.png. Existing HMR7460, backend7462 and persistent fixture preserved.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- spec/src/api.md
- spec/src/milestones.md
- spec/src/ui.md
- spec/src/projects.md
- web/src/lib/ui/forum-topic-list/ForumTopicList.svelte
- web/src/lib/ui/forum-topic-list/forum-topic-list.css
- web/src/lib/ui/store-work/StoreWorkTaskDetail.svelte
- web/src/lib/ui/store-work/task-detail-layout.css
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-UI-069 details 2](details-02.md)
