---
schema: 1
id: E-UI-102
---

# E-UI-102

<!-- evidence-field:id markdown -->
## Id

E-UI-102
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4.9
- M4.10
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-005
- JST-UI-019
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-05
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Repository state toast and skeleton audit
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change uklvrtou
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

The graph-recovery producer still rendered preparation and cancellation paragraphs plus a body button. File summary/row metadata, paging and history search used loading text. Graph warnings, evolution feedback and index preparation progress/status used body callouts or rows.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Presentation-only audit of core browsing, operation/evolution views, history query and the index preparation controls delivered by the bounded-read work. Reuse StoreNotice, the existing session notification service and public Skeleton. One application-owned task toast survives Settings and page changes; isolated composites share one fallback host. Known target choices, validation and content empty states retain their controls. Current progress does not replay a dismissed running toast; final counters are shown in a separate persistent summary.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No backend query, task polling, API/schema, native/companion/storage/WAL/receipt, credential, fixture or installed executable changes. No foundational package changes in this slice. Existing local source dependency validation setup retained; no registry/release qualification claimed.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; Node 26.9.0; pnpm 10.32.1; Svelte 5.57.1; Vitest 4.1.11; Storybook 10.5.7.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/community-projects/CommunityProjectsRepositoryDirectory.stories.svelte
- catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte
- catalog/components/community-projects/CommunityProjectsRevisionGraph.stories.svelte
- catalog/components/store-application/StoreApplication.stories.svelte
- catalog/components/store-application/StoreEvolutionColumn.stories.svelte
- catalog/components/store-application/StoreKindPreparation.stories.svelte
- catalog/components/store-application/StoreRepositoryApplication.stories.svelte
- catalog/components/store-application/StoreRepositoryFeedback.stories.svelte
- catalog/fixtures/RepositoryPreparationFixture.svelte
- spec/src/api.md
- spec/src/ui.md
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryDirectory.svelte
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryDirectoryRows.svelte
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryFiles.svelte
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryLog.svelte
- web/src/lib/ui/store-application/StoreRepositoryLogNotices.svelte
- web/src/lib/ui/community-projects/CommunityProjectsVirtualRows.svelte
- web/src/lib/ui/store-application/StoreApplication.svelte
- web/src/lib/ui/store-application/StoreEvolutionColumn.svelte
- web/src/lib/ui/store-application/StoreIndexPreparationNotice.svelte
- web/src/lib/ui/store-application/StoreRepositoryApplication.svelte
- web/src/lib/ui/store-application/StoreRepositoryNotices.svelte
- web/src/lib/ui/store-history-query/StoreHistoryQuery.svelte
- web/src/lib/ui/store-settings/StoreGraphIndexSettings.svelte
- spec/src/milestones.md
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-UI-102 details 2](details-02.md)
- [E-UI-102 details 3](details-03.md)
