---
schema: 1
id: E-UI-VIEW-READINESS-001
---

# E-UI-VIEW-READINESS-001 details 4

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
pnpm check:progress
```

**Notes**

146 tasks, 735 evidence records, bookkeeping only. /tmp/readiness-progress-final.log.
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
pnpm spec:validate --only summary,fileLimits,governance,verification,book
```

**Notes**

5 validators, 3493 chapters, 181 requirements. M24 index restored to 400 physical lines after formatting. /tmp/readiness-spec-complete.log.
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
pnpm exec prettier --write catalog/components/store-appearance/StoreAppearance.stories.svelte catalog/components/store-application/AgentLayout.stories.svelte catalog/components/store-application/AgentSessionSettings.stories.svelte catalog/components/store-application/AgentToolGroups.stories.svelte catalog/components/store-application/AgentTranscriptSettings.stories.svelte catalog/components/store-application/ConversationCaret.stories.svelte catalog/components/store-application/LocalCompute.stories.svelte catalog/components/store-application/NativeAgentMenus.stories.svelte catalog/components/store-application/NativeReviewDetail.stories.svelte catalog/components/store-application/RepositoryApplication.stories.svelte catalog/components/store-application/RepositoryLayout.stories.svelte catalog/components/store-application/RepositoryLoading.stories.svelte catalog/components/store-application/RepositoryNotices.stories.svelte catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-application/StoreCollaboration.stories.svelte catalog/components/store-application/StoreDirectoryNavigation.stories.svelte catalog/components/store-application/StoreEvolutionColumn.stories.svelte catalog/components/store-application/StoreFileEditor.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte catalog/components/store-application/StoreRepositoryBranchContext.stories.svelte catalog/components/store-application/StoreRepositoryFeedback.stories.svelte catalog/components/store-application/StoreRepositoryNavigation.stories.svelte catalog/components/store-application/StoreRepositorySearch.stories.svelte catalog/components/store-application/StoreWorkspaces.stories.svelte catalog/components/store-application/WorkspaceFiles.stories.svelte catalog/components/store-application/agent-sidebar-play.ts catalog/components/store-application/appearance-tool-discussion-play.ts catalog/components/store-application/capture-harness-play.ts catalog/components/store-application/repository-layout-story-assertions.ts catalog/components/store-application/settings-shell-play.ts catalog/components/store-stack-workspace/StoreStackWorkspaceLoading.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceSettings.stories.svelte catalog/components/store-threads/StoreThreadAlignment.stories.svelte catalog/components/store-threads/StoreThreads.stories.svelte catalog/components/store-work/ProjectsActivity.stories.svelte catalog/components/store-work/ProjectsAgentActions.stories.svelte catalog/components/store-work/ProjectsFilters.stories.svelte catalog/components/store-work/ProjectsHub.stories.svelte catalog/components/store-work/ProjectsReviewDetail.stories.svelte catalog/fixtures/AgentToolGroupsFixture.svelte docs/maintenance/coverage.json docs/site/guides/browsing.md docs/ui-provenance.json spec/src/current/contracts/api/16-browsing-ui-execution.md spec/src/current/contracts/documentation/03-delivery-and-validation.md spec/src/current/contracts/ui/07-application-startup.md spec/src/current/contracts/ui/37-document-conversations.md spec/src/current/contracts/ui/39-intentional-thread-inbox.md spec/src/current/governance/spec-governance.md spec/src/current/progress/M24/history/M24/index.md spec/src/current/progress/M24/index.md spec/src/current/progress/M24/view-loading-review.md spec/src/history/2026/10/10/E-UI-VIEW-READINESS-001/details-02.md spec/src/history/2026/10/10/E-UI-VIEW-READINESS-001/details-03.md spec/src/history/2026/10/10/E-UI-VIEW-READINESS-001/details-04.md spec/src/history/2026/10/10/E-UI-VIEW-READINESS-001/index.md spec/src/history/2026/10/10/index.md web/src/lib/ui/StoreToolbarSearch.svelte web/src/lib/ui/community-projects/CommunityProjectsRepositoryFileTree.svelte web/src/lib/ui/community-projects/repository-file-tree.css web/src/lib/ui/community-projects/repository-files.css web/tests/browser/appearance.spec.ts web/tests/browser/repository-navigation.spec.ts
```

**Notes**

Explicit changed-file arguments from JJ, passed through Python subprocess; all authored/source files formatted. SUMMARY.nav.md retains generator formatting. /tmp/readiness-format-complete.log. Earlier corrected formatter and post-edit formatting also completed in /tmp/readiness-doc-format-final.log and /tmp/readiness-format-acceptance.log.
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

Final scope and evidence coverage: 146 tasks, 735 records at 5b08e232; bookkeeping only. /tmp/readiness-progress-complete.log.
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
pnpm spec:validate --only summary,fileLimits,governance,verification,book
```

**Notes**

Final owning chapters/history/navigation: all 5 validators pass, 3493 chapters/181 requirements. /tmp/readiness-spec-delivery.log.
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
pnpm spec:first --base d1a4b3f3 --head 5b08e2325aa1f75bf033bf98309d6ae2a60e01a3
```

**Notes**

Explicit committed parent/current snapshot selection: 49 protected files and 6 canonical chapters pass; full UI/API/documentation/governance coverage retained. /tmp/readiness-spec-first-complete.log.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- "Broader interaction qualification remains pending: live reply/toolbar
  expectations (AgentDiscussion and AgentReplyIndependence), native Stack
  Application/Forms/Production/Setup/Presentation flows, ManagedAgentIdentity,
  RepositoryReview status/header geometry and StackWorkspaceSpacer float
  equality. Intermediate 704/53 audit is not a final current count."
- ResizeObserver loop warnings recur in wider and affected interaction runs. No
  diagnostic guard suppressed; complete ownership and remediation remain
  pending.
- Previously recorded web/catalog unit failures and architecture violations
  remain open; not rerun by this UI-only slice. M24 and visual baseline
  acceptance remain unapproved.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; Node26.9.0; pnpm10.32.1; JJ0.45.1; Vite8.3.1; Svelte5.57.1; Storybook10.5.7; Vitest4.1.11; Playwright1.63.0.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Existing appearance, route, source, draft and submission formats are unchanged. Fixture-local Classic is explicit only for tests protecting that renderer.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Default workspace twuyslro. Repair semantic lazy-view readiness and superseded presentation queries while preserving loading phases, draft/selection assertions, geometry tolerances and diagnostic guards. Reproduce and fix the owned file-tree CSS import boundary and stable toolbar-search height. Inspect Community and Design Core before changing presentation; consume the public InputGroup HTML layout API. The broader Storybook audit is required by shared view loading and story consumers. Root aggregate checks/build/tests, Rust/storage/provider/desktop lanes and visual baseline updates are out of scope for this UI/test slice. Live Vite/backend/RustFS and .local/dev-demo data are retained; isolated cold preview owns its disposable server/browser. Canonical presentation clauses are clarified; generated API/schema checks are out of scope because public types, request/response schemas and persisted formats are unchanged.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: "Clarified JST-UI-015/018 presentation in the owning UI chapters: shared
  file-tree rows/toolbars load independently of a prior Files-page visit, and
  expanding toolbar search retains neighboring control height and header
  geometry. The production CSS import and public InputGroup layout changes
  implement these clauses. JST-UI-005/010 loading/accessibility, JST-UI-017
  drafts, JST-API-001 browser behavior and JST-DOC-003 manual delivery remain
  unchanged. Readiness repairs preserve loading phases, zero-padding
  Overview/divider/margin, document read-only behavior, inert measurement-node
  exclusion, keyboard access, geometry tolerances and diagnostics. No shared
  package defect is concealed or external source dependency introduced;
  provenance retains source revision/license."

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: "M24 stays In progress. Enclosing change twuyslro qualifies immediate
  consumers; wider interaction, unit/architecture, registry-only dependency
  release, provider/native/desktop/platform and visual gates remain separately
  open. Current review: spec/src/current/progress/M24/view-loading-review.md."

```
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Repair the independent live-reply/native Stack consumers and diagnostic owners, repeat the complete broader interaction lane and qualify remaining unit/architecture gates before a release claim.
<!-- /evidence-field -->

<!-- evidence-field:notes markdown -->
## Notes

Discarded an experiment selecting Classic in the shared AgentExecutionFixture: it did not repair superseded reply-panel expectations. Restored that fixture and AgentDiscussion/AgentReplyIndependence from parent; no change to those production/default boundaries. Spec search was unavailable due forwarded -- and Node ABI147 native modules; canonical sources inspected with rg fallback. Read-only Vitest API attempt found the completed run already closed (ECONNREFUSED); no API mutation or credential output. Visual baselines remain unchanged. Build documentation staging was sequential; no parallel manifest replacement. Final formatter uses Python subprocess with explicit JJ changed paths, excluding SUMMARY.nav.md; no shell expansion or changed assertions. Bounded M24 history normalization preserves all 74 original evidence IDs/results/context labels and adds only the new readiness row.
<!-- /evidence-field -->

<!-- evidence-field:stories list -->
## Stories

- catalog/components/store-appearance/StoreAppearance.stories.svelte
- catalog/components/store-application/AgentLayout.stories.svelte
- catalog/components/store-application/AgentSessionSettings.stories.svelte
- catalog/components/store-application/AgentToolGroups.stories.svelte
- catalog/components/store-application/AgentTranscriptSettings.stories.svelte
- catalog/components/store-application/ConversationCaret.stories.svelte
- catalog/components/store-application/LocalCompute.stories.svelte
- catalog/components/store-application/NativeAgentMenus.stories.svelte
- catalog/components/store-application/NativeReviewDetail.stories.svelte
- catalog/components/store-application/RepositoryApplication.stories.svelte
- catalog/components/store-application/RepositoryLayout.stories.svelte
- catalog/components/store-application/RepositoryLoading.stories.svelte
- catalog/components/store-application/RepositoryNotices.stories.svelte
- catalog/components/store-application/StoreApplication.stories.svelte
- catalog/components/store-application/StoreCollaboration.stories.svelte
- catalog/components/store-application/StoreDirectoryNavigation.stories.svelte
- catalog/components/store-application/StoreEvolutionColumn.stories.svelte
- catalog/components/store-application/StoreFileEditor.stories.svelte
- catalog/components/store-application/StoreRepositoryApplication.stories.svelte
- catalog/components/store-application/StoreRepositoryBranchContext.stories.svelte
- catalog/components/store-application/StoreRepositoryFeedback.stories.svelte
- catalog/components/store-application/StoreRepositoryNavigation.stories.svelte
- catalog/components/store-application/StoreRepositorySearch.stories.svelte
- catalog/components/store-application/StoreWorkspaces.stories.svelte
- catalog/components/store-application/WorkspaceFiles.stories.svelte
- catalog/components/store-stack-workspace/StoreStackWorkspaceLoading.stories.svelte
- catalog/components/store-stack-workspace/StoreStackWorkspaceSettings.stories.svelte
- catalog/components/store-threads/StoreThreadAlignment.stories.svelte
- catalog/components/store-threads/StoreThreads.stories.svelte
- catalog/components/store-work/ProjectsActivity.stories.svelte
- catalog/components/store-work/ProjectsAgentActions.stories.svelte
- catalog/components/store-work/ProjectsFilters.stories.svelte
- catalog/components/store-work/ProjectsHub.stories.svelte
- catalog/components/store-work/ProjectsReviewDetail.stories.svelte
- catalog/components/store-application/AppearanceDiscussions.stories.svelte
- catalog/components/store-application/StoreCaptureSettings.stories.svelte
- catalog/components/store-stack-workspace/StoreStackWorkspaceConflictVersions.stories.svelte
- catalog/components/store-mutations/StoreConflictEditor.stories.svelte
- catalog/components/store-stack-workspace/StoreStackWorkspaceConflicts.stories.svelte
- catalog/components/store-application/InboxControls.stories.svelte
- catalog/components/store-threads/StoreThreadSearch.stories.svelte
- catalog/components/store-application/ResponsiveNavigation.stories.svelte
<!-- /evidence-field -->

<!-- evidence-field:open_story_audit yaml -->
## Open story audit

```yaml
- file: catalog/components/store-application/AgentDiscussion.stories.svelte
  story: ResumeReplayPreservesExactRepliesAndRecoversARetainedDraft
  status: requires separate final repair/verification
- file: catalog/components/store-application/AgentDiscussion.stories.svelte
  story: ExactReactionsShareTheMessageToolbarAndBar
  status: requires separate final repair/verification
- file: catalog/components/store-application/AgentDiscussion.stories.svelte
  story: SavedRepliesRetainDraftsAndDoNotPrompt
  status: requires separate final repair/verification
- file: catalog/components/store-application/AgentDiscussion.stories.svelte
  story: UnsupportedRepositoryRetainsReplyDraftWithoutDispatch
  status: requires separate final repair/verification
- file: catalog/components/store-application/AgentDiscussion.stories.svelte
  story: ReplyWaitsForOriginalPublication
  status: requires separate final repair/verification
- file: catalog/components/store-application/AgentDiscussion.stories.svelte
  story: UnknownReplyKeepsExactRequestForStatusRecovery
  status: requires separate final repair/verification
- file: catalog/components/store-application/AgentReplyIndependence.stories.svelte
  story: OfflineRepliesStayEditableAndSaveAfterFeedbackRefreshFails
  status: requires separate final repair/verification
- file: catalog/components/store-application/AgentReplyIndependence.stories.svelte
  story: OrdinaryNestedRepliesAndEditsStayIndependentOfAWorkingAgent
  status: requires separate final repair/verification
- file: catalog/components/store-application/AgentReplyIndependence.stories.svelte
  story: SharedChatRowsShowTimedLifecycleXMLContextAndExactToolReplies
  status: requires separate final repair/verification
- file: catalog/components/store-application/AgentReplyIndependence.stories.svelte
  story: ConsecutiveToolsExpandTogetherAndRetainExactCallActions
  status: requires separate final repair/verification
- file: catalog/components/store-application/ManagedAgentIdentity.stories.svelte
  story: RecordedIdentityLastActivityAndSelectedResponseSurviveDeletedSetup
  status: requires separate final repair/verification
- file: catalog/components/store-application/RepositoryReview.stories.svelte
  story: ShellAndRowLocalGraphSpacing
  status: requires separate final repair/verification
- file: catalog/components/store-application/RepositoryReview.stories.svelte
  story: RepositoryDirectoryNavigation
  status: requires separate final repair/verification
- file: catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte
  story: SelectBranchAndApplyToWorkspace
  status: requires separate final repair/verification
- file: catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte
  story: MultipleAdjacentDiffs
  status: requires separate final repair/verification
- file: catalog/components/store-stack-workspace/StoreStackWorkspaceForms.stories.svelte
  story: AssignSelectedChangesWithSettingsRowsAndBranchCommandPicker
  status: requires separate final repair/verification
- file: catalog/components/store-stack-workspace/StoreStackWorkspacePresentation.stories.svelte
  story: DiffHeaderAndWideActionMenus
  status: requires separate final repair/verification
- file: catalog/components/store-stack-workspace/StoreStackWorkspaceProduction.stories.svelte
  story: BoundedRealAdapterInApplicationShell
  status: requires separate final repair/verification
- file: catalog/components/store-stack-workspace/StoreStackWorkspaceProduction.stories.svelte
  story: ColdGraphPreparationRestoresOnlyStacks
  status: requires separate final repair/verification
- file: catalog/components/store-stack-workspace/StoreStackWorkspaceProduction.stories.svelte
  story: GraphPreparationSurvivesClearedFeedback
  status: requires separate final repair/verification
- file: catalog/components/store-stack-workspace/StoreStackWorkspaceProduction.stories.svelte
  story: DependentNativeBranchSegments
  status: requires separate final repair/verification
- file: catalog/components/store-stack-workspace/StoreStackWorkspaceSetup.stories.svelte
  story: UnconfiguredWorkspace
  status: requires separate final repair/verification
- file: catalog/components/store-stack-workspace/StoreStackWorkspaceSpacer.stories.svelte
  story: EmptyWorkspaceKeepsItsBranchPromptCentered
  status: requires separate final repair/verification

```
<!-- /evidence-field -->

