---
schema: 1
id: E-FLEET-010
---

# E-FLEET-010 details 2

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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/agent-freshness.test.ts
```

**Notes**

before.log: two regressions failed before implementation; focused.log: both passed after the initial fix.
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
CARGO_BUILD_JOBS=2 cargo test -p jj-store-server --lib agent_execution::freshness --locked
```

**Notes**

rust-focused.log: 1 passed.
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
CARGO_BUILD_JOBS=2 cargo test -p jj-store-http --lib application_events::tests --locked
```

**Notes**

http-events.log: 4 passed; agent producer no-read, replay, idle and Admin boundary included.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/agent-*.test.ts
```

**Notes**

agent-unit.log: 65 passed, old two-second discussion polling expectation failed. Updated to the governed outage delay.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/agent-freshness.test.ts web/tests/vite/agent-discussions.test.ts web/tests/vite/application-freshness.test.ts web/tests/vite/automatic-refresh.test.ts
```

**Notes**

recovery-focused-final.log: 18 passed. Earlier expanded-focused.log had 6 pass/1 fail; recovery-focused.log had 17 pass/1 fail. These exposed successful outage polls resetting backoff and an initial delay regression; final scheduler retains 5/10/20/30 backoff.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit --maxWorkers=2
```

**Notes**

catalog-unit.log: 128 files, 794 passed, 1 existing skip. Escalation: automatic-refresh is shared runtime infrastructure used by independent application consumers.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/agent-freshness.test.ts
```

**Notes**

explicit-retry-before.log: 5 passed/1 failed; explicit failed status had no future retry. Added retry and boolean discussion acceptance.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/agent-*.test.ts
```

**Notes**

agent-unit-final.log: 70 passed in 14 files after explicit retry and discussion recovery.
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
CARGO_BUILD_JOBS=2 cargo test -p jj-store-server --lib agent_execution:: --locked
```

**Notes**

server-agents.log: 29 passed, 1 explicit download/install test ignored.
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
CARGO_BUILD_JOBS=2 cargo test -p jj-store-http --lib agent_execution:: --locked
```

**Notes**

http-agents.log: 7 passed.
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
CARGO_BUILD_JOBS=2 cargo test -p jj-store-service --lib freshness:: --locked
```

**Notes**

service-freshness.log: 2 passed.
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
CARGO_BUILD_JOBS=2 cargo check -p jj-store-server -p jj-store-http -p jj-store-service --all-targets --locked
```

**Notes**

rust-check.log: affected crates/all targets checked.
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
CARGO_BUILD_JOBS=2 pnpm contracts:generate
pnpm docs:generate
CARGO_BUILD_JOBS=2 pnpm contracts:check
```

**Notes**

contracts-check.log: generated files match; 300 actual HTTP/SSE responses conform; 7 contract tests and TypeScript captures pass. Only additive application domain enum entries changed.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentExecution.stories.svelte catalog/components/store-application/AgentFleet.stories.svelte catalog/components/store-application/AgentChatRuntime.stories.svelte catalog/components/store-application/AgentChatLayout.stories.svelte catalog/components/store-application/AgentChatFeedback.stories.svelte catalog/components/store-application/AgentChatChanges.stories.svelte catalog/components/store-application/AgentSessionSettings.stories.svelte
```

**Notes**

stories.log: 38 pass, 1 fail. Navigation/configuration fixture depended on two-second polling and advanced receipts during reads.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatRuntime.stories.svelte
```

**Notes**

runtime-story-final.log: 6 pass with asynchronous receipt settlement and production event delivery; original assertions retained.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatFeedback.stories.svelte catalog/components/store-application/AgentChatLayout.stories.svelte catalog/components/store-application/AgentChatRuntime.stories.svelte catalog/components/store-application/AgentDiscussion.stories.svelte catalog/components/store-application/AgentExecution.stories.svelte catalog/components/store-application/AgentHarnessModels.stories.svelte catalog/components/store-application/AgentInboxNavigation.stories.svelte catalog/components/store-application/AgentReplyIndependence.stories.svelte catalog/components/store-application/AgentRevisionIndex.stories.svelte catalog/components/store-application/AgentSessionSettings.stories.svelte catalog/components/store-application/AgentSessions.stories.svelte catalog/components/store-application/AgentFleet.stories.svelte catalog/components/store-application/AgentChatChanges.stories.svelte
```

**Notes**

stories-final.log: all 13 direct fixture/reader families, 66 interaction/accessibility tests passed in a fresh Vitest browser process. Escalation covers every consumer of the changed fixture. Visual baselines unchanged.
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
CARGO_BUILD_JOBS=2 pnpm exec playwright test --config web/playwright.config.ts agent-fleet.spec.ts agent-chat-changes.spec.ts
```

**Notes**

mocked-browser-final.log: 3 pass including Axe. Initial mocked-browser.log: 2 pass/1 fail at five-second legacy recovery deadline; recovery assertion now permits the governed bounded 5/10-second outage fallback instead of the removed one-second poll.
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
pnpm web:check
pnpm catalog:check
```

**Notes**

Initial checks passed. web-check-final.log exposed one callback Promise&lt;void&gt; type mismatch after boolean feedback acceptance; callback corrected. catalog-check-final.log: 0 errors/warnings, 298 modules.
<!-- /run -->
<!-- /evidence-field -->

