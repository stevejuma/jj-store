---
schema: 1
id: E-UI-080
---

# E-UI-080 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 7
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-application/AgentLayout.stories.svelte (initial before)
```

**Notes**

7failed/1passed. Two selected-project setup races: clicked Agents before route initialization. /tmp/jj-agent-layout-before.log.
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
pnpm test:storybook catalog/components/store-application/AgentLayout.stories.svelte (corrected before)
```

**Notes**

6failed/2passed: five Focus scenarios rendered sidebar; repository row gap0.03125px. Waited for Edit project before switching its tab and settled virtual row frames. /tmp/jj-agent-layout-before-corrected.log.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-application/AgentLayout.stories.svelte (first after)
```

**Notes**

6passed/2failed: mobile teleported sheet stayed after Close when conditionally destroyed while open. /tmp/jj-agent-layout-after.log.
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
pnpm test:storybook catalog/components/store-application/AgentLayout.stories.svelte (controlled open)
```

**Notes**

8/8. Persistent controlled-open sheet clears mobile reparented content. /tmp/jj-agent-layout-after-controlled.log.
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
pnpm test:storybook catalog/components/store-application/AgentLayout.stories.svelte (strengthened)
```

**Notes**

8passed/2failed: new Settings layout-switch assertions queried selected rows through the covering sheet accessible tree. Replaced only the new selection lookup with exact retained DOM row identity; behavioral assertions retained. /tmp/jj-agent-layout-final-regression.log.
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
pnpm test:storybook catalog/components/store-application/AgentLayout.stories.svelte (qualified)
```

**Notes**

10/10: spacing with neighboring selected/hovered rows; repository/project/hub desktop and mobile Focus, sheet bounds, Close/Escape/scrim, retained directory, focus restoration and selected session across Settings Focus/Split. /tmp/jj-agent-layout-qualified-regression.log.
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
pnpm web:check; pnpm catalog:check; pnpm check:ui-ownership; pnpm check:architecture; pnpm spec:first; pnpm spec:validate (focused checks)
```

**Notes**

Initial and final web/catalog0errors0warnings;142catalog modules. Other focused gates passed; final formatted ledger gates follow. Logs /tmp/jj-agent-layout-*.log.
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
Live browser read-only Focus deep link and row geometry
```

**Notes**

Cold selected Projects session restores sheet. Main left242px, sheet left354px, both bottom712px and right1272px;112px desktop allowance. Gaps4.015625px. Screenshot inspected /tmp/jj-agent-layout-focus.png. Later browser setting changed to Split while user continued interacting; cold reload honors sidebar. No live data writes.
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
Read-only discovery and live verification errors
```

**Notes**

Some guessed source paths/globs were absent, corrected using rg --files. Live Close locator no longer matched after page changed to Split; inspected fresh state. Restricted DOM evaluate did not expose localStorage; used UI/cold reload instead. No state mutation.
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
Design Core: final formatting/spec gates and jj commit
```

**Notes**

Completed prior InputGroup source repair qualification: final pnpm checks passed /tmp/dc-input-group-checks-qualified.log; final spec-first/check pass; committed zukowoxl/4922feea. Earlier readiness and visual-cache failures remain documented upstream. No publication.
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
pnpm test:storybook catalog/components/store-work catalog/components/store-application catalog/components/store-threads (initial consumer run)
```

**Notes**

192passed/1failed and3runtime errors: nested agent/tool Focus sheets collided. Inline tool discussion replaces nested layers. /tmp/jj-agent-layout-consumers.log.
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
pnpm test:storybook catalog/components/store-application/AgentToolGroups.stories.svelte -t ToolAndCommentHover (before)
```

**Notes**

1failed/4skipped: parent tool toolbar remained visible on comment hover. /tmp/jj-agent-tool-hover-before.log.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-application/AgentToolGroups.stories.svelte -t AgentCommentReplies (before/first after)
```

**Notes**

Before1failed/5skipped: depths0,1,2. First after geometry passed but Reply click raced toolbar opacity. Wait for actual visibility; geometry retained. Logs indent-before/after.
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
pnpm test:storybook catalog/components/store-application/AgentToolGroups.stories.svelte -t AgentCommentReplies (qualified/lifecycle diagnosis)
```

**Notes**

1interaction passed/5skipped but derived_inert warning remained. Reproduced warning-stack/warning-caught; stack2 run had no warning. Temporary guard stack capture proved shared composer submit read removed StoreReviewComposer binding. Instrumentation fully removed; no diagnostic suppression. /tmp/jj-agent-comment-runtime-stack.log.
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
pnpm test:storybook catalog/components/store-application/AgentToolGroups.stories.svelte -t AgentCommentReplies (lifecycle repaired)
```

**Notes**

1passed/5skipped, no derived_inert warning. /tmp/jj-agent-comment-lifecycle-qualified.log.
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
pnpm test:storybook catalog/components/store-application/AgentComments.stories.svelte (before)
```

**Notes**

Missing session footer;2failed. /tmp/jj-agent-session-footer-before.log.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-application/AgentToolGroups.stories.svelte catalog/components/store-application/AgentComments.stories.svelte (first after)
```

**Notes**

3failed/3passed: placeholder counted as editor text; footer made textbox lookup ambiguous. Scoped editor and asserted cleared submitted text/disabled submit. /tmp/jj-agent-comments-stories-after.log.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-application/AgentToolGroups.stories.svelte catalog/components/store-application/AgentComments.stories.svelte catalog/components/store-application/AgentLayout.stories.svelte (qualification attempt)
```

**Notes**

12passed/5failed: hover targeted old outer wrapper, and conditional composer wrapper destruction orphaned mobile sheet. Fixed true tool-row hover and always-mounted optional-controller wrapper. /tmp/jj-agent-comments-stories-qualified.log.
<!-- /run -->
<!-- /evidence-field -->

