---
schema: 1
id: E-FLEET-009
---

# E-FLEET-009

<!-- evidence-field:id markdown -->
## Id

E-FLEET-009
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M24
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-GOV-007
- JST-GOV-009
- JST-GOV-011
- JST-CLI-004
- JST-COLLAB-031
- JST-COLLAB-034
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-08
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Default bundled agent runtime across development restarts
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change sspomstv above audit commit 6f323d19
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

After the requested preserved dev restart, bootstrap omitted local_agent_execution and GET /v1/local/agents returned 503 unavailable. Server preparation only creates the LocalTrusted Admin supervisor when JJ_STORE_ACP_EXECUTABLE exists; neither development launcher supplied it. Desktop runtime already supplies its optional staged resource. The ignored desktop/resources/jj-acp additionally advertised control 1/2 and rejected 3; the current sibling development binary advertises 1/2/3. The exact regression is availability after startup/restart with no ACP/JJ shell exports, without resetting installation or saved state.
<!-- /evidence-field -->

<!-- evidence-field:findings markdown -->
## Findings

Both development launchers now use one trusted preflight: select bundled desktop/resources/jj-acp by default, preserve explicit overrides, pair jj with the selected server, require regular executable files and inspect bounded --help output for control version 3. The full launcher builds its paired development binaries first so a fresh checkout does not require an existing target/debug/jj. All preflight checks precede stopping an existing service. Full, HMR and backend-only launches supply the selected paths every time. No adapter is launched by preflight and no permission, saved enabled state, prompt or credential policy changes. The ignored staged ACP was refreshed from the already-built sibling binary, retaining the previous binary in the ignored audit directory. This is a local development resource refresh, not a package/distribution release.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- AGENTS.md
- scripts/dev-agent-runtime.mjs
- scripts/dev-agent-runtime.test.mjs
- scripts/dev-demo.mjs
- scripts/dev-ui.mjs
- scripts/dev-ui.test.mjs
- scripts/dev-ui.acceptance.mjs
- spec/src/current/governance/06-verification-scope.md
- spec/src/current/contracts/cli/35-interactive-agent-execution.md
- spec/src/current/progress/M24/index.md
- spec/src/current/progress/M24/history/M24/index.md
- spec/src/history/2026/10/08/E-FLEET-009/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- scripts/dev-agent-runtime.test.mjs#development restart selects bundled ACP
  without inherited shell configuration
- scripts/dev-agent-runtime.test.mjs#explicit trusted resources override stale
  bundled ACP and pair an alternate server
- scripts/dev-agent-runtime.test.mjs#missing or stale bundled ACP fails before a
  service can be replaced
- scripts/dev-agent-runtime.test.mjs#agent preflight refuses symlinks and never
  silently falls back from an explicit selection
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
env -u JJ_STORE_ACP_EXECUTABLE -u JJ_STORE_AGENT_JJ_EXECUTABLE CARGO_BUILD_JOBS=2 RUST_TEST_THREADS=2 JJ_STORE_UI_TEST_FULL_RESTART=1 pnpm test:dev:ui (final launcher order)
```

**Notes**

Full cold acceptance repeated after moving paired development binary compilation before preflight. HMR, backend-only restart, full built-UI restart, capability/status, identity, saved project and owned cleanup all passed. Raw output: .local/agent-startup-2026-10-08/cold-restarts-final.log.
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
pnpm check:progress; pnpm spec:validate; pnpm spec:first
```

**Notes**

After correcting evidence file#test references: progress passed with 146 tasks and 662 evidence records; spec validation passed with 7 validators, 3209 chapters and 181 requirements; spec-first passed for 7 protected files and 2 canonical chapters.
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
pnpm check:progress; pnpm spec:first (initial evidence draft)
```

**Notes**

Evidence test entries initially used test names without file paths; corrected to scripts/dev-agent-runtime.test.mjs#&lt;test name&gt;. This was an evidence-authoring error, not a runtime failure.
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
node --test scripts/dev-agent-runtime.test.mjs scripts/dev-ui.test.mjs
```

**Notes**

11 passed. Includes repeated default resolution, explicit paired overrides, stale/missing/symlink refusals and proof that failed preflight leaves an existing process and runtime marker intact. Raw output: .local/agent-startup-2026-10-08/focused-node.log.
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
node --test scripts/*.test.mjs scripts/spec/*.test.mjs (explicit file inventory excluding separately governed catalog-browser.test.mjs)
```

**Notes**

123 passed, 1 failed. Shared startup infrastructure affects built and HMR consumers, so the full Node script/spec regression inventory was selected before execution under JST-GOV-007. Only migration_preserves_original_evidence_tasks_requirements_and_byte_archives failed, reproducing the unchanged historical task/gate and JST-UI-015 differences already recorded in E-GOV-005. No assertion was weakened. Raw output: .local/agent-startup-2026-10-08/node-suites.log.
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
env -u JJ_STORE_ACP_EXECUTABLE -u JJ_STORE_AGENT_JJ_EXECUTABLE CARGO_BUILD_JOBS=2 RUST_TEST_THREADS=2 JJ_STORE_UI_TEST_FULL_RESTART=1 pnpm test:dev:ui
```

**Notes**

Initial cold fixture passed agent capability/status but timed out on First component-after during the existing Svelte HMR proof. Vite logged the file update; the page retained component-before. The exact whole command was rerun without weakening it. Raw output: .local/agent-startup-2026-10-08/cold-restarts.log.
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
env -u JJ_STORE_ACP_EXECUTABLE -u JJ_STORE_AGENT_JJ_EXECUTABLE CARGO_BUILD_JOBS=2 RUST_TEST_THREADS=2 JJ_STORE_UI_TEST_FULL_RESTART=1 pnpm test:dev:ui
```

**Notes**

Fresh rerun passed: real native writes, SSE, Svelte/TypeScript/CSS HMR without document reload, backend replacement retaining Vite, Origin/Host guards, cleanup, full built-UI restart and retained installation/project identity. Agent bootstrap capability is true and /v1/local/agents responds before and after backend/full restarts. Owned temporary service/fixture cleaned. Initial HMR timing failure remains recorded as intermittent. Raw output: .local/agent-startup-2026-10-08/cold-restarts-recheck.log.
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
Copy the already-built /Users/stevejuma/code/lapismd/jj-acp/target/debug/jj-acp to ignored desktop/resources/jj-acp using a temporary file and rename; retain original in .local/agent-startup-2026-10-08/previous-jj-acp
```

**Notes**

Only local staged ACP changed. Old SHA-256 86f2f016318e40f59e8ffaf0e6f7ed1a539623b89c9c5ee3633326ddfa78aba3; new e04a7f93cba9dbbb4875a0c219c06b688772c756f8cee0b6db729abc795f2b0d. Current helper reports jj-acp 0.1.0 and help lists control 1/2/3. No installed CLI or sibling source changed.
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
env -u JJ_STORE_ACP_EXECUTABLE -u JJ_STORE_AGENT_JJ_EXECUTABLE CARGO_BUILD_JOBS=2 pnpm dev:restart
```

**Notes**

Interactive .local/dev-demo fixture preserved at http://127.0.0.1:7460/. Search ready, restart 14,138 ms, server/storage 327,568 KiB. Subsequent admitted GETs: bootstrap 200 with local_agent_execution=true; /v1/local/agents 200, runtime ready, one saved setup and two sessions retained. Raw output: .local/agent-startup-2026-10-08/dev-restart.log and after-status.json.
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
CUA hidden browser tab: http://127.0.0.1:7460/#/agents; inspect accessibility state after loading
```

**Notes**

Agents directory displays Add agent and the retained Nyra setup; Agent execution unavailable is absent. Runtime is ready. Nyra remains explicitly stopped; its Start is disabled because the codex-acp adapter is not discoverable. Cursor is available. No agent Start, Send or installation action was issued.
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
pnpm exec prettier --write scripts/dev-agent-runtime.mjs scripts/dev-agent-runtime.test.mjs scripts/dev-demo.mjs scripts/dev-ui.mjs scripts/dev-ui.test.mjs scripts/dev-ui.acceptance.mjs AGENTS.md spec/src/current/governance/06-verification-scope.md spec/src/current/contracts/cli/35-interactive-agent-execution.md spec/src/current/progress/M24/index.md
```

**Notes**

Changed authored files formatted; history uses the bounded evidence renderer. Regeneration reintroduced M24 index spacing; the final Prettier check caught it, the index was reformatted and the check passed.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- The existing governance migration assertion still fails; this record does not
  waive that full-script gate or call the repository suite green. The exact
  startup targets pass.
- One initial HMR timing failure cleared in the complete cold rerun; retain it
  as intermittent rather than repaired.
- Nyra uses the Codex ACP adapter. codex-acp is not installed/discoverable in
  this installation, so its provider runtime remains unavailable until
  explicitly installed through Settings. Bundled harness readiness does not
  establish adapter installation/authentication.
- Signed desktop/WKWebView and additional platform/adapter qualification remain
  independent. This fix changes development launchers; bare server operator
  environment and desktop resource contracts remain unchanged.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; Node 24.10.0; pnpm 10.32.1; Rust/Cargo 1.98.1; JJ 0.45.1; Git 2.55.0; jj-acp 0.1.0 with control 3; Playwright 1.63.0.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No native objects, manifest/CAS, WAL, receipts, public API schemas, execution journals or stored setup formats changed. Existing format-2 LocalTrusted fixture and saved Start/Stop state are retained.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Development startup/resource selection, corresponding regressions and canonical evidence only. Full Node suites and cold HMR/backend/full restart acceptance cover the shared launcher consumers. No Rust implementation or UI production change: unrelated Rust/UI suites are out of scope for this slice. The earlier full audit remains E-GOV-005. No visual baseline updates, live prompts, provider authentication, package release or remote push.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Keep bundled control-3 resources staged; install the missing provider adapter explicitly if Nyra should run. Triage the independent governance/HMR audit findings at their owning boundaries.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-GOV-009/JST-CLI-004 development launch selection now defaults to the
  bundled ACP and rejects missing/stale resources before stopping services.
  Owning CLI and governance topics document the change.
  JST-COLLAB-031/JST-COLLAB-034 authorization, saved agent control, explicit
  Send and no replay boundaries remain unchanged.

```
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-FLEET-009 details 2](details-02.md)
