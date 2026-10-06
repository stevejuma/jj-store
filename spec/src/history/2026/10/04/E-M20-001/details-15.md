---
schema: 1
id: E-M20-001
---

# E-M20-001 details 15

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
pnpm checks (Design Core)
```

**Notes**

Output: .local/stack-workspace-ui-evidence/source/design-core-stack-final-checks.log. Complete final source lanes: 715 units, 651 stories, 14 Workspace, 11 Shell, 3 AI, 36 shadcn browser cases and one platform skip; governance, types, static build and metadata audit passed. Three unsuppressed nonfatal ResizeObserver notices remain in unrelated AppShell scenarios.
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
pnpm packages:pack (Design Core)
```

**Notes**

Output: .local/stack-workspace-ui-evidence/source/design-core-stack-final-pack.log. No unpublished version selected; this did not create or publish a release.
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
pnpm pack --pack-destination .local/stack-ui-pack (Design Core)
```

**Notes**

Output: .local/stack-workspace-ui-evidence/source/design-core-stack-final-tarball.log. Local lapismd-design-core-0.1.2.tgz archive created; no publication.
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
pnpm spec:check (Design Core)
```

**Notes**

Output: .local/stack-workspace-ui-evidence/source/design-core-stack-final-spec2.log. Exact final source specification check passed before the source change was committed as xmmtwsry/3a2ba27c.
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
node scripts/check-progress.mjs
```

**Notes**

Output: .local/stack-workspace-ui-evidence/composer-progress-final.log. progress ok (141 tasks; evidence bookkeeping only)
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
pnpm exec prettier --write spec/src/api.md spec/src/verification.md
```

**Notes**

Output: .local/stack-workspace-ui-evidence/final-docs-format.log. Command completed; formatter output or installation completion retained.
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
pnpm exec prettier --check .storybook/vite.config.ts catalog/components/store-stack-workspace/StoreStackWorkspace.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceCommands.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceDrag.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceInline.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceMenus.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspacePresentation.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceRecovery.stories.svelte catalog/fixtures/StackWorkspaceApplicationFixture.svelte catalog/fixtures/StackWorkspaceFixture.svelte catalog/fixtures/stack-workspace/adapter.ts catalog/fixtures/stack-workspace/changes.ts catalog/fixtures/stack-workspace/data.ts catalog/fixtures/stack-workspace/menu-reducer.ts catalog/fixtures/stack-workspace/menu-validation.ts catalog/fixtures/stack-workspace/preview.ts catalog/fixtures/stack-workspace/reducer.ts catalog/fixtures/stack-workspace/story-actions.ts catalog/fixtures/stack-workspace/validation.ts docs/ui-provenance.json spec/src/api.md spec/src/spec-governance.md spec/src/ui.md spec/src/verification.md web/src/lib/controller/stack-workspace/commit-drafts.svelte.ts web/src/lib/controller/stack-workspace/controller.svelte.ts web/src/lib/controller/stack-workspace/drag-session.svelte.ts web/src/lib/controller/stack-workspace/drag.ts web/src/lib/controller/stack-workspace/file-drag.ts web/src/lib/controller/stack-workspace/files.ts web/src/lib/controller/stack-workspace/presentation.ts web/src/lib/controller/stack-workspace/types.ts web/src/lib/controller/store-route-codec.ts web/src/lib/ui/community-commit-diff/CommunityCommitDiff.svelte web/src/lib/ui/community-navigation/icons.ts web/src/lib/ui/community-navigation/model.ts web/src/lib/ui/community-projects/CommunityProjectsRepositoryLogFiles.svelte web/src/lib/ui/community-projects/CommunityProjectsRevisionGraph.svelte web/src/lib/ui/community-projects/CommunityProjectsRevisionGraphCopy.svelte web/src/lib/ui/community-projects/CommunityProjectsRevisionGraphSvg.svelte web/src/lib/ui/community-repository-branch-command/CommunityRepositoryBranchCommand.svelte web/src/lib/ui/store-application/StoreApplication.svelte web/src/lib/ui/store-application/StoreApplicationMain.svelte web/src/lib/ui/store-application/StoreApplicationNavigation.svelte web/src/lib/ui/store-application/StoreRepositoryApplication.svelte web/src/lib/ui/store-application/StoreRepositoryPicker.svelte web/src/lib/ui/store-application/application-props.ts web/src/lib/ui/store-application/store-navigation.ts web/src/lib/ui/store-stack-workspace/StoreStackBookmarkActions.svelte web/src/lib/ui/store-stack-workspace/StoreStackBookmarkCard.svelte web/src/lib/ui/store-stack-workspace/StoreStackCommitActions.svelte web/src/lib/ui/store-stack-workspace/StoreStackCommitComposer.svelte web/src/lib/ui/store-stack-workspace/StoreStackCopyMenu.svelte web/src/lib/ui/store-stack-workspace/StoreStackDiffHunk.svelte web/src/lib/ui/store-stack-workspace/StoreStackDropGuide.svelte web/src/lib/ui/store-stack-workspace/StoreStackLane.svelte web/src/lib/ui/store-stack-workspace/StoreStackRelativeActions.svelte web/src/lib/ui/store-stack-workspace/StoreStackStagedFiles.svelte web/src/lib/ui/store-stack-workspace/StoreStackUnstaged.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspace.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceActions.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceCanvas.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceChoice.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceConflict.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceDiff.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceForm.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspacePicker.svelte web/src/lib/ui/store-stack-workspace/StoreStackWorkspacePreview.svelte web/src/lib/ui/store-stack-workspace/stack-workspace.css web/tests/vite/stack-workspace-branches.test.ts web/tests/vite/stack-workspace-menus.test.ts web/tests/vite/stack-workspace.test.ts web/tests/vite/store-route-history.test.ts
```

**Notes**

Output: .local/stack-workspace-ui-evidence/final-slice-format.log. All matched files use Prettier code style!
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
spec-validator first
```

**Notes**

Output: .local/stack-workspace-ui-evidence/final-slice-spec-first.log. Spec-first gate passed: 70 protected file(s), 4 canonical chapter(s).
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
spec-validator validate
```

**Notes**

Output: .local/stack-workspace-ui-evidence/final-slice-spec-validate.log. Specification validated: 5 validators, 23 chapters, 173 requirements.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
node scripts/check-progress.mjs
```

**Notes**

Output: .local/stack-workspace-ui-evidence/final-slice-progress.log. progress ok (141 tasks; evidence bookkeeping only)
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- UI review and visual approval remain separate; no screenshot comparison or
  baseline update.
- Design Core source qualification passed; registry-resolved foundational
  release acceptance remains separate. Three nonfatal ResizeObserver notices
  remain in unrelated source AppShell stories; the final 90-story consumer run
  contains none.
- Phase 2 backend, physical checkout and durable workspace integration require
  explicit human approval.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Phase 1 UI only: owned Changeyard adaptations and existing application/public foundational components, injected production controller and isolated stateful mock. Branch headers reveal only their own segment files inline, including the top branch. Idle commit cards separate drop and start sections, then become staged-file/composer cards. Independent/dependent creation, Command View parent picking, copy/above/below/reword/edit/delete menus, exact preview/recovery/undo, native file/folder/hunk drag guides, resizable columns, sticky Unstaged and adjacent independent diffs are exercised. Production has no catalog import or live adapter. No persisted format, generated transport, storage, Rust or checkout implementation changed. Full jj-store aggregates and Rust lanes are out of scope under the controlling UI verification policy. All 434 historical evidence objects remain byte-identical.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Review the mocked app-shell Storybook on port 6057. Publish/qualify the foundational release separately; require explicit human approval before Phase 2 backend work.
<!-- /evidence-field -->

<!-- evidence-field:ui_acceptance markdown -->
## Ui acceptance

57/57 focused controller/history units and 90/90 affected interaction/accessibility stories, zero web/catalog diagnostics, architecture and UI ownership passed. Browser native README.md drag opened a staged card and retained the source until apply. Captures: .local/stack-workspace-ui-evidence/inline-branch-card.png and inline-staged-card.png.
<!-- /evidence-field -->

