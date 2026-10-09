---
schema: 1
id: E-CHAT-LIVE-007
---

# E-CHAT-LIVE-007 final validation

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 18

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown
```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/ConversationMarkdown.stories.svelte
```

**Notes**

New late-styles regression fails before using embedded preview: 450px bottom padding instead of 0px; live screenshot showed 494px at its viewport height.
<!-- /run -->

<!-- run -->
### Run 19

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown
```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/ConversationMarkdown.stories.svelte
```

**Notes**

Exact test passes after configuring the public embed prop. Cold live app comment cards measure 24px with 0px bottom padding; final browser logs contain no errors or warnings, including ResizeObserver.
<!-- /run -->

<!-- run -->
### Run 20

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown
```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChat*.stories.svelte catalog/components/store-application/AgentExecution.stories.svelte catalog/components/store-application/Conversation*.stories.svelte catalog/components/store-application/AgentPromptMarkup.stories.svelte catalog/components/store-threads/*.stories.svelte catalog/components/store-appearance/*.stories.svelte
```

**Notes**

105 pass, 2 fail in 20 files. This wider family is required because embedded Markdown is shared by comments, ordinary messages and Sent context. Copy-message submenu lookup was transient and passed in isolation. The reconnect story reproduced a needless async yield during a no-pending refresh; guard receipt reconciliation by an actual pending request.
<!-- /run -->

<!-- run -->
### Run 21

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown
```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentPromptMarkup.stories.svelte catalog/components/store-application/AgentChatChanges.stories.svelte -t "OpenCodeTypedTextMessageParts|ReconnectWarningStaysInsideTheComposerAndClearsAfterRefresh"
```

**Notes**

Copy-menu interaction passes; reconnect regression remains failing before the pending guard. Assertions unchanged.
<!-- /run -->

<!-- run -->
### Run 22

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown
```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatChanges.stories.svelte -t ReconnectWarningStaysInsideTheComposerAndClearsAfterRefresh
```

**Notes**

Exact reconnect regression passes after the pending guard, preserving failed refresh and draft behavior.
<!-- /run -->

<!-- run -->
### Run 23

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown
```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/agent-chat.test.ts web/tests/vite/conversation-projection.test.ts
```

**Notes**

Final 19 tests pass after the pending guard.
<!-- /run -->

<!-- run -->
### Run 24

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown
```

```shell
pnpm web:check; pnpm catalog:check; pnpm check:ui-ownership
```

**Notes**

After embedded preview, web/catalog each report 0 errors and 0 warnings; catalog boundary has 345 modules. Ownership passes.
<!-- /run -->

<!-- run -->
### Run 25

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown
```

```shell
pnpm progress:history; pnpm check:progress; pnpm exec spec-validator validate --only summary,fileLimits,governance,verification,book; pnpm spec:first
```

**Notes**

Corrected initial evidence test references to file#name and bounded-page naming to details-NN. Removed unrelated generated Bootstrap whitespace. Final ledger reports 146 tasks/690 evidence records; 5 document validators and spec-first pass. Generated histories retained for M24/M25 only.
<!-- /run -->

<!-- run -->
### Run 26

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown
```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChat*.stories.svelte catalog/components/store-application/AgentExecution.stories.svelte catalog/components/store-application/Conversation*.stories.svelte catalog/components/store-application/AgentPromptMarkup.stories.svelte catalog/components/store-threads/*.stories.svelte catalog/components/store-appearance/*.stories.svelte
```

**Notes**

Final affected interaction/accessibility family: 107 tests pass in 20 files (141.15s). The original copy-menu and reconnect stories both pass without assertion changes. Log: .local/chat-live-stories-final2.log. This is the directly affected family, not the full repository suite.
<!-- /run -->

<!-- run -->
### Run 27

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown
```

```shell
pnpm check:progress
pnpm exec spec-validator validate --only summary,fileLimits,governance,verification,book
pnpm spec:first
pnpm web:check
```

**Notes**

After recording the final story run and regenerating histories: 146 tasks/690 evidence records; five document validators pass; spec-first covers 12 protected files and five chapters. The last web type check, including the pending guard, reports zero errors and warnings. Changed Markdown/TypeScript/Svelte files were formatted using the JJ diff list; historical evidence remains in its repository-excluded canonical format. Generated Bootstrap whitespace was restored again. Source contains no temporary chat-live-probe instrumentation.
<!-- /run -->

<!-- /evidence-field -->
