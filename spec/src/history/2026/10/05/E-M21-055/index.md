---
schema: 1
id: E-M21-055
---

# E-M21-055

<!-- evidence-field:id markdown -->
## Id

E-M21-055
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

<!-- evidence-field:kind markdown -->
## Kind

cross-layer
<!-- /evidence-field -->

<!-- evidence-field:change markdown -->
## Change

Enclosing Jujutsu change: Reuse repository diff context and conflict editor in Stacks
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Retain bounded text context in draggable hunk cards and share the existing Op Log conflict surface/browser/text/term presentation. Expose ordered bounded multi-sided local text versions through the existing guarded conflict reader and complete-file native Resolve command. Multi-sided conflicts also show the editable Merge Editor immediately, with explicit pairwise native-version comparison and whole-file staging.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-stack-workspace/StoreStackWorkspace.stories.svelte
- catalog/components/store-stack-workspace/StoreStackWorkspaceConflictVersions.stories.svelte
- catalog/components/store-stack-workspace/StoreStackWorkspaceConflicts.stories.svelte
- catalog/fixtures/StackWorkspaceConflictApplicationFixture.svelte
- catalog/fixtures/StackWorkspaceFixture.svelte
- crates/jj-store-jj/src/local_stacks/conflicts.rs
- crates/jj-store-jj/tests/local_stacks.rs
- docs/maintenance/coverage.json
- docs/site/guides/browsing.md
- docs/site/guides/editing.md
- docs/site/technical/local.md
- docs/ui-provenance.json
- spec/src/api.md
- spec/src/documentation.md
- spec/src/jj-integration.md
- spec/src/local-jj.md
- spec/src/milestones.md
- spec/src/spec-governance.md
- spec/src/ui.md
- web/src/lib/controller/stack-workspace/hunk-context.ts
- web/src/lib/controller/stack-workspace/local-session.ts
- web/src/lib/controller/stack-workspace/local-state.ts
- web/src/lib/controller/stack-workspace/read-presentation.ts
- web/src/lib/controller/stack-workspace/types.ts
- web/src/lib/ui/store-application/StoreApplication.svelte
- web/src/lib/ui/store-conflicts/StoreConflictEditor.svelte
- web/src/lib/ui/store-conflicts/StoreConflictPath.svelte
- web/src/lib/ui/store-conflicts/StoreConflictSurface.svelte
- web/src/lib/ui/store-conflicts/StoreConflictTerms.svelte
- web/src/lib/ui/store-conflicts/StoreConflictText.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackConflictVersions.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackDiffHunk.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackWorkspace.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceConflict.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceDiff.svelte
- web/src/lib/ui/store-stack-workspace/StoreStackWorkspacePicker.svelte
- web/tests/vite/production-stack-adapter.test.ts
- web/tests/vite/stack-hunk-context.test.ts
- web/tests/vite/stack-workspace-reads.test.ts
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ 0.45.1; pnpm 10.32.1; Svelte 5.57.1; existing local Design Core validation dependency unchanged.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No HTTP/RPC DTO, command, TOML, layout, manifest or WAL format changes. Optional WorkspaceFile.patch and WorkspaceConflict terms/reason are frontend presentation fields. Native terms, hunk tokens and operation/configuration guards remain authoritative.
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/vite/stack-hunk-context.test.ts#hunk context partitions all prefix,
  intervening and suffix lines without changing native descriptors
- crates/jj-store-jj/tests/local_stacks.rs#multisided_text_conflicts_offer_in_app_resolution_and_preserve_other_paths
- crates/jj-store-jj/tests/local_stacks.rs#conflict_versions_remain_bounded_and_binary_values_are_not_text
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M21-055 details 2](details-02.md)
- [E-M21-055 details 3](details-03.md)
- [E-M21-055 details 4](details-04.md)
