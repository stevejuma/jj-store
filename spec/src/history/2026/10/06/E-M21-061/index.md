---
schema: 1
id: E-M21-061
---

# E-M21-061

<!-- evidence-field:id markdown -->
## Id

E-M21-061
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M21
- M21.1
- M21.2
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-06
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

native topology and focused history correction
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

Enclosing Jujutsu change lmpkknlw: Correct native stack topology and focused workspace history
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Separate inventory/saved membership/derived columns; group proven native dependencies and aliases; target-inclusive retained merge rewriting; atomic grouped unapply; conservative legacy cleanup preview; live/pinned focused history and managed-index Stacks reads.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-stack-workspace/StoreStackTopology.stories.svelte
- catalog/fixtures/StackTopologyFixture.svelte
- crates/jj-store-cli/tests/local_browser/managed_http.rs
- crates/jj-store-cli/tests/local_browser/managed_lifecycle.rs
- crates/jj-store-cli/tests/local_browser/ordinary_stacks.rs
- crates/jj-store-cli/tests/local_browser/stacks.rs
- crates/jj-store-http/src/local_browser.rs
- crates/jj-store-jj/src/local_browser/code_index.rs
- crates/jj-store-jj/src/local_stacks/cleanup.rs
- crates/jj-store-jj/src/local_stacks/mod.rs
- crates/jj-store-jj/src/local_stacks/preview.rs
- crates/jj-store-jj/src/local_stacks/state.rs
- crates/jj-store-jj/src/local_stacks/topology.rs
- crates/jj-store-jj/tests/local_stacks/topology.rs
- crates/jj-store-jj/tests/local_stacks.rs
- crates/jj-store-service/src/code_indexes.rs
- docs/maintenance/coverage.json
- docs/site/guides/browsing.md
- docs/site/reference/api/installation.md
- docs/site/reference/api/types.md
- docs/site/reference/transport.md
- docs/site/technical/local.md
- schemas/openapi.json
- spec/src/api.md
- spec/src/cli.md
- spec/src/compatibility.md
- spec/src/documentation.md
- spec/src/jj-integration.md
- spec/src/local-jj.md
- spec/src/mutations.md
- spec/src/ui.md
- web/src/lib/api/generated.ts
- web/src/lib/controller/controller-actions.ts
- web/src/lib/controller/production-route-restorer.ts
- web/src/lib/controller/production-stacks-history.ts
- web/src/lib/controller/production-stacks.svelte.ts
- web/src/lib/controller/production-store-actions.ts
- web/src/lib/controller/production-store-controller.svelte.ts
- web/src/lib/controller/production-store-refresh.ts
- web/src/lib/controller/stack-workspace/card-selection.ts
- web/src/lib/controller/stack-workspace/commit-placement.ts
- web/src/lib/controller/stack-workspace/controller-snapshot.ts
- web/src/lib/controller/stack-workspace/controller.svelte.ts
- web/src/lib/controller/stack-workspace/loaded-ranges.ts
- web/src/lib/controller/stack-workspace/local-mutations.ts
- web/src/lib/controller/stack-workspace/local-session.ts
- web/src/lib/controller/stack-workspace/local-state.ts
- web/src/lib/controller/stack-workspace/presentation.ts
- web/src/lib/controller/stack-workspace/production-adapter.ts
- web/src/lib/controller/stack-workspace/retained-diffs.ts
- web/src/lib/controller/stack-workspace/stack-segments.ts
- web/src/lib/controller/stack-workspace/types.ts
- web/src/lib/controller/store-route-fields.ts
- web/src/lib/controller/store-route-history.svelte.ts
- web/src/lib/controller/store-route-types.ts
- web/src/lib/controller/store-stacks-navigation.ts
- web/src/lib/controller/store-stacks-route-codec.ts
- web/src/lib/ui/store-stack-workspace/StoreStackBookmarkActions.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackBookmarkCard.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackLane.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackWorkspace.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceActions.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceCanvas.svelte
- web/src/lib/ui/store-stack-workspace/StoreStacksApplicationView.svelte
- web/tests/vite/production-stacks-application.test.ts
- web/tests/vite/production-stacks-lifecycle.test.ts
- web/tests/vite/stack-workspace-topology.test.ts
- web/tests/vite/store-route-history.test.ts
- web/tests/vite/store-stacks-navigation.test.ts
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ/jj-lib0.45.1; macOS ARM64; pnpm10.32.1; Svelte5.57.1; Storybook10.5.7; Vitest4.1.11. Existing local Design Core validation setup unchanged.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive optional LocalStacksState.topology and UnapplyBranches command; regenerated OpenAPI/TypeScript. Frontend-only stack_history=live|pinned. TOMLv1,bookmark identities,native change identities,manifest/WAL and column layout formats unchanged.
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Per-bookmark columns duplicated dependent branches; focused legacy link remained at eee00143;target trunk omitted when any branch applied;fresh merges accumulated seven obsolete heads. WIP was already a merge child. Existing15-test UI baseline did not cover these defects.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M21-061 details 2](details-02.md)
- [E-M21-061 details 3](details-03.md)
