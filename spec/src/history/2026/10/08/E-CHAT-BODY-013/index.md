---
schema: 1
id: E-CHAT-BODY-013
---

# E-CHAT-BODY-013

<!-- evidence-field:id markdown -->
## Id

E-CHAT-BODY-013
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M24
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-001
- JST-UI-015
- JST-API-001
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-08
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Transcript reply context and double-check navigation
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change ynttlpwq above pqmyoyot in jj-store-chat-body
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Agent answers also appeared under their original source. Expanded historical batches reduced the answer context to a count/link. Delivered double ticks were status images with a separate hidden View reply button. The updated DeltaMainThreadReference regression initially failed because Go to reply was absent.
<!-- /evidence-field -->

<!-- evidence-field:findings markdown -->
## Findings

Original source discussions retain user comments, while linked agent answers render only in the main transcript. Each answer retains its shaded source quote and exact user comments regardless of historical-batch disclosure. Existing stable thread/comment IDs provide exact destinations; captured replies and tools keep transcript ordering within a shared turn. The catalog simulator groups same-anchor comments within each submitted batch into one answer with multiple comment IDs; separate anchors receive distinct answers in the same turn, and later submissions retain earlier history. Linked green double ticks are named Go to reply buttons. Hover and keyboard focus replace ticks with a Lucide navigation arrow and display a tooltip; click, Enter and touch focus the exact reply. Sent comments without replies remain noninteractive status icons. Community ActivityTurnList and the released Design Core Tooltip composition were inspected; the owned delivery component composes public Button/Tooltip APIs and supplies its tooltip role. CodeMirror widget height estimates exclude transcript-only reply records, preserving measured-widget cleanup and stable editor identity.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/fixtures/delta-chat/comment-lifecycle-play.ts
- catalog/fixtures/delta-chat/comment-queue-native.mjs
- catalog/fixtures/delta-chat/comments.test.ts
- catalog/fixtures/delta-chat/discussions.test.ts
- catalog/fixtures/delta-chat/next-message.svelte.ts
- catalog/fixtures/delta-chat/reference-discussion-play.ts
- spec/src/current/contracts/api/41-interactive-agent-execution.md
- spec/src/current/contracts/ui/36-commentable-chat-body-prototype.md
- spec/src/current/progress/M24/index.md
- spec/src/current/verification/ui-02.md
- spec/src/current/verification/ui.md
- spec/src/history/2026/10/08/E-CHAT-BODY-013/SUMMARY.nav.md
- spec/src/history/2026/10/08/E-CHAT-BODY-013/details-02.md
- spec/src/history/2026/10/08/E-CHAT-BODY-013/index.md
- spec/src/history/2026/10/08/SUMMARY.nav.md
- spec/src/history/2026/10/08/index.md
- web/src/lib/ui/store-agent-chat-body/ChatCommentArea.svelte
- web/src/lib/ui/store-agent-chat-body/ChatCommentDelivery.svelte
- web/src/lib/ui/store-agent-chat-body/ChatReplyContext.svelte
- web/src/lib/ui/store-agent-chat-body/comment-card.css
- web/src/lib/ui/store-agent-chat-body/comment-widgets.ts
- web/src/lib/ui/store-agent-chat-body/comments.svelte.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
<!-- /evidence-field -->

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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatBody.stories.svelte -t DeltaMainThreadReference
```

**Notes**

Before: 1 failed, 20 skipped; Go to reply absent. .local/chat-reply-before.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatBody.stories.svelte -t DeltaMainThreadReference
```

**Notes**

Intermediate runs in .local/chat-reply-after.log and .local/chat-reply-focus.log exposed a missing tooltip role. Actual keyboard focus and open tooltip state were confirmed, then the owned Tooltip.Content supplied role=tooltip.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit --project storybook catalog/fixtures/delta-chat/comments.test.ts catalog/fixtures/delta-chat/discussions.test.ts catalog/components/store-application/AgentChatBody.stories.svelte
```

**Notes**

Intermediate 31 passed, 1 failed at the tooltip assertion before the accessibility role fix. .local/chat-reply-tests.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit --project storybook catalog/fixtures/delta-chat/comments.test.ts catalog/fixtures/delta-chat/discussions.test.ts catalog/components/store-application/AgentChatBody.stories.svelte
```

**Notes**

32 passed: 11 state/adapter tests and 21 affected interaction/accessibility stories, including the exact original regression. .local/chat-reply-tests-final.log.
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
pnpm web:check
```

**Notes**

aria-description is not in the installed Svelte button attributes; replaced with an owned sr-only status and aria-describedby.
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
pnpm catalog:check
```

**Notes**

First run shared the aria-description issue. Second run found unsupported Testing Library exact option; use an anchored accessible-name regex instead.
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
node catalog/fixtures/delta-chat/browser.acceptance.mjs > .local/chat-reply-cold.log 2>&1
```

**Notes**

New touch assertion tried to tap an inline comment inside a still-collapsed source tool. Test now uses the reply context Go to source action to open/reveal it before tapping. Pointer tooltip/icon/navigation checks already passed.
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

0 errors, 0 warnings. .local/chat-reply-web-final.log.
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
node catalog/fixtures/delta-chat/browser.acceptance.mjs > .local/chat-reply-cold.log 2>&1
```

**Notes**

Fresh isolated Storybook and Chromium acceptance: pointer hover/tooltip/icon transition, exact reply navigation and touch navigation, existing selection, remount, fold, gutter, read sidebar, footer and IME-event checks. Private processes cleaned up.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm catalog:check
```
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm check:ui-ownership
```
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm check:progress
```
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm spec:first
```
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec spec-validator validate --only summary,fileLimits,governance,verification,book
```
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec prettier --check <changed authored files>
```
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-CHAT-BODY-013 details 2](details-02.md)
