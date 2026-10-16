---
schema: 1
id: E-MAIN-LOCAL-QUALIFICATION-001
---

# E-MAIN-LOCAL-QUALIFICATION-001 details 27

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
JJ_STORE_QUALIFICATION_REPORT=./storybook-shard-4-67.jsonl pnpm exec vitest run --config catalog/vitest.config.ts --project storybook --shard=4/4 --reporter=default --reporter=./.local/main-qualification-2026-10-16/live-reporter.mjs
```

**Notes**

storybook-shard-4-67: exit 0, 323.93 seconds. 
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
pnpm web:check
```

**Notes**

web-check-67: exit 0, 23.15 seconds. 
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
pnpm catalog:check
```

**Notes**

catalog-check-67: exit 0, 37.01 seconds. 
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
pnpm check:ui-ownership
```

**Notes**

ownership-67: exit 0, 6.07 seconds. 
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
pnpm check:architecture
```

**Notes**

architecture-67: exit 0, 0.78 seconds. 
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/SettingsShell.stories.svelte catalog/components/store-application/ManagedAgentIdentity.stories.svelte
```

**Notes**

lazy-before-87: exit 0, 34.96 seconds. 
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
pnpm exec prettier --write catalog/components/store-application/SettingsShell.stories.svelte catalog/components/store-application/ManagedAgentIdentity.stories.svelte
```

**Notes**

lazy-format-87: exit 0, 0.71 seconds. 
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/SettingsShell.stories.svelte catalog/components/store-application/ManagedAgentIdentity.stories.svelte
```

**Notes**

lazy-after-87: exit 0, 24.89 seconds. 
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
JJ_STORE_QUALIFICATION_REPORT=./storybook-shard-2-67.jsonl pnpm exec vitest run --config catalog/vitest.config.ts --project storybook --shard=2/4 --reporter=default --reporter=./.local/main-qualification-2026-10-16/live-reporter.mjs
```

**Notes**

storybook-shard-2-67: exit 1, 233.07 seconds. 
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
python3 .local/main-qualification-2026-10-16/check-storybook-union.py
```

**Notes**

storybook-coverage-87: exit 1, 0.08 seconds. 
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
pnpm catalog:check
```

**Notes**

catalog-check-67: exit 0, 18.73 seconds. 
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
pnpm exec prettier --write catalog/components/store-application/SettingsShell.stories.svelte catalog/components/store-application/ManagedAgentIdentity.stories.svelte catalog/components/store-application/StoreDirectoryNavigation.stories.svelte
```

**Notes**

lazy-format-87: exit 0, 0.42 seconds. 
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/SettingsShell.stories.svelte catalog/components/store-application/ManagedAgentIdentity.stories.svelte catalog/components/store-application/StoreDirectoryNavigation.stories.svelte
```

**Notes**

lazy-after-87: exit 0, 22.91 seconds. 
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
JJ_STORE_QUALIFICATION_REPORT=./storybook-shard-2-67.jsonl pnpm exec vitest run --config catalog/vitest.config.ts --project storybook --shard=2/4 --reporter=default --reporter=./.local/main-qualification-2026-10-16/live-reporter.mjs
```

**Notes**

storybook-shard-2-67: exit 0, 223.67 seconds. 
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
python3 .local/main-qualification-2026-10-16/check-storybook-union.py
```

**Notes**

storybook-coverage-87: exit 0, 0.06 seconds. 
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
pnpm catalog:check
```

**Notes**

catalog-check-67: exit 0, 21.02 seconds. 
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
pnpm exec prettier --write spec/src/current/progress/M16/index.md spec/src/current/progress/M24/main-local-qualification.md spec/src/current/progress/M30/index.md spec/src/history/2026/10/16/E-MAIN-SYNC-STORIES-001/*.md spec/src/history/2026/10/16/index.md spec/src/history/2026/10/16/SUMMARY.nav.md
```

**Notes**

sync-format-88: exit 0, 0.43 seconds. 
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
pnpm spec:validate
```

**Notes**

sync-spec-validate-88: exit 0, 2.96 seconds. 
<!-- /run -->
<!-- /evidence-field -->

