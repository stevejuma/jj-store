---
schema: 1
id: E-WORKFLOW-LIVE-001
---

# E-WORKFLOW-LIVE-001 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
failures: 0
result: pass
hasNote: true
noteEncoding: markdown

```

```shell
jj --no-pager st; read tracked/parent AGENTS and owning workflow contracts/M28; inspect runtime, TOML and private host JSON
```

**Notes**

Before clean wtltlprp 42f99b95, parent wrpzywwx a9088414. Explicit human request: can we admit the test host please. Server admission empty; run 1ed1a78ecfa7695ed2e6a3af53660a3d pending. Native CLI/definition owner a9d260b7d6a4e958d668b7978606f983 differs from browser principal. Host admits only that owner, live-demo-only label and three demo command names.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
failures: 0
result: pass
hasNote: true
noteEncoding: markdown

```

```shell
pnpm dev:disk
```

**Notes**

51.6 GiB managed storage; root target 31.4 GiB; 192.2 GiB free. No cleanup.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
failures: 0
result: pass
hasNote: true
noteEncoding: markdown

```

```shell
pnpm dev:backend:build > /tmp/jj-store-workflow-live-build.log 2>&1
```

**Notes**

Managed affected-package binaries rebuilt, passes in 12.75 seconds. Existing macOS compact-unwind linker warning remains; prior focused checks/tests are in E-WORKFLOW-MULTISTEP-001, not rerun or claimed as new passes.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
failures: 0
result: pass
hasNote: true
noteEncoding: markdown

```

```shell
Python: backup/hash ui-server.toml privately; prepend workflow_hosts = ["a9d260b7d6a4e958d668b7978606f983"]
pnpm dev:backend:restart > /tmp/jj-store-workflow-live-admit-restart.log 2>&1
```

**Notes**

Native host admission was explicitly authorized. Backup mode 0600 outside checkout; no other setting changed. Backend 24571 -&gt; 64319, embedded RustFS -&gt; 64383; Vite 24572 retained. Native fixture/repository and ACP resource preflight preserved.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
failures: 0
result: pass
hasNote: true
noteEncoding: markdown

```

```shell
JJ_CONFIG=/Users/stevejuma/code/lapismd/jj-store/.local/dev-demo/jj.toml /Users/stevejuma/code/lapismd/jj-store/target/debug/jj-store workflow-host --config /var/folders/_3/k1jv9rtn47s_0k5bm6qn_lk80000gn/T/jj-store-workflow-demo-5fd5_hpn/host.json --check
```

**Notes**

Operator config validates. No secrets, agents, remote network execution or repository-selected executables. Existing Persistence test owner/names cannot match this host.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
failures: 0
result: pass
hasNote: true
noteEncoding: markdown

```

```shell
JJ_CONFIG=/Users/stevejuma/code/lapismd/jj-store/.local/dev-demo/jj.toml /Users/stevejuma/code/lapismd/jj-store/target/debug/jj --ignore-working-copy --no-pager -R /Users/stevejuma/code/lapismd/jj-store/.local/dev-demo/repository workflow --format json runs > /var/folders/_3/k1jv9rtn47s_0k5bm6qn_lk80000gn/T/jj-store-workflow-demo-5fd5_hpn/runs-before.json
```

**Notes**

Exactly two pending runs before execution: new live demo and existing Persistence test 6ec48651e74d9c668259be6ca6b03c45. Disabled manual demo definition retained.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
failures: 0
result: pass
hasNote: true
noteEncoding: markdown

```

```shell
JJ_CONFIG=/Users/stevejuma/code/lapismd/jj-store/.local/dev-demo/jj.toml /Users/stevejuma/code/lapismd/jj-store/target/debug/jj-store workflow-host --config /var/folders/_3/k1jv9rtn47s_0k5bm6qn_lk80000gn/T/jj-store-workflow-demo-5fd5_hpn/host.json --once > /tmp/jj-store-workflow-live-host.log 2>&1
```

**Notes**

Exit 0, one run claimed with existing attempt 9e21a97d8dca7213da29911487374388. prepare/test/summary finish in 10.696 seconds. Private host journal and reporting receipts retained; process exited, no background host installed.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
failures: 0
result: pass
hasNote: true
noteEncoding: markdown

```

```shell
JJ_CONFIG=/Users/stevejuma/code/lapismd/jj-store/.local/dev-demo/jj.toml /Users/stevejuma/code/lapismd/jj-store/target/debug/jj --ignore-working-copy --no-pager -R /Users/stevejuma/code/lapismd/jj-store/.local/dev-demo/repository workflow --format json runs --run 1ed1a78ecfa7695ed2e6a3af53660a3d > /var/folders/_3/k1jv9rtn47s_0k5bm6qn_lk80000gn/T/jj-store-workflow-demo-5fd5_hpn/run-completed.json
JJ_CONFIG=/Users/stevejuma/code/lapismd/jj-store/.local/dev-demo/jj.toml /Users/stevejuma/code/lapismd/jj-store/target/debug/jj --ignore-working-copy --no-pager -R /Users/stevejuma/code/lapismd/jj-store/.local/dev-demo/repository workflow --format json logs 1ed1a78ecfa7695ed2e6a3af53660a3d > /var/folders/_3/k1jv9rtn47s_0k5bm6qn_lk80000gn/T/jj-store-workflow-demo-5fd5_hpn/logs-completed.json
```

**Notes**

Server reports completed at position 124, sequence 17. Logs command emits plain text even with global format json; output retained as logs-completed.txt after inspection.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
failures: 0
result: pass
hasNote: true
noteEncoding: markdown

```

```shell
Python assertions over run-completed.json
```

**Notes**

Pass: one selected owner; exactly ordered prepare running/completed, test running/completed, summary running/completed; contiguous evidence sequence 0..16; ten log records including stderr diagnostic; one demo-result.txt artifact with exact source prefix a3affd34f4b6. Shared previous-step file read and exact pinned source validated by trusted strict scripts.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
failures: 1
result: fail
hasNote: true
noteEncoding: markdown

```

```shell
Read-only rg probe at absent web/src/lib/workflows; Python json.loads(logs-completed.json)
```

**Notes**

Exploratory diagnostics only: absent path from rg, and JSONDecodeError because logs output is plain text. Corrected to existing web/src/lib/ui/store-workflows and retained logs-completed.txt. No product execution, write or history failure.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
failures: 0
result: pass
hasNote: true
noteEncoding: markdown

```

```shell
cua_repl: create background verification tab, inspect/expand Steps and Outputs, close own tab, inspect existing exact run tab
```

**Notes**

Native execution succeeds. Secondary tab showed lag/partial reports; existing user tab had completed state. Console error/warn snapshot empty. Closed extra tab; existing tab inspected without navigation, reload or closure. Expanded Steps/Outputs visibly contain all six state reports, all ten logs and Download demo-result.txt. This is live data inspection, not a visual baseline or multi-tab latency qualification.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
failures: 0
result: pass
hasNote: true
noteEncoding: markdown

```

```shell
Python: remove only exact temporary top-level admission line, preserve other settings
pnpm dev:backend:restart > /tmp/jj-store-workflow-live-restore-restart.log 2>&1
```

**Notes**

Original empty admission and exact config bytes restored, SHA-256 86335ce781f975951b1f5b3da846f3634870d4aec18e4c7be669f2d2d6072c48. Final backend 67794, RustFS 67795, Vite 24572 unchanged. No host process remains.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
failures: 0
result: pass
hasNote: true
noteEncoding: markdown

```

```shell
JJ_CONFIG=/Users/stevejuma/code/lapismd/jj-store/.local/dev-demo/jj.toml /Users/stevejuma/code/lapismd/jj-store/target/debug/jj --ignore-working-copy --no-pager -R /Users/stevejuma/code/lapismd/jj-store/.local/dev-demo/repository workflow --format json runs > /var/folders/_3/k1jv9rtn47s_0k5bm6qn_lk80000gn/T/jj-store-workflow-demo-5fd5_hpn/runs-after-restart.json
JJ_CONFIG=/Users/stevejuma/code/lapismd/jj-store/.local/dev-demo/jj.toml /Users/stevejuma/code/lapismd/jj-store/target/debug/jj --ignore-working-copy --no-pager -R /Users/stevejuma/code/lapismd/jj-store/.local/dev-demo/repository workflow --format json runs --run 1ed1a78ecfa7695ed2e6a3af53660a3d > /var/folders/_3/k1jv9rtn47s_0k5bm6qn_lk80000gn/T/jj-store-workflow-demo-5fd5_hpn/run-after-restart.json
Python exact history/config/runtime assertions
```

**Notes**

Pass: complete run/workflow/evidence page identical before/after backend restart; existing pending Persistence test unchanged; demo disabled; exact original server config restored, admission empty; repository ID and Vite PID retained; host exited. Recovery state kept outside checkout.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
failures: 0
result: pass
hasNote: true
noteEncoding: markdown

```

```shell
cua_repl: existing run details after restart; screenshot steps; scroll Outputs; screenshot logs/artifact
```

**Notes**

App retains completed steps, logs and downloadable artifact. Screenshots /tmp/jj-store-workflow-demo-completed-steps.png and /tmp/jj-store-workflow-demo-completed-logs.png saved and latter visually inspected. User tab remains open at original run URL; no workflow retry/enable/delete mutation.
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
pnpm spec:first; pnpm check:progress
```

**Notes**

First documentation check: fails on invalid prose in tests field and pass-with-gaps. Corrected tests to [] (live assertions are recorded as runs) and classified unrelated Forgejo/provider/visual/multi-tab gates as explicitly out of scope. Requested native live target is complete; M28 remains In progress. No requirement/test weakened.
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
pnpm spec:validate
```

**Notes**

First pass: 7 validators, 3749 chapters, 196 requirements; /tmp/jj-store-workflow-live-spec.log.
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
pnpm exec prettier --check spec/src/current/progress/M28/index.md spec/src/current/verification/workflows.md
```

**Notes**

Pass; generated evidence/navigation are formatter-ignored. /tmp/jj-store-workflow-live-format.log.
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
node --version; pnpm --version; target/debug/jj --version
```

**Notes**

Node 26.9.0 and pnpm 10.32.1 verified; custom jj does not expose --version (exit 2). Read-only installed jj --version and Cargo.lock verify JJ 0.45.1; installed executable untouched.
<!-- /run -->
<!-- /evidence-field -->

