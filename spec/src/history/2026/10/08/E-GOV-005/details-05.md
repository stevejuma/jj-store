---
schema: 1
id: E-GOV-005
---

# E-GOV-005 details 5

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
node scripts/stacks-browser-acceptance.mjs
```

**Notes**

stacks-browser: exit 1; 140.43s. 2 failed; 1 passed (2.3m).  Raw output retained in ignored .local/test-audit-2026-10-08/stacks-browser.log.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_DEV_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs node scripts/storage-browser-acceptance.mjs
```

**Notes**

storage-browser: exit 1; 51.19s. 1 failed. Failed axe color contrast: .store-repository-picker__value (Storage acceptance), foreground #9a9ea4 on white, 2.69:1 versus required 4.5:1. Teardown also logged five-second graceful-shutdown timeouts; retain these diagnostics. Raw output retained in ignored .local/test-audit-2026-10-08/storage-browser.log.
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
JJ_STORE_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs cargo test -p jj-store-cli --test server_executable --locked real_rustfs_native_git_external_merge_cycle_survives_cold_restart -- --ignored
```

**Notes**

git-bridge-browser: exit 0; 38.46s. 1 passed, 0 failed, 0 ignored (reported Rust test results). Passed the native real-RustFS cold cycle and its mandatory git-bridge.spec.ts Playwright child. Source at server_executable/git_bridge.rs:312-333 checks the browser child exit. The Git browser failure in the generic native suite clears with this required specialized fixture. Raw output retained in ignored .local/test-audit-2026-10-08/git-bridge-browser.log.
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
cargo test -p jj-store-server --lib --locked capture_import_state::tests::import_callback_retains_gate_until_settlement -- --exact
```

**Notes**

capture-settlement-recheck: exit 0; 2.23s. 1 passed, 0 failed, 0 ignored (reported Rust test results). Passed in isolation after Unavailable in the broad server-library run; intermittent, not repaired. Raw output retained in ignored .local/test-audit-2026-10-08/capture-settlement-recheck.log.
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
cargo test -p jj-store-server --lib --locked process::tests::capture_factory::cold_capture_factory_prepares_private_resources_and_retains_ordered_lifetime -- --exact
```

**Notes**

capture-factory-recheck: exit 101; 90.85s. 0 passed, 1 failed, 0 ignored (reported Rust test results). Reproduced the foreground-stop child failure: store.upgrade().is_none() at capture_factory/foreground.rs:66. The child helper is not counted as a second independent failure. Raw output retained in ignored .local/test-audit-2026-10-08/capture-factory-recheck.log.
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
pnpm test:dev:ui
```

**Notes**

dev-ui: exit 0; 16.03s. See retained log and execution exit status. Passed native writes, SSE, Svelte/TypeScript/CSS HMR without document reload, backend-only restart retaining Vite, Origin/Host guards and owned process cleanup. Raw output retained in ignored .local/test-audit-2026-10-08/dev-ui.log.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
node .local/test-audit-2026-10-08/local-index-acceptance.mjs
```

**Notes**

local-index-browser: exit 1; 276.73s. 1 failed. Audit-local copy of local-browser-acceptance.mjs retains its isolated fixture and teardown, selects local-index-cancel.spec.ts, and changes output directory. No tracked source/test edits. Raw output retained in ignored .local/test-audit-2026-10-08/local-index-browser.log.
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
JJ_STORE_DEV_PORT=7490 node .local/test-audit-2026-10-08/seeded-task-acceptance.mjs
```

**Notes**

seeded-task-browser: exit 1; 494.3s. See retained log and execution exit status. Audit-local wrapper failed before Playwright: its GET omitted required LocalTrusted admission headers, so work.projects was absent. This is an audit-wrapper error, not a repository assertion failure. Three seeded-task browser tests remain unverified. The isolated service and fixture were cleaned up; no tracked source/test edits. Raw output retained in ignored .local/test-audit-2026-10-08/seeded-task-browser.log.
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
JJ_STORE_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs cargo test -p jj-store-server --test embedded_storage --locked -- --ignored
```

**Notes**

embedded-storage: exit 0; 14.36s. 2 passed, 0 failed, 0 ignored (reported Rust test results).  Raw output retained in ignored .local/test-audit-2026-10-08/embedded-storage.log.
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
pnpm test:local-trusted
```

**Notes**

local-trusted: exit 0; 24.16s. 1 passed, 0 failed, 0 ignored (reported Rust test results).  Raw output retained in ignored .local/test-audit-2026-10-08/local-trusted.log.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

fail
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Repair or reconcile the named failing assertions and rerun their exact
  boundaries; this audit does not implement fixes.
- The capture-settlement Rust failure and workspace-name browser failure were
  intermittent. Initial catalog worker timeout and Storybook import/connection
  errors cleared on controlled reruns; they remain recorded.
- Docker-backed M0 transport/container qualification was blocked by the
  unresponsive Docker daemon. Embedded real-RustFS passes are separate evidence.
- Authenticated live Codex/Cursor dispatch, explicitly prepared
  GGUF/native-model inference, large/Linux metadata probes and idle-host release
  performance remain opt-in gates with missing prerequisites or unsuitable host
  conditions. Subprocess-helper ignore markers are not independent skipped
  acceptance gates.
- The generic real-browser harness skipped 13 cases. Dedicated filesystem,
  Stacks, storage and local-index lanes exercised 10. The audit-local
  seeded-task wrapper omitted required LocalTrusted GET admission headers and
  failed before Playwright; automatic-updates.spec.ts (2 cases) and
  task-activity-layout.spec.ts remain unverified. This wrapper error is separate
  from repository failures. Tracked tests/harnesses are unchanged; dedicated
  fixture outcomes do not imply release qualification.
- The old staged ACP executable supports only protocols 1/2; the current
  development executable passes protocol 3 acceptance. Staging was not replaced
  in the application.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Darwin arm64
v24.10.0
10.32.1
rustc 1.98.1 (48a229cea 2026-09-01)
cargo 1.98.1 (797e8a9bc 2026-08-05)
jj 0.45.1
git version 2.55.0; Vitest 4.1.11, Playwright 1.63.0. Installed Design Core resolves to an existing file-backed pnpm artifact. Most later lanes use CARGO_BUILD_JOBS=2 and RUST_TEST_THREADS=2; catalog rerun uses maxWorkers=2. The final main Rust invocation and later fixtures use a private shared compiled-rule cache outside the checkout via JJ_STORE_TRANSCRIPT_CACHE.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No production, public schema, stored object, WAL, receipt, configuration or migration format changed. Existing format-2 LocalTrusted fixture and explicit development executables retained; no installed services, credentials or user repositories migrated.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Read-only product/test audit; only generated build outputs, isolated test state, preserved interactive restart and this evidence/Bootstrap maintenance review. Initial unconstrained Rust/browser attempts were interrupted to reduce host contention. A second Rust run was stopped after 201 completed passing tests to enable the supported shared transcript cache; its incomplete outputs remain explicitly unqualified. No baseline updates, package publication, remote push or broad lint/release claim.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Triage the reproduced failures by owning test/product boundary; keep intermittent and missing-fixture outcomes distinct, then rerun the affected lanes after fixes.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: This record observes existing behavior and tests; it changes no
  implementation or owning contract. Failed audit results are retained and do
  not serve as a qualified unchanged-contract refactor confirmation.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: Bootstrap and all product task statuses, historical evidence and
  original gate selections remain unchanged. No task is promoted by this audit,
  and passing subsets do not waive current failures or deferred
  storage/platform/release gates.

```
<!-- /evidence-field -->

