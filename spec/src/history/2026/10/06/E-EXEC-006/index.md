---
schema: 1
id: E-EXEC-006
---

# E-EXEC-006

<!-- evidence-field:date markdown -->
## Date

2026-10-06
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M23
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/AgentChatFeedback.stories.svelte
- catalog/components/store-application/AgentChatRuntime.stories.svelte
- catalog/fixtures/AgentExecutionFixture.svelte
- catalog/fixtures/agent-execution-events.ts
- catalog/fixtures/agent-execution-session.ts
- crates/jj-store-cli/tests/agent_execution/setups.rs
- crates/jj-store-server/src/agent_execution/journal.rs
- crates/jj-store-server/src/agent_execution/mod.rs
- crates/jj-store-server/src/agent_execution/publication.rs
- crates/jj-store-service/src/agent_execution.rs
- docs/agent-execution.md
- docs/maintenance/coverage.json
- docs/site/guides/agent-execution.md
- docs/site/reference/api/types.md
- docs/site/technical/architecture.md
- docs/site/technical/lifecycle.md
- docs/ui-provenance.json
- schemas/openapi.json
- scripts/agent-execution-acceptance.py
- scripts/agent-execution-browser.mjs
- scripts/agent-execution-chat-feedback.mjs
- spec/src/current/contracts/api/41-interactive-agent-execution.md
- spec/src/current/contracts/collaboration/36-interactive-agent-execution.md
- spec/src/current/contracts/ui/34-interactive-agent-sessions.md
- spec/src/current/progress/M23/history/M23/index.md
- spec/src/current/progress/M23/index.md
- web/src/lib/api/generated.ts
- web/src/lib/ui/community-agent-activity/CommunityAgentTranscriptMessage.svelte
- web/src/lib/ui/community-agent-composer-activity/CommunityAgentComposerActivity.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentConversation.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentExecutionHost.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentRequestRecovery.svelte
- web/src/lib/ui/store-agent-execution/StoreAgentTranscript.svelte
- web/src/lib/ui/store-agent-execution/presentation.ts
- web/tests/agent-execution.test.ts
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Enclosing Jujutsu change qukluvnk. macOS ARM64; JJ 0.45.1, Rust 1.98.1, Node 24.10.0, pnpm 10.32.1, Svelte 5.57.1, Storybook 10.5.7, Vitest 4.1.11. Owned Community presentation pinned to 741b8285e2081f1c9af58afe5a9b72f538fad781, Apache-2.0. Released Design Core ^0.2.0.
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

[]
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M23 remains Done for the previously qualified local macOS profile. This
  maintenance slice preserves the cold lifecycle gates and adds truthful chat
  timestamps and Community feedback; no wider release, additional
  runner/platform or visual baseline qualification is claimed.

```
<!-- /evidence-field -->

<!-- evidence-field:id markdown -->
## Id

E-EXEC-006
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-015
- JST-COLLAB-031
- JST-COLLAB-032
- JST-COLLAB-033
- JST-API-006
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/agent-execution.test.ts#message times retain first observation
  across chunks and never fabricate legacy times
- crates/jj-store-server/src/agent_execution/journal.rs#journal_restart_retains_agent_association_but_never_connection
- crates/jj-store-cli/tests/agent_execution/setups.rs#saved_setup_revision_is_independent_of_session_activity
- web/tests/collaboration-presentation.test.ts#agent messages use captured times
  without inferring from sequence or session
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatFeedback.stories.svelte
```

**Notes**

Before implementation: 3 stories failed. Completion also needed a real record in its fixture; the extracted event helper now supplies a completed turn plus separately timed user/coalesced assistant and untimed legacy message. No assertion was weakened. The narrow after run passed all 4 stories.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatFeedback.stories.svelte
```

**Notes**

After: 4 interaction/accessibility stories passed. Completion is a system message inside the log; connected Working uses streaming Reasoning and a compact avatar strip; pending uncertainty retains its exact recovery action/draft; timestamp metadata uses first observation and shows Time unknown without time elements for legacy messages.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatFeedback.stories.svelte catalog/components/store-application/AgentChatLayout.stories.svelte catalog/components/store-application/AgentChatRuntime.stories.svelte catalog/components/store-application/AgentExecution.stories.svelte catalog/components/store-application/AgentHarnessModels.stories.svelte catalog/components/store-application/AgentLayout.stories.svelte catalog/components/store-application/AgentRevisionIndex.stories.svelte catalog/components/store-application/AgentSessionSettings.stories.svelte catalog/components/store-application/AgentSessions.stories.svelte
```

**Notes**

All 53 stories in the 9 direct consumers passed, including composer growth/scroll recovery, exact permission actions, retained pending outcomes, runtime readiness and route-scoped Working. Visual-pending is unchanged.
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
pnpm exec tsx --test web/tests/agent-execution.test.ts
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/agent-execution-controller.test.ts web/tests/vite/agent-runtime-preparation.test.ts
pnpm web:check
pnpm catalog:check
```

**Notes**

4 presentation tests and 9 controller/runtime tests passed. Web/catalog reported zero errors/warnings; catalog boundaries passed. Earlier catalog checks found 3, then 1 missing nullable timestamp fixture fields; these were corrected without weakening the generated DTO.
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
cargo check -p jj-store-service -p jj-store-server -p jj-store-http --locked
cargo test -p jj-store-server --lib agent_execution::journal::tests::journal_restart_retains_agent_association_but_never_connection --locked
JJ_STORE_AGENT_PRIVACY_CACHE=/private/tmp/jj-store-agent-fixture-cache-20261006 cargo test -p jj-store-cli --test agent_execution setups::saved_setup_revision_is_independent_of_session_activity --locked -- --exact
```

**Notes**

Affected checks and both exact regressions passed. Journal read/write preserves event observation time and legacy None; live append captures real times; status/setup reads do not change them; shutdown/reopen preserves exact event data. Initial journal fixture had last_sequence=0 after adding an event and was repaired to 1. An initial u64/i64 capture mismatch was repaired with checked conversion; compilation-dependent contract/docs/build attempts were rerun. An early test compared mutable publication flags before shutdown; it now captures the final shutdown state and verifies the full persisted event. The macOS linker reported a large unwind-section warning; no Rust diagnostic was suppressed.
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
PATH=/Users/stevejuma/.npm/_npx/bb64e4387f65cc68/node_modules/.bin:$PATH JJ_STORE_AGENT_PRIVACY_CACHE=/private/tmp/jj-store-agent-fixture-cache-20261006 python3 scripts/agent-execution-acceptance.py --server target/debug/jj-store --jj target/debug/jj --rustfs runtime/embedded-rustfs/target/debug/jj-store-rustfs --acp ../jj-acp/target/debug/jj-acp --web web/dist --browser --browser-only
```

**Notes**

Cold built production UI with disposable native real-RustFS service and installed Codex passed three times, including the final shared clock-time renderer. New user/assistant timestamps match exact retained events, assistant chunks keep their first time, completion is a SystemMessage, Working appears in log and composer strip, and reload preserves times. Full navigation, presence cycle, light/dark accessibility, end and restart/resume without prompt replay passed. Both services shut down and removed sockets. Repeat was warranted by transient theme capture in the first screenshot; screenshot capture now disables finite transitions. .local/agent-chat-feedback-final.png and .local/agent-chat-working-final.png are observations, not baselines. No Cursor/provider-edit requalification or user-session prompt was performed.
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
pnpm dev:backend:build
PATH=/Users/stevejuma/.npm/_npx/bb64e4387f65cc68/node_modules/.bin:$PATH JJ_STORE_ACP_EXECUTABLE=/Users/stevejuma/code/lapismd/jj-acp/target/debug/jj-acp JJ_STORE_AGENT_JJ_EXECUTABLE=/Users/stevejuma/code/lapismd/jj-store/target/debug/jj JJ_STORE_AGENT_PRIVACY_CACHE=/private/tmp/jj-store-agent-fixture-cache-20261006 pnpm dev:backend:restart
```

**Notes**

Development backend rebuilt/replaced; Vite HMR and existing fixture retained. Read-only verification confirmed the user session retained/published 271 events and ended without replay; legacy events have null timing. New timestamps are active. Prior user session 269/269 was fully published before restart. No reset, installed binary replacement, unrelated daemon restart or external publication occurred.
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
pnpm contracts:generate
pnpm contracts:check
```

**Notes**

Generated OpenAPI/TypeScript contracts match; 300 actual HTTP/SSE responses conform, 7 contract tests and TypeScript contract compilation passed. This wider contract lane is required because the public event DTO has multiple HTTP/SSE consumers; unrelated aggregate Rust/UI suites remain out of scope.
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
pnpm exec tsx --test web/tests/agent-execution.test.ts web/tests/collaboration-presentation.test.ts
pnpm web:check
pnpm catalog:check
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatFeedback.stories.svelte
```

**Notes**

Final timestamp comparison found recorded repository agent chats use clock text rather than Community live relative text. Reused their exact CommunityMessageMeta and agentMessageTimestamp instead of maintaining separate formatting. All 11 presentation/time tests and 4 focused feedback stories passed; known metadata text matches the shared formatter, ISO time and exact-date title; legacy Time unknown has no fabricated time element. Web/catalog report zero errors and warnings. All direct live-chat consumers and built cold flow were rerun after this final presentation adjustment. No recorded-chat reactions/actions or storage authority changed.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatLayout.stories.svelte catalog/components/store-application/AgentChatRuntime.stories.svelte catalog/components/store-application/AgentExecution.stories.svelte catalog/components/store-application/AgentHarnessModels.stories.svelte catalog/components/store-application/AgentLayout.stories.svelte catalog/components/store-application/AgentRevisionIndex.stories.svelte catalog/components/store-application/AgentSessionSettings.stories.svelte catalog/components/store-application/AgentSessions.stories.svelte
```

**Notes**

Final shared-metadata change: all 49 direct-consumer stories passed, in addition to the 4 focused feedback stories. The final built cold real-RustFS flow and restart also passed after that change. Latest screenshots show clock timestamps and shared system/Working presentation. Public/Rust contracts were unchanged by the final presentation adjustment and were not needlessly rerun.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

AgentExecutionEvent gains optional persisted observed_at_millis, exposed as nullable read-only uint64. Legacy journal deserialization defaults to None. New captures reuse the existing signed occurred_at_millis observation field with checked conversion; prior captures are immutable. No native identity, WAL, receipt, cursor/source sequence or publication format changes.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Align the inbox with owned Community presentation: shared Message/MessageBubble composition and the existing CommunityMessageMeta/agentMessageTimestamp clock formatting and exact-date tooltip, SystemMessage completion, streaming Reasoning Working inside the chat, and compact composer activity strip. Existing recorded agent chats use the same Design Core primitives; the inbox now uses that same metadata component and formatter, including Time unknown without a fabricated time element for legacy messages. First observed message time survives chunk coalescing, refresh, resume and restart. Older untimed history is never backfilled from latest activity or rendering time. Preserve exact pending request IDs, drafts, permission actions and explicit submission. Submitted chats enter the private execution journal then publish through the ordinary repository AgentStore capture path; saved agents remain installation-owned.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Maintain captured observation times and exact request/draft semantics; visual baselines and broader release/platform qualification remain separately gated.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: Updated owning JST-UI-015, JST-COLLAB-031/032/033 and JST-API-006
  chapters for shared chat feedback, read-only retained event observation times
  and timestamp propagation through existing capture metadata. Future messages
  carry truthful timestamps; old untimed history remains unknown.

```
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


