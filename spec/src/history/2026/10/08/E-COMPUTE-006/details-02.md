---
schema: 1
id: E-COMPUTE-006
---

# E-COMPUTE-006 details 2

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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/agent-mention-settings.test.ts
```

**Notes**

Before production edits: 2 failed; missing preference-only operation.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentMentionSettings.stories.svelte
```

**Notes**

Before: 2 failed as described above. Implementation iterations: 2, 2, 2, 1, then 0 failures. Test fixes scoped duplicate Harness default options to the Select menu, waited for portal closure, used the actual button role and verified disabled options by data-disabled plus keyboard skipping. Final exact target: 2 passed.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/agent-mention-settings.test.ts web/tests/vite/agent-settings.test.ts web/tests/vite/settings-selects.test.ts
```

**Notes**

Intermediate 4 then 5 tests passed; final 6 passed, including exact u64 stale observation ordering.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentMentionRecovery.stories.svelte catalog/components/store-application/LocalCompute.stories.svelte
```

**Notes**

Initial recovery fixture used the wrong constructor, then accidentally refreshed through tracked fixture state. Correcting the isolated fixture produced recovery 1/1 passing. Compute 10/10 passed. No agent prompt or compute command was sent to real services.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentMentionRecovery.stories.svelte catalog/components/store-application/AgentMentionSettings.stories.svelte catalog/components/store-application/AgentFleet.stories.svelte
```

**Notes**

The two Settings regressions and 7 existing fleet cases passed; the recovery fixture’s unintended tracked refresh was corrected and its exact case passed separately.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/Agent*.stories.svelte catalog/components/store-application/LocalCompute.stories.svelte catalog/components/store-threads/StoreThreads.stories.svelte catalog/components/store-appearance/AppearanceComposer.stories.svelte
```

**Notes**

Fresh Vitest/Vite/Chromium process: 136 passed, 1 failed in 25 files. OpenCodeTypedTextMessageParts could not find Copy message in a submenu. It passed in isolation. This read-only transcript fixture does not exercise the changed controls; no assertion was weakened.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentPromptMarkup.stories.svelte -t OpenCodeTypedTextMessageParts
```

**Notes**

1 passed, 8 skipped. Investigate menu timing if this unrelated intermittent failure recurs.
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

Initial 4 declaration-order errors and later 1 callback type error fixed. Final 0 errors, 0 warnings.
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

Initial 3 invalid test query options and later 1 callback type error fixed. Final 0 errors, 0 warnings; boundaries passed for 301 modules.
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

Released foundational controls only; no Community dependency or copied dependency implementation.
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
pnpm check:architecture
```

**Notes**

New component count violations were resolved by extracting the immediate preference field and simplifying composer error handling. Final remaining failure: unchanged scripts/dev-ui.acceptance.mjs has 311 counted lines (limit 300), already recorded in E-GOV-005.
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
pnpm docs:review application "Reviewed shared saved automatic mentions, exact recovery without sending, retained defaults and secret drafts, and shadcn Settings choices against the updated Settings guide and production sources."
```

**Notes**

Updated application manual and source fingerprint. pnpm exec prettier --write was run on changed source, tests and documents only.
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
pnpm docs:check
```

**Notes**

Links and anchors valid; documentation build passed. Final source review rerun after last UI edits.
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
pnpm dev:ui
```

**Notes**

Fresh Vite at 7460 and backend at 7462; fixture preserved and Search ready. CUA temporary tab showed saved Automatically mention agents off, immediate-save copy, shadcn Thinking and Execution mode. Thinking opened advertised options and Escape returned focus. DOM: 0 native selects, 2 shadcn triggers. Closed temporary tab; existing user tab/draft and preference values untouched.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/{AgentDefaults,AgentFleet,AgentMentionSettings,AgentMentionRecovery,AgentSessionSettings,LocalCompute}.stories.svelte
```

**Notes**

Final cold run after all product edits: 28 passed across 6 files, including interaction/accessibility and all newly changed control families.
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
jj workspace add --name mention-parent --revision @- /tmp/jj-store-mention-parent
```

**Notes**

Isolated unchanged parent revision 16d8ea7a for OpenCode menu investigation. Main fixture untouched.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentPromptMarkup.stories.svelte
```

**Notes**

First parent attempt with symlinked node_modules: setup import URL failed, zero tests ran. No product assertion failed.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm install --offline --frozen-lockfile --ignore-scripts
```

**Notes**

Disposable parent only: existing sibling-relative local-package hook could not match the file dependency lock outside its original location. Subsequent vitest invocation had no installed binary. No manifest/lock changes retained.
<!-- /run -->
<!-- /evidence-field -->

