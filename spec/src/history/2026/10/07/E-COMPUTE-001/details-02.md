---
schema: 1
id: E-COMPUTE-001
---

# E-COMPUTE-001 details 2

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
CARGO_TARGET_AARCH64_APPLE_DARWIN_RUNNER="/usr/bin/env HOME=/Users/stevejuma/code/lapismd/jj-store-local-compute/.local/compute-acceptance PATH=/usr/bin:/bin" JJ_STORE_COMPUTE_TEST_ROOT=/Users/stevejuma/code/lapismd/jj-store-local-compute/.local/compute-acceptance MESH_LLM_NATIVE_RUNTIME_CACHE_DIR=/Users/stevejuma/code/lapismd/jj-store-local-compute/.local/compute-acceptance/native-runtime cargo test --manifest-path runtime/compute/Cargo.toml --locked native_installed_model_starts_and_serves_without_public_discovery -- --ignored --nocapture
```

**Notes**

Real installed Qwen3 GGUF inference passed in isolated HOME. Later logical model identity/context bound changes are covered by the cold helper rerun.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentExecution.stories.svelte catalog/components/store-application/AgentFleet.stories.svelte catalog/components/store-application/AgentHarnessModels.stories.svelte catalog/components/store-application/AgentSessionSettings.stories.svelte catalog/components/store-application/AgentSessions.stories.svelte catalog/components/store-application/AgentChatRuntime.stories.svelte catalog/components/store-application/AgentDiscussion.stories.svelte catalog/components/store-application/AgentInboxNavigation.stories.svelte catalog/components/store-application/AgentChatLayout.stories.svelte catalog/components/store-application/AgentChatFeedback.stories.svelte catalog/components/store-application/AgentReplyIndependence.stories.svelte catalog/components/store-application/AgentRevisionIndex.stories.svelte
```

**Notes**

Cold rerun: 12 families, 61 tests passed after discovery notice fixes.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentHarnessModels.stories.svelte
```

**Notes**

Exact two harness cases passed after remembered-model notice precedence and async observation fixes.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/LocalCompute.stories.svelte
```

**Notes**

One lifecycle case passed; synthetic keyboard events did not trigger browser-native details activation. No production change.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/LocalCompute.stories.svelte
```

**Notes**

Two cases passed: explicit preparation/download/start/stop sequence and captured rich Markdown/code with exact bigint raw context. Native keyboard interaction remains a separate preview check.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 7
hasNote: true
noteEncoding: markdown

```

```shell
cargo test --manifest-path runtime/local-agent/Cargo.toml -p buzz-agent --test permission_boundary --locked
```

**Notes**

Model GET consumed scripted completions in an upstream fixture. Added an independent catalog response; authorization assertions unchanged.
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
cargo test --manifest-path runtime/local-agent/Cargo.toml -p buzz-agent --test permission_boundary --locked
```

**Notes**

9 exact permission tests passed after correcting the provider fixture.
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
cargo test --manifest-path runtime/local-agent/Cargo.toml -p buzz-agent --locked
```

**Notes**

Missing pinned normative test corpus in standalone vendor resources; restored from the same upstream source revision.
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
cargo test --manifest-path runtime/local-agent/Cargo.toml -p buzz-agent --locked
```

**Notes**

541 passed, one byte-exact corpus drift after consumer formatting. Restored upstream bytes and excluded that generated corpus from Prettier.
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
cargo test --manifest-path runtime/local-agent/Cargo.toml -p buzz-agent --lib corpus_matches_generated_snapshot --locked
```

**Notes**

Exact upstream corpus snapshot test passed.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 12
hasNote: true
noteEncoding: markdown

```

```shell
cargo test --manifest-path runtime/local-agent/Cargo.toml -p buzz-agent --locked
```

**Notes**

542 unit tests passed; 12 fake LLM cases assumed session creation never reads a catalog. Scoped discovery to the owned local-agent entrypoint, retaining upstream configured-model behavior.
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
cargo clippy --manifest-path runtime/compute/Cargo.toml --locked -- -D warnings && cargo build --manifest-path runtime/compute/Cargo.toml --locked && python3 scripts/local-compute.acceptance.py --compute runtime/compute/target/debug/jj-store-compute --root .local/compute-acceptance --native-runtime .local/compute-acceptance/native-runtime/0.76.0-rc9/meshllm-native-runtime-darwin-aarch64-metal
```

**Notes**

Clippy/build passed with retained linker warning; prepare/download passed. Obsolete startup attempt cancelled after profiling showed general host initializer in runtime-install path. Removed that initializer; embedded startup validates local artifacts without installation. Acceptance now has an absolute deadline unaffected by progress frames.
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
node --test scripts/governance.test.mjs scripts/spec/vcs-regression.test.mjs
```

**Notes**

30 governance/VCS tests passed; final rerun confirms evidence command provenance.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/agent-settings.test.ts web/tests/vite/agent-chat.test.ts web/tests/vite/agent-model-catalog.test.ts
```

**Notes**

12 focused controller cases passed.
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
cargo test --manifest-path runtime/local-agent/Cargo.toml -p buzz-agent --test fake_llm --locked
```

**Notes**

All 20 exact fake LLM cases passed after scoping discovery.
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
cargo test --manifest-path runtime/local-agent/Cargo.toml -p buzz-agent --locked
```

**Notes**

Complete maintained upstream agent suite passed after restoring standalone resources and scoping model discovery.
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
python3 scripts/local-agent-acp.acceptance.py --acp /Users/stevejuma/code/lapismd/jj-acp/target/debug/jj-acp --agent runtime/local-agent/target/debug/jj-store-local-agent --tools runtime/local-agent/target/debug/jj-store-agent-tools
```

**Notes**

Final rebuilt owned local agent: 2 cold allow/deny subprocess cases passed.
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
python3 scripts/local-compute.acceptance.py --compute runtime/compute/target/debug/jj-store-compute --root .local/compute-acceptance
```

**Notes**

After extensionless HF blob alias and readiness polling fixes, native start still timed out during model loading. Native startup diagnostics retained separately.
<!-- /run -->
<!-- /evidence-field -->

