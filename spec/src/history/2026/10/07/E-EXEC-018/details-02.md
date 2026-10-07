---
schema: 1
id: E-EXEC-018
---

# E-EXEC-018 details 2

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
pnpm exec tsx --test web/tests/agent-execution.test.ts
```

**Notes**

Before state: four passed, first available timestamp regression failed. An untimed first assistant chunk permanently hid a later recorded observation. Corrected coalescing without inventing missing times.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 6
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check
pnpm catalog:check
```

**Notes**

Initial composition: web had four diagnostics (closure narrowing and missing required timestamp), catalog added two invalid Testing Library exact options. Kept validation blocking and corrected the source and assertions. A second run isolated Button href misuse; replaced it with a native anchor using the existing public button styling identity.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentSessions.stories.svelte catalog/components/store-application/AgentChatFeedback.stories.svelte catalog/components/store-application/AgentChatLayout.stories.svelte
```

**Notes**

First focused run: 17 passed; hover assertion ran before the existing 180 ms delay; repository control was incorrectly a button; tooltips needed their public role prop. Corrected actual link semantics, tooltip role and waited for visible hover actions.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentSessions.stories.svelte catalog/components/store-application/AgentChatFeedback.stories.svelte catalog/components/store-application/AgentChatLayout.stories.svelte
```

**Notes**

Second run: 18 passed, two tooltip-role assertions failed. Current-UI browser reproduced absent tooltip role although tooltip text rendered. Supplied role="tooltip" through the existing Tooltip.Content public native-attribute API.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentSessions.stories.svelte catalog/components/store-application/AgentChatFeedback.stories.svelte catalog/components/store-application/AgentChatLayout.stories.svelte
```

**Notes**

Third run: all 20 targeted stories passed with accessibility enforcement, including day/turn borders, timestamps, icon navigation, keyboard resize/reset, first-message hover placement and ordinary replies.
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
pnpm check:ui-ownership
```

**Notes**

New session model adapter required an explicit ownership record. Added it as locally authored; copied row/resize behavior separately pins Community 741b8285e2081f1c9af58afe5a9b72f538fad781 and Apache-2.0.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:architecture
```

**Notes**

Across two runs: added stories exceeded the 400 counted-line limit, then the shared fixture exceeded 300. Extracted AgentInboxNavigation stories and moved revision output into the existing fixture output component. Budgets unchanged.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/Agent*.stories.svelte
```

**Notes**

Expanded direct consumers because gallery/inbox/fixture composition is shared. First run: 85 passed, one exact draft-row accessible-name regression. Restored the existing draft name while retaining readable session status in session-row accessible names.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentInboxNavigation.stories.svelte catalog/components/store-application/AgentExecution.stories.svelte
```

**Notes**

17 stories passed, including persistence across directory navigation and draft-row compatibility. Framework then reported initial storage capture; made the intentional owner-lifetime capture explicit with untrack.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/Agent*.stories.svelte
```

**Notes**

Final 86 stories across 15 families pass in a new Vite/browser process with blocking interaction/accessibility checks. Shared session settings, discussion/reply recovery, launcher, harness, permissions and recorded-agent direct consumers pass. Visual baselines remain unchanged.
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
pnpm exec tsx --test web/tests/agent-execution.test.ts web/tests/agent-inbox-layout.test.ts web/tests/agent-directory.test.ts web/tests/agents-route.test.ts
```

**Notes**

12 tests pass: first available timestamps, day/legacy grouping, replay exclusion, independent/versioned layout persistence and storage refusal, presence and exact global routes.
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
node .local/verify-inbox-ui.mjs
```

**Notes**

Fresh current-UI context reproduced missing tooltip role; role fixed through public Tooltip.Content props. This helper sent no execution command or user message.
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
node scripts/agent-inbox-presentation.mjs http://127.0.0.1:7460 442e819f72434178bfa58bd56d2a44bd light
node scripts/agent-inbox-presentation.mjs http://127.0.0.1:7460 442e819f72434178bfa58bd56d2a44bd dark
```

**Notes**

Two fresh isolated browser contexts passed against the current development UI; actual light/dark mode confirmed. Mouse and keyboard resize/reset, reload persistence, mobile hidden handle, repository anchor, avatar presence, date/turn presentation, unclipped first hover toolbar and desktop/mobile axe pass. Zero agent command/launch/setup POSTs and zero page errors. User tab and draft untouched.
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
env JJ_STORE_ACP_EXECUTABLE=/Users/stevejuma/code/lapismd/jj-acp/target/debug/jj-acp JJ_STORE_AGENT_JJ_EXECUTABLE=/Users/stevejuma/code/lapismd/jj-store/target/debug/jj JJ_STORE_AGENT_PRIVACY_CACHE=/private/tmp/jj-store-agent-fixture-cache-20261006 pnpm dev:backend:restart
```

**Notes**

Read-only UI diagnosis found current backend serving without ACP execution environment (503 unavailable). Restarted only owned development backend with explicit existing executable paths. Vite/fixture preserved; runtime ready. No Rust build, installed-service restart or prompt dispatch.
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
pnpm spec:search -- JST-UI-015
```

**Notes**

Installed spec-validator rejects pnpm-forwarded --. Reran pnpm spec:search JST-UI-015 and read the returned owning requirements.
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
pnpm web:check
pnpm catalog:check
pnpm check:architecture
pnpm check:ui-ownership
```

**Notes**

Final web/catalog zero errors and warnings; 274 catalog modules, 1268 production/fixture modules and Community ownership pass.
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
node scripts/docs.mjs review application "Reviewed Community inbox resizing with isolated, versioned device persistence, independent session repository links/presence, shared day and turn dividers, readable legacy timing groups and shadcn icon-action tooltips. The read-only current-UI acceptance covers pointer/keyboard/reload/mobile and light/dark accessibility."
pnpm web:build
```

**Notes**

Application documentation ownership reviewed and production web/docs built; nine diagrams and 141-file closure with valid links/anchors. Existing large chunk warning remains.
<!-- /run -->
<!-- /evidence-field -->

