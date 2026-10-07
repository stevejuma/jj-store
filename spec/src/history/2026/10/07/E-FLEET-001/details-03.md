---
schema: 1
id: E-FLEET-001
---

# E-FLEET-001 details 3

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
CARGO_TARGET_DIR=../jj-store/target pnpm docs:generate; pnpm docs:review local/services/configuration/cli/http/application/contracts <recorded topic-specific explanations>; CARGO_TARGET_DIR=../jj-store/target pnpm web:build
```

**Notes**

Updated exact primary fleet behavior and retained legacy controls; generated additive API reference; production build and bundled Help closure passed (141 files, valid links/anchors). Source review is documentation evidence only.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_TARGET_DIR=../jj-store/target pnpm exec playwright test --config web/playwright.config.ts agent-fleet.spec.ts (initial cold attempts)
```

**Notes**

Cargo build-lock timeouts ran no tests. First UI attempt omitted the available runner and was blocked by a fixture directory toast; supplied the runner and explicitly dismissed the warning. Subsequent cold typing exposed missing whitespace between encoded recipient and text. Added an explicit closing token delimiter and its regression, rather than weakening the recipient assertion.
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
CARGO_TARGET_DIR=../jj-store/target cargo build --locked -p jj-store-http --example browser_preview; CARGO_TARGET_DIR=../jj-store/target pnpm exec playwright test --config web/playwright.config.ts agent-fleet.spec.ts
```

**Notes**

Two cold production Chromium tests passed: contextual exact autocomplete/Send, retained pins/reopening and Axe; agent Start stays in place with authoritative Idle and New chat remains draft-only. screenshot off; no baseline change.
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
CARGO_TARGET_DIR=../jj-store/target cargo clippy --locked -p jj-store-server -p jj-store-service -p jj-store-http --lib
```

**Notes**

Initial run hit a corrupted zero-space build-script artifact; cleaned the affected generated jj-store-jj cache and reran. Final check completed with existing warnings in jj-store-jj, native_inventory, application_events, feedback and publication worker. No warning was reported in the new fleet/conversation modules. Strict -D warnings lane remains failing as recorded separately.
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
PATH=<existing adapter cache>/node_modules/.bin:$PATH python3 scripts/agent-fleet-acceptance.py --server <development jj-store> --jj <paired jj> --rustfs <existing embedded RustFS> --acp <rebuilt jj-acp> --web <fresh built UI> (initial native/browser run)
```

**Notes**

Native personal home, two registered native materializations, isolated edits/snapshots and canonical publication passed with zero permissions. Cold browser caught the same lost recipient separator; fixed in the owning app token model. An earlier warm attempt lacked the cached adapter in PATH; corrected the test launch environment. Disposable services shut down after failure.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo build --locked -p jj-store-cli --bins; PATH=/Users/stevejuma/.npm/_npx/bb64e4387f65cc68/node_modules/.bin:$PATH CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target python3 scripts/agent-fleet-acceptance.py --server /Users/stevejuma/code/lapismd/jj-store/target/debug/jj-store --jj /Users/stevejuma/code/lapismd/jj-store/target/debug/jj --rustfs /Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs --acp /Users/stevejuma/code/lapismd/jj-acp/target/debug/jj-acp --web /Users/stevejuma/code/lapismd/jj-store-agent-fleet/web/dist (two diagnostic runs)
```

**Notes**

Both native API portions passed with two materializations, isolated edits/snapshots, canonical publication and zero live permissions. Fresh contextual browser Send failed at preparation with no prompt. Safe diagnosis confirmed selected repository operation differed from the older source checkout and the UI omitted its visible revision. Fixed complete immutable code pins while preserving named workspace fences and dirty source files. Temporary safe diagnostics were removed. An initial instrumentation build used an unavailable tracing dependency; removed without adding a dependency.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test --locked -p jj-store-cli --test agent_execution conversations::repository_free_chat_reuses_isolated_bindings_and_never_replays_after_restart -- --exact
```

**Notes**

One passed in 115.72 seconds after extending the regression. A new conversation selects a prior isolated snapshot newer than the source checkout, verifies the exact base revision/selected operation in a different directory, retains dirty source files, then restarts without replay. Earlier complete CLI family: 5 passed/2 installed tests ignored.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test --locked -p jj-store-server --lib agent_execution
```

**Notes**

Final source: 17 passed/1 explicitly installed-tooling test ignored. Reliable Stop under a full queue, generation/receipt recovery and unsent context release covered. The exact context-reservation test also passed after the earlier release fix. HTTP admission remains 5 passing.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/agent-chat.test.ts web/tests/vite/agent-execution-controller.test.ts; pnpm exec tsx --test web/tests/agent-mentions.test.ts
```

**Notes**

10 controller tests and 3 exact mention tests pass. New regressions cover visible code pins, separate project metadata pins and authoritative Idle during workspace-discovery failure/recovery. An initial test response omitted the required JSON content type and failed once; repaired the fixture, preserving production admission.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentFleet.stories.svelte catalog/components/store-application/AgentChatRuntime.stories.svelte catalog/components/store-application/AgentChatFeedback.stories.svelte catalog/components/store-application/ManagedAgentIdentity.stories.svelte catalog/components/store-application/AgentReplyIndependence.stories.svelte catalog/components/store-work/ProjectsAgents.stories.svelte
```

**Notes**

Final 6 direct-consumer Storybook files: 21 passed. Interaction and accessibility only; no visual baseline changes.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target pnpm exec playwright test --config .scratch/agent-fleet-playwright.config.ts agent-fleet.spec.ts
```

**Notes**

Final built UI in a cold fixture host: 2 passed. Exact tags, retained pins/reopening, independent details and Axe; Start warms in place and New chat is draft-only. Temporary ignored configuration starts the prebuilt production host to avoid Cargo startup-lock contention.
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
pnpm web:check; pnpm catalog:check; pnpm check:architecture; pnpm check:ui-ownership; pnpm check:boundaries
```

**Notes**

Types: zero errors/warnings. Architecture: 1298 modules. UI ownership passes. Package boundaries: 17 packages. Formatting initially exposed one extra controller line; the existing newChat forwarding method was consolidated without changing behavior or increasing limits.
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
pnpm docs:review configuration 'Reviewed reusable ACP lifecycle, immutable repository pins and stop/context-reservation fences'; pnpm docs:review application 'Reviewed contextual code pins, independent avatar runtime status and scoped exact agent tags'; CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target pnpm docs:check; CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target pnpm web:build
```

**Notes**

Final source fingerprints, production web build and bundled Help closure pass: 141 files, 9 Mermaid diagrams rendered in both themes, valid links/anchors. Retained existing large-chunk/linker warnings. An unprefixed docs command was stopped and its newly generated target cleaned; reran with shared target. Concurrent documentation builds initially raced their generated output; sequential rebuild passed. No source, user state or installed binaries removed.
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
PATH=/Users/stevejuma/.npm/_npx/bb64e4387f65cc68/node_modules/.bin:$PATH CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target python3 scripts/agent-fleet-acceptance.py --server /Users/stevejuma/code/lapismd/jj-store/target/debug/jj-store --jj /Users/stevejuma/code/lapismd/jj-store/target/debug/jj --rustfs /Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs --acp /Users/stevejuma/code/lapismd/jj-acp/target/debug/jj-acp --web /Users/stevejuma/code/lapismd/jj-store-agent-fleet/web/dist (first final attempt)
```

**Notes**

Native materialization/edit/publication passed; browser correctly failed before Send because the editor-mounted mention avatar showed Unknown rather than authoritative Idle. CodeMirror mounted its Svelte member chip without the application context. Fixed context inheritance in the owned composer widget boundary, without changing the runtime provider or substituting status.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentFleet.stories.svelte; pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentFleet.stories.svelte catalog/components/store-application/AgentChatRuntime.stories.svelte catalog/components/store-application/AgentChatFeedback.stories.svelte catalog/components/store-application/ManagedAgentIdentity.stories.svelte catalog/components/store-application/AgentReplyIndependence.stories.svelte catalog/components/store-work/ProjectsAgents.stories.svelte; pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit
```

**Notes**

Editor-chip status regression: 2 story tests passed, followed by all 21 consumer stories. Final direct-consumer unit suite: 121 files, 754 tests passed/1 skipped. Shared composer widgets justify the consumer unit suite. An initial Storybook type check rejected Playwright-specific exact options; corrected to Testing Library exact name matching. Final web/catalog checks have zero errors/warnings.
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
pnpm docs:review application 'Reviewed editor-mounted agent chips inheriting authoritative app runtime context'; CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target pnpm web:build; CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target pnpm exec playwright test --config .scratch/agent-fleet-playwright.config.ts agent-fleet.spec.ts
```

**Notes**

Final widget source fingerprints, production build and bundled Help closure pass. Cold built fixture browser remains 2 passing; real native browser is separate.
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
PATH=/Users/stevejuma/.npm/_npx/bb64e4387f65cc68/node_modules/.bin:$PATH CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target python3 scripts/agent-fleet-acceptance.py --server /Users/stevejuma/code/lapismd/jj-store/target/debug/jj-store --jj /Users/stevejuma/code/lapismd/jj-store/target/debug/jj --rustfs /Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs --acp /Users/stevejuma/code/lapismd/jj-acp/target/debug/jj-acp --web /Users/stevejuma/code/lapismd/jj-store-agent-fleet/web/dist
```

**Notes**

Final real-RustFS/installed Codex run passed: warm worker without session, native personal WorkStore home, general real turn, two registered native repositories without selected workspaces materialized, isolated edits and independent snapshots, dirty source preservation, canonical history and exact duplicate Send without replay. Cold production browser pinned the visible code operation/revision, sent a tagged fresh conversation, received its canonical reply, retained pins/reopening, showed authoritative Idle in editor avatars and passed Axe with no page errors. Cold host restart retained sessions/history/conversations without replay; foreground shutdown released native RPC socket. Zero live permission requests; fixture evidence does not qualify installed permission/cancellation exchange.
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
pnpm format:rust:check; pnpm spec:validate; pnpm progress:history; pnpm check:progress
```

**Notes**

Rust formatting and 7 spec validators passed: 3138 chapters/181 requirements. Progress: 145 tasks/647 evidence records. Evidence coverage initially lagged generated legacy milestone histories; formatting restored unchanged historical scopes/statuses and final changed-file coverage was refreshed. Root aggregate check/test/build lanes remain out of scope under targeted verification policy.
<!-- /run -->
<!-- /evidence-field -->

