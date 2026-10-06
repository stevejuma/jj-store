---
schema: 1
id: E-M21-042
---

# E-M21-042

<!-- evidence-field:id markdown -->
## Id

E-M21-042
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M21
- M21.1
- M21.2
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-05
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

usqkzsnlnnmswoxpqqlumuuxzkrlpltu
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Ordinary local workspace Settings and configuration-only removal
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ/jj-lib 0.45.1; local settings/command version 1; Node 26.9.0, pnpm 10.32.1, Cargo 1.98.1, Vitest 4.1.11. Foundation package ranges unchanged.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Added strict Update and Delete variants to existing version-1 local Stacks command; regenerated OpenAPI and TypeScript. .jj/jj-store/stacks.toml format remains version 1. Native identities, manifest, WAL, companion, layout and retained-executor formats unchanged.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte
- catalog/components/store-stack-workspace/StoreStackWorkspaceSettings.stories.svelte
- catalog/fixtures/StackWorkspaceApplicationFixture.svelte
- catalog/fixtures/stack-workspace/canvas-layout.ts
- catalog/fixtures/stack-workspace/data.ts
- catalog/fixtures/stack-workspace/preview.ts
- catalog/fixtures/stack-workspace/reducer.ts
- catalog/fixtures/stack-workspace/validation.ts
- crates/jj-store-cli/tests/local_browser/ordinary_stacks.rs
- crates/jj-store-jj/src/local_stacks/config.rs
- crates/jj-store-jj/src/local_stacks/mod.rs
- crates/jj-store-jj/tests/local_stacks.rs
- docs/maintenance/coverage.json
- docs/site/guides/settings.md
- docs/site/reference/api/installation.md
- docs/site/reference/transport.md
- docs/site/technical/local.md
- schemas/openapi.json
- spec/src/api.md
- spec/src/cli.md
- spec/src/compatibility.md
- spec/src/documentation.md
- spec/src/jj-integration.md
- spec/src/local-jj.md
- spec/src/milestones.md
- spec/src/ui.md
- web/src/lib/api/generated.ts
- web/src/lib/controller/stack-workspace/controller.svelte.ts
- web/src/lib/controller/stack-workspace/local-session.ts
- web/src/lib/controller/stack-workspace/local-state.ts
- web/src/lib/controller/stack-workspace/presentation.ts
- web/src/lib/controller/stack-workspace/types.ts
- web/src/lib/ui/community-channel-settings/CommunityChannelSettings.svelte
- web/src/lib/ui/store-application/StoreApplication.svelte
- web/src/lib/ui/store-application/StoreApplicationMain.svelte
- web/src/lib/ui/store-application/StoreApplicationRightPanels.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackWorkspace.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceActions.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackWorkspacePreview.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceSettings.svelte
- web/tests/real/stacks-browser.spec.ts
- web/tests/vite/production-stack-adapter.test.ts
- web/tests/vite/stack-workspace.test.ts
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-LOCAL-005
- JST-JJ-003
- JST-JJ-004
- JST-UI-020
- JST-API-001
- JST-API-002
- JST-DOC-002
- JST-GOV-002
- JST-GOV-003
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Native command enum lacked Update/Delete (three compile diagnostics); two new Settings stories failed because the menu item was absent. Settings had no name/target edit or active-workspace removal flow.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Active named workspace Settings reuses owned Threads sidebar identity/detail/danger-zone composition, bordered Settings field rows and branch Command View. Exact advisory previews retain canceled drafts and stale command identity. Native Update validates name and target, preserving merge/WIP on name-only edits and using existing WIP rebase on target changes. Delete removes only the TOML entry and active selection, retaining physical checkout, user and retention bookmarks and native unfinished work. Standard close restores focus; leaving Settings does not close destination Thread/Agent panels. Cross-layer checks cover only this engine/HTTP command and direct UI consumers. No new recovery/storage framework or backend graph/file/hunk capability.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M21-042 details 2](details-02.md)
- [E-M21-042 details 3](details-03.md)
