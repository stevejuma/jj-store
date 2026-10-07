---
schema: 1
id: E-EXEC-012
---

# E-EXEC-012 details 2

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
pnpm exec tsx --test web/tests/agent-execution.test.ts
```

**Notes**

Before implementation: four existing tests pass; resume history is not a new message or feedback target fails because opening-request replay creates extra assistant/plan rows.
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
pnpm exec tsx --test web/tests/agent-execution.test.ts
```

**Notes**

After implementation: all five tests pass. Original row IDs/times, subsequent genuine streaming, mismatched opening IDs and retained raw inputs are covered.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentDiscussion.stories.svelte
```

**Notes**

Initial fixture attempt: five existing stories pass; recovery fails because seeded draft used canonical fixture target session 99 instead of execution session 44. Corrected fixture scope.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentDiscussion.stories.svelte
```

**Notes**

Second attempt: recovery present; test expected combobox but released shadcn Select exposes a labelled button with listbox popup. Corrected locator to documented actual control.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentDiscussion.stories.svelte
```

**Notes**

Third attempt: recovery test expected aria-disabled instead of actual Bits UI data-disabled marker; failure left popup open and blocked the following reaction story. Corrected the assertion while retaining keyboard skip and draft non-overwrite proof.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentDiscussion.stories.svelte
```

**Notes**

Fourth attempt: Select close animation still held body pointer lock when Restore was clicked. Wait for the actual pointer lock to release; no timeout weakening or forced click.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentDiscussion.stories.svelte
```

**Notes**

Fresh isolated process/browser: all six stories pass, including keyboard selection, no extra replay message, visible orphan draft, protected existing draft, restored enabled Save reply, exact assistant reply persistence and no lifecycle/prompt dispatch while disconnected. Accessibility runs with test=error; visual-pending unchanged.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentDiscussion.stories.svelte catalog/components/store-application/AgentChatFeedback.stories.svelte catalog/components/store-application/AgentExecution.stories.svelte
```

**Notes**

24 stories across three affected inbox families pass, including exact permissions and unknown reply receipt recovery.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatLayout.stories.svelte catalog/components/store-application/AgentChatRuntime.stories.svelte catalog/components/store-application/AgentHarnessModels.stories.svelte catalog/components/store-application/AgentRevisionIndex.stories.svelte catalog/components/store-application/AgentSessions.stories.svelte catalog/components/store-application/AgentSessionSettings.stories.svelte
```

**Notes**

25 stories in the other six direct AgentExecutionFixture consumers pass. Together the affected nine families have 49 passing stories; cold isolated processes preserve the current demo and real draft.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/agent-execution-controller.test.ts web/tests/vite/agent-runtime-preparation.test.ts
```

**Notes**

Nine direct controller/runtime tests pass in two files; no runtimeChoices behavior change.
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
pnpm web:check
pnpm catalog:check
```

**Notes**

Preliminary and final runs have zero errors/warnings; 272 catalog modules pass ownership boundaries.
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
pnpm exec tsx --eval 'import { readFileSync } from "node:fs"; import assert from "node:assert/strict"; import { activityRows, openingReplayRowIds } from "./web/src/lib/ui/store-agent-execution/presentation.ts"; const saved = JSON.parse(readFileSync(".local/inbox-disabled-reply-all-events.json", "utf8")); const events = saved.events ?? saved; const feedback = JSON.parse(readFileSync(".local/inbox-disabled-reply-feedback.json", "utf8")); const rows = activityRows(events).filter(row => row.kind === "user" || row.kind === "assistant"); assert.equal(rows.length, 6); assert.equal(feedback.messages.length, 6); for(const row of rows) assert.ok(feedback.messages.some(message => message.row_id === row.id)); assert.deepEqual(openingReplayRowIds(events), ["open-442e819f72434178bfa58bd56d2a44bd/assistant"]); console.log("Captured current session: six original chat rows match six exact published targets; resume replay excluded.");'
```

**Notes**

Read-only captured current session has 271 events and six exact published feedback targets. Corrected projection has exactly six original message rows matching those targets; the opening replay row is excluded. Ignored captured JSON stays uncommitted.
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
await cua.rewriteDocumentation()
await tab.playwright.domSnapshot()
```

**Notes**

Current in-app tab refuses CDP focus emulation. No reload, real feedback save, prompt, resume or lifecycle action was performed. Live post-HMR draft state could not be inspected; isolated production-component fixtures are the browser proof, not a claimed current-tab pass.
<!-- /run -->
<!-- /evidence-field -->

