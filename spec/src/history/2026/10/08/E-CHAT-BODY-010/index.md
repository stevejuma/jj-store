---
schema: 1
id: E-CHAT-BODY-010
---

# E-CHAT-BODY-010

<!-- evidence-field:id markdown -->
## Id

E-CHAT-BODY-010
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

Remove the surrounding selected text-section outline
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change stllokrm above kyyuzynv in jj-store-chat-body
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Owned text-reader.css applied a 1px ring outline with a 4px offset to every chat-text-block:focus-within. The user requested removal. Existing message reaction/context interaction passed before editing (1 passed, 20 skipped); this is a requested style change, not a pre-existing functional failure.
<!-- /evidence-field -->

<!-- evidence-field:findings markdown -->
## Findings

Removed only the chat-text-block:focus-within rule. Prose, code, patches and sidebar readers no longer acquire a surrounding section outline when selected. Carets, selection/comment highlights, comment card borders, hover actions, control/link keyboard indicators and all spacing remain unchanged. Community control focus styling and the released Design Core editor boundary were inspected; this outline belonged to the local reader stylesheet, so no dependency change or override was needed.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- spec/src/current/contracts/api/41-interactive-agent-execution.md
- spec/src/current/contracts/ui/36-commentable-chat-body-prototype.md
- spec/src/current/progress/M24/index.md
- spec/src/current/verification/ui-02.md
- spec/src/current/verification/ui.md
- spec/src/history/2026/10/08/E-CHAT-BODY-010/SUMMARY.nav.md
- spec/src/history/2026/10/08/E-CHAT-BODY-010/index.md
- spec/src/history/2026/10/08/SUMMARY.nav.md
- spec/src/history/2026/10/08/index.md
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
pnpm spec:first
```

**Notes**

Initial ledger update omitted the mapped API chapter. Added the explicit unchanged JST-API-001 confirmation for this CSS-only behavior.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatBody.stories.svelte -t MessageReactionsAndXMLContext
```

**Notes**

Before: 1 existing interaction/accessibility story passed, 20 skipped. .local/chat-focus-before.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatBody.stories.svelte -t 'MessageReactionsAndXMLContext|CapturedLineNumberGutter|PatchSelectionCompositionAndTeardownPreserveContent|ConsistentToolFlowSpacing'
```

**Notes**

After: 4 existing stories pass, 17 skipped. Covers focused prose/actions, code gutter and sidebar, patch selection/composition and consistent spacing. .local/chat-focus-after.log.
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
pnpm web:check
```

**Notes**

0 errors and 0 warnings. .local/chat-focus-web.log.
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
pnpm check:ui-ownership
```

**Notes**

Owned presentation boundary passes. .local/chat-focus-ownership.log.
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
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

[]
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Node v26.9.0; pnpm 10.32.1; jj 0.45.1; Vitest 4.1.11; dependency versions unchanged.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No public type, HTTP, ACP, AgentStore or persisted-format changes.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

One local CSS rule, its UI contract and ledger. Existing focused interaction/accessibility stories and web type/ownership checks are the complete impact surface. No new tests for this reversible styling edit. Catalog types, state/adapter units, Rust, aggregate suites and architecture audit are out of scope. Startup/lifecycle/assets/bundles are unchanged, so no additional cold run is required by the repository policy. Visual baselines remain unapproved; the known unrelated development-script line limit is not changed or requalified.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Review the updated 6048 HMR preview; retain independent live/visual/platform qualification.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-UI-001/JST-UI-015 specify borderless focused text sections with
  retained caret/highlights and keyboard indicators on controls. JST-API-001,
  delivery and navigation remain unchanged because this is a local CSS removal.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M24 remains In progress. A local styling change does not expand
  independent live adapter, permission/cancellation, native IME, desktop or
  visual qualification.

```
<!-- /evidence-field -->

