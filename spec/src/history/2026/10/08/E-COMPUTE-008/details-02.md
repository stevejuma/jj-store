---
schema: 1
id: E-COMPUTE-008
---

# E-COMPUTE-008 details 2

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
jj --no-pager st; jj --no-pager workspace list; jj --no-pager log -n 12
```

**Notes**

Main and provider workspaces were clean; main parent was lptotxop and separate chat-body work was preserved.
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
jj rebase -s tnyoxvlv -o lptotxop; jj edit tnyoxvlv
```

**Notes**

Seven files required conflict resolution. Retained the controller-owned CLI-path test and both provider tests, both documentation additions, all evidence references, and combined source fingerprints. No product implementation hunk conflicted. Initial append-only resolution script rejected the documentation label replacement; resolved from original revision contents and removed all remaining marker text before landing.
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
cargo fmt --all; pnpm exec prettier --write <seven resolved source/document paths>
```

**Notes**

Formatted resolved Rust and documentation. An early formatting attempt encountered unresolved coverage JSON after the guarded merge script stopped; final resolution and formatting passed.
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
jj new tnyoxvlv -m "Record provider integration and main preview verification"
```

**Notes**

Advanced the clean default workspace onto the provider change while preserving the newer main parent and the independent chat-body workspace.
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
pnpm dev:backend:build
```

**Notes**

Main jj-store/JJ binaries built in 2m36s. Existing macOS large unwind-table linker warning remains; no build error.
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
cargo build --manifest-path runtime/local-agent/Cargo.toml --locked -p jj-store-local-agent
```

**Notes**

Built the provider-aware local helper in main in 50.51s.
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
stageAgentResources via scripts/agent-runtime-resources.mjs, followed by same-filesystem rename into desktop/resources
```

**Notes**

Staged ignored local-agent/tools/compute companions with a checksum manifest beside the existing main ACP. Existing main binary/web resources retained. No installed executable or running model changed.
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
cargo test -p jj-store-server --lib --locked agent_execution::
```

**Notes**

38 passed, one explicit real npm installer test ignored. Both provider inheritance and reserved coordination-CLI path tests retained and passed; combined context tracking tests passed.
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
pnpm web:check; pnpm catalog:check
```

**Notes**

Zero errors/warnings; catalog boundaries cover 305 modules.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentProviderConfiguration.stories.svelte catalog/components/store-application/AgentContextDelivery.stories.svelte
```

**Notes**

Fresh main process: five interaction/Axe stories across both directly affected families passed. No visual baseline changes.
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
env PATH="$PATH:/Users/stevejuma/.local/bin" pnpm docs:check
```

**Notes**

Strict documentation build passed: 141 files, 17,817,490 bytes; local diagrams, links and anchors valid.
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
Python byte comparison against jj file show -r 7588ab55 for schemas/openapi.json, web/src/lib/api/generated.ts and runtime/local-agent/src/provider_catalog.rs
```

**Notes**

All three qualified provider artifacts are unchanged by integration.
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
env PATH="$PATH:/Users/stevejuma/.local/bin" pnpm dev:ui
```

**Notes**

Main Vite at 7460 and native backend at 7462 ready, search ready, fixture preserved. Repository 1bdb89c409c1b891ac7f180c01a265d5 unchanged. New server PID 71262, UI PID 71263; restart startup 8349ms.
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
CUA cold reload of http://127.0.0.1:7460/#/agents and unsaved Add agent review
```

**Notes**

Existing Nyra agent retained. JJ Agent/Codex/Cursor each have an icon. Eight LLM provider choices plus inherited choice. Anthropic reports authentication required without entering credentials, saving configuration or prompting. Collapsed Advanced has 1px border/14px outer radius and 13px trigger radius with a -3px inset focus outline.
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
env JJ_STORE_DEV_PORT=7466 pnpm dev:stop (agent-providers workspace)
```

**Notes**

Stopped the superseded isolated preview through its owner-checked launcher, preserving its fixture and workspace.
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
pnpm check:ui-ownership; pnpm check:boundaries
```

**Notes**

UI ownership and all 17 package boundaries passed.
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
cargo test -p jj-store-http --lib --locked agent_execution::
```

**Notes**

Eight agent HTTP tests passed, including Admin-only draft catalogs and credential non-echo.
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
pnpm check:progress
```

**Notes**

Evidence initially listed Storybook files as named test references, but this validator accepts function definitions only. Storybook coverage remains recorded by exact run commands; named references now point to the real Rust regression functions.
<!-- /run -->
<!-- /evidence-field -->

