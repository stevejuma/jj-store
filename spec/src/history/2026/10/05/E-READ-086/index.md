---
schema: 1
id: E-READ-086
---

# E-READ-086

<!-- evidence-field:id markdown -->
## Id

E-READ-086
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-05
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4.1
- M4.6
- M4.9
- M4.10
- M13.4
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-API-001
- JST-API-005
- JST-UI-003
- JST-JJ-003
- JST-LOCAL-004
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Bounded merge path metadata and directory navigation skeletons
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change yolnqxty
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Original Linux master/arch metadata requests returned HTTP200 with merge-unavailable for all 23 arch rows and all 41 root rows. File names loaded independently; Description and Updated were blank because the first-parent attribution walk stopped at its selected merge. Folder navigation retained a repository snapshot, so the UI skipped its initial-only skeleton while the new directory read was pending.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Remove only the unconditional merge stop from native and local path attribution. Compare exact path terms to the first parent: changed paths are attributed to their integration commit; unchanged paths continue through that parent. Keep one shared window traversal, 64 visits and existing work/bytes/deadline/authorization admission. Display the existing directory skeleton whenever its files are loading, including retained snapshots; keep pagination and optional attribution independent. No backend redesign or new indexes.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No DTO/schema/route/generated client, stored native identity, companion, index, cursor, WAL, manifest, receipt, credential or installed CLI change. Existing Merge enum value retained for compatibility; Budget and NotRequested results remain unchanged. First-parent attribution deliberately identifies branch integration, not an exhaustive search across every parent.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Same pinned macOS ARM64, JJ/jj-lib 0.45.1, rustc 1.98.1, Node 26.9.0, pnpm 10.32.1, Svelte 5.57.1, Storybook 10.5.7 and Vitest 4.1.11 development setup as E-READ-084/085.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/StoreDirectoryNavigation.stories.svelte
- catalog/fixtures/RepositoryDirectoryNavigationFixture.svelte
- crates/jj-store-cli/tests/local_browser/bounded_reads.rs
- crates/jj-store-cli/tests/native_lifecycle/path_metadata.rs
- crates/jj-store-cli/tests/native_lifecycle.rs
- crates/jj-store-jj/src/local_browser/paths.rs
- crates/jj-store-jj/src/query/content/paths.rs
- spec/src/api.md
- spec/src/cli.md
- spec/src/jj-integration.md
- spec/src/local-jj.md
- spec/src/ui.md
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryFiles.svelte
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/native_lifecycle/path_metadata.rs#path_metadata_crosses_merges_and_retains_the_ancestry_bound
- crates/jj-store-cli/tests/local_browser/bounded_reads.rs#local_path_metadata_crosses_merges_using_first_parent_history
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-READ-086 details 2](details-02.md)
- [E-READ-086 details 3](details-03.md)
