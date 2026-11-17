---
schema: 1
id: E-UI-VISIBLE-NAMES-001
---

# E-UI-VISIBLE-NAMES-001 details 3

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
pnpm test:storybook -- catalog/components/store-forge/StoreIntegrations.stories.svelte catalog/components/store-forge/StoreForgeInlineWorkspace.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceBranches.stories.svelte catalog/components/store-stack-workspace/StoreStackTopology.stories.svelte --testNamePattern 'GitHubAccountSetupInGlobalSettings|ConnectedAccountAndForget|^CreateInlineAndFloat$|AppliedBranchesHaveTheirOwnGroup|StackDiscoveryHidesDependentEntriesAndKeepsPartialApplicationExplicit'
```

**Notes**

Cold Docker6cd03569 repairs the Apply/Unapply visible label but exposes a punctuation-space mismatch in the new selector. An attempted subsequent patch did not match formatter-wrapped source; its unchanged repeat retains4 pass/1 fail. No passing gate is claimed for either attempt.
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
pnpm test:storybook -- catalog/components/store-forge/StoreIntegrations.stories.svelte catalog/components/store-forge/StoreForgeInlineWorkspace.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceBranches.stories.svelte catalog/components/store-stack-workspace/StoreStackTopology.stories.svelte --testNamePattern 'GitHubAccountSetupInGlobalSettings|ConnectedAccountAndForget|^CreateInlineAndFloat$|AppliedBranchesHaveTheirOwnGroup|StackDiscoveryHidesDependentEntriesAndKeepsPartialApplicationExplicit'
```

**Notes**

Cold Docker1e38b9a7:5 pass/14 unselected after keeping the complete visible Apply/Unapply text and exact stack context in natural names. No callback, capability, command, baseline or threshold change.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook -- catalog/components/store-forge catalog/components/store-stack-workspace catalog/components/store-search catalog/components/store-work/ProjectsFilters.stories.svelte catalog/components/store-threads/StoreThreadSearch.stories.svelte
```

**Notes**

Cold Dockere1e54be2:189 pass/4 fail across193 cases and47 families. Remaining cases are two inbox rows plus dynamic branch/remote-target selectors. This supersedes only the earlier193-case35-failure result, not the complete1049-case total.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook -- catalog/components/store-threads/StoreThreadSearch.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceSelection.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceTarget.stories.svelte --testNamePattern 'MobileSearchKeepsTheRepositoryGroupsAccessible|FilteredPaginationRemainsInTheInboxList|BranchesAndTargetOptionsToggleOffAndRetainTheirHistory|WorkspaceTargetSupportsKeyboardSelectionAboveTheFilters'
```

**Notes**

Cold Dockerf2ee7493: all four exact cases fail before the inbox/target edits; five unselected.
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
pnpm test:storybook -- catalog/components/store-threads/StoreThreadSearch.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceSelection.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceTarget.stories.svelte --testNamePattern 'MobileSearchKeepsTheRepositoryGroupsAccessible|FilteredPaginationRemainsInTheInboxList|BranchesAndTargetOptionsToggleOffAndRetainTheirHistory|WorkspaceTargetSupportsKeyboardSelectionAboveTheFilters'
```

**Notes**

Cold Docker526fe5e6:3 pass/1 fail/five unselected. Inbox names now retain visible labels and complete conversation identity/status in accessible descriptions; dynamic feature/api and origin/trunk selectors retain namespace boundaries. Mobile Search reaches a separate missing Search loaded conversations control.
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
pnpm test:storybook -- catalog/components/store-threads/StoreThreadSearch.stories.svelte --testNamePattern '^MobileSearchKeepsTheRepositoryGroupsAccessible$'
```

**Notes**

Cold Docker36ff9d24 reproduces the missing mobile search control alone; two unselected. It is not dismissed as a prior-story failure.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook -- catalog/components/store-threads/StoreThreadSearch.stories.svelte --testNamePattern '^MobileSearchKeepsTheRepositoryGroupsAccessible$'
```

**Notes**

Cold Docker7eecacf4: waiting for the actual resolved mobile display mode does not repair the missing control. Worker reports premature-close collection pending; the test failure is retained separately from collection.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook -- catalog/components/store-threads/StoreThreadSearch.stories.svelte --testNamePattern '^MobileSearchKeepsTheRepositoryGroupsAccessible$'
```

**Notes**

Cold Dockera541b317: a proposed selected-panel ownership change does not repair the control. Revert that unproven production change and its proposed canonical paragraph; keep the original inbox toggle behavior.
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
pnpm test:storybook -- catalog/components/store-threads/StoreThreadSearch.stories.svelte --testNamePattern '^MobileSearchKeepsTheRepositoryGroupsAccessible$'
```

**Notes**

Cold Docker9fdbec32: whole-document queries also fail to find the mobile control. Portal query scope alone is not a fix. Readiness and explicit pressed-state assertions pass, but mobile search remains blocking; continue source/DOM diagnosis without increasing timeouts or suppressing accessibility.
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
pnpm test:storybook -- catalog/components/store-threads/StoreThreadSearch.stories.svelte --testNamePattern '^MobileSearchKeepsTheRepositoryGroupsAccessible$'
```

**Notes**

Cold c6522e78: awaiting the owned complementary landmark and searchbox still fails. Diagnostic 7f6adb96 also fails1/1; its native click has isConnected=false and stops at detached ASIDE. The live replacement panel is a different element. StorePageSkeleton intentionally mounts CommunityInboxSkeleton/CommunityInbox while StoreLazyView loads; temporary event tracing is removed. A separate diagnostic run ddc9a6c9 passed3/3 with layout reads, which was not qualification.
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
pnpm test:storybook -- catalog/components/store-threads/StoreThreadSearch.stories.svelte --testNamePattern '^MobileSearchKeepsTheRepositoryGroupsAccessible$'
```

**Notes**

Cold55dfea7c: waiting for the fixture excerpt fails because rows with lastActivityAt deliberately show time instead of excerpt. Preserve the presentation; use the loaded planning title as the positive mount boundary.
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
pnpm test:storybook -- catalog/components/store-threads/StoreThreadSearch.stories.svelte --testNamePattern '^MobileSearchKeepsTheRepositoryGroupsAccessible$'
```

**Notes**

Cold b2877964:1 pass/2 unselected after waiting for the loaded planning title before retaining mobile controls. Explicit resolved mobile mode, pressed state, landmark visibility and actual filtering remain asserted. No delay, timeout, product workaround or accessibility waiver.
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
pnpm test:storybook -- catalog/components/store-threads/StoreThreadSearch.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceSelection.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceTarget.stories.svelte --testNamePattern 'MobileSearchKeepsTheRepositoryGroupsAccessible|FilteredPaginationRemainsInTheInboxList|BranchesAndTargetOptionsToggleOffAndRetainTheirHistory|WorkspaceTargetSupportsKeyboardSelectionAboveTheFilters'
```

**Notes**

All4 exact regressions pass/5 unselected in a new cold Docker/browser invocation. Broader affected-family acceptance is separate and still running.
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
pnpm test:storybook -- catalog/components/store-forge catalog/components/store-stack-workspace catalog/components/store-search catalog/components/store-work/ProjectsFilters.stories.svelte catalog/components/store-threads/StoreThreadSearch.stories.svelte catalog/components/store-application/AgentExecution.stories.svelte --testNamePattern '^(?!ExactPermissionChoices$|LocalProviderChoicesRemainVisibleWhileLoading$)'
```

**Notes**

Cold c519cbbc:205 pass/1 fail/1 unselected across207 cases. All193 cases in the original47 stack/forge/search/inbox files pass. Added AgentExecution has12 pass, CommunityAvatarAndHarnessPickers fails the agent-profile Activity log/ID labels, and ExactPermissionChoices is unselected by this invocation filter. That control is run explicitly in the next narrow target; complete AgentExecution acceptance remains required.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook -- catalog/components/store-application/AgentExecution.stories.svelte catalog/components/store-application/StoreProfileSettings.stories.svelte catalog/components/store-application/StoreCaptureSettings.stories.svelte catalog/components/store-application/AgentReviewDraft.stories.svelte catalog/components/store-work/ProjectsRepositoryActions.stories.svelte --testNamePattern 'CommunityAvatarAndHarnessPickers|ExactPermissionChoices|^EditProfileWithKeyboard$|^EditDisabledLocalCaptureConfiguration$|^ClassicReviewUsesAnExpandableCommentCard$|^ProjectAddMenuAndLinkDialog$'
```

**Notes**

Cold27e04962:1 pass/5 fail/42 unselected. ExactPermissionChoices passes. The five failures reproduce omitted visible content in agent Activity log/ID, avatar picker, member profile triggers, capture harness disclosures and repository directory buttons. Preserve actual behavior, images, full identifiers, actions and all accessibility rules.
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
pnpm test:storybook -- catalog/components/store-application/AgentExecution.stories.svelte catalog/components/store-application/StoreProfileSettings.stories.svelte catalog/components/store-application/StoreCaptureSettings.stories.svelte catalog/components/store-application/AgentReviewDraft.stories.svelte catalog/components/store-work/ProjectsRepositoryActions.stories.svelte --testNamePattern 'CommunityAvatarAndHarnessPickers|ExactPermissionChoices|^EditProfileWithKeyboard$|^EditDisabledLocalCaptureConfiguration$|^ClassicReviewUsesAnExpandableCommentCard$|^ProjectAddMenuAndLinkDialog$'
```

**Notes**

Cold7b4c4f58:6 pass/42 unselected after names derive from complete owned content and explicit screen-reader action prefixes. Selected-avatar image alt remains intact and is directly asserted in its draft-retention story; full Agent ID and live member-avatar presence remain content, callbacks unchanged. Type/ownership, expanded consumers and complete cold tooling qualification remain separate.
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
pnpm catalog:check
```

**Notes**

Cold Docker80eecd26:0 Svelte errors/0 warnings,489 catalog modules satisfy ownership boundaries and212 parity requirements pass after the profile/capture/repository repairs.
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
pnpm web:check
```

**Notes**

Cold Docker8e46bfea:0 Svelte errors/0 warnings; public client and conversation package builds pass.
<!-- /run -->
<!-- /evidence-field -->

