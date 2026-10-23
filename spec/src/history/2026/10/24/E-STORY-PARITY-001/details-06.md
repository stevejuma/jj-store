---
schema: 1
id: E-STORY-PARITY-001
---

# E-STORY-PARITY-001 details 6

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
node --test --test-name-pattern='bot PRs retain static spec and planned test lanes|real read-index and model impacts preserve the agreed standard cohort' scripts/automation-governance.test.mjs scripts/ci-selection.test.mjs
```

**Notes**

Both exact regressions passed. The governance check follows the routed lane while requiring every original test and package checker; model selection now asserts the complete exact reverse dependency inventory.
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
JJ_STORE_VALIDATION_RUNTIME=native pnpm test:release
```

**Notes**

Complete CI/release governance: 59/59 passed. This is a bounded source-policy suite, not release qualification.
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
JJ_STORE_VALIDATION_RUNTIME=native pnpm test:storybook catalog/components/store-application/DiscussionSheets.stories.svelte -t AttachedPaneRestoresItsWidthBesideAnIndependentOuterRail
```

**Notes**

The unchanged exact pane regression passed in isolation; one targeted case /two untargeted skipped.
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
JJ_STORE_VALIDATION_RUNTIME=native pnpm test:storybook catalog/components/store-application/DiscussionSheets.stories.svelte
```

**Notes**

Unchanged native family passed all three cases. The source failure was intermittent; the private play now resolves current controls and uses native pointer close/reopen, retains Main identity, joined-edge changes, rail geometry and width restoration.
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
JJ_STORE_VALIDATION_RUNTIME=native pnpm test:storybook catalog/components/store-application/DiscussionSheets.stories.svelte -t AttachedPaneRestoresItsWidthBesideAnIndependentOuterRail
```

**Notes**

After native pointer/live-control repair: exact case passes without changing assertions or deadlines.
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
JJ_STORE_VALIDATION_RUNTIME=native pnpm test:storybook catalog/components/store-application/DiscussionSheets.stories.svelte
```

**Notes**

After repair: all three family cases passed.
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
JJ_STORE_VALIDATION_RUNTIME=native pnpm test:storybook catalog/components/store-application/ApplicationMenu.stories.svelte catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.stories.svelte -t "BookmarkCards|MobileRepositorySettingsRetainTheToolbarAboveTheirBody"
```

**Notes**

Exact source check: mobile menu assertion failed /branch case passed. Old tests expected the mobile navigation landmark to be absent; they now assert its labelled target, contained body toolbar, no duplicate top toolbar and exactly one menu.
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
JJ_STORE_VALIDATION_RUNTIME=native pnpm test:storybook catalog/components/store-application/ApplicationMenu.stories.svelte catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.stories.svelte -t "BookmarkCards|MobileRepositorySettingsRetainTheToolbarAboveTheirBody"
```

**Notes**

Both exact cases pass. Branch style checks first move the actual pointer off the rows before recording resting color; keyboard cursor, hover/restoration and separator assertions remain.
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
JJ_STORE_VALIDATION_RUNTIME=native pnpm test:storybook catalog/components/store-application/ApplicationMenu.stories.svelte catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.stories.svelte
```

**Notes**

All eleven cases in both families passed; no assertions or thresholds weakened.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 7
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook
```

**Notes**

Final complete four-shard attempt executed all 216 files /1048 tests: 1041 passed /seven failed across four files. Shards: 275/275, 283/284, 233/238, 250/251. Findings: one pane close interaction; four obsolete mobile landmark assertions; one uncontrolled resting-pointer color assertion; one workflow same-attempt test that incorrectly closed retained output. Exact repairs and complete affected families follow; no omitted tests or browser disconnect.
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
JJ_STORE_VALIDATION_RUNTIME=native pnpm test:storybook catalog/components/store-workflows/WorkflowHistory.stories.svelte -t ASavedRunLinkRestoresPinnedDetailsAndAttemptSelection
```

**Notes**

Exact workflow case reproduced the hidden stdout failure. Waiting for closed output also failed because the fixture contains only attempt #1: selecting it explicitly preserves the already-open disclosure under the existing identity-keyed layout contract. The repaired test asserts retained expanded output and visible logs, rather than toggling it closed.
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
JJ_STORE_VALIDATION_RUNTIME=native pnpm test:storybook catalog/components/store-workflows/WorkflowHistory.stories.svelte -t ASavedRunLinkRestoresPinnedDetailsAndAttemptSelection
```

**Notes**

Exact retained-disclosure regression passed; eight unrelated cases skipped.
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
JJ_STORE_VALIDATION_RUNTIME=native pnpm test:storybook catalog/components/store-workflows/WorkflowHistory.stories.svelte
```

**Notes**

All nine workflow-history cases passed after correcting the same-attempt disclosure expectation.
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
pnpm test:storybook catalog/components/store-application/DiscussionSheets.stories.svelte catalog/components/store-workflows/WorkflowHistory.stories.svelte catalog/components/store-application/ApplicationMenu.stories.svelte catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.stories.svelte -t 'ASavedRunLinkRestoresPinnedDetailsAndAttemptSelection|AttachedPaneRestoresItsWidthBesideAnIndependentOuterRail|BookmarkCards|MobileRepositorySettingsRetainTheToolbarAboveTheirBody|UnknownReloadRetainsOriginalIntentWhenTheTriggerRelocates|MobileHidesTopNavigationAndLocatesItsOneMenuInTheBody|ReloadDialogAndFocusSurviveDesktopToMobileRelocation'
```

**Notes**

Docker 062f0da1-92e4-4f61-a38f-abb5a1680861: all seven exact regressions passed; sixteen unrelated cases skipped by the explicit target filter.
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
pnpm test:storybook catalog/components/store-application/DiscussionSheets.stories.svelte catalog/components/store-workflows/WorkflowHistory.stories.svelte catalog/components/store-application/ApplicationMenu.stories.svelte catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.stories.svelte
```

**Notes**

Docker 43714797-ad89-49a5-8768-7947d76df863: all 23 cases in four affected families passed. JUnit inventory audit matches these exact identities to the 1048-case full run 7cb2116a-58c4-42f9-8bc1-96a2c8af6ff2: 1048 unique covered cases and zero unresolved failures. This is a full run plus repaired-family retries, not one clean unfiltered command.
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
pnpm web:test
```

**Notes**

Docker 5b0fb5cc-f615-47c9-ab14-e13eb1acc11c: all 180 web unit tests passed.
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
node --test scripts/docs.test.mjs
```

**Notes**

All eight documentation tests passed on the final source-policy changes.
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
pnpm catalog:test:browser
```

**Notes**

Final Docker cfb10b74-1ce9-407a-8bed-177a54339bd7: complete static catalog build and cold catalog acceptance passed (1/1), covering desktop/mobile, Docs and the actual application.
<!-- /run -->
<!-- /evidence-field -->

