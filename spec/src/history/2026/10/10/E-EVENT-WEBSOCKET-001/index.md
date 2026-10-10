---
schema: 1
id: E-EVENT-WEBSOCKET-001
---

# E-EVENT-WEBSOCKET-001

<!-- evidence-field:id markdown -->
## Id

E-EVENT-WEBSOCKET-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M24
- M25
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-011
- JST-UI-017
- JST-API-001
- JST-CLI-001
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-10
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- Cargo.lock
- crates/jj-store-http/Cargo.toml
- crates/jj-store-http/src/application_events/subscription.rs
- crates/jj-store-http/src/application_events/tests.rs
- crates/jj-store-http/src/application_events/websocket/tests.rs
- crates/jj-store-http/src/application_events/websocket.rs
- crates/jj-store-http/src/application_events.rs
- crates/jj-store-http/src/contract/application_websocket.rs
- crates/jj-store-http/src/contract/tests.rs
- crates/jj-store-http/src/contract.rs
- crates/jj-store-http/src/lib.rs
- crates/jj-store-server/Cargo.toml
- crates/jj-store-server/src/listener/http/tests.rs
- crates/jj-store-server/src/listener/http.rs
- crates/jj-store-server/src/listener.rs
- crates/jj-store-server/src/process/running.rs
- package.json
- pnpm-lock.yaml
- schemas/openapi.json
- scripts/application-events-browser.mjs
- scripts/contracts.test.mjs
- scripts/dev-ui-config.mjs
- scripts/dev-ui.test.mjs
- spec/src/current/contracts/api/02-read-apis-and-browser-continued-2.md
- spec/src/current/contracts/api/35-scoped-application-freshness.md
- spec/src/current/contracts/cli/06-standalone-process.md
- spec/src/current/contracts/ui/32-event-driven-freshness-migration.md
- spec/src/current/progress/M24/history/M24/index.md
- spec/src/current/progress/M24/inbox-freshness-review.md
- spec/src/current/progress/M25/history/M25/index.md
- spec/src/history/2026/10/10/E-EVENT-WEBSOCKET-001/SUMMARY.nav.md
- spec/src/history/2026/10/10/E-EVENT-WEBSOCKET-001/index.md
- spec/src/history/2026/10/10/SUMMARY.nav.md
- spec/src/history/2026/10/10/index.md
- web/src/lib/api/application-event-hub.ts
- web/src/lib/api/application-events-websocket.ts
- web/src/lib/api/application-events.ts
- web/src/lib/api/generated.ts
- web/src/lib/api/shared-application-events.ts
- web/tests/fixtures/application-events-browser.html
- web/tests/fixtures/application-events-browser.ts
- web/tests/vite/application-events-websocket.test.ts
- web/tests/vite/event-websocket-fixture.ts
- web/tests/vite/shared-application-events.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-http/src/application_events/websocket/tests.rs#websocket_replays_resets_and_does_not_compete_with_http_admission
- crates/jj-store-server/src/listener/http/tests.rs#production_listener_keeps_upgraded_sockets_alive_serves_http2_and_closes_on_shutdown
- web/tests/vite/application-events-websocket.test.ts#unsupported or failed SharedWorker uses WebSocket only, including many independent tabs
<!-- /evidence-field -->

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
cargo check -p jj-store-http
cargo test -p jj-store-http --lib application_events --locked
cargo test -p jj-store-http --locked
```

**Notes**

Check passes. Initial focused events: 8 pass. Final complete HTTP package: 47 unit pass, one existing ignored; 9 asset tests pass. Full package is justified by shared admission middleware and subscription extraction, not repository aggregate qualification. First full run failed a stale route inventory assertion (145; unchanged parent already had 166); corrected exact inventory is 167 total, 82 local, 85 other.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/application-events-websocket.test.ts web/tests/vite/shared-application-events.test.ts web/tests/vite/application-event-hub.test.ts web/tests/vite/application-freshness.test.ts
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/agent-freshness.test.ts web/tests/vite/settings-freshness.test.ts web/tests/vite/projects-freshness.test.ts
```

**Notes**

26 transport/controller tests and 16 direct-consumer tests pass. Coverage includes credential isolation, twenty shared owners, no-worker fallback, no HTTP fallback, heartbeat without reload, invalid frames, abort, silent/stalled sockets, scoped invalidation and cached reads. Vite default-config and Node module.register deprecation diagnostics remain.
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
pnpm web:check
pnpm contracts:generate
pnpm test:contracts
node --test scripts/dev-ui.test.mjs
pnpm check:ui-ownership
```

**Notes**

Web check: zero errors/warnings. Generated OpenAPI and TypeScript updated. Eight contract tests and TypeScript consumer check pass; seven dev-proxy tests pass. Ownership validation passes. Ordinary HTTP contracts remain unchanged except the additive WebSocket endpoint.
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
cargo test -p jj-store-cli --test native_lifecycle application_events_share_committed_observation_keep_checkpoint_narrow_and_reauthorize_delivery --locked
```

**Notes**

One native lifecycle test passes, including shared observation, checkpoint and per-delivery authorization. Uses the same extracted subscription authorization semantics. Linker emits the existing large __eh_frame warning.
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
cargo test -p jj-store-server --lib listener:: --locked
cargo fmt -p jj-store-http -p jj-store-server -- --check
```

**Notes**

Three listener tests pass: retained connection capacity, idle release, and real upgrade echo plus HTTP/1, HTTP/2 and shutdown cleanup. Initial listener test compilation failed because the test-only reqwest build lacked its http2 feature; enabled that feature and reran successfully. Formatting passes.
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
pnpm dev:backend:build
pnpm dev:backend:restart
```

**Notes**

Initial build/restart and listener-repair build/restart run in the default workspace. First live upgrade returned 101 then closed 1006: Tonic 0.14.6 used serve_connection without upgrade support. Replaced only the shared TCP/TLS serving driver with Hyper upgrade support and explicit socket ownership. Second build/restart passes; backend PID 29704 serves 7462, original Vite PID 97556 serves 7460. Fixture preserved, search ready. No reset, installed binary replacement, sample edits or agent prompt. Existing linker warning retained.
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
cargo test -p jj-store-cli --test server_executable cold_tls_ --locked
cargo test -p jj-store-cli --test server_executable cold_server_and_serve_local_hold_state_lock_and_stop_on_termination --locked -- --exact
```

**Notes**

Three isolated cold TLS tests pass: HTTP/1 and HTTP/2 reads, certificate trust/hostname/plaintext refusal with native RPC, configured browser bundle. One cold state-lock and termination test passes. Expanded boundary verification is required because both HTTP and native RPC share the changed listener; Unix RPC serving stays unchanged.
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
node scripts/application-events-browser.mjs
CUA Chrome fixture controls: add owners, add independent sockets, publish fixture event, measure request, release
```

**Notes**

Cold built worker under same-origin CSP: 23 owners in two tabs share eight sockets (one shared, seven independent). Event delivery reaches every owner; probes 2.6–13.2ms with sockets open. Primary release leaves one shared socket for eight owners; final release records opened8/closed8/active0. Fixture deliberately fails any SSE request. Synthetic publications are fixture-owned only.
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
node scripts/application-events-browser.mjs
CUA Chrome /?without-worker: add seven owners, publish fixture event, measure request, release
```

**Notes**

Cold no-worker fixture executes the production fallback: eight owners, eight sockets, sixteen ready/change deliveries; ordinary HTTP probe 5.4ms. Final release opened8/closed8/active0; next probe 5.1ms. Attempted CDP preload override was unsupported, so fallback is explicitly controlled in the test fixture instead of modifying live app startup.
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
CUA fresh Chrome live app; CDP Network.enable and Runtime.evaluate; click TypeScript sample and greeting constant; Workspace > View files
```

**Notes**

Seven additional real /v1/events/ws readers receive authorized ready frames in 267ms and remain open alongside the application feed. Uncached second app instance renders five message articles, Hello from JJ and two animation frames in 607.9ms; history requests pre-send 1.9/1.2ms, server 15.1/5.3ms, total17.5/6.8ms. First View files render 405.8ms; inspect pre-send1.2ms, server333.4ms, total335ms. First chat observer accidentally selected hidden Help h1 and produced no render duration; corrected to match the intended h1 before the reported fresh run. Earlier live timing before the listener repair is excluded from acceptance.
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
cargo test -p jj-store-http --lib application_events --locked
pnpm dev:backend:build
pnpm dev:backend:restart
pnpm web:check
pnpm progress:history
pnpm check:progress
pnpm spec:validate
pnpm exec prettier --check web/src/lib/api/application-events-websocket.ts web/src/lib/api/application-events.ts web/src/lib/api/shared-application-events.ts web/src/lib/api/application-event-hub.ts web/tests/vite/application-events-websocket.test.ts web/tests/vite/event-websocket-fixture.ts web/tests/vite/shared-application-events.test.ts web/tests/fixtures/application-events-browser.ts web/tests/fixtures/application-events-browser.html scripts/application-events-browser.mjs scripts/contracts.test.mjs scripts/dev-ui-config.mjs scripts/dev-ui.test.mjs spec/src/current/contracts/api/02-read-apis-and-browser-continued-2.md spec/src/current/contracts/api/35-scoped-application-freshness.md spec/src/current/contracts/cli/06-standalone-process.md spec/src/current/contracts/ui/32-event-driven-freshness-migration.md spec/src/current/progress/M24/history/M24/index.md spec/src/current/progress/M25/history/M25/index.md spec/src/current/progress/M24/inbox-freshness-review.md spec/src/history/2026/10/10/E-EVENT-WEBSOCKET-001/index.md
pnpm check:boundaries
```

**Notes**

Final review pins event preparation through Ping/Pong rather than canceling an in-flight authorization/replay operation. The exact WebSocket regression now includes Ping/Pong followed by changed delivery. Final event suite: nine pass, including Ping/Pong; backend rebuilt/restarted as PID 38178. Final live reload renders the target chat; authorized ready arrives and an ordinary bootstrap read completes in 20.6ms. Diagnostic readers, timers, observers and temporary tabs are removed; fixture servers stopped; one live app tab retained. No warnings/errors in the inspected live browser log; this does not retire earlier ResizeObserver evidence. Progress checks initially caught an incomplete evidence Files list and test identifiers/result syntax; corrected all three before final revalidation. Generator-only changes to unrelated histories and historical evidence were restored. Spec validates 7 validators, 3449 chapters and 181 requirements. Final progress validation passes (146 tasks, 724 records). Web check remains zero errors/warnings after fixture fallback coverage; formatting and 17-package boundary validation pass. Broader repository aggregate, Rust workspace, catalog/visual and platform lanes are out of scope; presentation components did not change.
<!-- /run -->

<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Separate server feedback work still takes about 2.03 seconds; this transport change does not optimize it.
- Older application assets must reload to use WebSocket; their legacy SSE clients are not retroactively replaced.
- Existing ResizeObserver, independent native/platform/provider gates and unapproved visual baselines retain their status.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; rustc 1.98.1; Jujutsu 0.45.1; Node 26.9.0; pnpm 10.32.1; Chrome 155; Axum 0.8.9; Tonic 0.14.6; tokio-tungstenite 0.29; fixture ws 8.22.0. Enclosing default-workspace change: `kktlomnz`.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive `jj-store-events-v1` WebSocket subprotocol, version-1 bounded hello and `{id,event}` envelopes. Existing version-1 freshness events/cursors and native WAL cursors unchanged. OpenAPI/TypeScript regenerated. No storage or credential format changes.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Replace browser freshness SSE, including direct fallback, with WebSocket while retaining credential-scoped SharedWorker fanout. Preserve authorization, bounded replay/reset, finite lifetime and monitor leases. Keep ordinary requests on HTTP and existing SSE compatibility for old clients. Fix the production listener so upgrade ownership, HTTP/2 RPC, admission and graceful shutdown coexist.

Before: the recorded in-app chat/files delays were 35.434/19.492 seconds, primarily browser pre-send queueing. A controlled six-SSE experiment blocked an ordinary request for three seconds. After: Chrome live reads with seven extra WebSockets have millisecond pre-send waits. These are distinct browser runs and establish removal of the reproduced transport bottleneck, not zero latency or optimization of every backend read.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Profile remaining feedback/target server work separately. Retain M24/M25 independent gates and reload any older client assets. No polling or repository writes are added.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-API-001 adds scoped WebSocket delivery and unchanged HTTP admission outside its exact route; JST-CLI-001 requires upgrade ownership through listener shutdown; JST-UI-011 and JST-UI-017 retain event-only cached reads across shared and fallback transports. Owning API, CLI and UI chapters updated.
```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M24 and M25 remain In progress. Transport and cold listener/browser evidence qualify this slice only; platform/provider, permission and visual baseline gates remain independent.
```
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


