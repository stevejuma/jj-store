---
schema: 1
id: E-MAIN-LOCAL-QUALIFICATION-001
---

# E-MAIN-LOCAL-QUALIFICATION-001 details 7

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
pnpm web:check
```

**Notes**

web-check-03: exit 0. Zero errors and warnings before inbox resize batching.
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
pnpm catalog:check
```

**Notes**

catalog-check-03: exit 0. Zero errors and warnings; 461 module boundaries passed before later readiness edits.
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
JJ_STORE_DOC_REFERENCE=$PWD/target/debug/examples/docs_reference pnpm docs:check
```

**Notes**

docs-check-03: exit 0. 143 documentation files; nine Mermaid diagrams rendered without network; link/anchor closure passed.
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
node --test --test-concurrency=1 scripts/*.test.mjs scripts/spec/*.test.mjs
```

**Notes**

script-tests-03: exit 0. 219 passed, zero failed/cancelled/skipped; stable source run.
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
CARGO_BUILD_JOBS=2 pnpm dev:cargo -- test --workspace --locked --no-fail-fast
```

**Notes**

workspace-tests: exit 130. Interrupted diagnostic after discovering exact inventory/policy assertion failures and multiple deadline failures under default parallel test scheduling. Full serial qualification is required; no full-suite pass established.
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
pnpm dev:clean -- --apply
```

**Notes**

cleanup-apply-04: exit 0. After verifying all owned Cargo processes had stopped, eligible main-workspace artifacts were cleaned. Physical free space rose from 66.5 to 91.2 GiB. User forge target remained protected; sources, fixtures and staged resources were preserved.
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
pnpm dev:cargo -- test --locked -p jj-store-http --lib contract::tests::contract_inventory_covers_security_methods_media_and_task_bodies -- --exact
```

**Notes**

http-inventory-04: exit 0, 122.28 seconds. 
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
CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0 pnpm dev:cargo -- clippy --locked -p jj-forge --test planner -- -D warnings
```

**Notes**

forge-planner-clippy-06: exit 0. Strict focused planner Clippy passed after using borrowed single-item slices.
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
JJ_STORE_QUALIFICATION_REPORT=./storybook-full-03.jsonl pnpm exec vitest run --config catalog/vitest.config.ts --project storybook --reporter=default --reporter=./.local/main-qualification-2026-10-16/live-reporter.mjs
```

**Notes**

storybook-full-03: exit 1. 948 passed, 73 failed across 211 files. Diagnostic run overlapped source repairs; full same-source proof remains required.
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
JJ_STORE_QUALIFICATION_REPORT=./ui-regressions-09.jsonl pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatChanges.stories.svelte catalog/components/store-application/StoreRepositoryBranchContext.stories.svelte catalog/components/store-mutations/StoreRepositoryActions.stories.svelte catalog/components/store-workflows/WorkflowRun.stories.svelte catalog/components/store-application/GlobalSettingsLoading.stories.svelte catalog/components/store-application/RepositorySettingsLoading.stories.svelte catalog/components/store-forge/StoreForgeLinks.stories.svelte catalog/components/store-application/AgentProviderConfiguration.stories.svelte catalog/components/store-application/StoreLocalFolderDialog.stories.svelte --reporter=default --reporter=./.local/main-qualification-2026-10-16/live-reporter.mjs
```

**Notes**

ui-regressions-09: exit 1. 15 passed /24 failed; confirmed 16 loading geometry failures, inaccessible provider lists, dark diff contrast and stale commit-picker text queries.
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
JJ_STORE_QUALIFICATION_REPORT=./ui-regressions-10.jsonl pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatChanges.stories.svelte catalog/components/store-application/StoreRepositoryBranchContext.stories.svelte catalog/components/store-mutations/StoreRepositoryActions.stories.svelte catalog/components/store-workflows/WorkflowRun.stories.svelte catalog/components/store-application/GlobalSettingsLoading.stories.svelte catalog/components/store-application/RepositorySettingsLoading.stories.svelte catalog/components/store-forge/StoreForgeLinks.stories.svelte catalog/components/store-application/AgentProviderConfiguration.stories.svelte catalog/components/store-application/StoreLocalFolderDialog.stories.svelte --reporter=default --reporter=./.local/main-qualification-2026-10-16/live-reporter.mjs
```

**Notes**

ui-regressions-10: exit 1. 37 passed /2 failed: dark diff token precedence and provider-selector ambiguity during closing animation.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-mutations/StoreRepositoryActions.stories.svelte catalog/components/store-forge/StoreForgeLinks.stories.svelte catalog/components/store-application/AgentProviderConfiguration.stories.svelte
```

**Notes**

ui-regressions-11: exit 1. 8 passed /2 failed. Dark inline-PR diff and repository mutation families passed; new listbox assertion incorrectly included model suggestions, corrected to assert Select fields only.
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
pnpm web:check
```

**Notes**

web-check-12: exit 0, 22.14 seconds. 
<!-- /run -->

<!-- run -->
### Run 14

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

catalog-check-12: exit 1, 29.21 seconds. 
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
pnpm check:ui-ownership
```

**Notes**

ui-ownership-12: exit 0, 3.7 seconds. 
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
pnpm dev:cargo -- test --locked -p jj-store --test agent_kingfisher
```

**Notes**

kingfisher-04: exit 0, 179.7 seconds. 
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
pnpm check:boundaries
```

**Notes**

boundaries-14: exit 0, 0.34 seconds. 
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
pnpm check:architecture
```

**Notes**

architecture-14: exit 1, 0.72 seconds. 
<!-- /run -->
<!-- /evidence-field -->

