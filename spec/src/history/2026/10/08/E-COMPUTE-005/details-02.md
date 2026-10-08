---
schema: 1
id: E-COMPUTE-005
---

# E-COMPUTE-005 details 2

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
jj --no-pager st (both workspaces); jj --no-pager log; jj rebase -r tvyovoys -o oklpnryt; jj --no-pager diff --from local-compute@ --to @ --summary; jj --no-pager log -r '(6eca7200 | ecdbd93e | f5fc5696 | 5afbf238) ~ ancestors(@)'
```

**Notes**

Both workspaces clean before integration and retirement. Default workspace efc6d4e3 is empty over feature head 5afbf238ce93fcc0e27c055cdc63c8ff604f9840. Tree comparison and missing-ancestor revset return no output. No Git writes, conflicts or unrelated source changes.
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
pnpm install --frozen-lockfile
```

**Notes**

Main dependencies synchronized from cache. Lockfile unchanged; log .local/main-compute-install.log.
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
```

**Notes**

Main source type check: 0 errors and 0 warnings; log .local/main-compute-web-check.log.
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
pnpm exec tsx --test web/tests/layout-storage.test.ts web/tests/compute-model-catalog.test.ts
```

**Notes**

7 focused unit tests passed on main.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/ComputeModelPicker.stories.svelte catalog/components/store-work/ProjectsAgentActions.stories.svelte catalog/components/store-application/ManagedAgentIdentity.stories.svelte
```

**Notes**

Fresh main Chromium qualification: 12 tests in 3 story families passed. Log .local/main-compute-browser-check.log. Existing full selected feature evidence is retained in E-COMPUTE-004; unrelated lanes were not rerun.
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
pnpm dev:stop (temporary workspace)
```

**Notes**

Owned temporary services stopped; PIDs 9640 and 97934 gone. Preserved temporary .local as main .local/workspace-archives/local-compute-2026-10-08/development-data and bundled runtimes as main .local/dev-resources/local-compute. Latest development jj-store and jj binaries retained in main target/debug; previous main binaries and docs build archived. No fixture reset/reseed or installed CLI change.
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
JJ_STORE_DEV_PORT=7480 JJ_STORE_DEV_BACKEND_PORT=7482 JJ_STORE_DEV_ROOT=/Users/stevejuma/code/lapismd/jj-store/.local/dev-demo JJ_STORE_DEV_SERVER=/Users/stevejuma/code/lapismd/jj-store/target/debug/jj-store JJ_STORE_DEV_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/.local/dev-resources/local-compute/jj-store-rustfs JJ_STORE_ACP_EXECUTABLE=/Users/stevejuma/code/lapismd/jj-store/.local/dev-resources/local-compute/jj-acp JJ_STORE_AGENT_JJ_EXECUTABLE=/Users/stevejuma/code/lapismd/jj-store/target/debug/jj pnpm dev:ui
```

**Notes**

Main preview ready at 7480, backend 7482, existing main fixture retained and Search ready. Owned backend PID 80121 and Vite PID 80122 use main paths. Keep these explicit resource/environment overrides on future restarts.
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
Python urllib GET /v1/local/compute with only Origin and Sec-Fetch-Site headers
```

**Notes**

403: initial probe omitted the required protocol and complete Fetch Metadata headers. Admission stayed unchanged; corrected read-only probe below.
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
Python urllib GET / with JSON/API headers before probing endpoints
```

**Notes**

404 aborted the initial loop before its API probes. Normal document requests and complete-header API probes were separately retried below; no routing/admission change.
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
Python urllib GET /v1/bootstrap, /v1/local/compute, /v1/local/agents/runtimes with Origin=http://127.0.0.1:7480, X-JJ-Store-Protocol=1, Sec-Fetch-Site=same-origin, Sec-Fetch-Mode=cors, Sec-Fetch-Dest=empty and Accept=application/json; normal document GET / and /index.html
```

**Notes**

All five requests returned 200 after retirement. API payloads parsed as JSON with expected bootstrap, ComputeStatus and AgentRuntimesStatus fields. No credentials, configuration mutations, model operations or browser refresh used.
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
lsof -nP -d cwd -Fpcn; ps -axo pid=,command=; lsof -nP -iTCP:7480 -iTCP:7482 -sTCP:LISTEN; ps -p 80121,80122 -o pid=,command=
```

**Notes**

No process CWD/command referred to the temporary workspace before removal. Live preview/backend owned by main; no unrelated services restarted.
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
jj workspace forget local-compute; trash -v /Users/stevejuma/code/lapismd/jj-store-local-compute; jj workspace list; jj --no-pager st
```

**Notes**

Temporary workspace unregistered, then removed recoverably to /Users/stevejuma/.Trash/jj-store-local-compute. Original directory absent; other workspace registrations unchanged. Main clean after removal before this evidence-only record.
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
pnpm progress:history; jj restore -- spec/src/history/2026/10/07/E-COMPUTE-001/index.md spec/src/history/2026/10/07/E-EXEC-022/index.md spec/src/history/2026/10/08/E-COMPUTE-002/index.md spec/src/history/2026/10/08/E-COMPUTE-003/index.md; pnpm exec prettier --write spec/src/current/progress/M25/index.md
```

**Notes**

New M25 evidence/history navigation generated. Unrelated historical evidence navigation restored, preserving all prior records. Only the new evidence, history navigation and M25 ledger changed.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

Evidence test references require registered test identities, not file paths. These executions already have exact commands and results in runs. Removed invalid metadata-only references; no verification or source changed.
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
pnpm spec:validate; pnpm check:progress; pnpm exec prettier --check spec/src/current/progress/M25/index.md
```

**Notes**

Focused evidence/ledger specification, progress-integrity and authored formatting validation. Full root aggregate, builds and unrelated suites remain out of scope for landing an identical already-verified source tree.
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
Python Path assertions for absent temporary checkout and retained main archive/runtime/binary paths
```

**Notes**

Original temporary directory absent. Preserved development data, bundled runtime directory and main development binaries exist.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:validate
```

**Notes**

Expanded evidence split into two additional detail pages after metadata correction; generated navigation still referenced only the initial page. Regenerated navigation before final validation.
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
node --input-type=module (writeEvidence and updateNavigation); jj restore -- spec/src/history/2026/10/07/E-COMPUTE-001/index.md spec/src/history/2026/10/07/E-EXEC-022/index.md spec/src/history/2026/10/08/E-COMPUTE-002/index.md spec/src/history/2026/10/08/E-COMPUTE-003/index.md
```

**Notes**

Fresh navigation covers all new evidence detail pages. Existing historical records restored after generation.
<!-- /run -->
<!-- /evidence-field -->

