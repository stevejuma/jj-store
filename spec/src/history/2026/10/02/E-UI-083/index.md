---
schema: 1
id: E-UI-083
---

# E-UI-083

<!-- evidence-field:id markdown -->
## Id

E-UI-083
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4.13
- M9.5
- M11.4
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-02
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Persisted page disclosures and transparent sidebar toggles
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

Enclosing Jujutsu change zqzuymro
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
  landing, dependency changes and publication are out of scope for this UI-only
  slice.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Reuse the appearance workspace and existing Vite HMR. Community shell/disclosure persistence inspected first, released Design Core Button tokens and Collapsible API second. Expanded ghost buttons previously acquired a muted background; header sidebar actions now use public transparent Button tokens with unchanged expansion semantics, size, icons, hover and focus. Projects collection groups, repository Agent sessions, task Description/Subtasks/Activity, review Description/Files/Commits/Activity/agent context and Op Log operation summary retain choices across tab switches, remount and reload. Owned context uses stable scope/entity/section identities, omits filters and operation pins, and persists bounded opaque SHA256 keys plus booleans only. Corrupt or unavailable storage falls back safely. Explicit task-comment navigation opens Activity; independently mounted composers and existing drafts remain intact. Native helpers and fixtures add no Community dependency. Directly affected UI story families and focused storage tests form the complete verification surface.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-work/DisclosurePersistence.stories.svelte
- catalog/fixtures/DisclosurePersistenceFixture.svelte
- spec/src/api.md
- spec/src/milestones.md
- spec/src/ui.md
- web/src/lib/ui/community-app-shell/CommunityAppShell.svelte
- web/src/lib/ui/community-app-shell/right-sidebar.css
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryLogDiff.svelte
- web/src/lib/ui/disclosure-layout.svelte.ts
- web/src/lib/ui/disclosure-storage.ts
- web/src/lib/ui/store-application/StoreApplication.svelte
- web/src/lib/ui/store-application/StoreApplicationHeaderActions.svelte
- web/src/lib/ui/store-application/entity-context.ts
- web/src/lib/ui/store-collaboration/StoreAgentSessions.svelte
- web/src/lib/ui/store-collaboration/StoreNativeAgentContext.svelte
- web/src/lib/ui/store-collaboration/StoreReviewDetail.svelte
- web/src/lib/ui/store-work/StoreProjects.svelte
- web/src/lib/ui/store-work/StoreProjectsAgentDirectory.svelte
- web/src/lib/ui/store-work/StoreProjectsDirectory.svelte
- web/src/lib/ui/store-work/StoreProjectsRepositories.svelte
- web/src/lib/ui/store-work/StoreProjectsReviews.svelte
- web/src/lib/ui/store-work/StoreWorkCollection.svelte
- web/src/lib/ui/store-work/StoreWorkComments.svelte
- web/src/lib/ui/store-work/StoreWorkOwnershipGroups.svelte
- web/src/lib/ui/store-work/StoreWorkSubtasks.svelte
- web/src/lib/ui/store-work/StoreWorkTaskDetail.svelte
- web/tests/vite/disclosure-layout.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Node26.9.0; pnpm10.32.1; Svelte5.57.1; Vitest4.1.11; Storybook10.5.7; existing Design Core ^0.2.0 and @noble/hashes ^2.4.0. Dependencies and owned Community provenance unchanged.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

New device-local jj-store/layout/v1/disclosures JSON version1 with at most1024 opaque64hex keys and boolean flags. No credentials, display names, repository IDs, selections, snapshots, cursors, drafts or command data stored. Native API/OpenAPI, routes, storage objects, manifest, WAL and receipts unchanged.
<!-- /evidence-field -->

<!-- evidence-field:artifacts markdown -->
## Artifacts

/tmp/jj-disclosures-*.log; /tmp/jj-disclosures-live.jpg
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Retain visual-pending and the committed appearance workspace for further UI changes.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-UI-083 details 2](details-02.md)
