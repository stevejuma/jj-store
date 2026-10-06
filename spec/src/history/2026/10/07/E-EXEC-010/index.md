---
schema: 1
id: E-EXEC-010
---

# E-EXEC-010

<!-- evidence-field:date markdown -->
## Date

2026-10-07
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

- catalog/fixtures/thread-transport.ts
- crates/jj-store-http/src/agent_execution/tests.rs
- crates/jj-store-http/src/local_browser/collaboration.rs
- crates/jj-store-http/src/local_browser.rs
- crates/jj-store-server/src/agent_execution/feedback.rs
- crates/jj-store-service/src/agent_execution.rs
- crates/jj-store-service/src/agent_feedback/tests.rs
- crates/jj-store-service/src/agent_feedback.rs
- crates/jj-store-service/src/collab_reads/agent_feedback.rs
- crates/jj-store-service/src/local_repository/browser_collab.rs
- crates/jj-store-service/src/local_repository.rs
- crates/jj-store-service/tests/local_collab/agent_discussion.rs
- docs/agent-execution.md
- docs/maintenance/coverage.json
- docs/site/guides/agent-execution.md
- docs/site/reference/api/installation.md
- docs/site/reference/api/types.md
- schemas/openapi.json
- scripts/agent-execution-acceptance.py
- scripts/agent-execution-browser-input.mjs
- scripts/agent-execution-browser-ui.mjs
- scripts/agent-execution-browser.mjs
- scripts/agent-execution-discussion-api.mjs
- scripts/agent-execution-discussion.mjs
- spec/src/current/contracts/api/41-interactive-agent-execution.md
- spec/src/current/contracts/collaboration/36-interactive-agent-execution.md
- spec/src/current/contracts/local-jj/07-m15-additive-collaboration-publication.md
- spec/src/current/contracts/ui/34-interactive-agent-sessions.md
- spec/src/current/progress/M23/history/M23/index.md
- spec/src/current/progress/M23/index.md
- spec/src/history/2026/10/07/E-EXEC-010/SUMMARY.nav.md
- spec/src/history/2026/10/07/E-EXEC-010/details-02.md
- spec/src/history/2026/10/07/E-EXEC-010/index.md
- spec/src/history/2026/10/07/E-EXEC-011/SUMMARY.nav.md
- spec/src/history/2026/10/07/E-EXEC-011/index.md
- spec/src/history/2026/10/07/SUMMARY.nav.md
- spec/src/history/2026/10/07/index.md
- spec/src/history/2026/10/SUMMARY.nav.md
- spec/src/history/2026/10/index.md
- web/src/lib/api/generated.ts
- web/src/lib/controller/agent-discussions.svelte.ts
- web/src/lib/controller/local-inbox-threads.ts
- web/src/lib/controller/production-threads.svelte.ts
- web/src/lib/controller/repository-thread-navigation.svelte.ts
- web/src/lib/controller/thread-commands.ts
- web/src/lib/controller/thread-history.ts
- web/src/lib/controller/thread-types.ts
- web/tests/vite/local-inbox-threads.test.ts
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Enclosing Jujutsu change tzmwuwnp. macOS ARM64; JJ 0.45.1, Rust 1.98.1, Node 24.10.0, pnpm 10.32.1, Python 3.14.7, Svelte 5.57.1, Storybook 10.5.7, Vitest 4.1.11. Owned Community presentation revision 741b8285e2081f1c9af58afe5a9b72f538fad781, Apache-2.0; released Design Core ^0.2.0. Cold acceptance uses installed Codex CLI 0.156.1 through the explicit sibling jj-acp debug executable and real embedded RustFS; no other installed runner requalified.
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

[]
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M23 remains Done for its previously qualified execution profile. This
  record additionally qualifies registered-folder inbox feedback and
  next-explicit-Send context; native code writes remain deferred to M15.4.
  E-EXEC-008 retains its original pending result and E-EXEC-009 its scoped
  maintenance outcome. No M13/M15, installed Cursor requalification, visual
  baseline or release/platform completion is claimed.

```
<!-- /evidence-field -->

<!-- evidence-field:id markdown -->
## Id

E-EXEC-010
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-COLLAB-009
- JST-COLLAB-010
- JST-COLLAB-031
- JST-COLLAB-032
- JST-COLLAB-033
- JST-UI-015
- JST-LOCAL-001
- JST-LOCAL-003
- JST-LOCAL-006
- JST-API-004
- JST-API-006
- JST-COLLAB-027
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-service/tests/local_collab/agent_discussion.rs#saved_agent_replies_and_reactions_survive_restart_and_keep_exact_targets
- crates/jj-store-http/src/agent_execution/tests.rs#local_inbox_collaboration_routes_require_local_admin_without_actor_input
<!-- /evidence-field -->

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
Read-only current-chat GET /v1/local/repos/{repo}/collaboration/snapshot
```

**Notes**

Before: 404 unknown_route. Current recorded chat is a registered local Git-backed folder; native-only transport could not persist inbox feedback. No feedback or prompt was submitted to the real chat.
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
cargo check --locked -p jj-store-server
cargo test --locked -p jj-store-service --test local_collab
cargo test --locked -p jj-store-http --lib local_inbox_collaboration_routes_require_local_admin_without_actor_input
```

**Notes**

Affected server check passed, 11 companion-provider tests passed, and the exact HTTP test passed. It rejects all six routes for authenticated/native and LocalTrusted Read callers plus actor injection. Provider regression covers retained discussion targets, reopening, pagination, foreign event cursors and missing selection pins.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check
pnpm catalog:check
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentDiscussion.stories.svelte
```

**Notes**

Initial types exposed the real native/local snapshot union and uint64 exactness, then fixture widening; corrected actual consumers and generated operation input types. Two story assertions failed because readers created during asynchronous preparation retained a stale controller: exact reactions and Check reply status. Fixed reactive profile tracking and adoption of the current prepared reader without relaxing assertions.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentDiscussion.stories.svelte
```

**Notes**

Original regression: all 5 stories pass after the reader-adoption repair, including unknown outcome recovery and exact reaction presentation.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentDiscussion.stories.svelte catalog/components/store-application/AgentExecution.stories.svelte catalog/components/store-threads/StoreThreads.stories.svelte catalog/components/store-threads/StoreCapturedReactions.stories.svelte
```

**Notes**

All 30 stories in four directly affected families pass with interaction/accessibility checks. Visual baselines remain separately gated.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/local-inbox-threads.test.ts web/tests/vite/agent-execution-controller.test.ts web/tests/vite/production-threads.test.ts web/tests/vite/reaction-feedback.test.ts web/tests/vite/reaction-library.test.ts web/tests/vite/repository-thread-navigation.test.ts web/tests/vite/thread-context.test.ts web/tests/vite/thread-indexes.test.ts web/tests/vite/thread-navigation.test.ts web/tests/vite/thread-query-cache.test.ts
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/thread-navigation-integration.test.ts
```

**Notes**

45 tests in 10 files plus 4 native navigation integration tests passed. Earlier focused run also passed 50 tests in 10 files. Local source tests verify exact request/domain bytes, omitted credentials, shared actual companion pin without fabricated native fields, foreign identity refusal and bounded image bytes. A preliminary selection with three nonexistent filters still ran 9 tests in 3 existing files; the explicit complete file list above replaced it.
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
pnpm web:check
pnpm catalog:check
pnpm contracts:check
```

**Notes**

Web and catalog: zero errors/warnings, 272 catalog modules. Generated contracts match; 300 actual HTTP/SSE responses conform, seven schema tests and TypeScript public-contract checks pass.
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
env PATH=/Users/stevejuma/.npm/_npx/bb64e4387f65cc68/node_modules/.bin:$PATH JJ_STORE_AGENT_PRIVACY_CACHE=/private/tmp/jj-store-agent-fixture-cache-20261006 python3 scripts/agent-execution-acceptance.py --server target/debug/jj-store --jj target/debug/jj --rustfs runtime/embedded-rustfs/target/debug/jj-store-rustfs --acp ../jj-acp/target/debug/jj-acp --web web/dist --browser --browser-only
```

**Notes**

Native flow passed; local reply/reaction saves reached the second explicit Send, then Chromium Network.getResponseBody returned No data found. Test now reads durable AgentExecutionOutcome by the exact captured prompt request ID, retaining HTTP acceptance, action/session identity and error checks. Persistence assertions were not weakened.
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
env PATH=/Users/stevejuma/.npm/_npx/bb64e4387f65cc68/node_modules/.bin:$PATH JJ_STORE_AGENT_PRIVACY_CACHE=/private/tmp/jj-store-agent-fixture-cache-20261006 python3 scripts/agent-execution-acceptance.py --server target/debug/jj-store --jj target/debug/jj --rustfs runtime/embedded-rustfs/target/debug/jj-store-rustfs --acp ../jj-acp/target/debug/jj-acp --web web/dist --browser --browser-only
```

**Notes**

After the response-body repair, both profile discussion/context flows passed. Local navigation then failed because the exact timestamp-row locator still used the native fixture agent name. Corrected the selector to the captured profile agent name; changed diagnostic labels to identify local/native proof and added explicit post-restart resume/continuation without prompting for each retained session.
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
env PATH=/Users/stevejuma/.npm/_npx/bb64e4387f65cc68/node_modules/.bin:$PATH JJ_STORE_AGENT_PRIVACY_CACHE=/private/tmp/jj-store-agent-fixture-cache-20261006 python3 scripts/agent-execution-acceptance.py --server target/debug/jj-store --jj target/debug/jj --rustfs runtime/embedded-rustfs/target/debug/jj-store-rustfs --acp ../jj-acp/target/debug/jj-acp --web web/dist --browser --browser-only
```

**Notes**

Final cold run passed against real embedded RustFS and the installed Codex harness, with disposable native WorkStore and registered local Git-backed folders. Both profiles save exact user/assistant replies through ordinary ThreadStore, react/clear with exact targets, retain replies across reload, dispatch no prompt on feedback, include both saved bodies and exact revision IDs on the next explicit Send, and keep the original prepared prompt on exact retry after later feedback. Source dirty bytes are preserved. Navigation, review/history routes, composer drafts, timestamp rows, Online/Working/Online/end/Offline, light/dark accessibility pass. Graceful restart preserves identical feedback/events, setup association and observation times for both; explicit resume/continuation for each sends no new prompt. Managed shutdown releases the native RPC socket. Native code-repository writes still fail with collaboration_replication_unsupported; explicit disposable WorkStore fixture metadata is the supported native profile, not code replication qualification.
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
Current in-app chat DOM inspection after Vite HMR/backend replacement
```

**Notes**

Real session 442e819f72434178bfa58bd56d2a44bd exposes six Add reaction controls and editable reply actions with the registered-folder unavailable explanation removed. It remains ended/disconnected with Published 271/271; no reply/reaction/prompt or lifecycle command was submitted. Legacy Time unknown is retained without fabricated timestamps.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive installation-local feedback profile and six registered-folder collaboration routes, with generated OpenAPI/TypeScript schemas. Actual local companion commits/receipts and device actor remain local. No native identity codec, manifest/ref-CAS publication authority, WAL/receipt/companion envelope, upstream JJ version or stored execution event format changed.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-EXEC-010 details 2](details-02.md)
