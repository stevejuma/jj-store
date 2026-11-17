---
schema: 1
id: E-UI-VISIBLE-NAMES-001
---

# E-UI-VISIBLE-NAMES-001 details 4

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 91
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook
```

**Notes**

Complete cold Dockerb92e3717:all1049 cases execute across four disjoint invocations. Shards262pass/11fail/2skip,263pass/21fail,201pass/37fail and230pass/22fail give956 pass/91 fail/two existing skips. Original complete counts621pass/426fail/2skip remain historical, not current. No accessibility rule, assertion, browser lane or baseline is waived. Remaining failures include capture filter names and selection, emoji/reaction names, tool-output/execution-access/console-step controls, review title/IDs, metadata/sent-comment controls, stale branch selectors, workflow history/author controls and contrast. Actual full diagnostic log:/tmp/jj-store-visible-names-full-storybook-20261117b.log; classified boundaries:/tmp/jj-store-visible-names-remaining-20261117b.json.
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
pnpm check:progress
```

**Notes**

Cold e2502dc8:187 tasks and1064 evidence records pass after retaining complete956pass/91fail/two-skip results. This is evidence bookkeeping only, not product qualification.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first
```

**Notes**

Cold78150cf1: SPEC-FIRST-VCS JST-GOV-002 refuses E-S6-UI-TOOLING-001 unchanged-contract confirmation while its completed validation is not passing. Keep that gate and pending result intact. The91 blocking full UI failures still prevent broad tool-cohort qualification; no record is relabeled pass to bypass the guard.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 7
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook -- catalog/components/store-application/StoreCaptureSettings.stories.svelte catalog/components/store-application/StoreMultiHarnessSettings.stories.svelte catalog/components/store-application/StoreProfileSettings.stories.svelte catalog/components/store-threads/StoreThreads.stories.svelte catalog/components/store-application/AgentRichTools.stories.svelte catalog/components/store-application/WorkflowHostSettings.stories.svelte catalog/components/store-workflows/WorkflowConsole.stories.svelte --testNamePattern '^SourceAvatarsExternalFiltersIconImportAndObservedFooter$|^IndependentCursorAndOpenCodeSettingsSaveAndReload$|^AddAndRemoveOwnedEmoji$|^SharedComposerRepliesAndExactReactions$|^CapturedReadContentHighlightsAndTogglesTheCompleteRawSnapshot$|^UnknownSaveRetainsOriginalDraft$|^ThreeActionsLoadLogsOnDemandAndRetainExpansionAcrossRefresh$'
```

**Notes**

Cold beb4792a:7 failures/59 unselected. Reproduces capture popover names, the stale harness selector, emoji upload, raw output, execution disclosure and workflow step names. SharedComposerRepliesAndExactReactions first fails while the lazy reply sidebar still renders its loading skeleton.
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
pnpm test:storybook -- catalog/components/store-application/StoreCaptureSettings.stories.svelte catalog/components/store-application/StoreMultiHarnessSettings.stories.svelte catalog/components/store-application/StoreProfileSettings.stories.svelte catalog/components/store-threads/StoreThreads.stories.svelte catalog/components/store-application/AgentRichTools.stories.svelte catalog/components/store-application/WorkflowHostSettings.stories.svelte catalog/components/store-workflows/WorkflowConsole.stories.svelte --testNamePattern '^SourceAvatarsExternalFiltersIconImportAndObservedFooter$|^IndependentCursorAndOpenCodeSettingsSaveAndReload$|^AddAndRemoveOwnedEmoji$|^SharedComposerRepliesAndExactReactions$|^CapturedReadContentHighlightsAndTogglesTheCompleteRawSnapshot$|^UnknownSaveRetainsOriginalDraft$|^ThreeActionsLoadLogsOnDemandAndRetainExpansionAcrossRefresh$'
```

**Notes**

Cold093e9f0e:5 pass/2 fail/59 unselected after complete visible names and reply-editor readiness. Reaction audit remains. Capture now exposes a nameless combobox: combobox requires an explicit author name, so restore its existing Filter by harness label and queries rather than treating its selected value as button content. Popover filter repairs remain valid.
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
pnpm test:storybook -- catalog/components/store-application/StoreCaptureSettings.stories.svelte catalog/components/store-application/StoreMultiHarnessSettings.stories.svelte catalog/components/store-application/StoreProfileSettings.stories.svelte catalog/components/store-threads/StoreThreads.stories.svelte catalog/components/store-application/AgentRichTools.stories.svelte catalog/components/store-application/WorkflowHostSettings.stories.svelte catalog/components/store-workflows/WorkflowConsole.stories.svelte --testNamePattern '^SourceAvatarsExternalFiltersIconImportAndObservedFooter$|^IndependentCursorAndOpenCodeSettingsSaveAndReload$|^AddAndRemoveOwnedEmoji$|^SharedComposerRepliesAndExactReactions$|^CapturedReadContentHighlightsAndTogglesTheCompleteRawSnapshot$|^UnknownSaveRetainsOriginalDraft$|^ThreeActionsLoadLogsOnDemandAndRetainExpansionAcrossRefresh$'
```

**Notes**

Cold8b046b01:6 pass/1 fail/59 unselected after restoring combobox naming and mapping the retained customEmojiUrl into the released public custom-reaction model. The reply reaction now directly asserts its retained blob/data image. A separate textual fallback reaction still fails the label/content rule; no label-only consumer workaround is added. Core0.13.4 registry archive integrity and source equality are verified (SHA256936d2bca9002998fbcc5d249ef5f953f01c3c3e09e23d6bbdd4f8d35e10439cb). Upstream Core change swolpwus reproduces two source regressions and repairs default/supplied textual fallback/count names; all3 owning Core stories pass. Full Core qualification, genuine patch release, registry adoption and this exact consumer target remain required.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook -- catalog/components/store-application/StoreCaptureSettings.stories.svelte catalog/components/store-application/StoreMultiHarnessSettings.stories.svelte catalog/components/store-application/StoreProfileSettings.stories.svelte catalog/components/store-threads/StoreThreads.stories.svelte catalog/components/store-application/AgentRichTools.stories.svelte catalog/components/store-application/WorkflowHostSettings.stories.svelte catalog/components/store-workflows/WorkflowConsole.stories.svelte
```

**Notes**

Cold a606cff6:62 pass/4 fail across all66 cases. Core textual-reaction naming remains blocked on its release; two stale recorded-console/runner-disclosure selectors and one message-editor pointer readiness failure need focused investigation. /tmp/jj-store-remaining-seven-families.log. Full956pass/91fail/two-skip counts remain the latest complete run, not recomputed from focused families.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook -- catalog/components/store-threads/StoreThreads.stories.svelte catalog/components/store-workflows/WorkflowConsole.stories.svelte catalog/components/store-application/WorkflowHostSettings.stories.svelte --testNamePattern '^MessageEditIsAHoverMenuActionAndUpdatesTheExactMessage$|^RecordedStepHasAForgejoConsoleAndCollapsedInlineOutput$|^EnvironmentFormDerivesRepositoryContext$'
```

**Notes**

Cold74e353af:1 pass/2 fail/28 unselected. MessageEditIsAHoverMenuActionAndUpdatesTheExactMessage passes unchanged in isolation; the two exact-name selectors fail. No editor timeout, menu lease, pointer behavior or assertion is changed. /tmp/jj-store-family-followups-before.log.
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
pnpm test:storybook -- catalog/components/store-threads/StoreThreads.stories.svelte catalog/components/store-workflows/WorkflowConsole.stories.svelte catalog/components/store-application/WorkflowHostSettings.stories.svelte --testNamePattern '^MessageEditIsAHoverMenuActionAndUpdatesTheExactMessage$|^RecordedStepHasAForgejoConsoleAndCollapsedInlineOutput$|^EnvironmentFormDerivesRepositoryContext$'
```

**Notes**

Cold0046fb8d:2 pass/1 fail/28 unselected after matching complete step/disclosure names. Console advances to a duplicated Run output accessible prefix introduced by the earlier step-name repair. Repair the owning unscoped presentation so its visible Run output provides its name once; preserve scoped status/duration and every assertion. /tmp/jj-store-family-followups-after.log.
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
pnpm test:storybook -- catalog/components/store-workflows/WorkflowConsole.stories.svelte --testNamePattern '^RecordedStepHasAForgejoConsoleAndCollapsedInlineOutput$'
```

**Notes**

Cold961de455:1 pass/17 unselected. Recorded console retains its collapsed step, exact run attempt, output loading and stdout assertions. /tmp/jj-store-recorded-console-final.log.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook -- catalog/components/store-work/ProjectsDirectory.stories.svelte catalog/components/store-application/AgentChatBody.stories.svelte catalog/components/store-application/ServerConnections.stories.svelte catalog/components/store-appearance/StoreLinkRendering.stories.svelte --testNamePattern '^ReviewMetadataInListAndCards$|^TypingAnchorsADiscussionAndPreservesTheCapturedText$|^Expanded$|^CompactCardsOnlyForStandaloneContent$'
```

**Notes**

Cold484c7e82:4 fail/40 unselected before the next product repairs. Exact axe failures reproduce review excerpt/abbreviated native ID, singular sent-comment text, visible server initial plus name and link-card host/title naming. /tmp/jj-store-visible-names-next-four-before.log.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook -- catalog/components/store-work/ProjectsDirectory.stories.svelte catalog/components/store-application/AgentChatBody.stories.svelte catalog/components/store-application/ServerConnections.stories.svelte catalog/components/store-appearance/StoreLinkRendering.stories.svelte --testNamePattern '^ReviewMetadataInListAndCards$|^TypingAnchorsADiscussionAndPreservesTheCapturedText$|^Expanded$|^CompactCardsOnlyForStandaloneContent$'
```

**Notes**

Coldd7192bfd:3 pass/1 fail/40 unselected. Server, comment and compact-card axe cases pass. Review cannot finish initial rendering: the newly added nested $props.id initializer violates the Svelte compiler placement rule. Preserve this failed result; no readiness timeout changes. /tmp/jj-store-visible-names-next-four-after.log.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook -- catalog/components/store-work/ProjectsDirectory.stories.svelte --testNamePattern '^ReviewMetadataInListAndCards$'
```

**Notes**

Cold556cdb94:1 fail/5 unselected reproduces initial review render failure. Direct svelte/compiler inspection identifies props_id_invalid_placement; change only to a direct $props.id() declaration, then the same compiler check passes. /tmp/jj-store-review-names-exact-after.log.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook -- catalog/components/store-work/ProjectsDirectory.stories.svelte --testNamePattern '^ReviewMetadataInListAndCards$'
```

**Notes**

Coldf0d45241:1 fail/5 unselected after compiler repair. Actual review controls and new full-ID description assertions now execute, exposing the extra sr-only description element in the direct-child layout measurement (negative gap). Preserve all geometry assertions. /tmp/jj-store-review-names-exact-final.log.
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
pnpm test:storybook -- catalog/components/store-work/ProjectsDirectory.stories.svelte --testNamePattern '^ReviewMetadataInListAndCards$'
```

**Notes**

Coldcee26404:1 pass/5 unselected. Use a truly hidden, explicitly referenced native-ID description node so it adds no layout box; browser accessible-description assertion retains the full32-character native ID. Existing exact ID title, list/grid geometry, comment count, author and width assertions pass; no CSS/layout/assertion workaround. /tmp/jj-store-review-names-hidden-description.log.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook -- catalog/components/store-work/ProjectsDirectory.stories.svelte catalog/components/store-application/AgentChatBody.stories.svelte catalog/components/store-application/ServerConnections.stories.svelte catalog/components/store-appearance/StoreLinkRendering.stories.svelte catalog/components/store-application/StoreCollaboration.stories.svelte
```

**Notes**

Coldf300f961:52 pass/2 fail across all54 cases. All four changed presentation families pass; two collaboration stories still select obsolete anchor-derived Revision labels instead of the displayed review excerpt. Preserve their exact interaction/outcome checks and select the complete fixture excerpt/status/count name. /tmp/jj-store-visible-names-next-families.log.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook -- catalog/components/store-application/StoreCollaboration.stories.svelte --testNamePattern '^ReviewDiscussionActions$|^ReviewOutcomeUnknown$'
```

**Notes**

Cold0d237289:2 fail/8 unselected at obsolete review name selectors. /tmp/jj-store-review-actions-selectors-before.log.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook -- catalog/components/store-application/StoreCollaboration.stories.svelte --testNamePattern '^ReviewDiscussionActions$|^ReviewOutcomeUnknown$'
```

**Notes**

Coldcb9b8436:2 pass/8 unselected after selecting the exact displayed fixture excerpt, state and comment count. Discussion reply depth, exact comment content, send/resolve and unknown-outcome recovery assertions remain. /tmp/jj-store-review-actions-selectors-after.log.
<!-- /run -->
<!-- /evidence-field -->

