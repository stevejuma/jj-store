---
schema: 1
id: E-CHAT-BODY-002
---

# E-CHAT-BODY-002 details 2

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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatBody.stories.svelte
```

**Notes**

Initial refinement: 10/11 passed; scroll-to-turn assertion raced a stale at-end flag during smooth scroll. The assertion now waits for the same required aria-current result.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatBody.stories.svelte -t AppBodyShell
```

**Notes**

Exact navigation regression passes: 1 passed, 10 skipped.
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
pnpm catalog:check
```

**Notes**

A key handler directly on the noninteractive navigation group produced one Svelte accessibility warning. Scoped key handling moved into the owned listener lifecycle with teardown, keeping the same keyboard behavior.
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
node catalog/fixtures/delta-chat/browser.acceptance.mjs
```

**Notes**

A focus-ring assertion used programmatic focus after pointer input, which does not establish :focus-visible. The native harness now uses actual Tab navigation and retains both no-hover-outline and visible-keyboard-focus requirements.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit catalog/fixtures/delta-chat/adapter.test.ts catalog/fixtures/delta-chat/middle-fold.test.ts
```

**Notes**

Initial fold units passed 10/10, including threshold boundaries, complete Unicode source offsets and honest reveal behavior for valid versus stale anchors.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatBody.stories.svelte -t LargeCode
```

**Notes**

Initial large-capture interaction passes: 1 passed, 11 skipped. Revealing code preserves exact middle-line comments across remount and leaves the patch folded.
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

The new large story autoplay raced the independent native interaction and removed its fold button. The native fixture URL now uses supported Storybook embed=true to disable autoplay; the story play remains independently tested.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatBody.stories.svelte -t 'MessageReactions|LargeCode|AppBodyShell'
```

**Notes**

2 selected passed and 1 failed, 10 skipped. A one-paragraph XML fixture correctly rendered as a compact prompt element; the expandable-context test now uses a multi-paragraph fixture.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatBody.stories.svelte -t 'MessageReactions|InlineNextMessage|LargeCode'
```

**Notes**

2 selected passed and 1 failed, 11 skipped. Default CodeMirror Mod+Enter precedence inserted content instead of sending. The owned draft extension now uses Prec.highest; ordinary Enter remains newline insertion.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatBody.stories.svelte -t InlineNextMessage
```

**Notes**

Exact draft shortcut regression passes: 1 passed, 13 skipped, including multiline input, fixed footer, local Send, cleared draft/focus return and shortcut submission.
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
node catalog/fixtures/delta-chat/browser.acceptance.mjs
```

**Notes**

XML group matching was ambiguous with its Actions group under Playwright substring matching. An exact named group selector fixes the harness without altering rendering.
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
node catalog/fixtures/delta-chat/browser.acceptance.mjs
```

**Notes**

After the exact selector and shortcut fixes the complete cold native harness passed, including folds, real hover, reactions/XML, multiline next message and fixed footer. Final viewport/focus acceptance is recorded below.
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
pnpm check:architecture
```

**Notes**

The only failure remains inherited scripts/dev-ui.acceptance.mjs:311 counted lines over 300. New sources remain within their budgets; no unrelated script/baseline changes. Body styles were separated into a component-owned CSS file.
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
pnpm check:ui-ownership
```

**Notes**

Owned Community/Mira/public Design Core composition only; production has no catalog/Community checkout imports. No copied component or provenance change.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatBody.stories.svelte
```

**Notes**

Final affected family: 14/14 interaction/accessibility stories pass. Existing immutability, Unicode selection, focus, drafts, collapses, raw details, custom controls and narrow/dark cases remain covered.
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

Final cold native run passes with zero page errors. Includes actual pointer/keyboard selection, immutable documents, real CSS hover transparency and Comment hover, keyboard rail focus/Jump to End, fold reveal via Space without opening a comment, exact line-13 anchors, XML/reactions and the fixed footer/multiline draft. Send clears/focuses the draft and brings it into view. All owned processes clean up; only the review server remains.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit catalog/fixtures/delta-chat/adapter.test.ts catalog/fixtures/delta-chat/middle-fold.test.ts
```

**Notes**

Final 10/10 units, including returned-context display kind and whole-section XML anchors, honest one-turn reference mapping, source identity/Unicode and fold boundaries/visibility.
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

Final source and composer: 0 errors, 0 warnings.
<!-- /run -->
<!-- /evidence-field -->

