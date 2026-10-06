---
schema: 1
id: E-UI-028
---

# E-UI-028

<!-- evidence-field:id markdown -->
## Id

E-UI-028
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4.2
- M4.8
- M4.15
- M4.16
- M5
- M5.1
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-30
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

repository-tabs-file-edit-and-search-lifecycle
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- .storybook/vite.config.ts
- AGENTS.md
- catalog/components/community-projects/CommunityProjectsRepositoryDirectory.stories.svelte
- catalog/components/community-projects/repository-browser.test.ts
- catalog/components/community-projects/repository-editor-language.test.ts
- catalog/components/store-application/RepositoryReview.stories.svelte
- catalog/components/store-application/StoreApplication.stories.svelte
- catalog/components/store-application/StoreFileEditor.stories.svelte
- catalog/components/store-application/StoreRepositoryApplication.stories.svelte
- catalog/components/store-application/StoreRepositorySearch.stories.svelte
- catalog/components/store-application/column-canvas-story-assertions.ts
- catalog/controller/in-memory-store-controller.svelte.ts
- catalog/controller/in-memory-store-mutation-commit.ts
- catalog/controller/in-memory-store-mutations.svelte.ts
- catalog/controller/in-memory-store-repository.svelte.ts
- catalog/controller/in-memory-store-route.ts
- crates/jj-store-cli/tests/native_lifecycle/commands.rs
- crates/jj-store-cli/tests/native_lifecycle/http_mutations.rs
- crates/jj-store-http/src/contract/tests.rs
- crates/jj-store-http/src/dto.rs
- crates/jj-store-jj/src/commands/edit.rs
- crates/jj-store-jj/src/commands/execute.rs
- crates/jj-store-jj/src/commands/mod.rs
- crates/jj-store-jj/src/commands/resolve.rs
- package.json
- pnpm-lock.yaml
- schemas/openapi.json
- scripts/check-progress.mjs
- scripts/dev-demo-repository.mjs
- scripts/dev-demo-repository.test.mjs
- spec/src/architecture.md
- spec/src/api.md
- spec/src/cli.md
- spec/src/compatibility.md
- spec/src/jj-integration.md
- spec/src/milestones.md
- spec/src/mutations.md
- spec/src/spec-governance.md
- spec/src/ui.md
- spec/src/verification.md
- web/src/lib/api/generated.ts
- web/src/lib/controller/production-landing-documents.ts
- web/src/lib/controller/production-mutation-commit.ts
- web/src/lib/controller/production-store-controller.svelte.ts
- web/src/lib/controller/production-store-loaders.ts
- web/src/lib/controller/production-store-mutations.ts
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryBreadcrumb.svelte
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryDirectory.svelte
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryDocuments.svelte
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryFilePanel.svelte
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryFilePanelHeader.svelte
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryFiles.svelte
- web/src/lib/ui/community-projects/RepositoryFileCodeEditor.svelte
- web/src/lib/ui/community-projects/StoreFileEditor.svelte
- web/src/lib/ui/community-projects/repository-browser.ts
- web/src/lib/ui/community-projects/repository-directory.css
- web/src/lib/ui/community-projects/repository-documents.css
- web/src/lib/ui/community-projects/repository-editor-language.ts
- web/src/lib/ui/community-projects/repository-files.css
- web/src/lib/ui/community-projects/store-file-editor.css
- web/src/lib/ui/store-application/StoreApplication.svelte
- web/src/lib/ui/store-application/store-application.css
- web/src/lib/ui/store-search/StoreSearchCommand.svelte
- web/src/lib/ui/store-search/StoreSearchOverlay.svelte
- web/tests/browser/mutations.spec.ts
- web/tests/browser/native-browser-fixture.ts
- web/tests/browser/navigation.spec.ts
- web/tests/browser/reads.spec.ts
- web/tests/browser/search.spec.ts
- web/tests/production-store-loaders.test.ts
<!-- /evidence-field -->

<!-- evidence-field:removed_files list -->
## Removed files

- web/src/lib/ui/store-application/repository-breadcrumb.css
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- catalog/components/community-projects/repository-editor-language.test.ts#selects
  source highlighters from repository file names
- crates/jj-store-cli/tests/native_lifecycle/commands.rs#typed_edit_file_creates_child_renames_text_and_moves_exact_bookmark
- crates/jj-store-cli/tests/native_lifecycle/commands.rs#typed_edit_file_rejects_ambiguous_targets_invalid_paths_and_oversized_text
- web/tests/production-store-loaders.test.ts#repository landing documents
  include community files in stable tab order
- web/tests/browser/mutations.spec.ts#text file edit previews one request then
  routes to the renamed committed file
- web/tests/browser/search.spec.ts#command search opens native results and
  routes through stable identities
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-UI-028 details 2](details-02.md)
- [E-UI-028 details 3](details-03.md)
- [E-UI-028 details 4](details-04.md)
- [E-UI-028 details 5](details-05.md)
- [E-UI-028 details 6](details-06.md)
- [E-UI-028 details 7](details-07.md)
