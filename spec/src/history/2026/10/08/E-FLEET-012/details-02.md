---
schema: 1
id: E-FLEET-012
---

# E-FLEET-012 details 2

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
cargo test -p jj-store-server --lib agent_execution::prompt_context --locked
pnpm exec tsx --test web/tests/agent-execution.test.ts web/tests/agent-inbox-layout.test.ts
```

**Notes**

Before production changes: 2 Rust and 8 TypeScript tests passed.
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
cargo test -p jj-store-server --lib agent_execution::prompt_context --locked
```

**Notes**

New native Markdown regression failed once against the old text/JSON fence implementation. Subsequent named and full agent module runs passed.
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
cargo check -p jj-store-server -p jj-store-service -p jj-store-jj -p jj-store-cli --locked
```

**Notes**

Affected packages pass. Intermediate compile failures exposed helper re-export visibility, then corrected it. No protocol schema change.
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
cargo test -p jj-store-server --lib agent_execution --locked
```

**Notes**

Final module run: 34 passed, 1 intentional private real-install download test ignored. Includes sanitization/markup limits, exact capture, cancellation, queueing, unknown/restart, duplicate started, independent generations and bounded continuation omission. Earlier runs: 33 then 34 passed; history omission rerun remains 34 passed.
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
cargo test -p jj-store-service --test local_collab agent_context --locked
```

**Notes**

2 passed, including newly typed exact reply identities, new/edited/nested replies, overlapping selection, limit enforcement and snapshot races; repeated after typed assertion.
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
cargo test -p jj-store-cli --test agent_execution conversations::repository_free_chat_reuses_isolated_bindings_and_never_replays_after_restart --locked
```

**Notes**

Existing contextual conversation regression passed (1).
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
cargo test -p jj-store-cli --test agent_execution context_delivery --locked
```

**Notes**

First fixture used reserved setup environment and was correctly rejected; fixture now sets trusted Resources instead. Next run exposed the pre-existing global -R/task --repository type collision and panic. Product flag fixed and documented; later runs passed.
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
cargo test -p jj-store-cli --test agent_execution --locked
```

**Notes**

Full relevant fixture binary: 7 passed, 2 real-provider tests intentionally ignored. No real provider invoked. Includes lifecycle/permissions, setups, fleet, tooling, existing conversations and new Markdown/CLI fixture.
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
cargo test -p jj-store-cli --test agent_execution context_delivery --locked
```

**Notes**

Final focused run: 1 passed in 169.67 seconds after legacy section-map simulation. Two recipients, unchanged/origin/attachment deltas, code snapshot stability, sanitized paths, quoted executable containing spaces, local coordination reads/writes/receipts, loaded sessions and fresh bounded continuations.
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
cargo test -p jj-store-cli --test task_commands --test review_commands --test local_work --locked
```

**Notes**

5 passed: distinct checkout/target task arguments, exact review identity/version command requirements and real local project/task/comment/receipt behavior across ordinary JJ backends.
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
pnpm exec tsx --test web/tests/agent-execution.test.ts web/tests/agent-inbox-layout.test.ts web/tests/agent-message-content.test.ts
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/agent-chat.test.ts
```

**Notes**

12 TypeScript tests and 9 chat tests passed; preserves message timestamps and exact pending request behavior.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentContextDelivery.stories.svelte catalog/components/store-application/AgentPromptMarkup.stories.svelte catalog/components/store-application/AgentReplyIndependence.stories.svelte catalog/components/store-application/LocalCompute.stories.svelte
```

**Notes**

Initial run: 3 failures. Fixed missing rich legacy data detection. Synthetic userEvent keyboard did not execute native summary default; native keyboard covered by Playwright, story uses pointer. Existing OpenCode clipboard timing failed once and passed unchanged on rerun.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentContextDelivery.stories.svelte catalog/components/store-application/AgentPromptMarkup.stories.svelte catalog/components/store-application/AgentReplyIndependence.stories.svelte catalog/components/store-application/LocalCompute.stories.svelte
```

**Notes**

26 passed in cold browser workers with accessibility checks. One disclosure, queued-to-sent open-state retention, native Markdown, inert embedded HTML/literal XML, raw evidence, empty omission and legacy/recorded surfaces.
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
pnpm web:check
pnpm catalog:check
pnpm check:ui-ownership
```

**Notes**

Final UI/catalog checks: 0 errors/warnings; catalog covers 303 modules. Ownership passed. Initial new story getByRole exact options were invalid and removed; unused CSS removed. Community then Design Core reference sources inspected; no new cross-checkout dependencies.
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
pnpm check:architecture
```

**Notes**

Unchanged pre-existing scripts/dev-ui.acceptance.mjs: 311 counted lines exceeds 300. Same failure recorded in E-FLEET-011 / E-COMPUTE-006; file remains identical to parent. No new line-limit violation.
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
pnpm docs:generate
pnpm docs:check
pnpm web:build
```

**Notes**

Intermediate generation required help text for target_repository; fixed Rust argument docs. Builds required refreshed local/services/configuration/application/cli/contracts source reviews. Vite built successfully on those runs, but documentation staging did not pass until reviews completed.
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
pnpm docs:generate
pnpm web:build
```

**Notes**

Generated CLI pages, built Vite UI and staged strict manual: 9 Mermaid diagrams and 141 files with valid links/anchors. Existing chunk-size/dynamic-import and Darwin linker warnings retained.
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
pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/agent-tool-rich.spec.ts
```

**Notes**

Initial browser webServer startup exceeded 30 seconds compiling the Rust preview while other builds contended. Prebuilt the unchanged preview example before retry.
<!-- /run -->
<!-- /evidence-field -->

