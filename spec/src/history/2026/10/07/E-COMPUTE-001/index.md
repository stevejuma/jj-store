---
schema: 1
id: E-COMPUTE-001
---

# E-COMPUTE-001

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
pnpm check:architecture && pnpm check:ui-ownership && pnpm check:progress
```

**Notes**

After final formatting, Capture story had 404 counted lines (400 limit) and Compute settings had 301 (300 limit). Extracted identical Capture play helper and compute peer controls, preserving JST-UI-012 behavior/labels/disclosure/defaults. Selected cold families and owning type checks rerun; no visual baseline changes.
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
pnpm docs:generate
```

**Notes**

Build failed after cache cleanup raced active compilers and removed dependency graphs. No source/fixture data was affected. Rerun with incremental disabled; never prune active caches.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
python3 scripts/local-compute.acceptance.py --compute runtime/compute/target/debug/jj-store-compute --root .local/compute-acceptance --native-runtime .local/compute-acceptance/native-runtime/0.76.0-rc9/meshllm-native-runtime-darwin-aarch64-metal
```

**Notes**

Configured-model SDK startup remained standby with no loaded runtime model after disabling plugins. Switched to previously working installed-file builder startup and local-required defaults.
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
cargo build --manifest-path runtime/compute/Cargo.toml --offline && python3 scripts/local-compute.acceptance.py --compute runtime/compute/target/debug/jj-store-compute --root .local/compute-acceptance
```

**Notes**

Real runtime prepare, explicit GGUF download, startup, authenticated loopback inference and graceful shutdown passed. Gateway maps logical model identity to native installed weights; 8192-token context/two slots bounded. Later cold bundle startup is recorded separately. Linker compact-unwind warning retained.
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
python3 scripts/local-compute.acceptance.py --compute runtime/compute/target/debug/jj-store-compute --root .local/compute-acceptance --native-runtime .local/compute-acceptance/native-runtime/0.76.0-rc9/meshllm-native-runtime-darwin-aarch64-metal --acp /Users/stevejuma/code/lapismd/jj-acp/target/debug/jj-acp --agent runtime/local-agent/target/debug/jj-store-local-agent --tools runtime/local-agent/target/debug/jj-store-agent-tools
```

**Notes**

Cold real bundled runtime, explicit prepare/download/start, authenticated native inference, real local model ACP permission and exact coding edit, then graceful shutdown passed. The file remained absent until allow-once approval. No fixture model provider; empty runtime cache confirms bundled preparation. Native KV-cache and linker warnings remain recorded.
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
CARGO_TARGET_AARCH64_APPLE_DARWIN_RUNNER='/usr/bin/env HOME=/Users/stevejuma/code/lapismd/jj-store-local-compute/.local/compute-acceptance PATH=/usr/bin:/bin' JJ_STORE_COMPUTE_TEST_ROOT=/Users/stevejuma/code/lapismd/jj-store-local-compute/.local/compute-acceptance MESH_LLM_NATIVE_RUNTIME_CACHE_DIR=/Users/stevejuma/code/lapismd/jj-store-local-compute/.local/compute-acceptance/bundled-runtime-empty-cache MESH_LLM_NATIVE_RUNTIME_BUNDLE_DIR=/Users/stevejuma/code/lapismd/jj-store-local-compute/.local/compute-acceptance/native-runtime/0.76.0-rc9/meshllm-native-runtime-darwin-aarch64-metal cargo test --manifest-path runtime/compute/Cargo.toml --locked native_installed_model_starts_and_serves_without_public_discovery -- --ignored --nocapture
```

**Notes**

Cold bundled native startup/inference/stop passed in 62.27 seconds after installed-file startup, local-only embedded initialization and development SHA-256 optimization. Linker compact-unwind and disabled model KV-cache warnings retained. Empty runtime cache; native model ACP coding remains separately tested.
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
pnpm check:boundaries && node --test scripts/docs.test.mjs
```

**Notes**

17 package boundaries and 8 documentation/reference/staging closure tests passed.
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
python3 scripts/local-compute.acceptance.py --compute runtime/compute/target/debug/jj-store-compute --root .local/compute-acceptance --native-runtime .local/compute-acceptance/native-runtime/0.76.0-rc9/meshllm-native-runtime-darwin-aarch64-metal
```

**Notes**

After removing general initializer, prepare/download passed but embedded model startup returned the bounded compute operation error. Profiling found model and runtime checksum work in unoptimized SHA-256; optimize the helper development hash package and retain diagnostic test evidence.
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
cargo test -p jj-store-server --lib agent_execution --locked
```

**Notes**

26 passed, 1 explicitly ignored private vendor install. Includes exact managed token defaults, environment precedence/privacy, context delivery/restart, fleet stop and existing publication invariants. Retained compact-unwind linker warning.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 16
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

Regenerated existing task histories were missing from refreshed file coverage. Added their generated-only changes to fresh evidence; milestone status unchanged.
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
pnpm check:progress
```

**Notes**

146 tasks / 650 evidence records passed after generated history coverage refresh.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/desktop/target cargo check --manifest-path desktop/Cargo.toml --locked
```

**Notes**

Native macOS source and scoped power command permission compile after wiring generated command permissions.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/agent-settings.test.ts
```

**Notes**

2 credential/durable-settings separation, exact lost-response identity and memory-only draft disposal tests.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/agent-chat.test.ts
```

**Notes**

7 context/chat tests including bounded Unicode file excerpts and exact full native version integers.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentExecution.stories.svelte catalog/components/store-application/AgentFleet.stories.svelte catalog/components/store-application/AgentHarnessModels.stories.svelte catalog/components/store-application/AgentSessionSettings.stories.svelte catalog/components/store-application/AgentSessions.stories.svelte catalog/components/store-application/AgentChatRuntime.stories.svelte catalog/components/store-application/AgentDiscussion.stories.svelte catalog/components/store-application/AgentInboxNavigation.stories.svelte catalog/components/store-application/AgentChatLayout.stories.svelte catalog/components/store-application/AgentChatFeedback.stories.svelte catalog/components/store-application/AgentReplyIndependence.stories.svelte catalog/components/store-application/AgentRevisionIndex.stories.svelte
```

**Notes**

59 passed; 2 harness-model cases exposed discovery errors hiding remembered-model availability guidance. Fixed product notice precedence and explicit asynchronous discovery waits; exact rerun pending. Exact selected file list recovered from the execution record.
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
python3 scripts/local-compute.acceptance.py --compute runtime/compute/target/debug/jj-store-compute --root .local/compute-acceptance
```

**Notes**

Real explicit runtime preparation and GGUF download passed; native model startup returned compute_operation_failed. Exact diagnosis and rerun pending.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: fail
failures: 62
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:check
```

**Notes**

58 stale-generated-contract errors plus 4 fixture payload omissions. Added explicit options/live fields and settings endpoint fixture; regenerate and rerun.
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
CARGO_TARGET_AARCH64_APPLE_DARWIN_RUNNER="/usr/bin/env HOME=/Users/stevejuma/code/lapismd/jj-store-local-compute/.local/compute-acceptance PATH=/usr/bin:/bin" JJ_STORE_COMPUTE_TEST_ROOT=/Users/stevejuma/code/lapismd/jj-store-local-compute/.local/compute-acceptance MESH_LLM_NATIVE_RUNTIME_CACHE_DIR=/Users/stevejuma/code/lapismd/jj-store-local-compute/.local/compute-acceptance/native-runtime cargo test --manifest-path runtime/compute/Cargo.toml --locked native_installed_model_starts_and_serves_without_public_discovery -- --ignored --nocapture
```

**Notes**

Model loaded; fixture client had a 2-second inference timeout. Corrected native-only timeout and cleanup-before-assertion.
<!-- /run -->
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-COMPUTE-001 details 2](details-02.md)
- [E-COMPUTE-001 details 3](details-03.md)
- [E-COMPUTE-001 details 4](details-04.md)
- [E-COMPUTE-001 details 5](details-05.md)
- [E-COMPUTE-001 details 6](details-06.md)
- [E-COMPUTE-001 details 7](details-07.md)
- [E-COMPUTE-001 details 8](details-08.md)
