---
schema: 1
id: E-RUNNER-ENVIRONMENT-001
---

# E-RUNNER-ENVIRONMENT-001 details 2

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
pnpm dev:cargo -- test -p jj-store-service --lib workflow_host::tests::managed_environment_requires_no_repository_fields -- --exact
```

**Notes**

Before-state regression: environment-only version 2 rejected as missing principal; /tmp/jj-store-runner-simplify-before.log.
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
pnpm format:rust
pnpm dev:cargo -- check -p jj-store-cli -p jj-store-server
```

**Notes**

First implementation check found an unclosed ManagedRuntime.poll brace. Fixed and reran; rust-format.log/rust-check.log.
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
pnpm dev:cargo -- check -p jj-store-cli -p jj-store-server
```

**Notes**

Pass after correction and again after final production edits; rust-check-fixed.log and rust-final.log.
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
pnpm contracts:generate
```

**Notes**

Generated additive environment union and runtime identity, no native workflow command changes; contracts.log.
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
pnpm dev:cargo -- test -p jj-store-service --lib workflow_host::tests -- --nocapture
pnpm dev:cargo -- test -p jj-store-cli --lib workflow_host::managed -- --nocapture
pnpm dev:cargo -- test -p jj-store-cli --lib workflow_host
```

**Notes**

Service 2 tests, managed 2, final CLI 8 including removed-home uncertain journal preservation; service-verified.log, managed-tests.log, cli-final.log. Earlier filtered reruns also passed.
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
pnpm dev:cargo -- test -p jj-store-server --test capture_management server_settings
```

**Notes**

First compile found two legacy test accesses to fields now behind HostSettings. Matched the legacy variant and used state_dir(); server-final.log.
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
pnpm dev:cargo -- test -p jj-store-server --test capture_management server_settings
```

**Notes**

Five tests pass, including version-2 save without repository or identity fields and legacy validation; server-corrected.log.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_RUSTFS_RUNTIME="$PWD/runtime/embedded-rustfs/target/debug/jj-store-rustfs" pnpm dev:cargo -- test -p jj-store-cli --test server_executable --locked -- --ignored --exact workflows::two_hosts_execute_one_pinned_run_and_preserve_history_after_restart
```

**Notes**

First fixture had no registered workspace links, so the enabled environment correctly waited with no host. Registered code/home through existing local workspace API. Second run compared /var paths against canonical /private/var links; corrected the assertion to the canonical verified path. native.log/native-fixed.log.
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
pnpm dev:cargo -- test -p jj-store-cli --test server_executable --locked -- --ignored --exact workflows::two_hosts_execute_one_pinned_run_and_preserve_history_after_restart
```

**Notes**

Rerun omitted required JJ_STORE_RUSTFS_RUNTIME and failed before setup. Restored explicit pinned executable; native-canonical.log.
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
JJ_STORE_RUSTFS_RUNTIME="$PWD/runtime/embedded-rustfs/target/debug/jj-store-rustfs" pnpm dev:cargo -- test -p jj-store-cli --test server_executable --locked -- --ignored --exact workflows::two_hosts_execute_one_pinned_run_and_preserve_history_after_restart
```

**Notes**

Pass: two hosts, one ordered three-step pinned attempt, shared files, stdout/stderr, artifacts, cold history, environment-only managed reload, derived identity, explicit admission revocation, busy deferred/idle applied reload and unchanged listener/PID. native-environment.log; disposable fixture cleaned by harness.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/workflow-host-settings.test.ts web/tests/vite/server-settings.test.ts web/tests/vite/runtime-reload.test.ts
```

**Notes**

21 tests pass; first and final runs. Version-2 defaults/validation, legacy settings, conditional save, unknown outcomes and reload custody; units-first.log/units-final.log.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:check
```

**Notes**

Initial story locator used unsupported exact option in the repository Storybook test wrapper (5 diagnostics). Removed that option; catalog-first.log.
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
pnpm web:check
pnpm catalog:check
```

**Notes**

Final zero errors/warnings; 433 catalog modules. web-final.log/catalog-final.log; earlier focused checks pass after correction.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/WorkflowHostSettings.stories.svelte catalog/components/store-application/StoreServerSettings.stories.svelte catalog/components/store-application/RuntimeReload.stories.svelte
```

**Notes**

32 interactions/Axe pass in fresh processes, including mobile layout, JSON/Form draft retention, toast failure assertions, selected agent avatars, 16px gap and keyboard gauge-to-JSON synchronization. stories-first.log/stories-verified.log/stories-latest.log.
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
pnpm check:ui-ownership
pnpm check:boundaries
```

**Notes**

Owned UI validation passes; 18 boundaries tests pass, first and final runs. ownership.log/ownership-final.log/boundaries.log/boundaries-final.log.
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
pnpm docs:generate
PATH="/Users/stevejuma/.local/bin:$PATH" pnpm docs:check
```

**Notes**

Generated CLI/API references and reviewed services/configuration/cli/application/contracts source fingerprints using scripts/docs/review.mjs. Documentation closure 143 files, 26,004,011 bytes, nine Mermaid diagrams in both themes and valid links/anchors; docs-generate.log/docs-check.log.
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
pnpm dev:disk
pnpm dev:backend:build
pnpm dev:backend:restart
```

**Notes**

Budget 29.4 GiB active target, 49.6 total, 179.9 free before upgrade; within policy. Verified bins; backend PID 62145 → 95261, Vite PID 62146 unchanged, complete fixture preserved. No reset, unrelated daemon restart, installation or publication. build/restart logs; inherited Apple unwind linker warning retained.
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
Live CUA: cold settings load; inspect derived identity and agent avatar; exercise polling gauge; Save equivalent V2 environment; reload saved settings; refresh
```

**Notes**

Save toast and workflows Applied confirmed. Effective/desired V2 equal; no principal/home/sources fields. Host disabled and admission empty throughout. Polling restored to saved 1000 ms; recovery paths, owners and command presets preserved. Backend 95261/Vite 62146 unchanged during config reload. Byte-identical demo and Persistence native run pages before/after upgrade and reload. Screenshots /tmp/jj-store-runner-simplify-settings.png and polling.png. User-owned workflow editor/tab retained; isolated verification tab closed after checks.
<!-- /run -->
<!-- /evidence-field -->

