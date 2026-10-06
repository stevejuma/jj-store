---
schema: 1
id: E-M21-064
---

# E-M21-064

<!-- evidence-field:id markdown -->
## Id

E-M21-064
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

stack selection and file row presentation
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

Enclosing Jujutsu change klusvmur: Refine stack selection, scrolling and sidebar controls
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Toggle selected headers, commits, picker and target rows; retain drafts, reads and adjacent diffs; native graph insets and two-axis scrolling; copyable folder footer, secondary empty text, folder-root main Projects icon, pointer rows, shared 2px File Listing gaps.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-stack-workspace/StoreStackWorkspaceConflicts.stories.svelte
- catalog/components/store-stack-workspace/StoreStackWorkspaceFooter.stories.svelte
- catalog/components/store-stack-workspace/StoreStackWorkspaceGraph.stories.svelte
- catalog/components/store-stack-workspace/StoreStackWorkspaceInline.stories.svelte
- catalog/components/store-stack-workspace/StoreStackWorkspaceSelection.stories.svelte
- catalog/components/store-work/ProjectsHub.stories.svelte
- catalog/fixtures/StackWorkspaceGraphFixture.svelte
- spec/src/api.md
- spec/src/ui.md
- web/src/lib/controller/stack-workspace/selection-actions.ts
- web/src/lib/ui/community-navigation/icons.ts
- web/src/lib/ui/store-stack-workspace/StoreStackBookmarkCard.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackFolderCopy.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackLane.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackTargetHistory.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackWorkspace.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceCanvas.svelte
- web/src/lib/ui/store-stack-workspace/stack-workspace-layout.css
- web/src/lib/ui/store-stack-workspace/stack-workspace.css
- web/src/lib/ui/store-work/StoreProjects.svelte
- web/src/lib/ui/store-work/StoreProjectsBreadcrumb.svelte
- web/src/lib/ui/store-work/StoreProjectsRail.svelte
- web/tests/vite/stack-workspace-selection.test.ts
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; pnpm10.32.1; Svelte5.57.1; Storybook10.5.7; Vitest4.1.11; JJ0.45.1. Local Design Core source validation retained without manifest/lockfile changes.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Presentation-only; public wire contracts, local TOML, native JJ identities and graph, durable column state unchanged. Clipboard stays browser-local. Design Core adds the public CSS token --ui-diff-file-listing-row-gap default2px.
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Repeated selected rows remained selected; shared graph alignment hid per-row lanes; footer path was not an action; main Projects reused project glyph; File Listing backgrounds touched.
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/vite/stack-workspace-selection.test.ts#deselect only the activated
  stack source, retaining drafts, other selections and diffs
- web/tests/vite/stack-workspace-selection.test.ts#branch and target deselection
  preserves cached target history without commands
- web/tests/vite/stack-workspace-selection.test.ts#locked selections cannot be
  toggled while a command owns the UI
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M21-064 details 2](details-02.md)
- [E-M21-064 details 3](details-03.md)
