---
schema: 1
id: E-UI-VISIBLE-NAMES-001
---

# E-UI-VISIBLE-NAMES-001 details 2

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
pnpm test:storybook -- catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte --testNamePattern BranchesToDiff
```

**Notes**

Fresh default Docker run ecfa824c: exact story fails label-content-name-mismatch on working-copy/branch cards and native ID controls; eight unselected cases. Full prior tooling audit retains351 pass/696 fail/two skips separately.
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
pnpm test:storybook -- catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte --testNamePattern BranchesToDiff
```

**Notes**

Fresh default Docker run9e5c9495: exact target passes, eight unselected cases. Derived card names retain visible metadata; native copy names include visible IDs while full identities remain described and copied. An initial aria-description implementation passed the story but introduced two Svelte warnings; final source uses standard aria-describedby with persistent full-identity text and reruns the exact target without those warnings.
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
pnpm test:storybook -- catalog/components/community-projects/CommunityProjectsRevisionGraphIds.stories.svelte
```

**Notes**

Fresh Docker80842918: all nine native-ID stories pass. Added exact accessible-name assertions for resting/expanded/copied states and full accessible descriptions; clipboard, keyboard, width and error assertions remain.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 11
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook -- catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte catalog/components/community-projects/CommunityProjectsRevisionGraphIds.stories.svelte catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.stories.svelte catalog/components/store-application/StoreRepositoryBranchContext.stories.svelte catalog/components/store-application/AgentChatChanges.stories.svelte catalog/components/store-mutations/StoreRepositoryActions.stories.svelte catalog/components/store-workflows/StoreWorkflows.stories.svelte
```

**Notes**

Fresh Docker0b29d1ae:27 pass/11 fail across seven files. Further shared profile/search/workflow-card names omit visible text; one working-copy selector still expects the old name.
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
pnpm test:storybook -- catalog/components/store-workflows/StoreWorkflows.stories.svelte catalog/components/store-application/StoreRepositoryBranchContext.stories.svelte --testNamePattern 'OverviewAndSequenceEditor|^Actions$'
```

**Notes**

Fresh Dockerfc242f21: both narrow cases fail; six unselected. Profile and search fixes reduce this to one Workflow overview failure in0691937b, exposing the workflow-card summary/name mismatch.
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
pnpm test:storybook -- catalog/components/store-workflows/StoreWorkflows.stories.svelte catalog/components/store-application/StoreRepositoryBranchContext.stories.svelte --testNamePattern 'OverviewAndSequenceEditor|^Actions$'
```

**Notes**

Fresh Docker0f7ba486: both exact cases pass; six unselected. Names retain visible content and exact workflow identity; expanded search retains its visible name and exposes both supported keyboard shortcuts.
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
pnpm test:storybook -- catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte catalog/components/community-projects/CommunityProjectsRevisionGraphIds.stories.svelte catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.stories.svelte catalog/components/store-application/StoreRepositoryBranchContext.stories.svelte catalog/components/store-application/AgentChatChanges.stories.svelte catalog/components/store-mutations/StoreRepositoryActions.stories.svelte catalog/components/store-workflows/StoreWorkflows.stories.svelte
```

**Notes**

Fresh Dockerf965fbe5: all38 cases pass across all seven directly affected families. No rule/threshold/baseline/timeout changes; wider complete tooling qualification remains pending.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 426
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook
```

**Notes**

Complete cold default Docker invocation:621 pass/426 fail/two existing skips across four complete shards. Per-shard pass/fail/skip:162/111/2,202/82/0,118/120/0,139/113/0. The original full audit had351 pass/696 fail/two skips. Further naming defects remain in stack/forge, inbox, repository, search and configuration controls; no complete-suite or tooling qualification is claimed.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook -- catalog/components/store-search/StoreSearchCommand.stories.svelte catalog/components/store-forge/StoreForgeWorkspace.stories.svelte --testNamePattern '^Results$|PullRequestDetailsBesideStacks|OpenAnotherPRAndPreviewStackSubmission'
```

**Notes**

Cold Docker352e89e5: all three exact cases fail before editing. Stack composer, PR buttons/disclosures, check-result summary and Esc close controls omit visible labels from their accessible names; six unselected cases.
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
pnpm test:storybook -- catalog/components/store-search/StoreSearchCommand.stories.svelte catalog/components/store-forge/StoreForgeWorkspace.stories.svelte --testNamePattern '^Results$|PullRequestDetailsBesideStacks|OpenAnotherPRAndPreviewStackSubmission'
```

**Notes**

Cold Docker523e3188: two pass/one fail/six unselected. Removing an already compliant stack submit label included the provider icon in its name, breaking the existing selector. Restored that unchanged compliant stack label rather than weakening the selector.
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
pnpm test:storybook -- catalog/components/store-search/StoreSearchCommand.stories.svelte catalog/components/store-forge/StoreForgeWorkspace.stories.svelte --testNamePattern '^Results$|PullRequestDetailsBesideStacks|OpenAnotherPRAndPreviewStackSubmission'
```

**Notes**

Cold Docker257d4f42: all three exact cases pass; six unselected. Natural names retain visible action/status/title/Esc text plus exact stack/PR context. Region/form identities, callbacks, capabilities and pending-state controls are unchanged; no accessibility waiver or baseline change.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 35
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook -- catalog/components/store-forge catalog/components/store-stack-workspace catalog/components/store-search catalog/components/store-work/ProjectsFilters.stories.svelte catalog/components/store-threads/StoreThreadSearch.stories.svelte
```

**Notes**

Cold Docker09f3174b:158 pass/35 fail across193 cases and47 files. Remaining failures include branch-option assertions left on old label-only names, Apply/Unapply and Publication status/account/inbox visible-name mismatches, and three incorrectly changed toolbar-search selectors. The toolbar is a separate icon-only component; restore its exact old Close search selectors without altering production behavior. No passing complete-family qualification is claimed.
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
pnpm catalog:check
```

**Notes**

Default Dockerd3b0c506: Svelte0 errors/0 warnings,489 owned catalog modules pass boundaries, Storybook specification parity passes212 requirements.
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
pnpm test:storybook -- catalog/components/store-work/ProjectsFilters.stories.svelte catalog/components/store-threads/StoreThreadSearch.stories.svelte --testNamePattern 'HeaderFiltersWithoutTheDetailsRail|MobileHeaderFilters|SearchLoadedThreadTitlesAndPreviewsWithoutRequests|MobileSearchKeepsTheRepositoryGroupsAccessible'
```

**Notes**

Cold Dockera7e97113 after restoring distinct icon-only toolbar selectors:3 pass/1 fail/1 unselected. All three selector regressions are repaired. The remaining mobile case fails on the already observed Open Thread planning visible-name mismatch in the separate owned inbox row; keep it blocking.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 19
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

Initial fresh evidence check identifies19 changed story/browser paths omitted from this record file list. Add those exact affected paths; retain failed interaction/source gates. No evidence checker or required gate is changed.
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
pnpm check:progress
```

**Notes**

Fresh evidence references CopyCompleteIdentity as a code-declared test, but its owner is CSF &lt;Story name="Copy complete identity"&gt;. Correct the evidence representation: keep actual story files and exact executed commands, remove invalid code-test references only; do not change source stories, checker or assertions.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook -- catalog/components/store-forge/StoreIntegrations.stories.svelte catalog/components/store-forge/StoreForgeInlineWorkspace.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceBranches.stories.svelte catalog/components/store-stack-workspace/StoreStackTopology.stories.svelte --testNamePattern 'GitHubAccountSetupInGlobalSettings|ConnectedAccountAndForget|^CreateInlineAndFloat$|AppliedBranchesHaveTheirOwnGroup|StackDiscoveryHidesDependentEntriesAndKeepsPartialApplicationExplicit'
```

**Notes**

Cold Docker8b62c46b: all five fail,14 unselected. Account/publication labels omit visible content; branch tests still assume removed label-only option names.
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
pnpm test:storybook -- catalog/components/store-forge/StoreIntegrations.stories.svelte catalog/components/store-forge/StoreForgeInlineWorkspace.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceBranches.stories.svelte catalog/components/store-stack-workspace/StoreStackTopology.stories.svelte --testNamePattern 'GitHubAccountSetupInGlobalSettings|ConnectedAccountAndForget|^CreateInlineAndFloat$|AppliedBranchesHaveTheirOwnGroup|StackDiscoveryHidesDependentEntriesAndKeepsPartialApplicationExplicit'
```

**Notes**

After account/status names and semantic branch selectors:4 pass/1 fail/14 unselected. The remaining exact StackDiscovery case reaches the existing Apply cloud-observability visible-label mismatch.
<!-- /run -->
<!-- /evidence-field -->

