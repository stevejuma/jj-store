---
schema: 1
id: E-REPOSITORY-TABS-SKELETON-001
---

# E-REPOSITORY-TABS-SKELETON-001

<!-- evidence-field:id markdown -->
## Id

E-REPOSITORY-TABS-SKELETON-001
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-16
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M21.1
- M4.9
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-020
- JST-UI-005
- JST-UI-010
- JST-GOV-008
- JST-API-001
- JST-API-005
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

repository-tab-loading-regression
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

Enclosing Jujutsu change pwtpnxrs; no future commit hash is required.
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Clean working copy on nqrwyvsr/c43a8761. All focused Stacks views used an unframed graph while connecting, discovering or reading the first workspace. Generic page fallback had no Workspace composition. Exact four-view startup/regression family: five failing cases before the fix (missing selected-view and canvas geometry).
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-stack-workspace/StoreStackWorkspaceLoading.stories.svelte
- catalog/fixtures/StackWorkspaceLoadingApplicationFixture.svelte
- docs/ui-provenance.json
- spec/src/current/contracts/api/16-browsing-ui-execution.md
- spec/src/current/contracts/ui/04-local-stack-workspace-catalog.md
- spec/src/current/contracts/ui/07-application-startup.md
- spec/src/current/progress/M21/history/M21.1/index.md
- spec/src/current/progress/M21/index.md
- spec/src/current/progress/M4/history/M4.9/index.md
- spec/src/current/progress/M4/index.md
- spec/src/history/2026/10/16/E-REPOSITORY-TABS-SKELETON-001/SUMMARY.nav.md
- spec/src/history/2026/10/16/E-REPOSITORY-TABS-SKELETON-001/details-02.md
- spec/src/history/2026/10/16/E-REPOSITORY-TABS-SKELETON-001/details-03.md
- spec/src/history/2026/10/16/E-REPOSITORY-TABS-SKELETON-001/index.md
- spec/src/history/2026/10/16/SUMMARY.nav.md
- spec/src/history/2026/10/16/index.md
- web/src/lib/ui/store-application/StoreApplication.svelte
- web/src/lib/ui/store-application/StoreApplicationMain.svelte
- web/src/lib/ui/store-application/StorePageSkeleton.svelte
- web/src/lib/ui/store-application/StoreRepositoryHistorySkeleton.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackHistorySkeleton.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackLane.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackWorkspace.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceSkeleton.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceState.svelte
- web/src/lib/ui/store-stack-workspace/StoreStacksApplicationView.svelte
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; Node 24.10.0; pnpm 10.32.1; JJ 0.45.1; Svelte 5.57.1; Vitest 4.1.11; Storybook 10.5.7; Playwright Chromium headless fixture. No Rust or native storage formats changed.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Existing layout/v1 storage keys and Design Core Column Canvas V2 persistence remain unchanged. Only public saved widths/collapse state and existing route selections are read. No credentials, drafts, mutations or new persistence formats.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

UI-only loading presentation: Stacks connection/discovery/module/first-read states, generic history module fallback, directly affected application/route compositions, controlled loading fixture and interaction/accessibility coverage. Reuses owned Community compositions and public Design Core primitives. No full root aggregate, Rust builds/tests, storage/API contract generation, provider qualification or visual baseline updates are required under the targeted policy. The final phase also frames initial/per-segment bounded stack-history reads with the same bookmark-card placeholder; the loading fixture holds/release those reads to verify replacement by actual commit rows.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-REPOSITORY-TABS-SKELETON-001 details 2](details-02.md)
- [E-REPOSITORY-TABS-SKELETON-001 details 3](details-03.md)
