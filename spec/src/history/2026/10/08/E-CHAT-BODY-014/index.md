---
schema: 1
id: E-CHAT-BODY-014
---

# E-CHAT-BODY-014

<!-- evidence-field:id markdown -->
## Id

E-CHAT-BODY-014
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

Inline agent-reply indicator
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change umsknszm above ynttlpwq in jj-store-chat-body
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Agent reply answers had no leading arrow. The updated DeltaMainThreadReference regression failed once because the expected reply indicator was absent.
<!-- /evidence-field -->

<!-- evidence-field:findings markdown -->
## Findings

Assistant messages with reply context receive a single muted 16px Lucide CornerUpLeft widget before their first answer line with an 8px text gap. Ordinary messages and quoted comment context retain their prior presentation. The inline CodeMirror decoration changes no captured text or offsets; wrapped continuation lines use the normal content edge. A compartment updates the decoration without remounting the editor. Widget equality and DOM-keyed Svelte cleanup prevent duplicate or retained icons across remounts. This uses the same public Lucide and CodeMirror composition as existing owned fold widgets; no Community or foundational source is copied or modified.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/fixtures/delta-chat/comment-navigation-play.ts
- catalog/fixtures/delta-chat/comment-queue-native.mjs
- catalog/fixtures/delta-chat/reference-discussion-play.ts
- spec/src/current/contracts/api/41-interactive-agent-execution.md
- spec/src/current/contracts/ui/36-commentable-chat-body-prototype.md
- spec/src/current/progress/M24/index.md
- spec/src/current/verification/ui-02.md
- spec/src/current/verification/ui.md
- spec/src/history/2026/10/08/E-CHAT-BODY-014/SUMMARY.nav.md
- spec/src/history/2026/10/08/E-CHAT-BODY-014/index.md
- spec/src/history/2026/10/08/SUMMARY.nav.md
- spec/src/history/2026/10/08/index.md
- web/src/lib/ui/store-agent-chat-body/ChatContent.svelte
- web/src/lib/ui/store-agent-chat-body/ChatTextBlock.svelte
- web/src/lib/ui/store-agent-chat-body/StoreAgentChatBody.svelte
- web/src/lib/ui/store-agent-chat-body/reply-indicator.ts
- web/src/lib/ui/store-agent-chat-body/text-reader.css
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

1 failed, 20 skipped before implementation; missing inline arrow. .local/chat-reply-icon-before.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatBody.stories.svelte
```

**Notes**

21 affected interaction/accessibility stories pass, including the exact regression and all three captured reply markers. .local/chat-reply-icon-tests.log.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm web:check
```
<!-- /run -->

<!-- run -->
### Run 4

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
### Run 5

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
### Run 6

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
### Run 7

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
### Run 8

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
### Run 9

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

<!-- run -->
### Run 10

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
node catalog/fixtures/delta-chat/browser.acceptance.mjs > .local/chat-reply-icon-cold.log 2>&1
```

**Notes**

Fresh isolated Storybook and Chromium: exactly one decorative 16px reply marker, 8px first-text gap and body remount retention, plus existing source/reply, hover/keyboard/touch, comment, footer, fold, gutter and sidebar checks. Private processes cleaned up.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Cold development logs retain 7 ResizeObserver-loop notifications. Browser
  assertions pass; runtime diagnostic removal remains unqualified.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Node v26.9.0; pnpm 10.32.1; jj 0.45.1; Vitest 4.1.11. Existing Design Core/Mira installation unchanged.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

UI-only decorative widget; no persistence, source-text, comment-anchor, API/ACP or dispatch change.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Owned chat reader and its direct Storybook/cold-browser consumers. No Rust, repository aggregates, native Delta inspection or visual baseline comparison/update. The existing 6048 preview stays running.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Review the inline reply indicator in the running reference. Keep independent native IME, live integration, desktop and visual gates open.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-UI-001/JST-UI-015 specify the inline reply indicator. JST-API-001 is
  explicitly unchanged because the decoration does not mutate text or dispatch.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M24 remains In progress. Visual baselines remain unapproved and
  independent live, native IME and desktop qualification is unchanged.

```
<!-- /evidence-field -->

