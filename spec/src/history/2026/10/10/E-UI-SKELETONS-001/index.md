---
schema: 1
id: E-UI-SKELETONS-001
---

# E-UI-SKELETONS-001

<!-- evidence-field:id markdown -->
## Id

E-UI-SKELETONS-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M24
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-005
- JST-UI-006
- JST-UI-015
- JST-API-001
- JST-DOC-001
- JST-GOV-002
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-10
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/AgentSessions.stories.svelte
- catalog/components/store-application/RepositoryLoading.stories.svelte
- catalog/components/store-work/ProjectsActivity.stories.svelte
- docs/maintenance/coverage.json
- docs/site/guides/browsing.md
- docs/ui-provenance.json
- spec/src/current/contracts/ui/07-application-startup.md
- spec/src/current/progress/M24/index.md
- spec/src/current/progress/M24/view-loading-review.md
- web/src/lib/ui/community-agents/CommunityAgents.svelte
- web/src/lib/ui/community-inbox/CommunityInboxSkeleton.svelte
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryFiles.svelte
- web/src/lib/ui/community-skeleton/CommunitySkeleton.svelte
- web/src/lib/ui/store-application/StoreApplicationAgents.svelte
- web/src/lib/ui/store-application/StoreApplicationMain.svelte
- web/src/lib/ui/store-application/StoreLazyView.svelte
- web/src/lib/ui/store-application/StorePageSkeleton.svelte
- web/src/lib/ui/store-application/StoreRepositoryApplication.svelte
- web/src/lib/ui/store-work/StoreProjects.svelte
- web/src/lib/ui/store-work/StoreProjectsDirectory.svelte
- web/src/lib/ui/store-work/StoreProjectsLoading.svelte
- web/src/lib/ui/store-work/StoreProjectsRepositories.svelte
- web/src/lib/ui/store-work/StoreProjectsReviews.svelte
- web/src/lib/ui/store-work/StoreWorkRowsSkeleton.svelte
- web/src/lib/ui/store-work/StoreWorkTaskSkeleton.svelte
- web/tests/browser/lazy-views.spec.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/browser/lazy-views.spec.ts#cold Threads import preserves the inbox columns before the view arrives
- web/tests/browser/lazy-views.spec.ts#saved inbox width survives the module-to-page transition
- web/tests/browser/lazy-views.spec.ts#saved open file tree is represented before the Files module loads
- web/tests/browser/lazy-views.spec.ts#Files deep-link keeps a source skeleton through the initial repository read
- web/tests/browser/lazy-views.spec.ts#mobile Threads loading keeps the inbox regions within the viewport
- web/tests/lazy-view.test.ts#view imports wait for demand and share completion across consumers
- web/tests/lazy-view.test.ts#a failed view import rejects all consumers and allows a fresh explicit retry
- scripts/web-manifest.test.mjs#browser manifest retains dynamic chunks and fonts while excluding unreferenced workers
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation


<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

[]
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Node 26.9.0; pnpm 10.32.1; jj 0.45.1; Vite 8.3.1; Svelte 5.57.1; installed Design Core 0.1.2 from the retained local built archive. No dependency or package-publication change.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Existing route hashes, version-1 device layout records, native API protocol 1, Vite asset manifest and evidence schema 1 remain unchanged.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Default workspace, UI-only. Module imports previously used one generic Details skeleton before page-specific initial reads. Projects list tabs also used activity timelines; the agent gallery placed placeholders beneath a separate static action row. Named route fallbacks now compose the live inbox, file/browser and history regions, matching saved dimensions and initial list/grid/detail geometry. Known selection-empty states remain available.

Community Skeleton and repository loading sources were inspected first; Design Core Skeleton and its official shadcn documentation were inspected second. New components are locally authored and declared in provenance. No upstream defect or source dependency was introduced. Source authority, authentication, query caching, event freshness, dispatch, drafts and pending-command identities are unchanged.

The directly affected browser and four Storybook families form the complete focused loading surface. Full repository, Rust and all-Storybook aggregates are out of scope under the repository targeted policy. No visual baseline was approved. The temporary QA browser tab was closed after restoring resource interception/cache settings; the user's tab and fixture were retained.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Retain delayed-module, retained-layout and initial-data regressions. Independent M24 live adapter, permission/cancellation, platform and baseline gates remain open.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: JST-UI-005, JST-UI-006 and JST-UI-015 already require representative loading, retained layouts and owned presentation. The startup chapter clarifies both loading phases; this repairs an implementation gap without changing native contracts, dispatch, credentials or refresh authority. JST-API-001 DTO/asset admission, JST-DOC-001 manual qualification and JST-GOV-002 owning-topic and evidence authority remain unchanged; the browsing manual and source review are current.
```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M24 remains In progress; view-loading-review.md records the enclosing change mpyvzmqo, focused scope and retained independent gates. No adapter, installed permission/cancellation, desktop or baseline gate is advanced.
```
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [Validation runs](details-02.md)
- [Verification follow-ups](details-03.md)

