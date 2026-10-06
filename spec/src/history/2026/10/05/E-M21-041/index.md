---
schema: 1
id: E-M21-041
---

# E-M21-041

<!-- evidence-field:id markdown -->
## Id

E-M21-041
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M21
- M21.1
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-05
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

ynzpksxxuzsx
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Scoped Stacks setup margins, retained opaque canvas gutter and fixed Branches picker
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Existing JJ 0.45.1, Design Core foundation and Storybook 10.5.7; no dependency change.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Presentation only; no public API, persisted layout, TOML or native JJ format changes.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceSetup.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceCanvas.svelte
- web/src/lib/ui/store-stack-workspace/stack-workspace-layout.css
- catalog/components/store-stack-workspace/StoreStackWorkspaceSetup.stories.svelte
- catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte
- catalog/components/store-stack-workspace/StoreStackWorkspacePinning.stories.svelte
- spec/src/ui.md
- spec/src/milestones.md
- spec/src/api.md
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-020
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Two setup stories measured zero margins; two app-shell stories measured 12px canvas padding instead of parent content padding. The new Branches pinning story first failed because fixed was absent. Live browser confirmed canvas 12px and parent 0px.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Only Stacks setup and horizontal canvas presentation on both tabs. A 12px leading gutter outside the viewport stays beside fixed columns and excludes scrolling content; canvas and content have zero leading padding. Nested scrollbars unchanged. Branches uses public fixed. Focused UI policy excludes Rust, aggregate suites, package publication and visual comparisons/baselines.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M21-041 details 2](details-02.md)
- [E-M21-041 details 3](details-03.md)
