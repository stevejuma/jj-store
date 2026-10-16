---
schema: 1
id: E-MAIN-LOCAL-REPAIRS-001
---

# E-MAIN-LOCAL-REPAIRS-001 details 20

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
CI=true JJ_STORE_CI_PREBUILT=1 JJ_STORE_CI_REPORTS=1 JJ_STORE_DEV_RUSTFS_RUNTIME=$PWD/.local/main-qualification-2026-10-16/desktop-resources/jj-store-rustfs JJ_STORE_DEV_PORT=17460 JJ_STORE_DEV_BACKEND_PORT=17462 JJ_STORE_REAL_TEST=native-repository.spec.ts pnpm test:local-release
```

**Notes**

mutation-receipt-73: exit 0, 353.64 seconds. 
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
JJ_STORE_RUSTFS_RUNTIME=$PWD/.local/main-qualification-2026-10-16/desktop-resources/jj-store-rustfs pnpm dev:cargo -- test --locked -p jj-store --test server_executable -- --ignored --exact git_bridge_acceptance::real_rustfs_native_git_external_merge_cycle_survives_cold_restart
```

**Notes**

git-cycle-current-69: exit 0, 44.88 seconds. 
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
pnpm web:test
```

**Notes**

web-tests-67: exit 0, 3.45 seconds. 
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
JJ_STORE_WORKING_OBSERVATION_CAPTURE=$PWD/.scratch/qualification-working-tree.json pnpm catalog:test
```

**Notes**

catalog-unit-67: exit 0, 18.56 seconds. 
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
JJ_STORE_QUALIFICATION_REPORT=./storybook-shard-1-67.jsonl pnpm exec vitest run --config catalog/vitest.config.ts --project storybook --shard=1/4 --reporter=default --reporter=./.local/main-qualification-2026-10-16/live-reporter.mjs
```

**Notes**

storybook-shard-1-67: exit 0, 227.6 seconds. 
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
JJ_STORE_QUALIFICATION_REPORT=./storybook-shard-2-67.jsonl pnpm exec vitest run --config catalog/vitest.config.ts --project storybook --shard=2/4 --reporter=default --reporter=./.local/main-qualification-2026-10-16/live-reporter.mjs
```

**Notes**

storybook-shard-2-67: exit 0, 255.4 seconds. 
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
JJ_STORE_QUALIFICATION_REPORT=./storybook-shard-3-67.jsonl pnpm exec vitest run --config catalog/vitest.config.ts --project storybook --shard=3/4 --reporter=default --reporter=./.local/main-qualification-2026-10-16/live-reporter.mjs
```

**Notes**

storybook-shard-3-67: exit 0, 243.69 seconds. 
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
JJ_STORE_QUALIFICATION_REPORT=./storybook-shard-4-67.jsonl pnpm exec vitest run --config catalog/vitest.config.ts --project storybook --shard=4/4 --reporter=default --reporter=./.local/main-qualification-2026-10-16/live-reporter.mjs
```

**Notes**

storybook-shard-4-67: exit 1, 258.67 seconds. 
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
pnpm web:check
```

**Notes**

web-check-67: exit 0, 13.7 seconds. 
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
pnpm catalog:check
```

**Notes**

catalog-check-67: exit 0, 20.36 seconds. 
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

ownership-67: exit 0, 3.81 seconds. 
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
pnpm check:architecture
```

**Notes**

architecture-67: exit 0, 0.45 seconds. 
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatLayout.stories.svelte catalog/components/store-workflows/WorkflowYaml.stories.svelte
```

**Notes**

geometry-before-74: exit 0, 35.03 seconds. 
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
pnpm exec prettier --write catalog/components/store-application/AgentChatLayout.stories.svelte catalog/components/store-workflows/WorkflowYaml.stories.svelte
```

**Notes**

geometry-format-74: exit 0, 0.47 seconds. 
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatLayout.stories.svelte catalog/components/store-workflows/WorkflowYaml.stories.svelte
```

**Notes**

geometry-after-74: exit 1, 19.72 seconds. 
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatLayout.stories.svelte
```

**Notes**

geometry-diagnostic-75: exit 1, 3.52 seconds. 
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatLayout.stories.svelte
```

**Notes**

geometry-diagnostic-75: exit 1, 16.69 seconds. 
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
pnpm exec prettier --write catalog/components/store-application/AgentChatLayout.stories.svelte catalog/components/store-workflows/WorkflowYaml.stories.svelte
```

**Notes**

geometry-format-74: exit 0, 0.42 seconds. 
<!-- /run -->
<!-- /evidence-field -->

