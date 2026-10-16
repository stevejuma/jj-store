---
schema: 1
id: E-MAIN-LOCAL-REPAIRS-001
---

# E-MAIN-LOCAL-REPAIRS-001 details 8

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
JJ_STORE_QUALIFICATION_REPORT=./storybook-shard-2-24.jsonl pnpm exec vitest run --config catalog/vitest.config.ts --project storybook --shard=2/4 --reporter=default --reporter=./.local/main-qualification-2026-10-16/live-reporter.mjs
```

**Notes**

storybook-shard-2-24: exit 0, 246.65 seconds. 
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
JJ_STORE_QUALIFICATION_REPORT=./storybook-shard-3-24.jsonl pnpm exec vitest run --config catalog/vitest.config.ts --project storybook --shard=3/4 --reporter=default --reporter=./.local/main-qualification-2026-10-16/live-reporter.mjs
```

**Notes**

storybook-shard-3-24: exit 1, 240.76 seconds. 
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
node --test --test-concurrency=1 scripts/*.test.mjs scripts/spec/*.test.mjs
```

**Notes**

script-tests-26: exit 0, 111.74 seconds. 
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
JJ_STORE_QUALIFICATION_REPORT=./storybook-shard-4-24.jsonl pnpm exec vitest run --config catalog/vitest.config.ts --project storybook --shard=4/4 --reporter=default --reporter=./.local/main-qualification-2026-10-16/live-reporter.mjs
```

**Notes**

storybook-shard-4-24: exit 0, 283.82 seconds. 
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackTopology.stories.svelte
```

**Notes**

topology-25: exit 0, 17.7 seconds. 
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreRepositoryPickerPending.stories.svelte
```

**Notes**

picker-pending-27: exit 0, 11.58 seconds. 
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
JJ_STORE_QUALIFICATION_REPORT=./storybook-shard-1-27.jsonl pnpm exec vitest run --config catalog/vitest.config.ts --project storybook --shard=1/4 --reporter=default --reporter=./.local/main-qualification-2026-10-16/live-reporter.mjs
```

**Notes**

storybook-shard-1-27: exit 0, 223.17 seconds. 
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
JJ_STORE_QUALIFICATION_REPORT=./storybook-shard-3-27.jsonl pnpm exec vitest run --config catalog/vitest.config.ts --project storybook --shard=3/4 --reporter=default --reporter=./.local/main-qualification-2026-10-16/live-reporter.mjs
```

**Notes**

storybook-shard-3-27: exit 1, 256.07 seconds. 
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreSyncReconciliation.stories.svelte catalog/components/store-application/WorkspaceInspector.stories.svelte
```

**Notes**

sync-inspector-before-28: exit 1. 2 passed, 1 reconciliation visibility failure; original assertions retained with explicit dialog and resulting state readiness.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreSyncReconciliation.stories.svelte
```

**Notes**

sync-recovery-28: exit 0, 10.97 seconds. 
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
pnpm spec:build
```

**Notes**

spec-build-28: exit 0. 
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
actionlint
```

**Notes**

actionlint-28: exit 0. 
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

web-check-29: exit 0. Zero errors and warnings on the current UI source.
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
pnpm catalog:check
```

**Notes**

catalog-check-29: exit 0. Zero errors/warnings and 465 catalog modules passed on current source.
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
JJ_STORE_QUALIFICATION_REPORT=./storybook-shard-3-28.jsonl pnpm exec vitest run --config catalog/vitest.config.ts --project storybook --shard=3/4 --reporter=default --reporter=./.local/main-qualification-2026-10-16/live-reporter.mjs
```

**Notes**

storybook-shard-3-28: exit 1, 249.43 seconds. 
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/WorkflowHostSettings.stories.svelte
```

**Notes**

workflow-reload-32: exit 0, 18.68 seconds. 
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
pnpm spec:links
```

**Notes**

spec-links-29: exit 0. Book links and anchors valid: 2,909 HTML files and 109,884 local references.
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
pnpm build-storybook
```

**Notes**

storybook-build-28: exit 0, 76.78 seconds. 
<!-- /run -->
<!-- /evidence-field -->

