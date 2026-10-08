---
schema: 1
id: E-CHAT-BODY-011
---

# E-CHAT-BODY-011 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check
```

**Notes**

Two diagnostics for Array.findLast outside the configured TypeScript library and its untyped callback. Replaced it with slice/reverse/find without changing the compiler target or mutating transcript order. .local/chat-reply-types-before.log.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:check
```

**Notes**

Same two findLast/type diagnostics as the web check; fixed by using the existing supported array APIs.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatBody.stories.svelte -t DeltaMainThreadReference
```

**Notes**

Expanded sent batch duplicated both case screenBlock comments in the following reply context: expected zero repeated comment articles, received two. .local/chat-reply-repeat-before.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatBody.stories.svelte -t DeltaMainThreadReference
```

**Notes**

Computed quote/body typography matches (font family, size and leading) for patch, code and prose quotes. Reply context compacts with the expanded batch, restores two original comments when collapsed, compacts again when reopened and still navigates to source. 1 passed, 20 skipped. .local/chat-quote-typography-after.log and .local/chat-reply-repeat-after.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatBody.stories.svelte -t DeltaMainThreadReference
```

**Notes**

After a first font inheritance/1rem attempt, computed typography exposed 16px/24px source quotes versus 14px/21px rendered Markdown comment bodies (same DM Sans family). Added renderer-parity assertions; the final styling uses the public Mira Markdown size/leading tokens. .local/chat-quote-typography-before.log.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
node catalog/fixtures/delta-chat/browser.acceptance.mjs > .local/chat-seeded-cold-run.log 2>&1
```

**Notes**

Five successive cold attempts each failed once. The existing native gutter check could not find the README fold control after adding the longer reference interaction. Returning focus without source-reveal did not fix a second run. A catalog-only play-completion marker was added to prevent overlapping reference/native interactions; this did not resolve the missing fold control. Folding and geometry assertions remain unchanged. Failed runs retained in .local/chat-seeded-cold-before.log, .local/chat-seeded-cold-repeat.log and .local/chat-seeded-cold-ready.log. An initial relative wheel scroll also failed (.local/chat-seeded-cold-scroll.log); browser geometry diagnostics (.local/chat-seeded-cold-diagnostics.log) showed a 1,679px virtualized reader in a 663px viewport with the fold control outside the rendered viewport. The native test now scrolls the sidebar ScrollArea directly to reveal the fold before activating it; the same fold/gutter assertions remain intact.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatBody.stories.svelte -t DeltaMainThreadReference
```

**Notes**

After seeding, the existing reference interaction/accessibility story passed: 1 passed, 20 skipped. Initial absence of reference comments was established by fixture source inspection, not a failing functional assertion. .local/chat-seeded-reference.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatBody.stories.svelte -t DeltaMainThreadReference
```

**Notes**

Extended the existing reference interaction with sidebar Sent state, empty pending queue/disabled Send, View message focus into the four-comment batch, linked reply/source navigation and the grouped two-comment patch discussion. 1 passed, 20 skipped. .local/chat-seeded-links.log.
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

0 errors, 0 warnings; catalog ownership boundaries pass for 338 modules. .local/chat-seeded-catalog.log.
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

Owned UI boundary passes. .local/chat-seeded-ownership.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatBody.stories.svelte -t DeltaMainThreadReference
```

**Notes**

Final reference interaction passes after adding the explicit single case screenBlock quote assertion and the cold-run readiness marker. 1 passed, 20 skipped. .local/chat-seeded-reference-final.log.
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
pnpm check:progress
```

**Notes**

The evidence record had a pass result with an explicit remaining development-runtime diagnostic. Updated its overall result to pending, preserving the passing scoped UI checks and the unresolved diagnostic.
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

The same pass-with-gaps evidence validation prevented spec-first from trusting the change set. Corrected the evidence status rather than removing the gap.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatBody.stories.svelte
```

**Notes**

21 affected chat interaction/accessibility stories pass, including the reference, source/reply navigation, comment sending, gutter/folds, sidebar, spacing and footer. .local/chat-seeded-stories.log.
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
pnpm web:check
```

**Notes**

0 errors and 0 warnings. An initial launcher attempt had a missing leading slash in workdir and did not execute; retried with the correct workspace. .local/chat-seeded-web.log.
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
node catalog/fixtures/delta-chat/browser.acceptance.mjs > .local/chat-seeded-cold-run.log 2>&1
```

**Notes**

Fresh isolated Storybook/Chromium acceptance passes the existing pointer/keyboard/comment, source navigation, gutter, resizing, fixed footer and runtime-menu scenarios with the seeded reference. Browser and private server cleaned up; the user preview remains running. Vite logged seven ResizeObserver-loop notifications in .local/chat-body-cold.log; they did not fail the browser assertions or page-error check. This slice does not qualify removal of those development-runtime diagnostics.
<!-- /run -->

<!-- run -->
### Run 17

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
### Run 18

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
### Run 19

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
<!-- /evidence-field -->

