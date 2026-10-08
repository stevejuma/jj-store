---
schema: 1
id: E-COMPUTE-004
---

# E-COMPUTE-004 details 2

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
jj --no-pager st; read tracked root/UI and parent AGENTS; pnpm spec:search JST-UI-013; inspect owning UI/API/documentation chapters, M4/M24/M25 ledgers, Community model picker and public Design Core Command View/Popover/Tooltip APIs
```

**Notes**

Clean oklpnryt on parent f5fc5696. Community then Design Core source inspection, JJ/shadcn skill guidance retained. Buzz ef2aa1ae and mesh-llm 9f192c9 source catalogs/license inspected read-only; owned source provenance recorded.
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
pnpm exec tsx --test web/tests/layout-storage.test.ts
```

**Notes**

Before default implementation: 3 passed / 2 failed. New collapsed-default and unavailable-storage assertions reproduced open=true; explicit saved state/width regression retained.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 6
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/ComputeModelPicker.stories.svelte catalog/components/store-application/ManagedAgentIdentity.stories.svelte
```

**Notes**

Initial 9 cases: 3 passed / 6 failed, with 3 runtime diagnostics. Missing tooltip provider fixed. Command input accessible role/name aligned to Command View root; no assertion was removed.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/ComputeModelPicker.stories.svelte
```

**Notes**

2 passed / 3 failed. Menu rendered; test input name/role mismatched Command View aria-labelledby. Root and input now share Search local models, role combobox.
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

Initial production diagnostics: 0 errors / 0 warnings.
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
pnpm exec tsx --test web/tests/layout-storage.test.ts web/tests/compute-model-catalog.test.ts
```

**Notes**

7 passed. RAM headroom, draft limits, BigInt observation, unknown/invalid hardware, fit boundaries, safe recommendation, exact installed/custom identities, collapsed defaults and saved open preference covered.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 6
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:check
```

**Notes**

First new test type check rejected unsupported exact option on ByRoleOptions. Removed that option; name strings already match exactly.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 12
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/ComputeModelPicker.stories.svelte catalog/components/store-application/ManagedAgentIdentity.stories.svelte catalog/components/store-application/LocalCompute.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte catalog/components/store-application/RepositoryReview.stories.svelte catalog/components/store-application/RepositoryLoading.stories.svelte catalog/components/store-application/StoreFileEditor.stories.svelte catalog/components/store-application/AgentFleet.stories.svelte
```

**Notes**

42 passed / 12 failed. Missing tooltip role/provider and overriding shared chat border fixed. Search root label fixed; file tests explicitly open the now-collapsed tree before tree-navigation assertions. Divider geometry accounts for its 1px border. Advanced label query scoped to disclosure trigger. One unrelated pre-existing Op Log role=status case remained.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:check
```

**Notes**

Three multiline exact options remained after first cleanup; removed them. Subsequent catalog diagnostics pass.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/ComputeModelPicker.stories.svelte catalog/components/store-application/ManagedAgentIdentity.stories.svelte catalog/components/store-application/LocalCompute.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte catalog/components/store-application/RepositoryReview.stories.svelte catalog/components/store-application/RepositoryLoading.stories.svelte catalog/components/store-application/StoreFileEditor.stories.svelte catalog/components/store-application/AgentFleet.stories.svelte
```

**Notes**

51 passed / 3 failed. Fuzzy Coder search also matched another row, so case now searches the model-family phrase Qwen2.5-Coder. Tooltip Content gets explicit role=tooltip, matching existing Community composition. Unrelated Op Log case isolated below.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/ComputeModelPicker.stories.svelte catalog/components/store-application/ManagedAgentIdentity.stories.svelte
```

**Notes**

8 passed / 1 failed before adding explicit tooltip content role. Model keyboard/search/metadata cases all pass.
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
jj workspace add .local/ui-followup-baseline --name ui-followup-baseline -r @-; link existing node_modules; pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/RepositoryReview.stories.svelte -t ShellAndRowLocalGraphSpacing
```

**Notes**

First isolated parent run could not import the shared node_modules setup file due to Vite filesystem allow roots; no tests executed. Temporary test-only config allowed the parent checkout and shared dependencies.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/RepositoryReview.stories.svelte -t ShellAndRowLocalGraphSpacing
```

**Notes**

In disposable parent f5fc5696 workspace: 1 failed / 2 skipped, identical missing role=status on Op Log. Confirms independent pre-existing failure. No production changes made in baseline checkout. Workspace forgotten and disposable directory removed after the run.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/ManagedAgentIdentity.stories.svelte
```

**Notes**

4 passed after tooltip-role fix, including icon/header placement, visible shadcn tooltips and retained captured identities.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:ui-ownership; pnpm check:architecture
```

**Notes**

Ownership passed. Architecture rejected growth past story limits: RepositoryReview 514 &gt; legacy 513, Connected application 425 &gt; 400. Extracted unchanged layout/directory assertion bodies into owned catalog helpers, retaining named cases and assertions.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/ComputeModelPicker.stories.svelte catalog/components/store-application/ManagedAgentIdentity.stories.svelte catalog/components/store-application/LocalCompute.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte catalog/components/store-application/RepositoryReview.stories.svelte catalog/components/store-application/RepositoryLoading.stories.svelte catalog/components/store-application/StoreFileEditor.stories.svelte catalog/components/store-application/AgentFleet.stories.svelte -t '^(?!ShellAndRowLocalGraphSpacing)'
```

**Notes**

Cold fresh Chromium run: 8 families, 53 passed / 1 skipped. Only independently reproduced parent Op Log failure excluded. Includes interaction and configured Axe checks, 390px menu bounds, saved layout, file edit/navigation, chat border, icon actions, model metadata/search/focus/draft retention and disabled serving. This is a focused pass, not a full suite pass.
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
pnpm web:check; pnpm catalog:check; pnpm check:architecture; pnpm check:ui-ownership
```

**Notes**

Final UI/catalog diagnostics 0 errors / 0 warnings; 290 catalog modules and 1334 architecture modules pass; ownership validator passes. Commands run as separate focused invocations.
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
pnpm docs:review application "Compute command menu metadata, repository agent header actions and Files default/layout; public API unchanged"; pnpm docs:check
```

**Notes**

Initial source fingerprint review passed; later tooltip-role source change required a fresh application review. No schema/reference change.
<!-- /run -->
<!-- /evidence-field -->

