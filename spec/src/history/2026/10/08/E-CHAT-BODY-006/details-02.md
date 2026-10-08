---
schema: 1
id: E-CHAT-BODY-006
---

# E-CHAT-BODY-006 details 2

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
jj --no-pager st
```

**Notes**

Clean tkxpllop before edits; preserve verified prototype parents.
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
pnpm spec:search -- "JST-UI-015"
```

**Notes**

The documented pnpm separator is forwarded to this installed CLI, which rejects unknown argument --. Read-only search; no source changes.
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
pnpm spec:search "JST-UI-015"
```

**Notes**

Retry without separator locates ui/03-requirements.md and ui/36-commentable-chat-body-prototype.md; both canonical sources read with API boundary and M24 ledger.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatBody.stories.svelte -t InlineNextMessageAndFixedSendActions
```

**Notes**

Before: expected public viewport missing. 1 failed, 13 skipped; chat-scroll-before.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatBody.stories.svelte -t InlineNextMessageAndFixedSendActions
```

**Notes**

After integration and final caret/font refinement: 1 passed, 13 skipped. Actual scroll movement, no outer inset, bounded frame, retained multiline draft, fixed footer, focus and local submission pass; chat-scroll-after.log and chat-scroll-final-target.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatBody.stories.svelte
```

**Notes**

14/14 affected interaction/accessibility stories before and after the caret precedence correction; final chat-scroll-stories-final.log. Covers immutable input/comments, focus, collapse/raw switching, folds, XML/reactions, navigation, dark/narrow and fixed local draft actions.
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
node catalog/fixtures/delta-chat/browser.acceptance.mjs
```

**Notes**

Initial new wheel assertion sampled Jump to End at scrollTop 2 during its existing smooth animation, then observed the settled 39. Test now waits for end navigation to settle and wheels over the viewport inset without hover-induced editor scrolling; preserves the actual movement assertion. Existing native scenarios had passed before this new assertion; chat-scroll-cold.log.
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
node catalog/fixtures/delta-chat/browser.acceptance.mjs
```

**Notes**

Fresh isolated Storybook/Chromium run after settling the native wheel scenario: zero page errors; actual scrolling, no page overflow, fixed toolbar/footer, retained multiline draft, local submit/refocus, append retention, exact anchors, mobile reads, remount, navigation, reactions and folds. Intermediate caret diagnostics caught ordinary foreground precedence, then final cold run verified the requested colors/shape and exact center alignment. Logs chat-scroll-cold-after.log and chat-scroll-cold-final.log; isolated processes cleaned up, running user preview preserved.
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
pnpm web:check
```

**Notes**

Initial and final changed-source checks: 0 errors, 0 warnings.
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
pnpm catalog:check
```

**Notes**

Initial and final changed-source checks: 0 errors, 0 warnings; 322 catalog modules pass boundaries.
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
pnpm check:ui-ownership
```

**Notes**

Owned Community ScrollArea usages inspected before the public Design Core ScrollArea implementation/docs. No Community source copy or production catalog import; no dependency/provenance change.
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
pnpm check:architecture
```

**Notes**

Only inherited scripts/dev-ui.acceptance.mjs:311 counted lines exceeds 300. Changed-source budgets pass; the unrelated file is byte-identical to d8956a14 and left unchanged.
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
jj --no-pager file show -r @- scripts/dev-ui.acceptance.mjs > .local/chat-scroll-parent-acceptance.mjs; cmp scripts/dev-ui.acceptance.mjs .local/chat-scroll-parent-acceptance.mjs
```

**Notes**

Read-only inherited architecture-gap confirmation; cmp returns zero.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

Initial evidence incorrectly listed JavaScript async helper names as named tests. The ledger accepts fn/def/test declarations; removed those unsupported references. Exact Storybook and native helper runs remain recorded in Validation; no tests or assertions changed.
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
pnpm spec:first
```

**Notes**

Initial gate blocked by those same two unsupported JavaScript helper references; evidence corrected.
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
pnpm exec spec-validator validate --only summary,fileLimits,governance,verification,book
```

**Notes**

Initial changed document check: 5 validators, 3250 chapters, 181 requirements.
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
pnpm exec prettier --check web/src/lib/ui/store-agent-chat-body/StoreAgentChatBody.svelte web/src/lib/ui/store-agent-chat-body/chat-body.css web/src/lib/ui/store-agent-chat-body/chat-caret.ts web/src/lib/ui/store-agent-chat-body/text-editor.ts web/src/lib/ui/store-agent-chat-body/text-reader.css web/src/lib/ui/store-agent-chat-body/ChatNextMessage.svelte catalog/components/store-application/AgentChatBody.stories.svelte catalog/fixtures/delta-chat/next-message-play.ts catalog/fixtures/delta-chat/browser.acceptance.mjs catalog/fixtures/delta-chat/next-message-native.mjs spec/src/current/contracts/ui/36-commentable-chat-body-prototype.md spec/src/current/contracts/api/41-interactive-agent-execution.md spec/src/current/progress/M24/index.md
```

**Notes**

All 13 changed authored files match Prettier; generated evidence/navigation keeps its governed writer formatting.
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

Corrected evidence references: 146 tasks, 673 evidence records. Storybook/native acceptance commands are recorded in runs, which support this harness form.
<!-- /run -->
<!-- /evidence-field -->

