---
schema: 1
id: E-REMOTE-AGENTS-001
---

# E-REMOTE-AGENTS-001 details 2

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
cargo test -p jj-store-http --lib --locked
```

**Notes**

Before route-inventory correction: 47 passed, 1 failed, 1 ignored. First failure was old route counts; rerun exposed the new agent family falling through a repository-param assertion. Updated exact inventory and bearer-only agent/SSE schemas; no assertion was removed.
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
cargo test -p jj-store-http --lib --locked
```

**Notes**

Final /tmp/jj-remote-http-regression3.log: 48 passed, 1 executable-dependent local working-tree test ignored. Agent stream schema keeps integer cursors and advertises only authenticated bearer admission.
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
cargo test -p jj-store-server --lib connections:: --locked
```

**Notes**

Four gateway tests: TLS/literal-loopback origins, public-path admission, private hint restart, bound sessions/binary transfer and installation replacement. WS limits are 1 MiB; disconnect cancels active streams. /tmp/jj-remote-connection-tests-final2.log.
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
JJ_STORE_AGENT_PRIVACY_CACHE=/tmp/jj-remote-privacy-cache JJ_STORE_REMOTE_TEST_JJ=/Users/stevejuma/code/lapismd/jj-store/target/debug/jj cargo test -p jj-store-server --lib remote_agents:: --locked -- --include-ignored
```

**Notes**

Two tests pass in /tmp/jj-remote-execution-final2.log. Real custom-JJ materialization, deterministic ACP process and MemoryStore verify positive execution, original receipt recovery, immutable home, changed retries, path/secret privacy and no restart replay. Earlier fixture attempts failed during readiness/private executable preparation; fixed executable copy and readiness wait. Scanner preparation is bounded at 300 seconds. This does not qualify an installed provider or real RustFS.
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
cargo test -p jj-store-service --test authorization delegated_access_rechecks_identity_scope_revocation_and_repository_grants --locked -- --exact
```

**Notes**

One focused delegation regression passed: current same-principal grants, scope restriction and revocation. /tmp/jj-remote-delegation-test3.log.
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
cargo test -p jj-store-rpc --test delegated --test local_trusted --locked
```

**Notes**

One delegated UDS test and one existing LocalTrusted UDS test pass in /tmp/jj-remote-rpc-regression.log. Same UID required; delegated sockets refuse bearer input and Admin/CreateRepository.
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
cargo test -p jj-store-server --test remote_agent_configuration --locked
```

**Notes**

One parser regression passes. The documented operator TOML validates; malformed principals, empty allowlists, inherited defaults, arbitrary runners and LocalTrusted mode are rejected. Default configuration leaves remote agents disabled.
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
cargo test -p jj-store-server --test configuration documentation_configuration_examples_use_production_validation --locked -- --exact
```

**Notes**

Existing complete documentation configuration examples pass production validation. /tmp/jj-remote-config-docs-test.log.
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
cargo test -p jj-store-server --lib remote_agents:: --locked
```

**Notes**

Earlier isolated Linux source fixture: 1 remote admission test passed, 1 custom-JJ execution test ignored. Delegated service test also passed separately. Disposable rust:1.98.1-bookworm container mounted source read-only and used ignored private caches. Initial compilation needed libboost-dev; corrected inside the disposable container. /tmp/jj-remote-linux-check2.log. Linux installed-runtime and real-storage gates remain open.
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
node node_modules/tsx/dist/cli.mjs --test web/tests/server-connections.test.ts
```

**Notes**

Six tests pass: route defaults/malformed duplicate selection, scoped fetch with no forwarded bearer, bounded preferences and layout isolation, inactive history, public agent transport and generation-bound invalidation. /tmp/jj-remote-routes-final2.log.
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
node node_modules/vitest/vitest.mjs run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/ServerConnections.stories.svelte catalog/components/store-application/ServerConnectionOwners.stories.svelte
```

**Notes**

Seven cold browser interaction/accessibility story cases pass, including keyboard/dialog/mobile and real production owner retention, guarded pending commands, sign-out, public share links and startup restoration. Latest /tmp/jj-remote-owner-stories-final3.log; visual baselines remain pending and untouched.
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
node node_modules/vitest/vitest.mjs run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/repository-settings-ownership.test.ts web/tests/vite/repository-settings-restoration.test.ts web/tests/vite/production-stacks-lifecycle.test.ts web/tests/vite/global-settings-history.test.ts
```

**Notes**

23 direct restoration/ownership/lifecycle tests pass in /tmp/jj-remote-restoration-tests-final2.log. Earlier focused settings/Stacks family had 17 passes; broader freshness/restoration family had 33. These counts are separate runs, not full-suite results.
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
node scripts/remote-connection-browser.mjs
```

**Notes**

Initial cold App checks lacked newly built Help assets. After documentation build, the same target exposed native fetch invoked as a LocalClient method: connection discovery silently failed and no switcher rendered. Injected-fetch owner stories had missed it. Wrapped native fetch to retain browser call context; diagnostic failure screenshot and logs stayed in /tmp.
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
node scripts/remote-connection-browser.mjs
```

**Notes**

Cold production App with fresh isolated Vite on port 7590 and deterministic APIs passes selected-server transport, browser Back/Forward, reload requiring sign-in, explicit Local links and memory-only credentials. /tmp/jj-remote-cold-browser5.log. Browser/Vite are cleaned up by the harness. Fixture acceptance does not qualify built production CSP, desktop packaging, a deployed remote service or real storage.
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
node node_modules/svelte-check/bin/svelte-check --tsconfig web/tsconfig.json
```

**Notes**

Zero errors and warnings; /tmp/jj-remote-web-final11.log. Native-fetch wrapper and generated public types included.
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
node node_modules/svelte-check/bin/svelte-check --tsconfig catalog/tsconfig.json --fail-on-warnings
```

**Notes**

Zero errors and warnings; /tmp/jj-remote-catalog-final5.log.
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
node scripts/check-catalog.mjs
```

**Notes**

Catalog boundary/default/visual-pending checks pass.
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
node scripts/check-boundaries.mjs
```

**Notes**

Storage/model/WAL package ownership checks pass; runtime networking remains server-owned.
<!-- /run -->
<!-- /evidence-field -->

