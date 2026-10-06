---
schema: 1
id: E-UI-089
---

# E-UI-089 details 4

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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit --maxWorkers=1 --no-file-parallelism web/tests/vite/automatic-work-refresh.test.ts (notification order before fix)
```

**Notes**

5pass/2fail for normal save and receipt-status recovery: changed callback observed the old cursor before post-commit reads. Move notification after refresh while retaining exact request identity and independent drafts.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit --maxWorkers=1 --no-file-parallelism web/tests/vite/automatic-work-refresh.test.ts web/tests/vite/local-browser.test.ts web/tests/vite/production-work.test.ts web/tests/vite/automatic-refresh.test.ts; pnpm catalog:test --maxWorkers=1 --no-file-parallelism; pnpm web:check; pnpm catalog:check; pnpm web:build
```

**Notes**

31/31 focused regressions across4files and298/298 complete catalog/controller tests across52files. Both type checks0errors/0warnings and153catalog modules. Latest source notification ordering and concurrent capture completion orders included. Production build passes; unchanged minifier/chunk-size warnings remain. Logs work-order-qualified/catalog-tests-complete-final/web-check-complete-final/catalog-check-complete-final/web-build-work-final.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
Temporary refresh-condition console probe; pnpm web:build; exact local_projects_tasks_comments_match_cli_and_recover_unknown_outcomes browser test against retained server
```

**Notes**

0pass/1fail. No new probe reached browser. Compare served and on-disk index: running server cached pre-build assets. Remove all temporary diagnostic source/test edits, rebuild production UI, SIGTERM only owned isolated server14065 and restart explicit target/debug/jj-store with the same disposable configuration. No assertion or timeout changes.
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
pnpm web:build; target/debug/jj-store run --config <retained isolated fixture>/state/server.toml --format=json; compare served index with web/dist/index.html; JJ_STORE_REAL_BASE_URL=http://127.0.0.1:7478/ JJ_STORE_LOCAL_BROWSER_FIXTURE=<retained isolated fixture>/browser-fixture.json pnpm exec playwright test --config web/playwright.real.config.ts local-browser.spec.ts -g local_projects_tasks_comments_match_cli_and_recover_unknown_outcomes --output .scratch/test-results-local-browser-cold-focused
```

**Notes**

Fresh server serves byte-identical latest built index; exact browser regression1/1 in15.3s. New task appears automatically while dialog remains open; unknown-outcome comment recovers without manual refresh. Logs web-build-cold-final/browser-cold-focused.
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
JJ_STORE_REAL_BASE_URL=http://127.0.0.1:7478/ JJ_STORE_LOCAL_BROWSER_FIXTURE=<retained isolated fixture>/browser-fixture.json pnpm exec playwright test --config web/playwright.real.config.ts local-browser.spec.ts --output .scratch/test-results-local-browser-qualified
```

**Notes**

4pass/1failure in50.0s on the current bundle. Task automatic refresh passes. Review reply appears in Activity but composer reports not confirmed and retains draft. Early ready status clears the global update gate before post-receipt reads finish; automatic refresh invalidates the command completion. Keep this failure explicit and qualify the exact scenario after correction.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit --maxWorkers=1 --no-file-parallelism web/tests/vite/production-store-collaboration.test.ts web/tests/vite/production-automatic-refresh.test.ts (review read race before fix)
```

**Notes**

23pass/4fail: normal and recovered receipts become ready before list reads complete; automatic refresh runs during refreshing/loading review reads. Exact controlled-read/fake-timer regressions reproduce both boundaries.
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
pnpm exec prettier --write web/src/lib/controller/production-review-actions.ts web/src/lib/controller/production-automatic-refresh.ts web/tests/vite/production-store-collaboration.test.ts web/tests/vite/production-automatic-refresh.test.ts spec/src/ui.md; pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit --maxWorkers=1 --no-file-parallelism web/tests/vite/production-store-collaboration.test.ts web/tests/vite/production-automatic-refresh.test.ts
```

**Notes**

27/27 across2files in0.818s. Four before-fix failures resolved without changing request identities, assertions or timeouts. Busy lifecycle spans accepted receipt to final reads; queued updates resume automatically after ready.
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
pnpm web:check; pnpm catalog:check; pnpm web:build (final review correction)
```

**Notes**

Both0errors/0warnings and153catalog modules. Latest production bundle built and isolated server restarted before the exact real-review scenario. Existing CSS minifier/chunk-size diagnostics retained; no new Rust or public schema changes.
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
pnpm catalog:test --maxWorkers=1 --no-file-parallelism (final review race consumers)
```

**Notes**

302/302 across52files in22.39s, including4new production collaboration/automatic-refresh regressions. Wider root aggregate lanes and unrelated Rust workspace lanes remain out of scope.
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
target/debug/jj-store run --config <retained isolated fixture>/state/server.toml --format=json after final build; compare served and built index; JJ_STORE_REAL_BASE_URL=http://127.0.0.1:7478/ JJ_STORE_LOCAL_BROWSER_FIXTURE=<retained isolated fixture>/browser-fixture.json pnpm exec playwright test --config web/playwright.real.config.ts local-browser.spec.ts -g "filesystem reviews create and reply" --output .scratch/test-results-local-browser-review-focused
```

**Notes**

Exact cold real-review regression1/1 in22.7s; confirmed comment renders and clears composer without manual refresh, and remains visible after reload. Native/local pinned historical reads, unknown-request recovery and CLI parity retained.
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
python3 parent-ledger identity/equality audit, dependency/runtime path audit and changed-file prettier check; jj ancestry/status checks in both default workspaces; lsof cwd audit
```

**Notes**

Main262historical entries exact; Appearance290historical entries exact except shared E-M15-031 using main newer runs. Main runtime/dependency metadata has no temporary paths; persistent fixture SHA256 remains cd4739ec7ede822b24c2682f043d6f8afd48122596b5c0902bd3d96bb0140075. Both parent histories and source tooltip quoystmy retained. All changed files formatted; no process depends on either temporary checkout. Temporary .local is empty after owned demo relocation.
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
JJ_STORE_REAL_BASE_URL=http://127.0.0.1:7478/ JJ_STORE_LOCAL_BROWSER_FIXTURE=<retained isolated fixture>/browser-fixture.json pnpm exec playwright test --config web/playwright.real.config.ts local-browser.spec.ts --output .scratch/test-results-local-browser-qualified-final
```

**Notes**

5/5 in58.5s on final freshly built/current served bundle. Qualified local browsing/agents/settings/accessibility; projects/tasks/comments and automatic refresh; review create/reply/CLI parity/historical pins; registration reconnect/removal cleanup; physical Save/unknown outcomes/stale draft retention. Persistent main app and user browser untouched.
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
SIGTERM owned isolated jj-store server16526; await session13708 exit; trash -v <retained isolated fixture root>
```

**Notes**

Owned disposable server stopped cleanly and fixture recoverably removed. No unrelated daemon or persistent storage touched. Final browser traces and logs retained under .scratch and /tmp.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:architecture; pnpm check:ui-ownership; pnpm spec:first --base 1b2dbc2f --head <actual merge hash>; pnpm spec:first --base 36c92787 --head <actual merge hash>; pnpm spec:validate; pnpm check:progress; changed-file pnpm exec prettier --check
```

**Notes**

Final source gates before commit:808production modules, ownership pass,104protected files/10chapters versus Appearance and379/13 versus main.5validators/21chapters/147requirements,120tasks and all35changed paths formatted. No conflicts. Root aggregate/Rust workspace and further Design Core visual lanes remain out of scope.
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
jj commit -m "Integrate Appearance UI into main and retire temporary workspaces"; jj st in both temporary checkouts; lsof cwd audit; jj workspace forget appearance (consumer main); jj workspace forget jj-store-appearance (Design Core main); trash -v /Users/stevejuma/code/lapismd/jj-store-appearance /Users/stevejuma/code/lapismd/design-core-jj-store-appearance
```

**Notes**

Verified merge committed as enclosing change wpmoyoqz with yyknxszk/kltumumn parents and full Appearance ancestry. Both temporary working copies clean; no active process depended on them. Only the requested Appearance registrations removed. Checkouts recoverably moved to ~/.Trash/jj-store-appearance and ~/.Trash/design-core-jj-store-appearance. Other registered workspaces untouched.
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
jj st/workspace list/retained ancestor checks in consumer and source main; fixture SHA256/runtime/PID/HTTP checks after retirement
```

**Notes**

Both main working copies clean; temporary paths absent and registrations removed. Source default retains tooltip quoystmy/bb40d39d. Persistent server95351/UI95352 stay alive,7460returns200 and fixture SHA256 unchanged. Final retirement evidence is folded into the enclosing verified merge as a ledger-only update; repeat document governance before squash.
<!-- /run -->
<!-- /evidence-field -->

