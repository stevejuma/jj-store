---
schema: 1
id: E-PR10-UI-READINESS-001
---

# E-PR10-UI-READINESS-001 details 2

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
pnpm test:storybook --shard=3/3 --reporter=default --reporter=junit --outputFile=.release/ci-results/storybook.xml
```

**Notes**

Hosted PR run 38880500898, job 116910856523, source fe0abc0c930676da9ff5380c314dd333e744deab: 314 passed/1 failed across 70 files. AddAgentDiscoversUnsavedProvidersAndUsesSettingsRows queried all listboxes while the dismissed Harness select still had data-state=closed and data-ending-style; the newly opened LLM provider listbox was also present. JUnit and full job output are retained in the ignored qualification directory. All other standard Rust/UI jobs passed; required correctly failed.
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
pnpm docs:review application "Reviewed mapped browsing, editing, settings and conversion guides after the shared chat Jump to end focus/scroll repair. Browsing now explains draft focus before scrolling, full-draft visibility and immediate reduced-motion navigation. Provider setup, draft retention, command authority, conversion and credential contracts remain unchanged."
```

**Notes**

Reviewed all four mapped guides and updated Browsing; explicit reviewed source fingerprint then allowed the replacement bundled manual build.
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
CI=true JJ_STORE_CI_REPORTS=1 pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentProviderConfiguration.stories.svelte --reporter=default --reporter=junit --outputFile=.local/main-qualification-2026-10-16/provider-before-94.xml
```

**Notes**

provider-before-94: exit 0, 19.57 seconds.
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
pnpm exec prettier --write catalog/components/store-application/AgentProviderConfiguration.stories.svelte
```

**Notes**

provider-format-94: exit 0, 0.51 seconds.
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
CI=true JJ_STORE_CI_REPORTS=1 pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentProviderConfiguration.stories.svelte --reporter=default --reporter=junit --outputFile=.local/main-qualification-2026-10-16/provider-after-94.xml
```

**Notes**

provider-after-94: exit 0, 15.67 seconds.
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
pnpm catalog:check
```

**Notes**

provider-catalog-check-94: exit 0, 21.46 seconds.
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
CI=true JJ_STORE_CI_REPORTS=1 pnpm test:storybook --shard=3/3 --reporter=default --reporter=junit --outputFile=.local/main-qualification-2026-10-16/storybook-ci-shard3-94.xml
```

**Notes**

storybook-ci-shard3-94: exit 1, 339.77 seconds.
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
CI=true JJ_STORE_CI_REPORTS=1 pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatLayout.stories.svelte --reporter=default --reporter=junit --outputFile=.local/main-qualification-2026-10-16/chat-layout-before-95.xml
```

**Notes**

chat-layout-before-95: exit 0, 21.63 seconds.
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
CI=true JJ_STORE_CI_REPORTS=1 pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatLayout.stories.svelte --reporter=default --reporter=junit --outputFile=.local/main-qualification-2026-10-16/chat-focus-regression-before-95.xml
```

**Notes**

chat-focus-regression-before-95: exit 1, 33.92 seconds.
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
pnpm exec prettier --write catalog/components/store-application/AgentChatLayout.stories.svelte web/src/lib/ui/store-agent-chat-body/ChatNavigation.svelte spec/src/current/contracts/ui/36-commentable-chat-body-prototype.md
```

**Notes**

chat-focus-format-95: exit 0, 0.56 seconds.
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
CI=true JJ_STORE_CI_REPORTS=1 pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatLayout.stories.svelte --reporter=default --reporter=junit --outputFile=.local/main-qualification-2026-10-16/chat-focus-after-95.xml
```

**Notes**

chat-focus-after-95: exit 0, 25.42 seconds.
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
pnpm web:check
```

**Notes**

chat-nav-web-check-95: exit 0, 13.83 seconds.
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
pnpm catalog:check
```

**Notes**

chat-nav-catalog-check-95: exit 0, 20.37 seconds.
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
pnpm check:ui-ownership
```

**Notes**

chat-nav-ownership-95: exit 0, 2.53 seconds.
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
pnpm check:architecture
```

**Notes**

chat-nav-architecture-95: exit 0, 0.34 seconds.
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
pnpm web:test
```

**Notes**

chat-nav-web-tests-95: exit 0, 1.86 seconds.
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
JJ_STORE_WORKING_OBSERVATION_CAPTURE=$PWD/.scratch/qualification-working-tree.json pnpm catalog:test
```

**Notes**

chat-nav-catalog-units-95: exit 0, 17.85 seconds.
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
CI=true JJ_STORE_CI_REPORTS=1 pnpm test:storybook --shard=3/3 --reporter=default --reporter=junit --outputFile=.local/main-qualification-2026-10-16/storybook-ci-shard3-95.xml
```

**Notes**

storybook-ci-shard3-95: exit 0, 353.76 seconds.
<!-- /run -->
<!-- /evidence-field -->

