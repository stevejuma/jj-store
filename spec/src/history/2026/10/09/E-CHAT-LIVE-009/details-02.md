---
schema: 1
id: E-CHAT-LIVE-009
---

# E-CHAT-LIVE-009 validation

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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/conversation-projection.test.ts -t 'execute output uses committed'
```

**Notes**

Before: one failure, seven skipped; JSON envelope replaces exact stdout. .local/execute-output-before.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatChanges.stories.svelte -t SelectedLiveAvatar
```

**Notes**

Before: one failure, one skipped; presence contains a status glyph. .local/avatar-dot-before.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatChanges.stories.svelte -t SelectedLiveAvatar
```

**Notes**

Exact avatar regression: one pass, one skipped; accessible Idle name retained. .local/avatar-dot-after.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/conversation-projection.test.ts -t 'execute output uses committed'
```

**Notes**

Exact stdout regression passes after alias projection; seven skipped. .local/execute-output-after.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/conversation-projection.test.ts -t 'terminal-only committed'
```

**Notes**

Real feedback exposed a second case, reproduced before the Shell fallback. .local/execute-terminal-before.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/conversation-projection.test.ts web/tests/vite/agent-tool-rich.test.ts
```

**Notes**

Final 19 tests pass, including exact stdout, empty aliases/stderr, Unicode and terminal-only whole-section source semantics. .local/execute-output-units-final.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentTerminalOutput.stories.svelte
```

**Notes**

Two new rich/raw terminal stories pass. .local/execute-terminal-stories.log.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 14
hasNote: true
noteEncoding: markdown
```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/Agent*.stories.svelte catalog/components/store-application/ManagedAgentIdentity.stories.svelte catalog/components/store-application/Conversation*.stories.svelte catalog/components/store-work/ProjectsAgents.stories.svelte catalog/components/store-work/ProjectsAgentActions.stories.svelte
```

**Notes**

145 pass, 14 fail in 30 files. Wider lane selected because agent avatars appear across these surfaces. Existing reply/Send/default-view and menu failures are reproduced on the previous revision below. .local/execute-avatar-stories.log.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 6
hasNote: true
noteEncoding: markdown
```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentDiscussion.stories.svelte catalog/components/store-application/AgentFleet.stories.svelte catalog/components/store-application/AgentPromptMarkup.stories.svelte catalog/components/store-application/AgentReplyIndependence.stories.svelte catalog/components/store-application/AgentSessionSettings.stories.svelte catalog/components/store-application/AgentSessions.stories.svelte -t "ResumeReplayPreservesExactRepliesAndRecoversARetainedDraft|ExactReactionsShareTheMessageToolbarAndBar|SavedRepliesRetainDraftsAndDoNotPrompt|UnsupportedRepositoryRetainsReplyDraftWithoutDispatch|ReplyWaitsForOriginalPublication|UnknownReplyKeepsExactRequestForStatusRecovery|ContextualSidebarAndPersistentAgentTags|OpenCodeTypedTextMessageParts|OfflineRepliesStayEditableAndSaveAfterFeedbackRefreshFails|OrdinaryNestedRepliesAndEditsStayIndependentOfAWorkingAgent|SharedChatRowsShowTimedLifecycleXMLContextAndExactToolReplies|ConsecutiveToolsExpandTogetherAndRetainExactCallActions|ChatHeaderMenuAndListToggleRetainTheDraft|NewChatKeepsDraftThroughConnectWithoutSubmittingIt"
```

**Notes**

First disposable parent baseline could not import setup-file through a shared node_modules symlink: six suite-load failures, no tests. .local/execute-avatar-baseline.log.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown
```

```shell
pnpm install --frozen-lockfile --offline --ignore-scripts
```

**Notes**

Disposable baseline install refused the pre-existing Design Core lockfile specifier mismatch. No lockfile repair was introduced. Existing dependencies were cloned with APFS cp -cR for identical baseline validation. .local/execute-avatar-baseline-install.log.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 14
hasNote: true
noteEncoding: markdown
```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentDiscussion.stories.svelte catalog/components/store-application/AgentFleet.stories.svelte catalog/components/store-application/AgentPromptMarkup.stories.svelte catalog/components/store-application/AgentReplyIndependence.stories.svelte catalog/components/store-application/AgentSessionSettings.stories.svelte catalog/components/store-application/AgentSessions.stories.svelte -t "ResumeReplayPreservesExactRepliesAndRecoversARetainedDraft|ExactReactionsShareTheMessageToolbarAndBar|SavedRepliesRetainDraftsAndDoNotPrompt|UnsupportedRepositoryRetainsReplyDraftWithoutDispatch|ReplyWaitsForOriginalPublication|UnknownReplyKeepsExactRequestForStatusRecovery|ContextualSidebarAndPersistentAgentTags|OpenCodeTypedTextMessageParts|OfflineRepliesStayEditableAndSaveAfterFeedbackRefreshFails|OrdinaryNestedRepliesAndEditsStayIndependentOfAWorkingAgent|SharedChatRowsShowTimedLifecycleXMLContextAndExactToolReplies|ConsecutiveToolsExpandTogetherAndRetainExactCallActions|ChatHeaderMenuAndListToggleRetainTheDraft|NewChatKeepsDraftThroughConnectWithoutSubmittingIt"
```

**Notes**

Parent f11e9b9b reproduces the same 14 failures, with 28 skipped, in 31.24s. .local/execute-avatar-baseline-final.log. Baseline workspace and cloned dependencies subsequently removed.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentTerminalOutput.stories.svelte catalog/components/store-application/AgentChatChanges.stories.svelte catalog/components/store-application/AgentRichTools.stories.svelte catalog/components/store-application/Conversation*.stories.svelte
```

**Notes**

Final 13 directly affected interaction/accessibility stories pass in five files after the terminal fallback and neutral offline tone. .local/execute-avatar-targeted-final.log.
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
pnpm web:check
```

**Notes**

Final zero errors/warnings. .local/execute-avatar-web-final.log.
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
pnpm catalog:check
```

**Notes**

Initial new fixture imported an API model type directly; catalog ownership guard refused that transport boundary. Changed to structurally inferred production presentation input. .local/execute-avatar-catalog-final.log.
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
pnpm catalog:check
```

**Notes**

Final zero errors/warnings, 347 catalog modules. .local/execute-avatar-catalog-verified.log.
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
pnpm check:ui-ownership
```

**Notes**

Owned UI boundary passes. .local/execute-avatar-ownership.log.
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
pnpm check:progress
pnpm spec:first
pnpm exec spec-validator validate --only summary,fileLimits,governance,verification,book
pnpm exec prettier --check web/src/lib/conversations/tool-adapter.ts web/src/lib/ui/community-member-avatar/CommunityMemberAvatar.svelte web/src/lib/ui/community-member-avatar/community-member-avatar.css web/tests/vite/conversation-projection.test.ts catalog/components/store-application/AgentChatChanges.stories.svelte catalog/components/store-application/AgentTerminalOutput.stories.svelte catalog/fixtures/AgentTerminalOutputFixture.svelte spec/src/current/contracts/ui/34-interactive-agent-sessions.md spec/src/current/contracts/ui/37-document-conversations.md spec/src/current/contracts/api/41-interactive-agent-execution.md spec/src/current/progress/M24/index.md spec/src/current/progress/M24/history/M24/index.md
```

**Notes**

Final ledger has 146 tasks/692 records. Spec-first covers seven protected files and three chapters. Five document validators and changed-source/current Markdown formatting pass. Unrelated generated M25/Bootstrap whitespace restored. Historical evidence retains the excluded canonical format. Disposable baseline workspace forgotten and removed after its test completed.
<!-- /run -->

<!-- /evidence-field -->
