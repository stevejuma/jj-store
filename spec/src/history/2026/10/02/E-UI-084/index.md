---
schema: 1
id: E-UI-084
---

# E-UI-084

<!-- evidence-field:id markdown -->
## Id

E-UI-084
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M9.5
- M11.4
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-02
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Shared Threads directory search and borderless toolbar
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

Enclosing Jujutsu change tlnzznzu
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-005
- JST-UI-006
- JST-UI-015
- JST-API-006
- JST-GOV-002
- JST-GOV-003
- JST-GOV-007
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Visual baselines remain visual-pending. No milestone status promotion. Rust,
  root aggregates, production bundle builds, backend rebuild/reset/restart, main
  landing, dependency changes and publication are out of scope under the
  targeted UI verification policy.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Reuse the appearance workspace and existing Vite HMR. Inspect Community first, then released Design Core and official shadcn-svelte Input Group/Button guidance. Extract the existing Projects search into one owned toolbar component and reuse it for the repository Threads directory. Preserve the circular outlined trigger size/position, themed resting background, hover Close, conditional smaller Clear, borderless input, Escape and focus restoration. Remove the Threads toolbar bottom border. Enter filters already loaded authorized thread titles and directory previews case-insensitively; continuation remains available and later pages join the filter. Preserve the query through thread selection and returning to the directory; clear on repository switch. No-match copy differs from an empty repository. Projects retains its existing search behavior. Remove obsolete Projects search CSS. Production remains independent of catalog and Community. Direct Threads and Projects navigation consumers are one UI verification surface.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-threads/StoreThreadSearch.stories.svelte
- catalog/components/store-work/ProjectsFilters.stories.svelte
- catalog/fixtures/StoreThreadsFixture.svelte
- spec/src/api.md
- spec/src/milestones.md
- spec/src/ui.md
- web/src/lib/ui/StoreToolbarSearch.svelte
- web/src/lib/ui/forum-topic-list/ForumTopicList.svelte
- web/src/lib/ui/store-threads/StoreThreadHeader.svelte
- web/src/lib/ui/store-threads/StoreThreads.svelte
- web/src/lib/ui/store-work/StoreProjectsNavigation.svelte
- web/src/lib/ui/store-work/community-work-layout.css
- web/src/lib/ui/store-work/store-projects.css
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Node26.9.0; pnpm10.32.1; Svelte5.57.1; Vitest4.1.11; Storybook10.5.7; existing Design Core ^0.2.0. Dependencies and owned Community provenance unchanged.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No persistence or protocol changes. Threads query stays in memory; no new query parameter, indexed-search/API contract, native objects, manifest, WAL or receipts. Exact native thread selection and posting permissions unchanged.
<!-- /evidence-field -->

<!-- evidence-field:artifacts markdown -->
## Artifacts

/tmp/jj-thread-search-*.log; /tmp/jj-thread-search-live.jpg
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Retain visual-pending and the committed appearance workspace for further UI changes.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-UI-084 details 2](details-02.md)
