---
schema: 1
id: E-CHAT-LIVE-008
---

# E-CHAT-LIVE-008

<!-- evidence-field:id markdown -->
## Id

E-CHAT-LIVE-008
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M24
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-012
- JST-UI-015
- JST-API-001
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-09
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Sent-context disclosure and rendered/raw inspection refinement
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change oxsnyzzn
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

The live inspector used a bordered native details summary with a leading browser marker and nested Raw context payload disclosure. Rendered and unhighlighted raw content appeared together. The exact revised story initially failed to find the required named disclosure button.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- web/src/lib/ui/store-agent-execution/StoreAgentSentContext.svelte
- catalog/components/store-application/AgentContextDelivery.stories.svelte
- catalog/components/store-application/AgentPromptMarkup.stories.svelte
- catalog/components/store-application/LocalCompute.stories.svelte
- spec/src/current/contracts/ui/35-compute-defaults-and-sent-context.md
- spec/src/current/contracts/api/41-interactive-agent-execution.md
- spec/src/current/progress/M24/index.md
- spec/src/current/progress/M24/history/M24/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
<!-- /evidence-field -->

<!-- evidence-field:findings markdown -->
## Findings

The owned inspector composes the released Design Core Collapsible, ghost Button (xs), CodeBlock and Lucide icons. One adjacent code icon toggles the same context panel. Keyboard focus, touch and active raw mode expose it. The rendered subtree stays mounted and hidden while raw mode is active, retaining nested XML disclosures; outer collapse and queued-to-sent acknowledgement retain local state. Lossless pretty serialization preserves captured values including 9007199254740993. Community tool inspection was reviewed before the Design Core public components; no source copies, dependency changes or CSS workaround were needed.
<!-- /evidence-field -->

<!-- evidence-field:live markdown -->
## Live browser review

A fresh in-app browser tab opened the retained sample session ddcc5f115b95bf017564735592db880f at http://127.0.0.1:7460/. Sent context measures 24px high, uses the ghost variant and has an SVG as its final child. Clicking the code toggle replaces XML with coloured JSON and the shared copy control; switching back retains the expanded agent_context section. Browser warning/error logs are empty, including ResizeObserver. Screenshot: .local/sent-context-raw.jpg. An initial stale tab focus command timed out; a fresh isolated tab completed acceptance without changing the user-selected session.
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Visual baselines remain unapproved.
- M24 independent adapter, permission, native IME and desktop/platform gates stay open.
- Existing development acceptance-script architecture limit remains outside this UI-only slice.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; Node 26.9.0; pnpm 10.32.1; Vitest 4.1.11. Existing locked Design Core/Mira dependencies unchanged. Vitest emits the existing missing-root-Svelte-config notices.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No public DTO, capture, source selector, comment/reply, receipt or persistence format changes. Captured context serialization and delivery labels remain unchanged.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Shared Sent inspector in recorded/live Document and Classic agent conversations, plus its direct context/compute catalog consumers. Fresh Storybook processes cover disclosure lifecycle and direct rendering consumers; live Vite serves the changed component. No Rust, root aggregate, backend restart, provider dispatch or fixture mutation was needed.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Continue the independently gated M24 qualification; visual-baseline approval remains separate.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-UI-012 and JST-UI-015 specify the compact trailing-chevron control and one keyboard-accessible rendered/JSON panel. JST-API-001 inspection and captured payload contracts remain unchanged.
```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M24 remains In progress. This inspector refinement does not close live adapter, permission, native IME, desktop/platform or visual gates.
```
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentContextDelivery.stories.svelte -t OneInspector
```

**Notes**

Before: one failure, one skipped; required named button is absent (.local/sent-context-before.log).
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentContextDelivery.stories.svelte -t OneInspector
```

**Notes**

First implementation run reached the mode-retention checks but the story retained a detached toggle after outer collapse. Reacquire the current toggle before keyboard activation; no assertion weakened.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentContextDelivery.stories.svelte -t OneInspector
```

**Notes**

Exact regression: one pass, one skipped (.local/sent-context-after2.log). Checks compact height, trailing icon, acknowledgement retention, exclusive modes and Enter/Space activation.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentContextDelivery.stories.svelte catalog/components/store-application/AgentPromptMarkup.stories.svelte catalog/components/store-application/LocalCompute.stories.svelte catalog/components/store-application/Conversation*.stories.svelte
```

**Notes**

Affected interaction/accessibility family: 25 tests pass in five files (.local/sent-context-stories.log). This is the shared inspector impact surface, not the complete repository suite.
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
pnpm web:check
```

**Notes**

Zero errors and warnings (.local/sent-context-web.log).
<!-- /run -->

<!-- run -->
### Run 6

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

Two test-only type errors: Testing Library getByRole does not accept Playwright exact. Removed the unsupported options; string names already match exactly.
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
pnpm check:ui-ownership
```

**Notes**

Owned UI boundary passes (.local/sent-context-ownership.log).
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
pnpm catalog:check
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentContextDelivery.stories.svelte catalog/components/store-application/AgentPromptMarkup.stories.svelte catalog/components/store-application/LocalCompute.stories.svelte catalog/components/store-application/Conversation*.stories.svelte
```

**Notes**

Final catalog: zero errors/warnings and 345 modules. Final affected family: 25 tests pass in five files, including explicit JSON language selection and exact historical numeric values. Logs: .local/sent-context-catalog-final.log and .local/sent-context-stories-final.log.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown
```

```shell
pnpm check:progress
pnpm spec:first
```

**Notes**

Initial evidence used three Storybook labels as named unit-test references, which the checker does not support, and marked pass while retaining independent qualification gaps. Retain the exact story commands above, remove unsupported test references, and use pending for the aggregate evidence; functional outcomes remain passing.
<!-- /run -->

<!-- run -->
### Run 10

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

After correcting evidence, the gate requires the mapped API chapter for the shared execution UI. Added an explicit JST-API-001 unchanged-contract confirmation for captured context; no endpoint or DTO changes.
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
pnpm check:progress
pnpm spec:first
pnpm exec spec-validator validate --only summary,fileLimits,governance,verification,book
```

**Notes**

Final ledger: 146 tasks/691 records. Spec-first: four protected files, two canonical chapters. Five document validators pass. Changed source/current Markdown formatted; unrelated generated M25/Bootstrap whitespace restored. No wider product lanes required beyond the recorded UI impact surface.
<!-- /run -->

<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


