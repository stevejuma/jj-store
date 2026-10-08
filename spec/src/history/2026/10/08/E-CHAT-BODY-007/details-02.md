---
schema: 1
id: E-CHAT-BODY-007
---

# E-CHAT-BODY-007 details 2

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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit catalog/fixtures/delta-chat
```

**Notes**

Final: 25 tests across 6 files (.local/chat-body-unit-complete.log). Initial 1/23 fixture failure was fixed by preserving comments-only source records. The two follow-up regressions for separate pending entries and comment navigation first failed (2 failed, 7 passed), then passed (9/9 in their two files) before the final 25/25 surface.
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

Final: 16/16 interaction and axe stories (.local/chat-body-stories-final-2.log). Intermediate runs reported 5 failures, then 2, then 0; fixes covered captured draft extraction, obsolete immediate-submit expectations, duplicate collapsed summaries, restored draft focus and comment contrast. A later new nested-editor typography assertion found 1 failure; Mira document mode for mixed readers fixed it. Exact affected target then passed 2/2 before the final full family. No visual baselines updated. After the screenshot follow-ups, the final family expanded to 18/18 passing stories (.local/chat-body-stories-complete.log).
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatBody.stories.svelte -t 'Consistent'
```

**Notes**

New screenshot regression reproduced 9px above/17px below the one-line comment content (.local/chat-comment-spacing-before.log). An earlier spaced test-name filter matched no tests and was corrected to the generated story name. Changed actions to the public icon-xs size and enabled owned Markdown fullWidth for quote/comment cards.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatBody.stories.svelte -t 'Consistent'
```

**Notes**

Exact card/quote regression passed 1/1, 15 skipped (.local/chat-comment-spacing-after.log). Checks both inline and queued card insets, quote start edge, 8px tool/header/file gaps and 16px message gaps.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatBody.stories.svelte -t 'PatchSelection|Consistent'
```

**Notes**

2 passed, 14 skipped (.local/chat-body-font-spacing.log). Patch comment font now matches the main composer with no inherited code padding. Selection, composition, teardown and spacing assertions remain enabled.
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
node catalog/fixtures/delta-chat/browser.acceptance.mjs
```

**Notes**

Initial runs found stale test selectors after Markdown quote rendering and source-pane changes. Kept an exact data-chat-quote assertion alongside rendered text, and replaced transient sidebar screenshot readiness with the stable captured patch. Third run passed all native scenarios (.local/chat-body-cold-run-3.log).
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

Fourth run aborted a navigation (net::ERR_ABORTED) while independent Vite/unit/type work ran concurrently (.local/chat-body-cold-run-4.log). Retried the isolated browser lane sequentially without relaxing assertions or changing runtime policy. Fifth run reached all desktop checks but the new touch assertion used an ambiguous Agent reply label; bound it to the exact reviewed reply source block (.local/chat-body-cold-run-5.log).
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

Sequential run 6 passed all native desktop and touch checks (.local/chat-body-cold-run-6.log). The added touch source locator now selects its exact reviewed reply block rather than a repeated Agent reply label. A final run after comment-navigation/icon/grouping follow-ups is recorded separately.
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

Latest follow-up: 0 errors and 0 warnings (.local/chat-body-web-complete.log). The first follow-up check exposed an unsupported ES2023 findLast call (2 diagnostics); replaced it with filter/at under the existing library target.
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

Latest follow-up: 0 errors and 0 warnings; 331 modules pass catalog boundaries (.local/chat-body-catalog-complete.log). The initial follow-up reported the shared findLast diagnostics and two unsupported test exact options; all corrected without changing targets or weakening selectors.
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

Owned component boundary passes after all follow-ups (.local/chat-body-ownership-complete.log). No Community runtime, build, type or checkout import added.
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

Only inherited scripts/dev-ui.acceptance.mjs: 311 counted lines exceeds 300 (.local/chat-body-architecture-final.log). Compared its bytes to jj file show -r @-; unchanged from parent b4e3bbab. Earlier extraction removed this slice's temporary over-budget fixture/harness files. This unrelated development-harness gap remains assigned to its owner.
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
python3 scripts/delta-chat-fixture.py .local/delta-hello-world.json .local/chat-body-hello-world-reprojected.json
python3 scripts/delta-chat-comments.py .local/delta-hello-world.json .local/chat-body-hello-world-reprojected.json catalog/fixtures/delta-chat/hello-world-sources.json
python3 -c "import json; from pathlib import Path; assert json.loads(Path('.local/chat-body-hello-world-reprojected.json').read_text()) == json.loads(Path('catalog/fixtures/delta-chat/hello-world.json').read_text())"
```

**Notes**

Reprojection produced 4 discussions and 3 linked replies and exactly matches committed fixture data. Raw export stays ignored; no CRDT fragment interpretation or external dispatch.
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
pnpm exec prettier --check catalog/components/store-application/AgentChatBody.stories.svelte catalog/fixtures/ChatBodyFixture.svelte catalog/fixtures/delta-chat/acceptance-helpers.ts catalog/fixtures/delta-chat/acceptance-plays.ts catalog/fixtures/delta-chat/adapter.ts catalog/fixtures/delta-chat/browser.acceptance.mjs catalog/fixtures/delta-chat/comment-followup-play.ts catalog/fixtures/delta-chat/comment-lifecycle-play.ts catalog/fixtures/delta-chat/comment-navigation-play.ts catalog/fixtures/delta-chat/comment-queue-native.mjs catalog/fixtures/delta-chat/comments.test.ts catalog/fixtures/delta-chat/discussions.test.ts catalog/fixtures/delta-chat/discussions.ts catalog/fixtures/delta-chat/example-source.ts catalog/fixtures/delta-chat/fixture.css catalog/fixtures/delta-chat/hello-world-sources.json catalog/fixtures/delta-chat/hello-world.json catalog/fixtures/delta-chat/large-capture-native.mjs catalog/fixtures/delta-chat/large-capture-play.ts catalog/fixtures/delta-chat/middle-fold.test.ts catalog/fixtures/delta-chat/model.ts catalog/fixtures/delta-chat/next-message.svelte.ts catalog/fixtures/delta-chat/reader-input-native.mjs catalog/fixtures/delta-chat/spacing-play.ts catalog/fixtures/delta-chat/turns.ts spec/src/current/contracts/api/41-interactive-agent-execution.md spec/src/current/contracts/ui/36-commentable-chat-body-prototype.md spec/src/current/governance/spec-governance.md spec/src/current/progress/M24/index.md spec/src/current/verification/ui-02.md spec/src/current/verification/ui.md web/src/lib/ui/store-agent-chat-body/ChatCollapsedComments.svelte web/src/lib/ui/store-agent-chat-body/ChatCommentArea.svelte web/src/lib/ui/store-agent-chat-body/ChatCommentBatch.svelte web/src/lib/ui/store-agent-chat-body/ChatCommentEditor.svelte web/src/lib/ui/store-agent-chat-body/ChatContent.svelte web/src/lib/ui/store-agent-chat-body/ChatMessageFooter.svelte web/src/lib/ui/store-agent-chat-body/ChatNavigation.svelte web/src/lib/ui/store-agent-chat-body/ChatNextMessage.svelte web/src/lib/ui/store-agent-chat-body/ChatTextBlock.svelte web/src/lib/ui/store-agent-chat-body/ChatToolBlocks.svelte web/src/lib/ui/store-agent-chat-body/ChatToolPanel.svelte web/src/lib/ui/store-agent-chat-body/ChatTools.svelte web/src/lib/ui/store-agent-chat-body/ChatTurnCard.svelte web/src/lib/ui/store-agent-chat-body/StoreAgentChatBody.svelte web/src/lib/ui/store-agent-chat-body/chat-body.css web/src/lib/ui/store-agent-chat-body/comment-card.css web/src/lib/ui/store-agent-chat-body/comment-navigation.ts web/src/lib/ui/store-agent-chat-body/comment-widgets.ts web/src/lib/ui/store-agent-chat-body/comments.svelte.ts web/src/lib/ui/store-agent-chat-body/custom-interaction.ts web/src/lib/ui/store-agent-chat-body/markdown-blocks.ts web/src/lib/ui/store-agent-chat-body/model.ts web/src/lib/ui/store-agent-chat-body/navigation-turns.ts web/src/lib/ui/store-agent-chat-body/navigation.css web/src/lib/ui/store-agent-chat-body/text-reader.css
```

**Notes**

Final changed authored paths all pass (.local/chat-body-format-complete.log); exact path set comes from jj --no-pager diff --name-only, excluding generated evidence. No repository-wide formatter or Rust lane.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatBody.stories.svelte -t 'GroupedComments|CommentNavigation'
```

**Notes**

2 passed, 16 skipped (.local/chat-body-followup-stories-2.log). The first attempt had two test-selection issues: synthetic hover does not set native CSS :hover, and two historical batches share the Sent comments label. Keyboard focus now verifies the equivalent story action, native cold input verifies pointer hover, and history focus uses the exact captured comment ID.
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
node catalog/fixtures/delta-chat/browser.acceptance.mjs
```

**Notes**

Final cold isolated Storybook process/browser passes all pointer/keyboard input, immutability, mirrored editing/caret retention, failure recovery, mixed send, source/reply focus, actual Reply icon hover, middle folds, resize/cleanup, mobile read pane and touch source/remove actions (.local/chat-body-cold-complete.log). Owned process/browser cleanup completed. No native Delta actions or backend sends.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: fail
failures: 6
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
pnpm spec:first
```

**Notes**

Initial evidence metadata used bare test file paths where the ledger requires path#exact-test-name. Corrected five named unit references; Storybook/native scenarios remain recorded under commands and changed files. No product or contract gate was bypassed.
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
pnpm exec spec-validator validate --only summary,fileLimits,governance,verification,book
```

**Notes**

5 consuming document validators pass, 3255 chapters and 181 requirements (.local/chat-body-spec-docs.log).
<!-- /run -->
<!-- /evidence-field -->

