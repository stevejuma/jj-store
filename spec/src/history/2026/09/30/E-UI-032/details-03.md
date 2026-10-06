---
schema: 1
id: E-UI-032
---

# E-UI-032 details 3

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --write 'catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.stories.svelte' 'catalog/components/store-application/StoreApplication.stories.svelte' 'catalog/components/store-application/StoreWorkspaces.stories.svelte' 'catalog/controller/in-memory-store-branch-actions.ts' 'catalog/controller/in-memory-store-controller.svelte.ts' 'catalog/controller/in-memory-store-navigation.ts' 'catalog/controller/in-memory-store-options.ts' 'catalog/controller/in-memory-store-route.ts' 'catalog/fixtures/StoreRepositoryApplicationFixture.svelte' 'spec/src/api.md' 'spec/src/milestones.md' 'spec/src/ui.md' 'spec/src/verification.md' 'web/src/lib/controller/contracts.ts' 'web/src/lib/controller/production-branch-actions.ts' 'web/src/lib/controller/production-route-restorer.ts' 'web/src/lib/controller/production-store-actions.ts' 'web/src/lib/controller/production-store-loaders.ts' 'web/src/lib/controller/production-store-reader.svelte.ts' 'web/src/lib/controller/store-route-codec.ts' 'web/src/lib/controller/store-route-history.svelte.ts' 'web/src/lib/controller/store-workspaces.ts' 'web/src/lib/ui/community-projects/CommunityProjectsRepositoryFiles.svelte' 'web/src/lib/ui/community-projects/CommunityProjectsRepositoryLog.svelte' 'web/src/lib/ui/community-projects/workspace-snapshots.ts' 'web/src/lib/ui/community-repository-branch-command/CommunityRepositoryBranchCommand.svelte' 'web/src/lib/ui/community-repository-branch-command/CommunityWorkingCopyCard.svelte' 'web/src/lib/ui/store-application/StoreApplication.svelte' 'web/src/lib/ui/store-application/StoreRepositoryApplication.svelte' 'web/src/lib/ui/store-application/StoreWorkspacePicker.svelte' 'web/src/lib/ui/store-application/store-application.css' 'web/src/lib/ui/store-application/workspace-editing.svelte.ts' 'web/src/lib/ui/store-application/workspace-navigation.ts' 'web/tests/browser/workspaces.spec.ts' 'web/tests/store-route-codec.test.ts' 'web/tests/vite/store-route-history.test.ts' 'web/tests/vite/store-workspaces.test.ts'
```

**Notes**

All 37 changed implementation, test, story and canonical files were formatted; no functional changes from formatting.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:validate
```

**Notes**

All 106 canonical requirements in 18 chapters validate.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first
```

**Notes**

All 33 protected files map to the three updated canonical chapters.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

The 72-task ledger and E-UI-032 evidence validate; this is bookkeeping evidence, separate from the focused product tests.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:build
```

**Notes**

The canonical mdBook builds successfully; generated spec/book remains ignored.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --check 'catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.stories.svelte' 'catalog/components/store-application/StoreApplication.stories.svelte' 'catalog/components/store-application/StoreWorkspaces.stories.svelte' 'catalog/controller/in-memory-store-branch-actions.ts' 'catalog/controller/in-memory-store-controller.svelte.ts' 'catalog/controller/in-memory-store-navigation.ts' 'catalog/controller/in-memory-store-options.ts' 'catalog/controller/in-memory-store-route.ts' 'catalog/fixtures/StoreRepositoryApplicationFixture.svelte' 'spec/src/api.md' 'spec/src/milestones.md' 'spec/src/ui.md' 'spec/src/verification.md' 'web/src/lib/controller/contracts.ts' 'web/src/lib/controller/production-branch-actions.ts' 'web/src/lib/controller/production-route-restorer.ts' 'web/src/lib/controller/production-store-actions.ts' 'web/src/lib/controller/production-store-loaders.ts' 'web/src/lib/controller/production-store-reader.svelte.ts' 'web/src/lib/controller/store-route-codec.ts' 'web/src/lib/controller/store-route-history.svelte.ts' 'web/src/lib/controller/store-workspaces.ts' 'web/src/lib/ui/community-projects/CommunityProjectsRepositoryFiles.svelte' 'web/src/lib/ui/community-projects/CommunityProjectsRepositoryLog.svelte' 'web/src/lib/ui/community-projects/workspace-snapshots.ts' 'web/src/lib/ui/community-repository-branch-command/CommunityRepositoryBranchCommand.svelte' 'web/src/lib/ui/community-repository-branch-command/CommunityWorkingCopyCard.svelte' 'web/src/lib/ui/store-application/StoreApplication.svelte' 'web/src/lib/ui/store-application/StoreRepositoryApplication.svelte' 'web/src/lib/ui/store-application/StoreWorkspacePicker.svelte' 'web/src/lib/ui/store-application/store-application.css' 'web/src/lib/ui/store-application/workspace-editing.svelte.ts' 'web/src/lib/ui/store-application/workspace-navigation.ts' 'web/tests/browser/workspaces.spec.ts' 'web/tests/store-route-codec.test.ts' 'web/tests/vite/store-route-history.test.ts' 'web/tests/vite/store-workspaces.test.ts'
```

**Notes**

All changed production, catalog, test and canonical files pass the final targeted formatting check.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --write spec/src/milestones.md web/src/lib/ui/store-application/StoreRepositoryApplication.svelte
```

**Notes**

Final canonical evidence and unavailable-view copy are formatted.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

[]
<!-- /evidence-field -->

<!-- evidence-field:deferred list -->
## Deferred

- Human visual review and baseline approval; all changed stories remain
  visual-pending.
- Root aggregate pnpm spec:check, pnpm check, pnpm test and pnpm build, full
  Storybook/browser suites, contract regeneration and Rust lanes are out of
  scope under the UI-only targeted verification policy. No independent server
  layer or wire contract changed.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

jj 0.45.1; pnpm 10.32.1; Svelte 5.57.1; Vitest 4.1.11; Storybook 10.5.7; Vite 8.3.1; existing Design Core public exports and dependency resolution unchanged.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Owned browser hashes add exact encoded workspace names with required at_op. Existing hashes remain valid. No HTTP, generated DTO, native identity, manifest, WAL, receipt or persistence-format changes. Workspace selection is read-only; logical names are distinct from client filesystem checkout state. Mutations and text drafts disable the picker until their active flow is finished. The sidebar adds no workspace children.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Retain the focused workspace/route/story/browser regressions; visual review remains separate.
<!-- /evidence-field -->

