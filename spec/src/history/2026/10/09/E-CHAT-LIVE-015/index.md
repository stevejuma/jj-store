---
schema: 1
id: E-CHAT-LIVE-015
---

# E-CHAT-LIVE-015

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
pnpm test:storybook catalog/components/store-application/AgentSessions.stories.svelte catalog/components/store-application/AgentChatBody.stories.svelte catalog/components/store-application/AgentContextDelivery.stories.svelte -t 'Inbox body waits|Empty captured history|Context button has'
```

**Notes**

The display-name filter selects no generated story names: 36 skipped. Not acceptance.
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
pnpm exec vitest list --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentSessions.stories.svelte catalog/components/store-application/AgentChatBody.stories.svelte catalog/components/store-application/AgentContextDelivery.stories.svelte
```

**Notes**

Lists generated CamelCase story identifiers; used to correct the exact selection.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-application/AgentSessions.stories.svelte catalog/components/store-application/AgentChatBody.stories.svelte catalog/components/store-application/AgentContextDelivery.stories.svelte -t 'InboxBodyWaits|EmptyCapturedHistory|ContextButtonHas'
```

**Notes**

Before implementation: 3 fail, 33 skipped (.local/chat-loading-before.log).
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
pnpm test:storybook catalog/components/store-application/AgentSessions.stories.svelte catalog/components/store-application/AgentChatBody.stories.svelte catalog/components/store-application/AgentContextDelivery.stories.svelte -t 'InboxBodyWaits|EmptyCapturedHistory|ContextButtonHas'
```

**Notes**

First implementation: 2 pass, 1 fail, 33 skipped (.local/chat-loading-after.log). Empty icon assertion used empty-media as data-slot; the released component exposes it as data-ui-part. Corrected the selector without weakening the icon requirement.
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
pnpm test:storybook catalog/components/store-application/AgentChatBody.stories.svelte catalog/components/store-application/AgentContextDelivery.stories.svelte catalog/components/store-application/AgentSessions.stories.svelte -t 'InboxBodyWaits|EmptyCapturedHistory|ContextButtonHas|UnknownDelivery|LoadingHistory|ViewSessionsKeeps|ActivityLogKeeps|LegacyInboxSessions|DraftEmptyState|WideRenderedContext|OneInspector|NoContext'
```

**Notes**

Expanded family initially has 11 pass, 2 fail, 26 skipped (.local/chat-loading-family.log). New retained-draft tests confused the Next message draft group label with the editor name Next message. Corrected the test target.
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
pnpm test:storybook catalog/components/store-application/AgentChatBody.stories.svelte catalog/components/store-application/AgentContextDelivery.stories.svelte catalog/components/store-application/AgentSessions.stories.svelte -t 'InboxBodyWaits|EmptyCapturedHistory|ContextButtonHas|UnknownDelivery|LoadingHistory|ViewSessionsKeeps|ActivityLogKeeps|LegacyInboxSessions|DraftEmptyState|WideRenderedContext|OneInspector|NoContext'
```

**Notes**

13 pass, 26 skipped (.local/chat-loading-family-final.log). Includes all 5 Sent-context cases, completed empty body, retained Document/Classic editors and drafts, live-history loading, retained session navigation and draft host empty setup. Interaction and configured accessibility checks pass in fresh Vitest/Vite browser processes.
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
pnpm test:storybook catalog/components/store-application/AgentChatBody.stories.svelte catalog/components/store-application/AgentContextDelivery.stories.svelte catalog/components/store-application/AgentSessions.stories.svelte -t 'InboxBodyWaits|EmptyCapturedHistory|ContextButtonHas|UnknownDelivery|LoadingHistory|ViewSessionsKeeps|ActivityLogKeeps|LegacyInboxSessions|DraftEmptyState|WideRenderedContext|OneInspector|NoContext'
```

**Notes**

Final strengthened no-message-skeleton selector rerun: 13 pass, 26 skipped (.local/chat-loading-family-verified.log).
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
pnpm web:check
```

**Notes**

Run initially and after final fixture changes; zero errors/warnings (.local/chat-loading-web-check.log and .local/chat-loading-web-final.log).
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
pnpm catalog:check
```

**Notes**

Run initially and after final fixture changes; zero errors/warnings and 349 catalog modules pass boundaries (.local/chat-loading-catalog-check.log and .local/chat-loading-catalog-final.log).
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
pnpm check:ui-ownership
```

**Notes**

One ownership validator passes (.local/chat-loading-ownership.log).
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
pnpm evidence:new -- --id E-CHAT-LIVE-015 --task M24 --requirement JST-UI-015 --requirement JST-UI-012 --requirement JST-API-001
```

**Notes**

Creates this fresh record; populated with actual outcomes using writeEvidence.
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
pnpm progress:history
```

**Notes**

Refreshes M24 history; unrelated M25/Bootstrap generation noise restored with Jujutsu.
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
pnpm spec:first
```

**Notes**

Initial evidence incorrectly used pending as a run result for the skipped-filter command. Run schema permits pass/fail only. Corrected to command pass with explicit no-test-selection note; no acceptance inferred from it.
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
pnpm check:progress
```

**Notes**

Same evidence run-schema error as spec-first; corrected without changing any milestone gate.
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
pnpm spec:validate --only governance,summary,fileLimits,book
```

**Notes**

Four validators pass; 3342 chapters and 181 requirements.
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
pnpm exec prettier --check web/src/lib/ui/store-agent-chat-body/StoreAgentChatBody.svelte web/src/lib/ui/store-agent-chat-body/model.ts web/src/lib/ui/store-agent-chat-body/chat-body.css web/src/lib/ui/store-agent-chat/StoreLiveConversation.svelte web/src/lib/ui/store-agent-chat/StoreAgentChatSidebar.svelte web/src/lib/ui/store-agent-execution/StoreAgentConversation.svelte web/src/lib/ui/store-agent-execution/StoreAgentSentContext.svelte catalog/components/store-application/AgentChatBody.stories.svelte catalog/components/store-application/AgentSessions.stories.svelte catalog/components/store-application/AgentContextDelivery.stories.svelte catalog/fixtures/ChatBodyFixture.svelte catalog/fixtures/AgentContextDeliveryFixture.svelte spec/src/current/contracts/ui/34-interactive-agent-sessions.md spec/src/current/contracts/ui/35-compute-defaults-and-sent-context.md spec/src/current/contracts/ui/37-document-conversations.md spec/src/current/contracts/api/41-interactive-agent-execution.md spec/src/current/progress/M24/index.md
```

**Notes**

All changed authored source, fixture/story and canonical Markdown paths pass; generated histories remain in generator format.
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
pnpm spec:first
```

**Notes**

12 protected files and 4 canonical chapters pass after correcting evidence run metadata.
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
pnpm check:progress
```

**Notes**

146 tasks and 698 evidence records pass; M24 stays In progress.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-CHAT-LIVE-015 details 2](details-02.md)
