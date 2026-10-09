---
schema: 1
id: E-CHAT-LIVE-011
---

# E-CHAT-LIVE-011 details 2

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
pnpm test:storybook catalog/components/store-application/AgentChatLayout.stories.svelte -t MessagesScrollAboveTheBottomComposerWithKeyboardJumpToLatest
```

**Notes**

Before: one failure/one skipped. Live Document input has no inline next-message ancestor.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-application/AgentProviderConfiguration.stories.svelte -t AddAgentDiscoversUnsavedProvidersAndUsesSettingsRows
```

**Notes**

Before: 13px hover radius rather than square; one failure/two skipped, using unchanged disclosure source.
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
pnpm test:storybook catalog/components/store-application/AgentDefaults.stories.svelte catalog/components/store-application/AgentProviderConfiguration.stories.svelte catalog/components/store-application/LocalCompute.stories.svelte
```

**Notes**

17 focused interaction/accessibility stories pass, including Advanced draft retention and full-width defaults geometry.
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
pnpm test:storybook catalog/components/store-application/AgentChatLayout.stories.svelte catalog/components/store-application/ConversationContinuity.stories.svelte
```

**Notes**

Five final exact stories pass: inline input, no frame/inner scroll, roomy editable region, deferred IME switch, selection/draft retention, remount and keyboard mixed send. Intermediate runs exposed a Svelte conditional-binding warning (fixed with state), a test copying class getters with object spread (fixed with explicit anchor/head), and a smooth-scroll assertion racing its final geometry (same assertion now awaited).
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 19
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-application/AgentChatBody.stories.svelte catalog/components/store-application/AgentChatChanges.stories.svelte catalog/components/store-application/AgentChatFeedback.stories.svelte catalog/components/store-application/AgentChatLayout.stories.svelte catalog/components/store-application/AgentChatRuntime.stories.svelte catalog/components/store-application/AgentComments.stories.svelte catalog/components/store-application/AgentContextDelivery.stories.svelte catalog/components/store-application/AgentDefaults.stories.svelte catalog/components/store-application/AgentDiscussion.stories.svelte catalog/components/store-application/AgentExecution.stories.svelte catalog/components/store-application/AgentFleet.stories.svelte catalog/components/store-application/AgentHarnessModels.stories.svelte catalog/components/store-application/AgentInboxNavigation.stories.svelte catalog/components/store-application/AgentLayout.stories.svelte catalog/components/store-application/AgentMentionRecovery.stories.svelte catalog/components/store-application/AgentMentionSettings.stories.svelte catalog/components/store-application/AgentPromptMarkup.stories.svelte catalog/components/store-application/AgentProviderConfiguration.stories.svelte catalog/components/store-application/AgentReplyIndependence.stories.svelte catalog/components/store-application/AgentRevisionIndex.stories.svelte catalog/components/store-application/AgentRichTools.stories.svelte catalog/components/store-application/AgentRuntimes.stories.svelte catalog/components/store-application/AgentSessionSettings.stories.svelte catalog/components/store-application/AgentSessions.stories.svelte catalog/components/store-application/AgentTerminalOutput.stories.svelte catalog/components/store-application/AgentToolGroups.stories.svelte catalog/components/store-application/ManagedAgentIdentity.stories.svelte catalog/components/store-application/NativeAgentMenus.stories.svelte catalog/components/store-application/NativeMessageActions.stories.svelte catalog/components/store-application/NativeReviewDetail.stories.svelte catalog/components/store-application/RepositoryReview.stories.svelte catalog/components/store-application/StoreCollaboration.stories.svelte catalog/components/store-application/AppearanceDiscussions.stories.svelte catalog/components/store-application/DiscussionSheets.stories.svelte catalog/components/store-application/StoreRepositoryFeedback.stories.svelte catalog/components/store-application/StoreFileEditor.stories.svelte catalog/components/store-application/LocalCompute.stories.svelte catalog/components/store-application/ConversationContinuity.stories.svelte catalog/components/store-threads/StoreCapturedReactions.stories.svelte catalog/components/store-threads/StoreThreadAlignment.stories.svelte catalog/components/store-threads/StoreThreadNavigation.stories.svelte catalog/components/store-threads/StoreThreadSearch.stories.svelte catalog/components/store-threads/StoreThreads.stories.svelte catalog/components/store-appearance/AppearanceComposer.stories.svelte catalog/components/store-appearance/AppearanceDialog.stories.svelte catalog/components/store-work/StoreWork.stories.svelte catalog/components/store-work/ProjectsFeedback.stories.svelte catalog/components/store-work/ProjectsDirectory.stories.svelte catalog/components/store-work/TaskScroll.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceForms.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceFooter.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceInline.stories.svelte catalog/components/store-mutations/StoreRepositoryActions.stories.svelte
```

**Notes**

Initial shared-surface run: 239 pass/19 fail across 53 files. Three new failures are old ai-chat-composer status ancestry assertions; updated to the shared owned input group while preserving all error/reconnect and draft checks. Other 16 failures reproduced unchanged on the parent.
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
pnpm test:storybook catalog/components/store-application/AgentChatChanges.stories.svelte catalog/components/store-application/AgentChatRuntime.stories.svelte
```

**Notes**

Eight status/recovery stories pass after aligning their presentation ancestor to Document.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 16
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-application/AgentDiscussion.stories.svelte catalog/components/store-application/AgentFleet.stories.svelte catalog/components/store-application/AgentReplyIndependence.stories.svelte catalog/components/store-application/AgentSessionSettings.stories.svelte catalog/components/store-application/AgentSessions.stories.svelte catalog/components/store-application/RepositoryReview.stories.svelte catalog/components/store-application/StoreCollaboration.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceForms.stories.svelte
```

**Notes**

Unchanged parent e2ffadad in isolated document-input-baseline: same 16 failures, 32 pass. Older reply/default-view/Send/menu/status/branch-picker expectations are independent. Baseline used an APFS clone of existing dependencies because the previously recorded temporary foundational lockfile does not support a frozen registry install. Its workspace was forgotten and checkout retained under ignored .local/document-input-baseline; broad rm cleanup was rejected, so no files were deleted.
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
pnpm test:storybook catalog/components/store-application/ConversationContinuity.stories.svelte catalog/components/store-appearance/AppearanceComposer.stories.svelte
```

**Notes**

Six fresh-process stories pass after lifecycle/send extraction, covering remounts, footer actions and unchanged Classic palettes/disabled reply context.
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
pnpm catalog:test web/tests/vite/conversation-session.test.ts web/tests/vite/agent-mention-settings.test.ts
```

**Notes**

Nine state/audience tests pass.
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
pnpm web:check
```

**Notes**

Final zero errors/warnings. Earlier conditional editor binding emitted one warning before its state fix.
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
pnpm catalog:check
```

**Notes**

Final zero errors/warnings and ownership of catalog boundaries passes. Initial warning-only run failed fail-on-warnings, then was corrected.
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
pnpm check:ui-ownership
```

**Notes**

Public foundational and owned Community boundaries pass with both extracted components attributed.
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
pnpm check:architecture
```

**Notes**

Final unchanged gap only: scripts/dev-ui.acceptance.mjs has 311 counted lines against 300. Intermediate additions exceeded component caps; those were repaired without exceptions or cap changes.
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
pnpm progress:history
```

**Notes**

M24 evidence entry refreshed; unrelated generated M25/Bootstrap formatting will be restored.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 16
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-application/AgentChatBody.stories.svelte catalog/components/store-application/AgentChatChanges.stories.svelte catalog/components/store-application/AgentChatFeedback.stories.svelte catalog/components/store-application/AgentChatLayout.stories.svelte catalog/components/store-application/AgentChatRuntime.stories.svelte catalog/components/store-application/AgentComments.stories.svelte catalog/components/store-application/AgentContextDelivery.stories.svelte catalog/components/store-application/AgentDefaults.stories.svelte catalog/components/store-application/AgentDiscussion.stories.svelte catalog/components/store-application/AgentExecution.stories.svelte catalog/components/store-application/AgentFleet.stories.svelte catalog/components/store-application/AgentHarnessModels.stories.svelte catalog/components/store-application/AgentInboxNavigation.stories.svelte catalog/components/store-application/AgentLayout.stories.svelte catalog/components/store-application/AgentMentionRecovery.stories.svelte catalog/components/store-application/AgentMentionSettings.stories.svelte catalog/components/store-application/AgentPromptMarkup.stories.svelte catalog/components/store-application/AgentProviderConfiguration.stories.svelte catalog/components/store-application/AgentReplyIndependence.stories.svelte catalog/components/store-application/AgentRevisionIndex.stories.svelte catalog/components/store-application/AgentRichTools.stories.svelte catalog/components/store-application/AgentRuntimes.stories.svelte catalog/components/store-application/AgentSessionSettings.stories.svelte catalog/components/store-application/AgentSessions.stories.svelte catalog/components/store-application/AgentTerminalOutput.stories.svelte catalog/components/store-application/AgentToolGroups.stories.svelte catalog/components/store-application/ManagedAgentIdentity.stories.svelte catalog/components/store-application/NativeAgentMenus.stories.svelte catalog/components/store-application/NativeMessageActions.stories.svelte catalog/components/store-application/NativeReviewDetail.stories.svelte catalog/components/store-application/RepositoryReview.stories.svelte catalog/components/store-application/StoreCollaboration.stories.svelte catalog/components/store-application/AppearanceDiscussions.stories.svelte catalog/components/store-application/DiscussionSheets.stories.svelte catalog/components/store-application/StoreRepositoryFeedback.stories.svelte catalog/components/store-application/StoreFileEditor.stories.svelte catalog/components/store-application/LocalCompute.stories.svelte catalog/components/store-application/ConversationContinuity.stories.svelte catalog/components/store-threads/StoreCapturedReactions.stories.svelte catalog/components/store-threads/StoreThreadAlignment.stories.svelte catalog/components/store-threads/StoreThreadNavigation.stories.svelte catalog/components/store-threads/StoreThreadSearch.stories.svelte catalog/components/store-threads/StoreThreads.stories.svelte catalog/components/store-appearance/AppearanceComposer.stories.svelte catalog/components/store-appearance/AppearanceDialog.stories.svelte catalog/components/store-work/StoreWork.stories.svelte catalog/components/store-work/ProjectsFeedback.stories.svelte catalog/components/store-work/ProjectsDirectory.stories.svelte catalog/components/store-work/TaskScroll.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceForms.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceFooter.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceInline.stories.svelte catalog/components/store-mutations/StoreRepositoryActions.stories.svelte
```

**Notes**

Final fresh shared-surface run: 242 pass/16 fail across 53 files. Exact failure names match unchanged-parent baseline. Three status ancestry regressions are repaired; no additional functional/accessibility failures are introduced. .local/document-consumer-stories-final.log.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --check <all changed source/current contract files>
```

**Notes**

M24 generated navigation needed formatting after history regeneration; source files passed. Final exact file list and result are recorded in the following check.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm check:progress
```
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:validate --only governance,summary,fileLimits,book
```

**Notes**

New split evidence validation page lacked generated navigation; fixed by the owning updateNavigation helper.
<!-- /run -->
<!-- /evidence-field -->

