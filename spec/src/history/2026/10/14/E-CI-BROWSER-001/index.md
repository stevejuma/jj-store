---
schema: 1
id: E-CI-BROWSER-001
---

# E-CI-BROWSER-001

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
JJ_STORE_DOC_REFERENCE=<isolated renderer> pnpm web:build (final threads-witness integration)
```

**Notes**

Fresh browser bundle and all Help documents built. Inherited released dependency
CSS minifier warnings remain separate; no dependency source changed.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit (final native threads-witness source)
```

**Notes**

177 test files: 1056 passed, one fixture skip. Complete affected unit suite; not
full repository/release qualification.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 9
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec playwright test --config web/playwright.real.config.ts <11 repaired browser files> (isolated rebuilt cold restart; /tmp/jj-store-ci-browser-focused-final.log)
```

**Notes**

13 passed and one specialized fixture skip. Remaining older status-role,
duplicate creation button, task/review Activity, retained settings, footer and
dialog lifecycle assumptions. The original
mutation/persistence/freshness/security assertions remain selected.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec playwright test --config web/playwright.real.config.ts <focused reruns> --grep-invert "native conflict|external publication|demo history" (/tmp/jj-store-ci-browser-focused-2.log)
```

**Notes**

5 passed. Native mutation receipt/feedback, mixed Thread creation/send/reload
and additive seeding now pass. Remaining UI layout assumptions continue to be
repaired against canonical contracts.
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
pnpm exec playwright test --config web/playwright.real.config.ts project-polish.spec.ts projects-activity.spec.ts settings-layout.spec.ts work-store.spec.ts native-repository.spec.ts --grep-invert "native conflict|demo history" --output=/tmp/jj-store-ci-focused-round3
```

**Notes**

4 passed, including exact final external publication and native mutation. Wait
for initial history/workspace capture before publishing: automatic refresh
intentionally defers while captured graph preparation is active. Task Activity
is initially expanded under JST-UI-015; comments and recorded Agent context
retain their own surfaces. Native Work commands expose committed receipts rather
than the removed Saved inline status. Link completion identified a remount gap
subsequently repaired under JST-UI-005; keep the captured dialog owner until its
receipt is reflected. Settings qualification ignores retained hidden categories
while keeping visible geometry and accessibility checks.
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
gh run view 38639600047 --repo stevejuma/jj-store --job 116112137145 --log-failed
```

**Notes**

Hosted governance rejected production-store-actions at 307 counted lines. Build,
web, ordinary tests and reusable heavy qualification were all skipped, proving
early failure gating.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec playwright test --config web/playwright.real.config.ts native-repository.spec.ts (owned retained fixture; /tmp/jj-store-ci-native-refreshed.log)
```

**Notes**

2 passed; outdated success-feedback scopes, absent explicit parent selection and
intermediate CLI snapshot selection exposed. Retained as diagnostic before
evidence.
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
pnpm exec playwright test --config web/playwright.real.config.ts event-freshness.spec.ts task-reactions.spec.ts --grep "two clients|task reaction" (owned fixture; /tmp/jj-store-ci-freshness-focused.log)
```

**Notes**

Work event/replay/draft/hidden test passed; task reaction observer failed. The
browser did not subscribe task views to reaction notices; fixed in the
production freshness owner rather than relaxing the assertion.
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
pnpm catalog:test -- web/tests/vite/runtime-reload.test.ts web/tests/vite/automatic-refresh.test.ts web/tests/vite/work-reactions.test.ts
```

**Notes**

CLI forwarding ran the full unit suite: 176 files, 1053 passed and one fixture
skip. Subsequent source changes have their own focused and final suite
qualification.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/runtime-reload.test.ts web/tests/vite/projects-freshness.test.ts web/tests/vite/production-automatic-refresh.test.ts web/tests/vite/working-copy-actions.test.ts
```

**Notes**

17 regressions in four files passed: idle reload observation, reaction event
scheduling/dirtiness and working-copy session/selection guards.
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
node --test scripts/ci.test.mjs
```

**Notes**

11 CI tests passed including actual child-process fixture isolation, cleanup
after success/failure, complete cold shard matrix and protected aggregation.
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
pnpm web:check
```

**Notes**

0 errors and 0 warnings at the initial controller repair; final changed-source
check remains selected.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:architecture (initial extraction and reaction follow-up)
```

**Notes**

Working-copy extraction removed the original 307-line violation; reaction
integration pushed Projects hub to 301 lines. Namespace composition retains
behavior while keeping the existing limit.
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
pnpm check:architecture
```

**Notes**

1903 modules passed without any baseline/limit increase.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm dev:cargo -- test -p jj-store --test server_executable --locked -- --ignored --exact git_bridge::real_rustfs_native_git_external_merge_cycle_survives_cold_restart
```

**Notes**

Incorrect module name selected zero tests. Not qualification. Actual module is
git_bridge_acceptance, confirmed by test inventory.
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
CARGO_TARGET_DIR=.local/build-targets/public-release CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0 JJ_STORE_BROWSER_ROOT=/tmp/jj-store-ci-cold-20261014 JJ_STORE_RUSTFS_RUNTIME=/tmp/jj-store-ci-cold-20261014/runtime/embedded-rustfs/target/debug/jj-store-rustfs pnpm dev:cargo -- test -p jj-store --test server_executable --locked -- --ignored --exact git_bridge_acceptance::real_rustfs_native_git_external_merge_cycle_survives_cold_restart
```

**Notes**

The original real-RustFS/Git/cold browser parent passed one test in 36.60
seconds, with all retained publication, cache-loss, merge, search and restart
assertions. Existing macOS debug unwind linker warning remains separate from
optimized release qualification.
<!-- /run -->
<!-- /evidence-field -->
