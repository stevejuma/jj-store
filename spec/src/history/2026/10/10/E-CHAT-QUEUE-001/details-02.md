---
schema: 1
id: E-CHAT-QUEUE-001
---

# E-CHAT-QUEUE-001 details 2

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
pnpm install --offline --frozen-lockfile
```

**Notes**

New workspace: inherited pnpmfile checksum mismatch. No tracked package policy changed.
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
pnpm install --offline --frozen-lockfile --ignore-pnpmfile
```

**Notes**

Installed registry dependencies for diagnosis; the released Design Core version did not contain APIs already used by the parent.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 65
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check
```

**Notes**

Registry Design Core 0.2.0 exposed 65 inherited missing-API errors. This was not a queued-message regression.
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
pnpm install --offline --no-frozen-lockfile
```

**Notes**

Used the existing local pnpmfile hook and prepared ../design-core/.local/packages/lapismd-design-core-0.1.2-built.tgz. Restored pnpm-lock.yaml with Jujutsu; no tracked dependency or upstream source changes.
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
pnpm spec:search -- "JST-UI-018"
```

**Notes**

Installed search CLI parsed the extra -- as query input; canonical source was read directly.
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
pnpm spec:search "JST-UI-018"
```

**Notes**

Owning canonical UI chapter selected and read.
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
pnpm exec tsx --test web/tests/agent-execution.test.ts
```

**Notes**

7 exact execution presentation tests; context_delivery queue acknowledgement regression passes. /tmp/jj-queue-node-verified.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/agent-chat.test.ts web/tests/vite/agent-execution-controller.test.ts web/tests/vite/queued-agent-messages.test.ts web/tests/vite/conversation-session.test.ts web/tests/vite/conversation-projection.test.ts catalog/fixtures/delta-chat/comments.test.ts
```

**Notes**

Initial full focused selection passed 48, then 52; final delivery/cancellation selection passes 54 tests in 6 files. /tmp/jj-queue-unit-verified.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/queued-agent-messages.test.ts
```

**Notes**

8 queue-registry cases at that stage; later exact-start/late-context regressions included in the 54-test selection.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/conversation-session.test.ts catalog/fixtures/delta-chat/comments.test.ts
```

**Notes**

15 direct comment/submission tests after captured author was added. /tmp/jj-queue-comments-complete.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/QueuedAgentMessages.stories.svelte
```

**Notes**

Initial 6 new cold Chromium interaction/axe cases passed; 8 cases with stable preview states also passed before styling refinements.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/QueuedAgentMessages.stories.svelte catalog/components/store-application/AgentSessionState.stories.svelte catalog/components/store-application/AgentDiscussion.stories.svelte catalog/components/store-application/AgentReplyIndependence.stories.svelte catalog/components/store-application/AgentFleet.stories.svelte catalog/components/store-application/ConversationContinuity.stories.svelte catalog/components/store-application/ConversationFooterLayout.stories.svelte catalog/components/store-application/AgentChatFeedback.stories.svelte
```

**Notes**

First direct-family run: 33 passed, 2 older stories expected unconditional Stop. Updated assertions to the intended active-content Queue selection.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/QueuedAgentMessages.stories.svelte catalog/components/store-application/AgentSessionState.stories.svelte catalog/components/store-application/AgentDiscussion.stories.svelte catalog/components/store-application/AgentReplyIndependence.stories.svelte catalog/components/store-application/AgentFleet.stories.svelte catalog/components/store-application/ConversationContinuity.stories.svelte catalog/components/store-application/ConversationFooterLayout.stories.svelte catalog/components/store-application/AgentChatFeedback.stories.svelte
```

**Notes**

Follow-up: 34 passed, 1 draft-clear interaction lost editor focus. Refocused the editor before clearing; retained exact Stop target assertions.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/QueuedAgentMessages.stories.svelte catalog/components/store-application/AgentSessionState.stories.svelte catalog/components/store-application/AgentDiscussion.stories.svelte catalog/components/store-application/AgentReplyIndependence.stories.svelte catalog/components/store-application/AgentFleet.stories.svelte catalog/components/store-application/ConversationContinuity.stories.svelte catalog/components/store-application/ConversationFooterLayout.stories.svelte catalog/components/store-application/AgentChatFeedback.stories.svelte
```

**Notes**

Cold direct-family verification: 35 passed in 8 story files. /tmp/jj-queue-stories-current.log. No full Storybook suite claim.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/QueuedAgentMessages.stories.svelte
```

**Notes**

Disclosure preview assertions expected removal/immediate hiding; the shared Collapsible retains content during its close animation. Changed to wait for actual invisibility and keyboard re-expansion; behavior and a11y gates retained.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/QueuedAgentMessages.stories.svelte
```

**Notes**

Avatar iteration: 2 synthetic-hover assertions did not create CSS :hover and 3 axe cases found faded initials contrast 3.45:1. Reused the existing hover CSS, verify keyboard focus in stories and native hover in preview; retained full-opacity avatar and faded only text.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/QueuedAgentMessages.stories.svelte
```

**Notes**

White-card refactor had a Svelte const-tag placement compile failure; moved the local author declaration directly under the each block. No executed tests in that failed import run.
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

Earlier complete implementation check: 0 errors/0 warnings with the existing prepared Design Core package. Final card check is pending.
<!-- /run -->
<!-- /evidence-field -->

