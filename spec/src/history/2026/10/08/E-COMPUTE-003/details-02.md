---
schema: 1
id: E-COMPUTE-003
---

# E-COMPUTE-003 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/LocalCompute.stories.svelte catalog/components/store-application/SettingsNavigation.stories.svelte
```

**Notes**

Before spacing/Compute joins: 5 failed / 7 passed. Four requested regressions reproduced; the fifth was a stale Capture assertion without selecting the now-default Runtimes tab. Explicit navigation preserves the unavailable-capture assertion.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/LocalCompute.stories.svelte catalog/components/store-application/SettingsNavigation.stories.svelte
```

**Notes**

Initial spacing and joins: 12 passed. Later user steering replaces full-width run buttons with fixed 10rem wrapping buttons below the description.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 8
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/LocalCompute.stories.svelte catalog/components/store-application/SettingsNavigation.stories.svelte catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-application/AgentRuntimes.stories.svelte catalog/components/store-application/StoreMultiHarnessSettings.stories.svelte catalog/components/store-application/StoreWorkspaceSettings.stories.svelte catalog/components/store-application/StoreCaptureSettings.stories.svelte catalog/components/store-application/StoreSearchSettings.stories.svelte catalog/components/store-application/StoreStorageSettings.stories.svelte catalog/components/store-application/StoreServerSettings.stories.svelte catalog/components/store-application/StoreSyncSettings.stories.svelte catalog/components/store-application/AgentFleet.stories.svelte
```

**Notes**

87 tests: 79 passed / 8 failed. Six runtime cases found a duplicate Agent runtimes heading; removed duplicate through the requested section composition. Multi-harness assumed expanded sections despite M25 collapsed-on-load contract; now verifies collapsed and explicitly opens each. New overflow assertion shadowed viewport width with action width; repaired the variable, retaining viewport overflow assertions.
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
pnpm web:check && pnpm catalog:check && pnpm check:architecture && pnpm check:ui-ownership
```

**Notes**

Initial catalog check passed Element to HTMLElement-only containment matcher. Added closest&lt;HTMLElement&gt;. Wider architecture/ownership commands did not execute on this attempt.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreServerSettings.stories.svelte catalog/components/store-application/AgentFleet.stories.svelte -t 'Joined|Defaults last'
```

**Notes**

New defaults ordering case failed; new dynamic Story names caused Svelte CSF import failure. Changed new server cases to three static named stories with one shared play function; production unchanged.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreServerSettings.stories.svelte -t Joined
```

**Notes**

All three configuration-table cases fail before implementation: service/protection save and saved/running rows are separate; Sync has multiple tables.
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
pnpm web:check
```

**Notes**

New runtime snippet shadowed the runtime manager prop in TypeScript. Renamed the snippet runtimeSection without changing runtime management.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreServerSettings.stories.svelte catalog/components/store-application/AgentFleet.stories.svelte catalog/components/store-application/LocalCompute.stories.svelte catalog/components/store-application/AgentRuntimes.stories.svelte catalog/components/store-application/StoreMultiHarnessSettings.stories.svelte
```

**Notes**

31 interaction/accessibility tests passed after joining configuration tables, moving defaults last and removing duplicate headings.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentFleet.stories.svelte -t Defaults
```

**Notes**

4 defaults cases passed, 7 skipped. Installed discovery, connected/remembered session models, retained selections on explicit failed retries, joined footer, Advanced draft and no chat dispatch. Fixture initial-only session selection subsequently wrapped in untrack; no warning waiver.
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
cargo test --locked -p jj-store-http --lib agent_execution
```

**Notes**

New admission test moved Access before reuse; cloned the fixture access. Product compiled; test compilation did not complete.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/LocalCompute.stories.svelte catalog/components/store-application/AgentFleet.stories.svelte catalog/components/store-application/StoreServerSettings.stories.svelte catalog/components/store-application/AgentRuntimes.stories.svelte
```

**Notes**

35 tests: 34 passed / 1 failed. Added jj-agent row correctly yields three Ready/Unknown rows; updated explicit inventory counts from two to three, retaining failed-refresh assertions.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check && pnpm catalog:check && pnpm check:architecture && pnpm check:ui-ownership
```

**Notes**

Web/catalog 0 errors/warnings; architecture found AgentFleet 457 counted lines and editor dialog 301. Split defaults stories into their own family and extracted the identical saved-preference summary with original source/license provenance. Limits unchanged.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/LocalCompute.stories.svelte catalog/components/store-application/SettingsNavigation.stories.svelte catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-application/AgentRuntimes.stories.svelte catalog/components/store-application/StoreMultiHarnessSettings.stories.svelte catalog/components/store-application/StoreWorkspaceSettings.stories.svelte catalog/components/store-application/StoreCaptureSettings.stories.svelte catalog/components/store-application/StoreSearchSettings.stories.svelte catalog/components/store-application/StoreStorageSettings.stories.svelte catalog/components/store-application/StoreServerSettings.stories.svelte catalog/components/store-application/StoreSyncSettings.stories.svelte catalog/components/store-application/AgentFleet.stories.svelte catalog/components/store-application/AgentDefaults.stories.svelte catalog/components/store-application/AgentExecution.stories.svelte catalog/components/store-application/AgentSessionSettings.stories.svelte catalog/components/store-application/AgentHarnessModels.stories.svelte
```

**Notes**

116 tests: 114 passed / 2 failed. During this run the user replaced full-width buttons with a wrapping row; two earlier full-width/column assertions became stale. Updated only the requested run-button presentation expectations and kept every other action-column assertion.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/LocalCompute.stories.svelte
```

**Notes**

9 passed after fixed-width wrapping: 1280px row, 390px wrap/no overflow, spacing, joined tables, browse selection/cancel, retained tab drafts and explicit commands.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/LocalCompute.stories.svelte catalog/components/store-application/SettingsNavigation.stories.svelte catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-application/AgentRuntimes.stories.svelte catalog/components/store-application/StoreMultiHarnessSettings.stories.svelte catalog/components/store-application/StoreWorkspaceSettings.stories.svelte catalog/components/store-application/StoreCaptureSettings.stories.svelte catalog/components/store-application/StoreSearchSettings.stories.svelte catalog/components/store-application/StoreStorageSettings.stories.svelte catalog/components/store-application/StoreServerSettings.stories.svelte catalog/components/store-application/StoreSyncSettings.stories.svelte catalog/components/store-application/AgentFleet.stories.svelte catalog/components/store-application/AgentDefaults.stories.svelte catalog/components/store-application/AgentExecution.stories.svelte catalog/components/store-application/AgentSessionSettings.stories.svelte catalog/components/store-application/AgentHarnessModels.stories.svelte
```

**Notes**

Final shared Settings/model-picker closure cold run: 16 families / 116 interactions and configured blocking accessibility cases passed in 53.43s. Covers all Settings consumers, model/session editors, capture disclosure, agent defaults discovery/retry, runtime inventory, draft/recovery and narrow layouts. Visual baselines remain unapproved.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/LocalCompute.stories.svelte
```

**Notes**

After final invitation change, fresh cold run: 9 passed in 7.11s, including syntax-highlighted JSON invitation in a wrapped CodeBlock at 390px. Peer invitation stays editable. Other 15 families were already passing and unaffected by this leaf change.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/agent-model-catalog.test.ts web/tests/vite/agent-compute-picker.test.ts
```

**Notes**

4 passed: cached/no-work model discovery, failed-read explicit recovery, setup-specific caches and single native picker request with abort/late-selection suppression.
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
pnpm exec tsx --test web/tests/agent-harness-catalog.test.ts
```

**Notes**

2 passed: latest remembered catalog and current negotiation precedence without harness mixing.
<!-- /run -->
<!-- /evidence-field -->

