---
schema: 1
id: E-UI-VISIBLE-NAMES-001
---

# E-UI-VISIBLE-NAMES-001 details 5

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
pnpm test:storybook -- catalog/components/store-application/AgentSessionSettings.stories.svelte catalog/components/store-application/AgentFleet.stories.svelte --testNamePattern '^RuntimeSettingsSubmitExactNegotiatedPreferencesWithoutSendingTheChatDraft$|^PickerPinSelectsAnAgentWithoutSending$'
```

**Notes**

One execution-lease guard failure; zero product tests executed. Attempt exits1 at the active-workspace validation lease before creating a worker or running any test. The five-family run was still live; retain its handle and await terminal state before the actual reproduction. No product failure count is inferred. /tmp/jj-store-runtime-names-before.log.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook -- catalog/components/store-application/AgentSessionSettings.stories.svelte catalog/components/store-application/AgentFleet.stories.svelte --testNamePattern '^RuntimeSettingsSubmitExactNegotiatedPreferencesWithoutSendingTheChatDraft$|^PickerPinSelectsAnAgentWithoutSending$'
```

**Notes**

Cold368d8b51:2 fail/9 unselected. Reproduces omitted visible field/current-value names in agent runtime settings and omitted avatar/presence semantics in the Model footer picker. The owned repairs preserve negotiated commands, values, drafts, callbacks and live presence. /tmp/jj-store-runtime-names-before2.log.
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
pnpm test:storybook -- catalog/components/store-application/AgentSessionSettings.stories.svelte catalog/components/store-application/AgentFleet.stories.svelte --testNamePattern '^RuntimeSettingsSubmitExactNegotiatedPreferencesWithoutSendingTheChatDraft$|^PickerPinSelectsAnAgentWithoutSending$'
```

**Notes**

Cold runtime focused rerun:2 pass/9 unselected. All four runtime fields retain their values in their natural names, and the Model footer preserves its public avatar/presence content. Exact negotiated preferences and unsent chat draft assertions remain. /tmp/jj-store-runtime-names-after.log.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook -- catalog/components/store-application/AgentSessionSettings.stories.svelte catalog/components/store-application/AgentFleet.stories.svelte catalog/components/store-application/AgentExecution.stories.svelte catalog/components/store-application/AgentChatBody.stories.svelte catalog/components/store-application/StoreCollaboration.stories.svelte
```

**Notes**

Colddb65675c:57 pass/1 fail across all58 cases. The remaining old avatar selector omits the selected-image name. A ResizeObserver console diagnostic is retained separately; it does not cancel or waive any interaction/accessibility assertion. /tmp/jj-store-runtime-and-review-families-after.log.
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
pnpm test:storybook -- catalog/components/store-application/AgentSessionSettings.stories.svelte --testNamePattern '^ChatHeaderMenuAndListToggleRetainTheDraft$'
```

**Notes**

Coldee1e1522:1 fail/3 unselected reproduces the old exact Choose agent image selector. /tmp/jj-store-runtime-avatar-selector-before.log.
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
pnpm test:storybook -- catalog/components/store-application/AgentSessionSettings.stories.svelte --testNamePattern '^ChatHeaderMenuAndListToggleRetainTheDraft$'
```

**Notes**

Colddd85b9e8:1 fail/3 unselected. A space-delimited prefix is also wrong: the actual accessible name uses a colon before Selected avatar. Read the rendered button and retain an exact assertion for its complete accessible name. /tmp/jj-store-runtime-avatar-selector-after.log.
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
pnpm test:storybook -- catalog/components/store-application/AgentSessionSettings.stories.svelte --testNamePattern '^ChatHeaderMenuAndListToggleRetainTheDraft$'
```

**Notes**

Coldada7aed8:1 pass/3 unselected. Exact Choose agent image: Selected avatar assertion passes, followed by unchanged configuration editing, cancellation, header/list toggling and draft retention. /tmp/jj-store-runtime-avatar-selector-final.log.
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

Coldd0685b47:zero errors/warnings;489-module ownership boundary and212-requirement story parity pass after the current runtime/review fixes. /tmp/jj-store-visible-names-catalog-check2.log.
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
pnpm web:check
```

**Notes**

Cold0030c5eb:zero errors/warnings in the production web surface. /tmp/jj-store-visible-names-web-check2.log.
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
pnpm catalog:test
```

**Notes**

Cold9ca22dc7:all193 files pass,1221 cases pass with one existing skip. This is the catalog-unit surface, not the whole Storybook suite or Rust qualification. /tmp/jj-store-visible-names-catalog-test2.log.
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
pnpm test:storybook -- catalog/components/store-application/AgentSessionSettings.stories.svelte catalog/components/store-application/AgentFleet.stories.svelte catalog/components/store-application/AgentExecution.stories.svelte catalog/components/store-application/AgentChatBody.stories.svelte catalog/components/store-application/StoreCollaboration.stories.svelte
```

**Notes**

Colda2068e23:all58 cases in the five affected runtime/review/chat families pass. No further unselected cases. Complete source UI remains held on the published Core fix and other previously recorded boundaries; latest1049-case full run remains956pass/91fail/two existing skips. /tmp/jj-store-runtime-and-review-families-final.log.
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
pnpm check:progress
```

**Notes**

Coldc9f00202:one evidence-schema failure: the failed execution-lease attempt incorrectly declares zero command failures. Preserve its fail result and zero executed product cases, but record the actual one guard failure; do not relax the validator or change any full UI count. /tmp/jj-store-visible-names-progress2.log.
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
pnpm check:progress
```

**Notes**

Cold666b7578:187 tasks/1064 evidence records pass after correcting one command-lease result count. This is bookkeeping only; full UI/tool-cohort/audit acceptance stays pending. /tmp/jj-store-visible-names-progress3.log.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: fail
failures: 6
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook -- catalog/components/store-application/AgentDefaults.stories.svelte catalog/components/store-application/QueuedAgentMessages.stories.svelte catalog/components/store-application/RepositoryReview.stories.svelte catalog/components/store-workflows/WorkflowHistoryNavigation.stories.svelte catalog/components/store-work/ProjectsHub.stories.svelte catalog/components/store-workflows/WorkflowYaml.stories.svelte catalog/components/store-application/NativeReviewDetail.stories.svelte --testNamePattern '^DefaultsLastWithSharedHarnessModelPickerAndJoinedSave$|^RefusalRetainsAnEditableDraft$|^ShellAndRowLocalGraphSpacing$|^ResizableWhiteWorkflowPanelAndAlignedAvatarRows$|^CompactProjectsSharingARepository$|^YAMLFillsCreateAndEditDialogsWithoutChangingFormInspectorState$|^ReviewPageIgnoresDiscussionLayoutPreference$'
```

**Notes**

Cold next-seven reproduction:1 pass/6 fail/30 unselected. ProjectsHub passes unchanged. Five failures reproduce stale disclosure/review/branch/card names and an attribute-only avatar query; the sixth is a real4.49:1 destructive Alert.Description contrast violation in released Core0.13.4. /tmp/jj-store-next-seven-boundaries-before.log.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook -- catalog/components/store-application/AgentDefaults.stories.svelte catalog/components/store-application/QueuedAgentMessages.stories.svelte catalog/components/store-application/RepositoryReview.stories.svelte catalog/components/store-workflows/WorkflowHistoryNavigation.stories.svelte catalog/components/store-work/ProjectsHub.stories.svelte catalog/components/store-workflows/WorkflowYaml.stories.svelte catalog/components/store-application/NativeReviewDetail.stories.svelte --testNamePattern '^DefaultsLastWithSharedHarnessModelPickerAndJoinedSave$|^RefusalRetainsAnEditableDraft$|^ShellAndRowLocalGraphSpacing$|^ResizableWhiteWorkflowPanelAndAlignedAvatarRows$|^CompactProjectsSharingARepository$|^YAMLFillsCreateAndEditDialogsWithoutChangingFormInspectorState$|^ReviewPageIgnoresDiscussionLayoutPreference$'
```

**Notes**

Cold exact rerun:6 pass/1 fail/30 unselected. Native review selection asserts the exact native ID description and visible excerpt; avatar geometry now uses semantic role/name queries. The original refusal assertion remains blocking on Core contrast. Core change kytkkzwq independently reproduces the identical4.49:1 source violation, repairs its owning colour rule and passes both new light/dark cases plus all5 Alert cases. No local colour override or accessibility exception is added. /tmp/jj-store-next-seven-boundaries-after.log.
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
pnpm test:storybook -- catalog/components/store-application/AgentDefaults.stories.svelte catalog/components/store-application/RepositoryReview.stories.svelte catalog/components/store-workflows/WorkflowHistoryNavigation.stories.svelte catalog/components/store-work/ProjectsHub.stories.svelte catalog/components/store-workflows/WorkflowYaml.stories.svelte catalog/components/store-application/NativeReviewDetail.stories.svelte
```

**Notes**

Cold543ee088:all27 cases across the six affected selector families pass. The refusal case remains separately held on the required Core release. /tmp/jj-store-next-six-families-after.log.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook -- catalog/components/store-application/StoreCaptureSettings.stories.svelte catalog/components/store-application/StoreCaptureSources.stories.svelte catalog/components/community-projects/CommunityProjectsRevisionGraph.stories.svelte catalog/components/store-application/AgentMentionSettings.stories.svelte --testNamePattern '^SourceStatusCommandSearchesTogglesAndCombinesMultipleSelections$|^SourceRepositoryCommandCombinesRepositoriesStatusesAndUnknownBindings$|^GitBackedOperationMessageCopies$|^SettingsDropdownsUseShadcnKeyboardSelectionAndPreserveDefaults$'
```

**Notes**

Cold last-four reproduction:3 pass/1 fail/30 unselected. Capture selection and settings keyboard interaction pass unchanged; the message-reference copy button overrides its visible abbreviation with the full ID and violates the existing naming contract. Preserve the test selector and repair its owning component. /tmp/jj-store-last-four-boundaries-before.log.
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
pnpm test:storybook -- catalog/components/store-application/StoreCaptureSettings.stories.svelte catalog/components/store-application/StoreCaptureSources.stories.svelte catalog/components/community-projects/CommunityProjectsRevisionGraph.stories.svelte catalog/components/store-application/AgentMentionSettings.stories.svelte --testNamePattern '^SourceStatusCommandSearchesTogglesAndCombinesMultipleSelections$|^SourceRepositoryCommandCombinesRepositoriesStatusesAndUnknownBindings$|^GitBackedOperationMessageCopies$|^SettingsDropdownsUseShadcnKeyboardSelectionAndPreserveDefaults$'
```

**Notes**

Cold5031de10:4 pass/30 unselected. The owned graph-message reference derives its name from its visible abbreviated ID and retains the full ID in a hidden, explicitly referenced description. The original exact-name, visible-text, mouse/keyboard clipboard payload and independent row-selection assertions remain; the full-ID description assertion is added. Community is inspected read-only for provenance; no excluded repository is changed. /tmp/jj-store-last-four-boundaries-after.log.
<!-- /run -->
<!-- /evidence-field -->

