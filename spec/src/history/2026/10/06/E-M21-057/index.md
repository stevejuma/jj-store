---
schema: 1
id: E-M21-057
---

# E-M21-057

<!-- evidence-field:id markdown -->
## Id

E-M21-057
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

Enclosing Jujutsu change: Add shared conflict loading skeleton and merge editor settings
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Center the compact version picker in the public Merge Editor header; share a standard four-option persisted settings menu across native and workspace/Op Log conflict editors; default horizontal synchronization on, preserve drafts across comparison changes, keep text beneath gutters and show a three-pane skeleton during lazy file reads.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-mutations/StoreConflictEditor.stories.svelte
- catalog/components/store-stack-workspace/StoreStackWorkspaceConflictVersions.stories.svelte
- catalog/components/store-stack-workspace/StoreStackWorkspaceConflicts.stories.svelte
- catalog/fixtures/StackWorkspaceConflictApplicationFixture.svelte
- docs/maintenance/coverage.json
- docs/site/guides/editing.md
- spec/src/api.md
- spec/src/documentation.md
- spec/src/milestones.md
- spec/src/ui.md
- web/src/lib/ui/store-conflicts/StoreConflictSkeleton.svelte
- web/src/lib/ui/store-conflicts/StoreConflictSurface.svelte
- web/src/lib/ui/store-conflicts/StoreConflictText.svelte
- web/src/lib/ui/store-conflicts/menu-context.ts
- web/src/lib/ui/store-conflicts/merge-editor-settings.ts
- web/src/lib/ui/store-stack-workspace/StoreStackConflictVersionPicker.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackConflictVersions.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceConflict.svelte
- web/tests/merge-editor-settings.test.ts
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ 0.45.1; pnpm 10.32.1; Svelte 5.57.1; Design Core source 0.1.2: rmlqkuql/5c8be23b and draft-retention follow-up yuktqxxq/61dfda30 through the existing supported temporary validation dependency. Portable ^0.2.0 manifest and registry lockfile retained.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Browser-local jj-store/merge-editor/v1 stores version and wrap, syncHorizontalScroll, ignoreWhitespace, ignoreCase booleans only; earlier two-field records retain values and default new filters off. No drafts, content, credentials, HTTP/RPC, native commands, TOML, layout, manifest or WAL changes.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M21-057 details 2](details-02.md)
- [E-M21-057 details 3](details-03.md)
