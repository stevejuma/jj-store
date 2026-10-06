---
schema: 1
id: E-UI-101
---

# E-UI-101

<!-- evidence-field:id markdown -->
## Id

E-UI-101
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4.9
- M4.10
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-003
- JST-UI-005
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-05
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Row-local revision graph layout and horizontal scrolling
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change zpvqmolk
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

One focused wide-graph story observes identical avatar positions (0px spread). Two corrected upstream native/styled Body stories observe forced vertical orientation. The original Linux page uses 320px padding for every visible row and approximately 30px of text width.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; Node 26.9.0; pnpm 10.32.1; Svelte 5.57.1; Vitest 4.1.11; Storybook 10.5.7. Existing ignored Design Core 0.1.2 local source substitution; optional Body API implemented and committed separately in source change wsqpzxvk. Manifest ranges remain unchanged.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

UI layout and additive Design Core Body orientation only. No API/schema, native/companion/storage/WAL/receipt, route, credential or installed executable changes. Existing preview fixture and source repositories retained.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Reuse per-row occupied-lane padding. Reserve graph width plus 20rem of readable content; enable both scroll axes on JJ/Op/Evo column bodies through the public upstream API. Retain existing SVG lanes, measured heights, selected files, virtualization and paging. Source-first API extension is a separate Design Core change, not a consumer override. No advanced-query work.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- web/src/lib/ui/community-projects/CommunityProjectsRevisionGraph.svelte
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryLog.svelte
- web/src/lib/ui/store-application/StoreEvolutionColumn.svelte
- catalog/components/community-projects/CommunityProjectsRevisionGraph.stories.svelte
- catalog/components/store-application/RepositoryReview.stories.svelte
- spec/src/ui.md
- spec/src/api.md
- spec/src/milestones.md
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-UI-101 details 2](details-02.md)
- [E-UI-101 details 3](details-03.md)
