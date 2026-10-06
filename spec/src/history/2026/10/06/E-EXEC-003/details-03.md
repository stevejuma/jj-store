# E-EXEC-003 validation

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
cargo check -p jj-store-server -p jj-store-http -p jj-store-service --locked
```

**Notes**

Affected Rust boundary compiled. No unrelated workspace test suite.
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
cargo test -p jj-store-server --lib agent_execution:: --locked
```

**Notes**

3/3: handshake/live-process presence, legacy/restart association, bounded inert avatars and atomic budget refusal.
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
cargo test -p jj-store-http --test assets --locked
```

**Notes**

9/9: admitted preloaded browser/document assets and scoped CSP; image data/HTTPS permits no script/connect widening.
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
cargo test -p jj-store-cli --test agent_execution --locked
```

**Notes**

2/2 mock lifecycle/setup checks; one installed-runner test explicitly ignored. Real installed runners exercised separately below.
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
pnpm exec tsx --test web/tests/agents-route.test.ts web/tests/agent-directory.test.ts web/tests/agent-execution.test.ts web/tests/store-route-codec.test.ts web/tests/agent-harness-catalog.test.ts web/tests/agent-revisions.test.ts
```

**Notes**

24/24 route, presence precedence, remembered harness models and repository-pinned revision choices. .local/agent-alignment-unit-final.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/agent-execution-controller.test.ts web/tests/vite/store-route-history.test.ts web/tests/vite/projects-agents.test.ts web/tests/vite/projects-code-navigation.test.ts web/tests/vite/agent-revision-index.test.ts web/tests/vite/agent-runtime-preparation.test.ts
```

**Notes**

40/40 direct consumers, including exact local/native index preparation pins and cancellation. Stop targets its exact session while retaining another selected chat/draft; history restores global routes. .local/agent-alignment-controller-chat-final.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentExecution.stories.svelte catalog/components/store-application/AgentSessions.stories.svelte catalog/components/store-application/AgentSessionSettings.stories.svelte catalog/components/store-application/AgentHarnessModels.stories.svelte catalog/components/store-application/AgentChatRuntime.stories.svelte catalog/components/store-application/AgentChatLayout.stories.svelte catalog/components/store-application/AgentRevisionIndex.stories.svelte catalog/components/store-application/StoreRepositoryPicker.stories.svelte catalog/components/store-application/StoreRepositoryPickerPending.stories.svelte catalog/components/store-appearance/AppearanceComposer.stories.svelte
```

**Notes**

44/44 production stories (39 agent stories and 5 directly affected shared consumers), including exact permissions, save recovery, drafts, configured-agent/session divergence, unassigned fallback, fixed skeleton/cards, shared editor, exact runtime/cleanup commands, explicit index recovery, remembered model catalogs, failed/retained resume errors in the composer, pending configure receipts, no prompt on uncertainty/navigation, and long-history bottom-composer/keyboard scroll-to-latest recovery. .local/agent-alignment-stories-complete.log.
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
pnpm web:check
pnpm catalog:check
```

**Notes**

Both report zero errors/warnings; catalog boundaries 267 modules. Final web/catalog logs .local/agent-alignment-web-check-complete.log and .local/agent-alignment-catalog-check-complete.log report zero diagnostics.
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
pnpm web:build
```

**Notes**

Cold built manifest/manual generated; retained existing large-chunk warning and inherited global-selector minifier warnings. No visual baseline changes.
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
PATH=/Users/stevejuma/.npm/_npx/bb64e4387f65cc68/node_modules/.bin:$PATH JJ_STORE_AGENT_PRIVACY_CACHE=/private/tmp/jj-store-agent-fixture-cache-20261006 python3 scripts/agent-execution-acceptance.py --server /Users/stevejuma/code/lapismd/jj-store/target/debug/jj-store --jj /Users/stevejuma/code/lapismd/jj-store/target/debug/jj --rustfs /Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs --acp /Users/stevejuma/code/lapismd/jj-acp/target/debug/jj-acp --web /Users/stevejuma/code/lapismd/jj-store/web/dist --browser --browser-only
```

**Notes**

Cold built native real-RustFS service: create/edit/save/upload/harness, avatar Start,
Online → Working → Online → Stop → Offline, empty inbox, active-session activity navigation, legacy activity redirect, populated
conversation, retained chat draft, direct routes, Back/Forward/review, independently
selected legacy recorded history and light/dark axe passed. Runtime sidebar and shared full agent editor light/dark accessibility passed. Session rows show actual retained event time. Cold server restart
retained association/history/timing and explicit resume sent no prompt. Foreground stop
released the RPC socket; disposable state cleaned. The same invocation without
--browser-only previously passed real Codex and Cursor isolated file edits,
source preservation and AgentStore publication before the browser-harness repair.
These are distinct evidence stages, not a claim of one combined passing run. Final browser log: .local/agent-alignment-cold-complete.log.
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
pnpm check:architecture
pnpm check:boundaries
pnpm check:ui-ownership
pnpm format:rust:check
```

**Notes**

1243 source modules; native dependency boundaries, copied-source provenance and
Rust formatting passed. No Rust/UI root aggregate lanes or visual baselines run.
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
pnpm contracts:check
```

**Notes**

Regenerated optional association, connected and timestamp contracts match; 300 actual HTTP/SSE responses conform, 7 contract tests and TypeScript fixtures pass. .local/agent-alignment-contracts-time-final.log.
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
pnpm dev:backend:build
pnpm dev:backend:restart
```

**Notes**

Rebuilt/restarted only the backend with explicit paired jj/jj-acp executables and preserved .local/dev-demo. Vite remains on 7460; subsequent UI changes used HMR. Timestamp build/restart logs are .local/agent-alignment-backend-build-time.log and .local/agent-alignment-backend-restart-time.log.
<!-- /run -->
<!-- /evidence-field -->
