---
schema: 1
id: E-VISUAL-DELTA-MACOS-001
---

# E-VISUAL-DELTA-MACOS-001 details 3

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 64
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook
```

**Notes**

589 tests pass, 63 story tests fail across 28 files, and one unhandled browser-connection closure aborts remaining discovery (215 total files). This is not a full-suite pass. All 63 story failures reproduce in the parent configuration without Visual Delta below.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 48
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config .local/visual-delta-setup/parent-vitest.config.ts catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte catalog/components/store-application/AgentChatBody.stories.svelte catalog/components/store-stack-workspace/StoreStacksNavigation.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspace.stories.svelte catalog/components/store-application/AgentToolGroups.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceRefinements.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceCommands.stories.svelte catalog/components/store-application/StoreCollaboration.stories.svelte catalog/components/store-forge/StoreForgeInlineWorkspace.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceConflicts.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceMenus.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspacePresentation.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte catalog/components/store-work/DisclosurePersistence.stories.svelte catalog/components/store-application/WorkspaceFilePresentation.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceHeaderDrops.stories.svelte catalog/components/store-application/RepositoryReview.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceSpacer.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceForms.stories.svelte catalog/components/store-application/AgentFleet.stories.svelte -t 'MultipleAdjacentDiffs|CapturedLineNumberGutter|ResizableReadSidebar|MessageReactionsAndXMLContext|WorkspaceRailUsesRepositoryDetailsNavigation|HistoryRetainsCanvasNodesDiffsDraftsAndSelectedCommits|UnstagedWorkingFiles|NamedCombinationsRetainEdits|CommitOneHunkAndUndo|BranchesApplyAndBaseOnlyWorkspace|NativeCheckout|DraggableHunksRetainExpandableFullContext|GroupedRowsAlignWithMessagesAndKeepHoverActionsClickable|RecordedDurationsPrecedeTheChevronWithoutChangingToolStatus|AdjacentCallsGroupAndFailedDetailsRetainTheActualError|GroupedCallsRetainNativeMenusAndIndependentReplyDrafts|ToolCommentsAndReplyComposerStayInAFullRowBelowTheirCall|ToolAndCommentHoverActionsStayIndependent|AgentCommentRepliesUseThreadIndentationAndConnectors|ContentPaddingSeparatesComponentsFromEdgeScrollbars|InlineCommitWithStagedFilesAndHunks|DropChangesOntoExistingCommit|SplitReorderSquashAndMove|DragAHunkToACommit|RenameRebaseUncommitAndDeleteStack|WorkspaceDraggableDiffHunks|AgentResponseActivity|RevisionCollaborationContext|OperationCollaborationContext|PRSummaryStaysAboveChanges|FileSelectionBringsTheMatchingPRIntoTheDiffColumn|UnstagedConflictIndicatorsAndFullWidthDiffAlert|EditCommitInInlineComposer|DiffHeaderAndWideActionMenus|PersistentLayout|RepositoryFiles|ConflictedSnapshot|HeaderTogglesStayTransparentInBothExpansionStates|WorkspaceRailHeaderAndFileDividerSpanThePanel|WorkspaceInspectorJoinsTheRailAtWideAndConstrainedWidths|WorkspaceSourceReaderHasSyntaxAndGuttersWithoutExtraCommentChrome|WorkspaceMenuKeepsMultiRepositoryInspectionScopedToTheSelection|DropGuidesHaveAnOpaqueForegroundBackground|ShellAndRowLocalGraphSpacing|ColumnTitlesReplaceBodyHeader|DropFilesIntoTheSpacerRetainsExactStagedChanges|AssignSelectedChangesWithSettingsRowsAndBranchCommandPicker|BodyRepositoryAndWorkspaceSelections'
```

**Notes**

The first 48 failures reproduce without the addon; 99 cases are outside the selected failure scope. Exact arrays are retained in .local/visual-delta-setup/failed-cases.json. No product assertion or threshold is changed.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 15
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config .local/visual-delta-setup/parent-vitest.config.ts catalog/components/store-stack-workspace/StoreStackWorkspaceNativeDrag.stories.svelte catalog/components/store-application/WorkspaceFilesPerformance.stories.svelte catalog/components/store-application/AgentSessions.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceDrag.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspacePinning.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte catalog/components/store-application/StoreFileEditor.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceSelection.stories.svelte -t 'NativeBranchFilesStageAndHunksPreviewExactTransfers|MenuOpensWithoutIOAndFilesReopenFromCache|RailLoadingMirrorsFilesAndChanges|AgentTilesRetainDimensionsWhenDetailsOpens|UnstagedFilesStageInAnyStackWithGuides|FolderDropsAmendCommitsAndFilesReturnToUnstaged|RejectedAndCancelledDropsClearTheirGuides|BranchesRemainFixedBesideAStackAndDiff|UnstagedHeaderAndStyledCanvas|BranchesToDiff|JJLog|OpLog|ReturnToRootClearsFileEditor|EditAndRename|SelectedCommitsAndHeadersToggleClosedWithoutLosingAdjacentDiffs'
```

**Notes**

All remaining 15 failures reproduce without the addon; 20 cases are outside the selected failure scope. Exact arrays are retained in .local/visual-delta-setup/remaining-failed-cases.json. Existing catalog assertions are owned by jj-store; the full-run transport closure remains a separate qualification gap.
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
pnpm test:storybook catalog/components/store-workflows/WorkflowEditor.stories.svelte
```

**Notes**

All 3 cases pass from a fresh process after this file was named by the broad run browser-connection error. This focused rerun does not qualify the unexecuted portion of the broad suite.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:architecture
```

**Notes**

Existing StoreStackWorkspaceApplication story has 401 counted lines and StoreStackWorkspace.svelte has 310. Both files are byte-identical to the parent revision; no architecture budget is increased.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:validate
```

**Notes**

Initial validation found 3 pre-existing unindexed Oct-23 history files and 2 new paginated evidence pages lacking navigation. Added minimal index/fragment links without changing historical records and scoped navigation generation to the new Oct-24 entry. The official evidence:new command initially regenerated 614 unrelated paths; all were restored to the parent, preserving original user edits.
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
pnpm install --lockfile-only --frozen-lockfile --ignore-pnpmfile --ignore-scripts
```

**Notes**

Portable registry lockfile validates without replacing the original packed development dependency.
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
pnpm exec tsc --noEmit --module ESNext --moduleResolution Bundler --target ES2022 --skipLibCheck playwright.config.ts tests/visual/storybook.spec.ts
```

**Notes**

The root capture config and portable suite typecheck against the released public export.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
node --input-type=module
```

**Notes**

The first panel probe used an unsupported URL panel selector and observed an inactive hidden panel. Corrected the harness to select the visible Visual Delta tab explicitly; no product change was needed.
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
pnpm spec:validate
```

**Notes**

7 validators, 4414 chapters, 208 requirements. Minimal navigation includes the existing Oct-23 records and new paginated evidence without modifying historical records.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
node .local/visual-delta-setup/native-panel-check.mjs
```

**Notes**

The first loaded-story probe required an exact Visual Delta tab label. Storybook adds a result-count badge to that accessible name after loading. Corrected the harness to match its stable name prefix; no addon or catalog change was required.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
node .local/visual-delta-setup/native-panel-check.mjs
```

**Notes**

After selector repair, two 30-second probe attempts timed out during dev navigation or an extra config request while Vite optimized the catalog. A loaded native panel and Darwin profile were observed before the latter timeout. The config endpoint subsequently returned successfully through curl; final cold-process verification is recorded separately.
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
JJ_STORE_CHECK_REVISION=vtstntlp pnpm check:progress
```

**Notes**

180 tasks, 955 evidence records. Validated this isolated task revision on the clean original parent; original unfinished M33 documents remain preserved in their separate change.
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
pnpm spec:first --base 47a73e1335334f7ec1ed9ebeeb2f9750b67f9948 --head cf22b8aba86ecbbb8f0dca165d1726d3aa2f79a4
```

**Notes**

The owned task revision passes all 5 protected files and 3 mapped canonical chapters. Later metadata-only evidence updates remain in the same Jujutsu change.
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
node .local/visual-delta-setup/native-panel-check.mjs
```

**Notes**

Fresh Storybook process on 6046 renders the Bookmark story and native macOS ARM64 panel; local /__visual-delta/config returns 200 with native runner available and onboarding ready. No page errors. Native screenshot retained at .local/visual-delta-setup/native-panel.png. Extending only the standalone probe timeout allowed Vite optimization to finish; production and interaction assertions remain unchanged.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Registry-only catalog acceptance remains blocked by existing unreleased Design
  Core shell APIs (23 type errors against registry 0.5.1; the original 0.6.0
  packed validation environment is preserved).
- Full catalog qualification retains 63 independently reproduced parent failures
  plus one browser connection closure; remaining broad cases were not executed.
- Two unchanged stack-workspace files exceed existing architecture limits.
- Visual baseline approval, Docker/Linux capture and native/Rust/server release
  qualification remain separate; no committed PNG is changed.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Visual Delta 0.1.0 from npm; Playwright runtime/test 1.63.0; Chromium 153.0.8010.12; macOS ARM64 (Darwin 27). Operator capture runs with Node 26.9.0. Package checks use Node 24.15.0/pnpm 10.32.1. Existing packed Design Core is 0.6.0; portable manifest remains ^0.5.1.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Shared/local Visual Delta JSON; Playwright suite; PNG; native capture-profile schema 2; result sidecar schema 4. No native repository/storage/API formats change.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Add the public addon to the jj-store server application catalog, portable capture entrypoints and dependency graph statistics. Shared defaults stay Docker; the ignored local file selects native on this Mac. Native snapshots separate profile/OS/architecture. CLI, Playwright and addon use static port 6047 with VISUAL_SERVER_PORT override. Automation and thresholds remain unchanged. Shared Storybook registration requires full catalog lanes; existing failures are retained rather than waived.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Use the configured native macOS capture locally; repair existing catalog assertions, architecture debt and the Design Core release gap in their owning slices before broad qualification.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: Extend JST-UI-001 with opt-in native visual capture; architecture and
  catalog-governance chapters retain package ownership, registry portability and
  separate baseline approval.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M4.4 keeps its existing status. This setup does not promote global
  catalog, foundation release, other platform or visual-baseline gates.

```
<!-- /evidence-field -->

