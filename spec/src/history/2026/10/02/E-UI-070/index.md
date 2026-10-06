---
schema: 1
id: E-UI-070
---

# E-UI-070

<!-- evidence-field:id markdown -->
## Id

E-UI-070
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M11.4
- M15.5
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-02
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

focused project add menu and Settings-style repository dialogs
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

Enclosing Jujutsu change xztvslzk
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-015
- JST-WORK-008
- JST-API-005
- JST-ARCH-004
- JST-GOV-007
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Visual baseline approval and existing released-registry qualification remain
  separate. Native full-flow write tests have updated selectors but were not run
  against the persistent fixture.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

UI-only existing web surface. Community repository/project dialogs inspected first, public Design Core DialogFrame/DropdownMenu/Field/Button second. One Add menu dispatches task creation and local registration using ClipboardPlus/GitBranchPlus icons. A link-icon control opens repository linking in a Settings-style dialog, retaining picker, manual native ID, draft, permission/version/pending gates and exact set_repository semantics. Local Add/Reconnect share divided Folder/Name/Inspection rows; the registration manager remains mounted outside menu content. Committed metadata refresh closes linking and registration dialogs; unknown outcomes preserve intent and support recovery. Toolbar and dialog extracted to respect300-line architecture limit. Relevant web/catalog checks, all direct Projects/StoreWork/navigation stories, mocked registration/picker browser tests and live read-only previews cover this slice. Rust, root aggregates, unrelated stories, built-asset qualification, baseline updates, fixture reset/reseed, backend restart, main landing and publication are out of scope.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Node26.9.0;pnpm10.32.1;JJ0.45.1;existing Svelte5.57.1,Storybook10.5.7,Vitest4.1.11 and Design Core public components retained.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No schema, API, native identity, request/receipt, permission or persistence format change.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Review through preserved HMR7460/backend7462. Continue the user-requested layout/search/focus-sheet/project-column/agent-row follow-ups as separate verified slices.
<!-- /evidence-field -->

<!-- evidence-field:artifacts markdown -->
## Artifacts

/tmp/jj-project-repository-*.log; /tmp/jj-project-repository-{menu,folder,link}-live.png; desktop/mobile mocked folder screenshots. Temporary CUA tab18 closed; user tab and persistent fixture retained.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- web/src/lib/ui/store-settings/StoreLocalFolders.svelte
- web/src/lib/ui/store-settings/StoreLocalFolderDialog.svelte
- web/src/lib/ui/store-work/StoreProjects.svelte
- web/src/lib/ui/store-work/StoreProjectsActions.svelte
- web/src/lib/ui/store-work/StoreProjectsAddMenu.svelte
- web/src/lib/ui/store-work/StoreProjectsFilterControls.svelte
- web/src/lib/ui/store-work/StoreWork.svelte
- web/src/lib/ui/store-work/StoreWorkRepositories.svelte
- catalog/components/store-work/ProjectsRepositoryActions.stories.svelte
- catalog/components/store-work/StoreWork.stories.svelte
- catalog/fixtures/StoreWorkFixture.svelte
- web/tests/real/project-repository-actions.spec.ts
- web/tests/real/local-browser.spec.ts
- web/tests/real/work-store.spec.ts
- spec/src/api.md
- spec/src/projects.md
- spec/src/ui.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-UI-070 details 2](details-02.md)
