---
schema: 1
id: E-UI-VISIBLE-NAMES-001
---

# E-UI-VISIBLE-NAMES-001 details 6

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 11
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook
```

**Notes**

Complete cold11bcd060:all1049 cases execute in four disjoint invocations:271pass/2fail/2skip,284pass,235pass/3fail and246pass/6fail. Current complete total is1036 pass/11 fail/two existing skips, replacing the earlier956pass/91fail baseline. Four failures match released Core reaction/contrast defects; six are stale review/step/branch selectors; one message-editor pointer readiness failure remains unexplained and blocking. No partial-family counts are substituted for this complete result. /tmp/jj-store-visible-names-full-storybook-20261117c.log.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 6
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook -- catalog/components/store-work/DisclosurePersistence.stories.svelte catalog/components/store-work/ProjectsReviewDetail.stories.svelte catalog/components/store-workflows/WorkflowHistory.stories.svelte catalog/components/store-application/RepositoryApplication.stories.svelte --testNamePattern '^ReviewSectionChoicesSurviveRemount$|^DirectoryReviewOpensAFullPage$|^SelectedProjectReviewOpensAFullPage$|^PendingRunHasHeaderTooltipsAndCollapsedEmptySections$|^RepositoryWorkspace$|^MobileWorkspace$'
```

**Notes**

Cold2c8c8989:6 fail/15 unselected at the obsolete review, pending-step and branch names. Preserve all page/remount, geometry, status and selection assertions. /tmp/jj-store-final-six-selectors-before.log.
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
pnpm test:storybook -- catalog/components/store-work/DisclosurePersistence.stories.svelte catalog/components/store-work/ProjectsReviewDetail.stories.svelte catalog/components/store-workflows/WorkflowHistory.stories.svelte catalog/components/store-application/RepositoryApplication.stories.svelte --testNamePattern '^ReviewSectionChoicesSurviveRemount$|^DirectoryReviewOpensAFullPage$|^SelectedProjectReviewOpensAFullPage$|^PendingRunHasHeaderTooltipsAndCollapsedEmptySections$|^RepositoryWorkspace$|^MobileWorkspace$'
```

**Notes**

Exact repaired rerun:6 pass/15 unselected. The previously qualified exact native-ID/visible-excerpt review opener is extracted without changing its assertions and shared by three fixture families. Pending-step and branch queries retain their complete status/name prefix. No product contract, readiness bound or assertion is relaxed. /tmp/jj-store-final-six-selectors-after.log.
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
pnpm test:storybook -- catalog/components/store-work/DisclosurePersistence.stories.svelte catalog/components/store-work/ProjectsReviewDetail.stories.svelte catalog/components/store-workflows/WorkflowHistory.stories.svelte catalog/components/store-application/RepositoryApplication.stories.svelte catalog/components/store-application/NativeReviewDetail.stories.svelte
```

**Notes**

Cold05a941b7: all24 cases in all five directly affected families pass, including all three users of the extracted exact native-ID review opener. /tmp/jj-store-final-selector-families.log.
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
pnpm catalog:check
```

**Notes**

Cold b8e173ff: zero errors/warnings; ownership490 modules and Storybook parity212 requirements pass after graph-message and shared-review-fixture changes. /tmp/jj-store-message-and-selector-catalog-check.log.
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
pnpm web:check
```

**Notes**

Cold e028ea5a: Svelte/TypeScript checks pass with zero errors and warnings. /tmp/jj-store-message-and-selector-web-check.log.
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
pnpm check:progress
```

**Notes**

Workspace lease preflight rejected concurrent validation while the first editor-family reproduction was active; no product test executed. /tmp/jj-store-selector-evidence-progress-20261117.log. Rerun serially after the authoritative handle is terminal.
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
pnpm test:storybook -- catalog/components/store-threads/StoreThreads.stories.svelte
```

**Notes**

Cold44b9ef37:6 pass/2 fail. Reaction accessibility still fails in Core0.13.4; the later editor also fails pointer readiness. /tmp/jj-store-message-editor-family-before-20261117.log.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook -- catalog/components/store-threads/StoreThreads.stories.svelte
```

**Notes**

Cold diagnostic family reproduces6 pass/2 fail. Temporary diagnostics show the same connected editable CodeMirror element; body pointerEvents is none and the single Edit message menu remains data-state closed/data-ending-style. /tmp/jj-store-message-editor-family-diagnostic-20261117.log. No product/timeout/assertion change.
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
pnpm test:storybook -- catalog/components/store-threads/StoreThreads.stories.svelte --testNamePattern '^MessageEditIsAHoverMenuActionAndUpdatesTheExactMessage$'
```

**Notes**

Exact cold editor isolation passes1 case/7 unselected with original readiness, editing, send and exact-message assertions. /tmp/jj-store-message-editor-exact-diagnostic-20261117.log.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook -- catalog/components/store-threads/StoreThreads.stories.svelte
```

**Notes**

Cold954f1581:6 pass/2 fail; closed menu retains its exit-animation presence and modal pointer lock. /tmp/jj-store-message-editor-menu-diagnostic-20261117.log.
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
pnpm test:storybook -- catalog/components/store-threads/StoreThreads.stories.svelte
```

**Notes**

Cold animation diagnostic:6 pass/2 fail. Closed Edit message menu animation is 0.1s reverse paused exit at currentTime0; body pointerEvents none. Installed Storybook10.6.7 preview runtime applyAfterEach runs after pauseAnimations and before cleanup without finally; an earlier reaction axe error skips cleanup. This supports a cascading test-host freeze, not a stale editor DOM. All temporary diagnostics removed. Original family must pass unchanged after actual Core0.13.5 adoption. /tmp/jj-store-message-editor-animation-diagnostic-20261117.log.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook -- catalog/components/store-threads/StoreThreads.stories.svelte catalog/components/store-threads/StoreCapturedReactions.stories.svelte catalog/components/store-application/AgentDiscussion.stories.svelte catalog/components/store-application/QueuedAgentMessages.stories.svelte --testNamePattern '^SharedComposerRepliesAndExactReactions$|^CapturedAgentMessages$|^ImmutableMessageReactionsUseTheSharedToolbarAndReactionBar$|^RefusalRetainsAnEditableDraft$|^MessageEditIsAHoverMenuActionAndUpdatesTheExactMessage$'
```

**Notes**

Correct four-file original reproduction against released Core0.13.4: all5 fail/23 unselected. Earlier mistaken AgentDiscussion path selected only4; /tmp/jj-store-core0135-exact-before.log retains4fail/18unselected, not a complete five-case proof. Correct baseline: /tmp/jj-store-core0135-five-before.log.
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
pnpm update @lapismd/design-core
```

**Notes**

Ordinary registry manifest^0.13.5 and lock adoption; no aliases or local source overrides. Published0.13.5 sourceec0d539a is independently signature/provenance verified, with retained release tarball SHA2565ca50dc137da020e381bf2742024c5b39125ac2b077cab36de7ffa9de0740ba7. Existing Bits patch unchanged. /tmp/jj-store-core0135-registry-update.log.
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
pnpm test:storybook -- catalog/components/store-threads/StoreThreads.stories.svelte catalog/components/store-threads/StoreCapturedReactions.stories.svelte catalog/components/store-application/AgentDiscussion.stories.svelte catalog/components/store-application/QueuedAgentMessages.stories.svelte --testNamePattern '^SharedComposerRepliesAndExactReactions$|^CapturedAgentMessages$|^ImmutableMessageReactionsUseTheSharedToolbarAndReactionBar$|^RefusalRetainsAnEditableDraft$|^MessageEditIsAHoverMenuActionAndUpdatesTheExactMessage$'
```

**Notes**

All5 original reaction/contrast/editor cases pass/23 unselected in fresh Docker workerbc876afc on actual released Core0.13.5. Original assertions, accessibility rules and bounds retained. /tmp/jj-store-core0135-five-after.log.
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
pnpm test:storybook -- catalog/components/store-threads/StoreThreads.stories.svelte catalog/components/store-threads/StoreCapturedReactions.stories.svelte catalog/components/store-application/AgentDiscussion.stories.svelte catalog/components/store-application/QueuedAgentMessages.stories.svelte
```

**Notes**

All28 cases/four directly affected families pass in a separate fresh workera7058327; the unchanged editor now passes in its complete family after upstream axe fixes. Complete1049-case acceptance remains separate. /tmp/jj-store-core0135-families-after.log.
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
pnpm test:storybook
```

**Notes**

Complete1049-case cold run against releasedCore0.13.5:1046pass/1fail/two existing skips. Four disjoint shards273/0/2,284/0/0,238/0/0,251/1/0. Sole remaining AgentChatBody MessageReactionsAndXMLContext expects the pre-release reaction accessible name. Original5 source failures and editor pass. /tmp/jj-store-core0135-full-storybook.log.
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
pnpm test:storybook -- catalog/components/store-application/AgentChatBody.stories.svelte --testNamePattern '^MessageReactionsAndXMLContext$'
```

**Notes**

Exact remaining selector regression reproduced in fresh worker0ad4ef9e. Core13.5 retains supplied actor label and adds visible emoji/count for label-content-name matching; helper still expects the shorter former label. Preserve pressed-state, hide/show persistence, untoggle removal and XML-context assertions. /tmp/jj-store-core0135-chat-reaction-exact-before.log.
<!-- /run -->
<!-- /evidence-field -->

