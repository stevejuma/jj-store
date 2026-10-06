---
schema: 1
id: E-M20-002
---

# E-M20-002 details 8

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:check
```

**Notes**

Output: .local/stack-header-drops-evidence/catalog-branch-footer-final.log. One story type error: ByRoleOptions does not accept exact. Replaced it with anchored Create stack accessible-name expression; query semantics unchanged.
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
pnpm check:architecture
```

**Notes**

Output: .local/stack-header-drops-evidence/architecture-branch-footer-final.log. 1016 modules passed.
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
pnpm exec prettier --write named changed UI/story/canonical/provenance files
```

**Notes**

Output: .local/stack-header-drops-evidence/branch-footer-format.log. Formatted only named changed files; historical milestone ledger excluded.
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
pnpm exec prettier --write named changed UI/story/canonical/provenance files
```

**Notes**

Output: .local/stack-header-drops-evidence/composer-borders-format.log. Formatted only named changed files; historical milestone ledger excluded.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-stack-workspace/StoreStackWorkspaceCommands.stories.svelte -t InlineCommitWithStagedFilesAndHunks --reporter=verbose
```

**Notes**

Output: .local/stack-header-drops-evidence/title-full-width-before.log. Rendered separator edges were inset from the card before the requested full-width title-row change.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-stack-workspace/StoreStackWorkspaceRefinements.stories.svelte -t ContentPaddingSeparatesComponentsFromEdgeScrollbars --reporter=verbose
```

**Notes**

Output: .local/stack-header-drops-evidence/branch-scroll-before-exact.log. Branches selected stack/diff viewports had 0px reserve instead of 8px before the requested scope extension; the Branches picker still expects 0px.
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
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceBranches.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceFooter.stories.svelte catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceCommands.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceInline.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceRefinements.stories.svelte -t 'AppliedBranchesHaveTheirOwnGroup|ApplyAndUnapplyActionsOccupyTheStackBody|WorkspaceTargetBranchAppearsAboveBodyTabs|RetainedDraftMatchesThe.*FooterChips|WorkspaceChipSitsAtTheTitleRowEdge|InlineCommitWithStagedFilesAndHunks|StartCommitBecomesAStagedStackCard|ContentPaddingSeparatesComponentsFromEdgeScrollbars' --reporter=verbose
```

**Notes**

Output: .local/stack-header-drops-evidence/branch-footer-composer-qualified.log. 9/9 passed, 18 skipped: current Applied group filtering, exact body Apply/Unapply transitions, fixed workspace target row, trailing Workspace chip geometry, light/dark matching footer surfaces and edit icon with retained form values, dashed staging drop border, normal-height title and full-width separator, and balanced stack/diff scrollbar spacing on both tabs while the Branches picker stays unchanged.
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
pnpm catalog:check
```

**Notes**

Output: .local/stack-header-drops-evidence/catalog-branch-footer-qualified.log. 0 errors and 0 warnings; 214 catalog modules passed.
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
pnpm check:ui-ownership
```

**Notes**

Output: .local/stack-header-drops-evidence/ownership-branch-footer-qualified.log. 1 ownership validator, 23 chapters and 173 requirements passed. No Changeyard/Community dependencies.
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
pnpm exec prettier --write named changed files
```

**Notes**

Output: .local/stack-header-drops-evidence/title-full-width-format.log. Formatted only named changed files; historical evidence excluded.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --write named changed files
```

**Notes**

Output: .local/stack-header-drops-evidence/docs-layout-final-format.log. Formatted only named changed files; historical evidence excluded.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace catalog/components/community-projects/CommunityProjectsRevisionGraph.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLogFiles.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLogDiff.stories.svelte catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.stories.svelte catalog/components/store-application/StoreRepositoryPicker.stories.svelte catalog/components/store-application/StoreRepositoryNavigation.stories.svelte catalog/components/store-application/StoreRepositoryBranchContext.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte --reporter=verbose
```

**Notes**

Output: .local/stack-header-drops-evidence/all-stories-branch-footer-qualified.log. 113/113 complete directly affected interaction/accessibility scenarios in 22 files passed on the final branch/body-action/footer-tone/composer/scrollbar code; not the full repository suite.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:architecture
```

**Notes**

Output: .local/stack-header-drops-evidence/architecture-branch-footer-qualified.log. 1016 modules passed on the final UI code.
<!-- /run -->

<!-- run -->
### Run 14

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

Output: .local/stack-header-drops-evidence/spec-complete-qualified.log. 5 validators, 23 chapters and 173 requirements passed.
<!-- /run -->

<!-- run -->
### Run 15

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

Output: .local/stack-header-drops-evidence/spec-first-complete-qualified.log. 37 protected files and 4 canonical chapters passed.
<!-- /run -->

<!-- run -->
### Run 16

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

Output: .local/stack-header-drops-evidence/progress-complete-qualified.log. 141 tasks passed; evidence bookkeeping only.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --check all changed files except spec/src/milestones.md and the mirrored evidence JSON
```

**Notes**

Output: .local/stack-header-drops-evidence/format-complete-qualified.log. All changed files passed formatting; historical evidence ledger was deliberately preserved without whole-file reformatting.
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
- Registry-resolved foundational release acceptance remains separate.
- Phase 2 backend, physical checkout and durable named workspace execution
  require explicit human approval.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

UI-only follow-up: exact branch header commit moves, threshold sorting and visible insertion borders, opaque drag guides, right-aligned staged controls, inset branch files aligned with commit content, plain Changed files headings with specific accessible names, wrapping header chips, left selection markers without Selected text chips, distinct action/neutral-chip surfaces, Settings-style creation/assignment forms with searchable branch Command View destinations, aligned kebab hover, and no redundant Applied/Base metadata row. Footer values use existing lightly shaded icon chips without Folder:/Base: prefixes, with readable applied-stack and unstaged-file counts; Edit retained draft uses the existing Pencil icon. Scrollbar content insets apply to stack/diff columns on both tabs: cards retain balanced 8px space before the edge-aligned rail; Unstaged accounts for the rail without card margins. The Branches picker alone retains its existing insets; its selected stack and adjacent diffs receive the balanced content reserve. Conflict status stays in the column header. Production uses owned copied components and public foundations; no live adapter or checkout/backend implementation. Root aggregates, Rust builds/tests and production built-bundle lanes are out of scope under the UI-only verification policy. Only owned Storybook 6057 was restarted for foundational token changes and remains running. Historical evidence objects remain byte-identical. Final branch presentation groups applied/local bookmarks with an opt-in existing Command View, aligns Workspace chips to the title row edge, shows the unchanged workspace target above the Branches tabs, and moves Apply/Unapply into a body Card with unavailable Delete local matching the copied source. Footer chips and retained draft action share a very light neutral theme surface. Commit creation has a visible dashed drop border and normal-height title input above a full-width row separator. Source provenance records Changeyard b0caf074; all behavior remains mocked.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Review the mocked workspace UI; obtain explicit approval before Phase 2 backend work.
<!-- /evidence-field -->

