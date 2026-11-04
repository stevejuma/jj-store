---
schema: 1
id: E-DESK-BROWSER-001
---

# E-DESK-BROWSER-001 details 2

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
MISE_PNPM_VERSION=10.32.1 pnpm view @lapismd/spec-validator version
```

**Notes**

Latest registry release is 0.2.0, already selected. No dependency update required.
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
MISE_PNPM_VERSION=10.32.1 pnpm exec spec-validator doctor
```

**Notes**

10 checks pass; existing custom-script-alias warning.
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
MISE_PNPM_VERSION=10.32.1 pnpm spec:search -- JST-DESK
```

**Notes**

pnpm shim initially required an explicit version; then this installed CLI rejects the forwarded --. Used pnpm exec spec-validator search JST-DESK-004 successfully and opened canonical chapters.
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
MISE_PNPM_VERSION=10.32.1 pnpm exec spec-validator search JST-DESK-004
```

**Notes**

QMD canonical search works.
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
MISE_PNPM_VERSION=10.32.1 pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/host-environment.test.ts
```

**Notes**

Before implementation: missing host/environment module, one failed suite.
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
node --test scripts/dev-ui-presentation.test.mjs
```

**Notes**

Before implementation: missing uiDevelopmentPresentation export, one failed test.
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
MISE_PNPM_VERSION=10.32.1 pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/host-environment.test.ts web/tests/vite/desktop-glass.test.ts web/tests/vite/terminal.test.ts web/tests/vite/appearance.test.ts
```

**Notes**

30 tests in four files.
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
MISE_PNPM_VERSION=10.32.1 pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/host-*.test.ts web/tests/vite/desktop-glass.test.ts web/tests/vite/terminal.test.ts web/tests/vite/appearance.test.ts
```

**Notes**

34 tests in six files; injected native methods, request/channel payloads and browser absence.
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
node --test scripts/dev-ui-presentation.test.mjs scripts/dev-ui.test.mjs
```

**Notes**

8 launcher tests; prerequisite failure, occupied port, retained settings and presentation selection.
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
MISE_PNPM_VERSION=10.32.1 pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit --maxWorkers=4
```

**Notes**

Escalated to complete catalog unit lane because host options cross connection/controller and appearance owners. 193 files; 1221 passed, one existing skip.
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
MISE_PNPM_VERSION=10.32.1 JJ_STORE_VALIDATION_RUNTIME=native pnpm web:test
```

**Notes**

Complete web unit lane for shared controller inputs: 180 passed.
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
MISE_PNPM_VERSION=10.32.1 JJ_STORE_VALIDATION_RUNTIME=native pnpm test:storybook -- catalog/components/store-application/{DesktopChrome,StoreApplication,SettingsShell,SettingsNavigation,AgentDefaults}.stories.svelte catalog/components/store-terminal/{Terminal,TerminalHeader}.stories.svelte catalog/components/store-appearance/*.stories.svelte
```

**Notes**

All direct fixture/host presentation families: 14 files; 63 interaction/accessibility tests pass, one existing skip. Stories remain visual-pending; no baselines changed.
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
MISE_PNPM_VERSION=10.32.1 pnpm test:dev:desktop:ui
```

**Notes**

First cold attempt: new Help assertion compared exact class and rejected Svelte scoped class; fixed to require desktop class membership. Second attempt encountered Chromium ERR_NETWORK_CHANGED during source load and startup timed out.
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
MISE_PNPM_VERSION=10.32.1 JJ_STORE_DEV_PORT=7490 JJ_STORE_DEV_BACKEND_PORT=7492 pnpm test:dev:desktop:ui
```

**Notes**

Passed twice on isolated real embedded RustFS and native service binaries. 40px/80px wide/narrow header, Help, saved/reloaded appearance, opaque glass, disabled keep-awake, absent terminal/IPC, zero Axe violations, real committed project write and SSE, Svelte/TS/CSS HMR without document reload, backend-only Vite PID/presentation retention, switch to ordinary browser, fixture identity and cleanup. Expected blocked avatar and restart disconnection network diagnostics remain distinct from zero uncaught renderer errors and zero IPC errors.
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
MISE_PNPM_VERSION=10.32.1 VITE_JJ_STORE_DESKTOP_PREVIEW=1 pnpm web:build
```

**Notes**

Initial top-level await enlarged chunk inventory; async bootstrap removed that expansion. Remaining adapter asset crossed the unchanged 256-file inventory by one. A group with includeDependenciesRecursively=false was incompatible with strict entry signatures; final shared host/connection/appearance grouping retains strict signatures and 256 assets. Native documentation stage additionally required explicit tool versions and existing reference binary.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
node .local/desktop-browser-validation/baseline-build.mjs
```

**Notes**

Diagnostic parent-source loader initially returned Svelte source for CSS subrequests; corrected loader to override plain source IDs only. No tracked files altered by diagnostic build.
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
node .local/desktop-browser-validation/baseline-build.mjs
```

**Notes**

Parent-revision source overlay compiled to ignored output: 251 application assets plus five icons. Final current build also has 251 plus five icons, 11516 manifest bytes.
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
MISE_PNPM_VERSION=10.32.1 MISE_RUST_VERSION=1.99.0 VITE_JJ_STORE_DESKTOP_PREVIEW=1 pnpm web:build
```

**Notes**

Vite compilation and bounded manifest passed. Default documentation stage attempted an unnecessary managed Rust reference build; stopped the owned cargo child and selected the existing docs_reference binary for final documentation checks/staging.
<!-- /run -->
<!-- /evidence-field -->

