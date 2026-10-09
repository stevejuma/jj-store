---
schema: 1
id: E-CHAT-LIVE-012
---

# E-CHAT-LIVE-012 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-application/AgentContextDelivery.stories.svelte catalog/components/store-application/AgentRuntimes.stories.svelte -t 'ContextButtonHasASubtleRestingSurface|WideRenderedContextStaysAtTheStart|RuntimeMenusAlignAtTheFarRightAfterOptionalActions'
```

**Notes**

Before: three product regressions reproduced. .local/context-runtime-before.log
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
pnpm test:storybook catalog/components/store-application/AgentContextDelivery.stories.svelte catalog/components/store-application/AgentRuntimes.stories.svelte -t 'ContextButtonHasASubtleRestingSurface|WideRenderedContextStaysAtTheStart|RuntimeMenusAlignAtTheFarRightAfterOptionalActions'
```

**Notes**

First after run: runtime alignment passed. Resting background passed, but synthetic userEvent hover did not activate CSS :hover. Real browser hover replaced it. The nested heading measured the legitimate 12px panel inset plus two 16px XML disclosure indents; the assertion now accounts for those computed indents while retaining the strict root inset limit. .local/context-runtime-after.log
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
pnpm test:storybook catalog/components/store-application/AgentContextDelivery.stories.svelte catalog/components/store-application/AgentRuntimes.stories.svelte -t 'ContextButtonHasASubtleRestingSurface|WideRenderedContextStaysAtTheStart|RuntimeMenusAlignAtTheFarRightAfterOptionalActions'
```

**Notes**

3 exact regressions pass. .local/context-runtime-after-final.log
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
pnpm test:storybook catalog/components/store-application/AgentContextDelivery.stories.svelte catalog/components/store-application/AgentRuntimes.stories.svelte catalog/components/store-application/AgentPromptMarkup.stories.svelte catalog/components/store-application/LocalCompute.stories.svelte
```

**Notes**

Initial direct-consumer family: 32 interaction/accessibility stories pass. .local/context-runtime-family.log
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
pnpm test:storybook catalog/components/store-application/LocalCompute.stories.svelte -t RichHistoricalContextAndExactRawVersion
```

**Notes**

Restored only CapturedOrigin to unchanged parent to reproduce legacy paragraph centering: 250px inset. Source fullWidth change then reapplied. .local/context-legacy-before.log
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
pnpm test:storybook catalog/components/store-application/LocalCompute.stories.svelte -t RichHistoricalContextAndExactRawVersion
```

**Notes**

Exact historical regression passes; retained bigint version and JSON presentation remain verified. .local/context-legacy-after.log
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
pnpm test:storybook catalog/components/store-application/AgentContextDelivery.stories.svelte catalog/components/store-application/AgentRuntimes.stories.svelte catalog/components/store-application/AgentPromptMarkup.stories.svelte catalog/components/store-application/LocalCompute.stories.svelte
```

**Notes**

Final source: 32 interaction/accessibility stories pass in four files. .local/context-runtime-family-final.log
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

Initial and final source: zero errors/warnings. .local/context-runtime-web-check-final.log
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

Initial and final source: zero errors/warnings; 347 module boundaries pass. .local/context-runtime-catalog-check-final.log
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:ui-ownership
```

**Notes**

Two existing app-authored composer helpers lacked local ownership declarations.
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

Ownership declarations repaired; no external Community dependency.
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
pnpm evidence:new -- --id E-CHAT-LIVE-012 --task M24 --requirement JST-UI-012 --requirement JST-API-001
```
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
pnpm progress:history
```

**Notes**

M24 evidence history refreshed. Unrelated M25/Bootstrap navigation formatting restored with Jujutsu.
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
pnpm check:progress
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
pnpm spec:validate --only governance,summary,fileLimits,book
```
<!-- /run -->

<!-- run -->
### Run 16

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

Provenance mapping required the canonical governance chapter. Added explicit unchanged JST-GOV-002/008 ownership confirmation.
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
pnpm spec:first
```

**Notes**

New governance confirmation was not yet listed in fresh evidence files. Included its exact source and JST-GOV-002 requirement here.
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
pnpm exec prettier --check catalog/components/store-application/AgentContextDelivery.stories.svelte catalog/components/store-application/AgentRuntimes.stories.svelte catalog/components/store-application/LocalCompute.stories.svelte catalog/fixtures/AgentContextDeliveryFixture.svelte docs/ui-provenance.json web/src/lib/ui/store-agent-execution/StoreAgentCapturedOrigin.svelte web/src/lib/ui/store-agent-execution/StoreAgentSentContext.svelte web/src/lib/ui/store-settings/StoreAgentRuntimes.svelte spec/src/current/contracts/ui/35-compute-defaults-and-sent-context.md spec/src/current/contracts/api/41-interactive-agent-execution.md spec/src/current/progress/M24/index.md spec/src/current/governance/spec-governance.md
```

**Notes**

All changed source/story/provenance and canonical contract/review files formatted. Authored pages were formatted with targeted prettier --write commands before this check.
<!-- /run -->

<!-- run -->
### Run 19

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first
```

**Notes**

8 protected files mapped to 3 canonical chapters with fresh evidence coverage.
<!-- /run -->
<!-- /evidence-field -->

