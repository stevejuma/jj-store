---
schema: 1
id: E-COMPUTE-007
---

# E-COMPUTE-007 details 2

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
cargo test -p jj-store-server --lib --locked agent_execution::configuration::tests::provider -- --nocapture
```

**Notes**

Before implementation: two exact provider regressions failed.
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
cargo test -p jj-store-server --lib --locked agent_execution::configuration::tests::provider -- --nocapture
```

**Notes**

After implementation: both exact regressions passed.
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
cargo test -p jj-store-server --lib --locked agent_execution::
```

**Notes**

Final: 33 passed, one ignored real npm installation out of scope.
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
cargo test -p jj-store-http --lib --locked agent_execution::
```

**Notes**

Eight passed, including local Admin preview admission and no secret echo.
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
cargo test --manifest-path runtime/local-agent/Cargo.toml --locked -p jj-store-local-agent
```

**Notes**

First integration build had a borrowed string thread-lifetime error; changed fixture response type to static.
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
cargo test --manifest-path runtime/local-agent/Cargo.toml --locked -p jj-store-local-agent
```

**Notes**

Two unit tests and four subprocess HTTP fixture tests passed. Anthropic pagination, native OpenAI/compatible, OpenRouter tools filter, Databricks/v2 honest empty responses and authentication/malformed results.
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
cargo test -p jj-store-cli --test agent_execution --locked provider_preview_is_ephemeral_credential_scoped_and_reaped
```

**Notes**

One real Supervisor/fixed-helper test passed: transient credentials, redaction, credential rotation cache isolation, unchanged journal/no sessions or workspaces, bounded shutdown.
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
cargo test -p jj-store-cli --test agent_execution --locked model_discovery_before_any_agent_or_chat_is_cached_and_never_prompts
```

**Notes**

Existing Codex catalog regression passed, including process shutdown and no prompt.
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
pnpm contracts:generate
```

**Notes**

Generated OpenAPI/TypeScript draft POST and metadata.
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
pnpm web:check
```

**Notes**

Final zero errors/warnings; initial inline request-body type and ref declaration issues corrected.
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

Final zero errors/warnings, 303 catalog modules. Initial 12 errors corrected: fixture provider metadata, recursive fixture type and invalid role-query options.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentProviderConfiguration.stories.svelte
```

**Notes**

First run hit symlinked dependency import failure, resolved by an ignored APFS copy of existing dependencies. Next run caught uninitialized Button ref; corrected to null.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/Agent{ProviderConfiguration,Defaults,Fleet,Execution,HarnessModels,SessionSettings,MentionSettings,Sessions}.stories.svelte
```

**Notes**

Fresh final process: eight families, 47 interaction/axe stories passed. Intermediate failures: 13, then 9, then 2, then zero; fixed selection catalog flicker, accessible names, old selectors and dropdown transition waits without changing baselines.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/agent-provider-configuration.test.ts web/tests/vite/agent-settings.test.ts web/tests/vite/agent-model-catalog.test.ts
```

**Notes**

Nine unit tests passed.
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
pnpm exec tsx --test web/tests/agent-harness-catalog.test.ts
```

**Notes**

Two harness fallback tests passed.
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
pnpm check:ui-ownership
```

**Notes**

Ownership validator passed.
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

17 package boundaries passed.
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

Only unchanged scripts/dev-ui.acceptance.mjs: 311 counted lines exceeds 300. Parent @- source independently measured 311. Changed modules pass. Owner: development UI acceptance script.
<!-- /run -->
<!-- /evidence-field -->

