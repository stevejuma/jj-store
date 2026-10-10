---
schema: 1
id: E-EVENT-CONNECTIONS-002
---

# E-EVENT-CONNECTIONS-002

<!-- evidence-field:id markdown -->
## Id

E-EVENT-CONNECTIONS-002
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

- spec/src/current/progress/M24/inbox-freshness-review.md
- spec/src/current/progress/M24/history/M24/index.md
- spec/src/current/progress/M25/history/M25/index.md
- spec/src/history/2026/10/10/E-EVENT-CONNECTIONS-002/index.md
- spec/src/history/2026/10/10/E-EVENT-CONNECTIONS-002/SUMMARY.nav.md
- spec/src/history/2026/10/10/index.md
- spec/src/history/2026/10/10/SUMMARY.nav.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
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
jj --no-pager st; lsof -nP -iTCP:7460 -iTCP:7462 -sTCP:LISTEN; pnpm dev:ui
```

**Notes**

Default workspace clean at c0a737ce; enclosing evidence change qoplzmtl. Neither development listener was running. dev:ui started backend PID 97555 on 7462 and Vite PID 97556 on 7460, preserving .local/dev-demo. No reset, build, sample edit or agent prompt.
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
CUA Chrome navigation; CDP Network.enable, Runtime.evaluate and Network request/response event collection
```

**Notes**

Chrome 155.0.0.0: fresh app on alternate session, then click TypeScript sample and greeting constant. Click-to-five rendered message articles, Hello from JJ, no loading indicator and two animation frames: 712ms. Resource timing: history after=0 total63.6ms, pre-send18.4ms, response44.8ms; after=128 total13.6ms, pre-send1.2ms, response11.9ms. CDP sendStart1.917/0.505ms starts after its later requestTime, explaining the smaller CDP numbers. Workspace &gt; View files: both paths visible after767.3ms; inspect total645.6ms, pre-send3.5ms, response641.6ms. Feedback response1825.9ms and targets1272.9ms are separate server waits. Five reaction requests had distinct targets/payloads. No browser console warnings/errors in the inspected primary-tab capture.
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
CUA opens five additional current-app Chrome tabs; lsof -nP -iTCP:7460 -sTCP:ESTABLISHED; kill -USR1 97556; Node Runtime.evaluate of owned Vite socket metadata
```

**Notes**

All five additional tabs reached Published7/7. Six app tabs shared exactly one unfinished /v1/events response at remote port65391, owned by Chrome network PID97704. A separate Brave stream at65352 belonged to PID2868 and did not share Chrome connection capacity. Vite HMR upgraded sockets were not counted as HTTP event slots. Sixth-tab uncached chat1638.4ms; history pre-send4.5/1.6ms, response49.1/10.1ms. No dedicated agent activity stream was observed for these current Chrome instances.
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
CUA/CDP bounded connection-pressure experiment using five direct GET /v1/events readers and GET /v1/local/agents/settings
```

**Notes**

Used the application protocol header x-jj-store-protocol:1, credentials omit, redirect error and cache no-store. Together with the one shared stream, five extra readers filled six HTTP/1.1 connections. A Settings request queued until a reader aborted after3000ms. CDP connectStart3000.166ms, sendStart3000.785ms, receiveHeadersStart4591.182ms; total4598.7ms. Thus3.00s is browser wait and1.59s is server response, not a single database delay. After all diagnostic readers were aborted and bodies cancelled, the next Settings request completed24.6ms, sendStart0.909ms, response wait17.45ms. Every reader had a10s watchdog and finally cleanup; no data mutation or recurring polling.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
CUA/CDP first pressure probe without protocol header; browser target discovery attempts
```

**Notes**

Diagnostic setup only: first extra-reader request returned403 because it omitted the required protocol header; no stream remained and the corrected request used normal admission. Target.setAutoAttach is unsupported by the raw CDP surface. chrome://inspect was blocked by browser URL policy, so no alternate route to that page was attempted. Safe server-owned socket metadata and ordinary HTTP page timings supplied the evidence instead. Initial Chrome browser connector was unavailable until Chrome was launched with CUA.
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
pnpm spec:search -- "JST-UI-032"
```

**Notes**

Installed spec-validator rejects the literal -- argument. Canonical owning freshness chapters were found with rg and read directly; no contract was inferred from this failed search.
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
CUA closes five diagnostic tabs, removes primary page instrumentation and disables CDP Network; owned Vite inspector close via process.getBuiltinModule
```

**Notes**

Primary Chrome tab remains on the requested chat with README.md and src/example.ts in View files. Final socket inspection shows only one Chrome application event stream, plus the independent Brave stream. Node inspector closes; development server remains running. No protected headers, credentials or response bodies were retained in evidence.
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
node --version; pnpm --version; jj --version
```

**Notes**

v26.9.0,10.32.1,jj0.45.1. macOS; Chrome navigator reports155.0.0.0.
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
pnpm evidence:new -- --id E-EVENT-CONNECTIONS-002 --task M24 --task M25 --requirement JST-UI-011 --requirement JST-UI-017 --requirement JST-API-001; pnpm progress:history; pnpm check:progress; pnpm spec:validate
```

**Notes**

Evidence created and histories generated. First progress check rejected a nonexistent split evidence page, prose used as test paths, and unrelated generator-only formatting deltas. Evidence metadata corrected, manual browser scenarios remain recorded as runs, and unrelated generated deltas restored. Spec validation independently passed7 validators/3447 chapters/181 requirements.
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
pnpm check:progress; pnpm spec:validate; pnpm exec prettier --check spec/src/current/progress/M24/inbox-freshness-review.md spec/src/current/progress/M24/history/M24/index.md spec/src/current/progress/M25/history/M25/index.md
```

**Notes**

Corrected progress record passes146 tasks/723 evidence records; owning specification validates7 validators/3447 chapters/181 requirements; changed current Markdown formatting passes. No production code changed, so Rust, browser builds/type suites, Storybook and repository aggregates were not rerun.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress (attempted overall pass result)
```

**Notes**

The checker correctly rejects an overall pass while diagnostic failures and independent gaps remain listed. Restored pending as the overall evidence status; the successful fresh-Chrome measurements and controlled proof remain individually recorded. No gap or failure was removed to satisfy the gate.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Earlier in-app browser/background-owner identities remain unproven; fresh
  Chrome does not qualify old clients.
- Feedback and target server response costs remain separately unoptimized.
- Existing ResizeObserver diagnostics, previous inherited web-test failures,
  native/provider/platform and visual gates are unchanged. Rust, UI build/type
  suites and aggregate lanes are out of scope for this evidence-only slice.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Source c0a737ce in default; enclosing evidence change qoplzmtl. Chrome155.0.0.0 on macOS, Node26.9.0, pnpm10.32.1, JJ0.45.1.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No format changes. Existing HTTP/1.1 API admission and version-1 application event stream; existing fixture retained.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Cold development-process and fresh Chrome live verification of the already committed shared event owner. Exact request-start delay is distinct from server time and render time. Controlled pressure proves the transport mechanism; it does not identify every legacy background owner.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Profile feedback/targets separately and verify any remaining older app views after reload; retain independent milestone gates.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: JST-UI-011/017 and JST-API-001 retain scoped shared event ownership and
  normal admission. No production code, persistence, polling, dispatch or API
  contract changes; the existing canonical freshness clauses are exercised.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M24/M25 remain In progress. Fresh live Chrome closes this specific
  connection-sharing observation gap, while old-client identity,
  provider/platform, inherited failures, ResizeObserver and unapproved
  visual-baseline gates remain independent.

```
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


