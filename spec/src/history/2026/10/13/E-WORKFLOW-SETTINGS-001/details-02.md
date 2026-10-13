---
schema: 1
id: E-WORKFLOW-SETTINGS-001
---

# E-WORKFLOW-SETTINGS-001 details 2

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
pnpm dev:cargo -- test -p jj-store-server --lib runtime_reload::tests::workflow_hosts_participate_in_saved_reload -- --exact
```

**Notes**

Before implementation: workflows absent from supported saved reload; /tmp/jj-store-workflow-settings-before.log.
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
pnpm dev:cargo -- check -p jj-store-cli -p jj-store-server -p jj-store-http -p jj-store-rpc
```

**Notes**

Affected initial checks passed after repairing shared URL dependency, admission guard type and moved Command import. Earlier compile failures are retained in /tmp/jj-store-workflow-settings-check-*.log.
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
pnpm contracts:generate
```

**Notes**

Generated OpenAPI and TypeScript contracts for the owned settings types; /tmp/jj-store-workflow-settings-contracts.log.
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
pnpm dev:cargo -- test -p jj-store-server --lib runtime_reload::tests
```

**Notes**

7 tests; workflow saved reload, exact original receipt, stale/infrastructure denial and independent token permissions. Apple linker emits inherited large __eh_frame compact-unwind warning.
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
pnpm dev:cargo -- test -p jj-store-service --lib workflow::admission_tests
```

**Notes**

1 test; shared transport clones and replacement blocked by an in-flight admission lease.
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
pnpm dev:cargo -- test -p jj-store-server --test capture_management server_settings
```

**Notes**

4 tests; atomic admission/local save, no execution, unrelated sections retained, invalid secret references unchanged file revision, HTTP and current LocalTrusted Admin authority.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/workflow-host-settings.test.ts web/tests/vite/server-settings.test.ts web/tests/vite/runtime-reload.test.ts
```

**Notes**

17 tests including host JSON defaults/shape, source and secret-reference bounds, agent endpoints, isolated conditional save, unapplied raw JSON and stale-draft retention.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/WorkflowHostSettings.stories.svelte catalog/components/store-application/StoreServerSettings.stories.svelte catalog/components/store-application/RuntimeReload.stories.svelte
```

**Notes**

27 interaction/Axe tests. First run had 3 assertion failures from an incorrect toast selector; corrected to released Design Core data-notification-id without weakening toast/outside-form assertions.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/WorkflowHostSettings.stories.svelte
```

**Notes**

7 interaction/Axe tests including explicit 390px mobile viewport, loading geometry, invalid JSON/admission toasts and retained unknown-save draft. Fresh Vitest/browser process, visual-pending.
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
pnpm web:check; pnpm catalog:check
```

**Notes**

0 errors/0 warnings, catalog ownership boundaries pass. Initial checks found missing nullable guards and newly required fixture configuration; repaired and rechecked.
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
pnpm check:ui-ownership; pnpm check:boundaries
```

**Notes**

Owned presentation and 18 package dependency boundaries pass; production remains independent of catalog and external checkouts.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:architecture; compare workflow-console.css with jj file show -r @-
```

**Notes**

Inherited unchanged workflow-console.css is 411 counted lines against 300. Parent content matches byte-for-byte; new Settings components are under 300 lines. No aggregate architecture pass claimed.
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
pnpm dev:cargo -- test -p jj-store-cli --lib workflow_host::managed
```

**Notes**

Both runtime tests passed, but managed postflight rejected target 40.4 GiB above its 40 GiB limit. This is a command failure, not a passing qualification lane. Subsequent docs-reference generation similarly stopped after postflight at 40.6 GiB.
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
pnpm dev:disk; pnpm dev:clean -- --dry-run; pnpm dev:stop; pnpm dev:clean -- --dry-run; pnpm dev:clean -- --apply
```

**Notes**

Initial cleanup skipped live target/PID 15386. After stopping only the owned dev demo, guarded workspace cache cleanup touched only jj-store/target, recovering 25.8 GiB (176.5 to 202.3 GiB free). Development fixture, state, staged resources and other target trees retained. No reset, manual deletion or budget override.
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
pnpm dev:cargo -- test -p jj-store-cli --lib workflow_host::managed
```

**Notes**

After guarded idle cache maintenance: both runtime tests pass, including deferred poll/shutdown drainage; /tmp/jj-store-workflow-settings-managed-tests-final.log.
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
JJ_STORE_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs pnpm dev:cargo -- test -p jj-store-cli --test server_executable --locked -- --ignored --exact workflows::two_hosts_execute_one_pinned_run_and_preserve_history_after_restart
```

**Notes**

Isolated real RustFS, 1 test in 40.79s: two hosts, three ordered actions, exact native source/workspace, logs/artifact/history, saved disabled→enabled→disabled activation, busy Deferred then idle apply, continuous retry, unchanged process PID/native socket inode, HTTP/RPC admission revocation and cold recovery. Test-owned services cleaned up; /tmp/jj-store-workflow-settings-native-acceptance.log.
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
pnpm dev:backend:build; pnpm dev:ui
```

**Notes**

Owned development binary upgraded once and fresh Vite/backend launched with preserved fixture and staged ACP resources; no reset or other service restart. Later configuration changes use reload.
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
Live CUA: Settings save polling 1000→1500→1000, Reload saved settings, close dialog and refresh
```

**Notes**

Single shared reload dialog and restored focus. Same backend PID 62145; desired/effective current, managed host remains disabled and admission empty. Native demo and Persistence run Pages exactly unchanged, including previously pending attempts. Proof in /tmp/jj-store-workflow-settings-live-proof.json; UI persisted across cold page reload.
<!-- /run -->
<!-- /evidence-field -->

