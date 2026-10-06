---
schema: 1
id: E-M21-060
---

# E-M21-060

<!-- evidence-field:id markdown -->
## Id

E-M21-060
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

ui
<!-- /evidence-field -->

<!-- evidence-field:change markdown -->
## Change

Enclosing Jujutsu change: Show the conflict banner only on the Workspace tab
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Scope the top Resolve conflicts banner to the actual selected Workspace tab while retaining rail and notification actions across focused tabs. Align the standalone conflict story with Workspace entry and repair its existing narrow overflow assertion.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-stack-workspace/StoreStackWorkspace.stories.svelte
- catalog/components/store-stack-workspace/StoreStackWorkspaceConflicts.stories.svelte
- docs/maintenance/coverage.json
- docs/site/guides/browsing.md
- spec/src/api.md
- spec/src/documentation.md
- spec/src/milestones.md
- spec/src/ui.md
- web/src/lib/ui/store-stack-workspace/StoreStackWorkspace.svelte
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ 0.45.1; pnpm 10.32.1; Svelte 5.57.1; Storybook 10.5.7; Vitest 4.1.11. Existing Design Core local validation dependency unchanged.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No API, command, TOML, native identity, layout, browser storage, manifest or WAL format change. One presentation condition uses focusedView with controller.page fallback.
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Main clean at empty kpsvovzq/d9aec901 above integrated zzpnwqpp/4b5539c9. Banner was outside canvas/history content and rendered on all four focused tabs. The retained canvas page stays Workspace during history, so checking controller.page alone is insufficient.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M21-060 details 2](details-02.md)
- [E-M21-060 details 3](details-03.md)
