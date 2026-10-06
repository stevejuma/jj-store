---
schema: 1
id: E-UI-057
---

# E-UI-057

<!-- evidence-field:id markdown -->
## Id

E-UI-057
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M16
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-02
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

vite-hmr-development-and-owned-stack-lifecycle
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change nmxuylws
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Appearance workspace is clean above klylkzvs/86083510 and committed main nxoksqrt/ba74e1ea. Its owned built-UI service runs on 7460 against the retained main .local/dev-demo fixture. Main has unrelated unfinished Thread UI work in wtqyykvt. Existing dev:restart rebuilds the UI and native binaries; web:dev has no native proxy or coordinated lifecycle. The exact new development regression fails once with missing dev-ui-config.mjs before implementation.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- .gitignore
- AGENTS.md
- README.md
- package.json
- web/vite.config.ts
- scripts/dev-demo-services.mjs
- scripts/dev-demo-processes.mjs
- scripts/dev-demo-processes.test.mjs
- scripts/dev-ui-config.mjs
- scripts/dev-ui-services.mjs
- scripts/dev-ui-vite.mjs
- scripts/dev-ui.mjs
- scripts/dev-ui.test.mjs
- scripts/dev-ui-native-fixture.mjs
- scripts/dev-ui.acceptance.mjs
- spec/src/api.md
- spec/src/architecture.md
- spec/src/spec-governance.md
- spec/src/verification.md
- spec/src/milestones.md
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
jj --no-pager st; jj --no-pager diff --stat; jj --no-pager diff (Appearance and main inspection)
```

**Notes**

Preserved unrelated unfinished main change. Implementation is confined to the reused Appearance workspace; no landing or rebase onto unfinished main work.
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
node --test scripts/dev-ui.test.mjs (before, then exact rerun)
```

**Notes**

Before: one failing test file, missing new module, /tmp/jj-hmr-before.log. Exact rerun passes4/4 in /tmp/jj-hmr-exact.log; final target now includes occupied-port preservation and passes5/5. No fixture builds or seeding occur in UI startup.
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
node --test scripts/dev-demo*.test.mjs scripts/dev-ui.test.mjs
```

**Notes**

Final28/28. Covers existing demo fixture/profile behavior, proxy/configuration, invalid ports, occupied-port preflight, two-owner validation, stop preservation and backend-only teardown. /tmp/jj-hmr-dev-tests-qualified.log.
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
pnpm web:check (initial and qualified reruns)
```

**Notes**

Two initial implicit-any diagnostics in the shared JS configuration resolved with JSDoc; final0 errors/warnings. /tmp/jj-hmr-web-check-qualified.log.
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
pnpm web:test; pnpm check:architecture; pnpm check:ui-ownership
```

**Notes**

Web70/70; architecture728 modules; ownership boundary passes. Complete direct UI test lane is warranted by shared Vite infrastructure. /tmp/jj-hmr-{web-unit,architecture,ownership}.log.
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
pnpm web:build
```

**Notes**

Production bundle still builds with source-distributed Design Core rune modules excluded from dependency prebundling and supported Mira nested dependencies included, matching existing catalog configuration. Existing large-chunk advisory remains. No dependency aliases, patches or package changes. /tmp/jj-hmr-production-build.log.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:dev:ui (initial cold attempts and diagnostics)
```

**Notes**

Development attempts exposed fixture readiness missing Fetch Metadata headers; fixed fixture and launcher requests. Vite optimizer then crashed on source-distributed Design Core .svelte.ts type syntax, including an ECONNREFUSED retry; fixed supported dependency optimization configuration. Two browser waits incorrectly expected a repository label in a collapsed rail; replaced them with actual Local profile and native commit visibility checks. /tmp/jj-hmr-native-{cold,cold-retry,optimizer-fixed,browser-diagnostic}.log. These were separate harness/startup failures, all cleared by the final cold run.
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
JJ_STORE_DEV_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs pnpm test:dev:ui
```

**Notes**

Fresh isolated native fixture and fresh Vite/Chromium. Application bootstrap and repository UI render; browser POST commits native WorkStore project; native SSE ready and repository-changed stream through proxy; Svelte, TypeScript and CSS HMR retain document marker with zero reloads. Wrong Origin and direct-backend Host rejected; ignored .local sentinel returns403; backend restart retains Vite PID, snapshot and installation/fixture identity; stop ends both processes and preserves data. Browser runtime errors0. Only exact optional Gravatar requests are aborted in the fixture, producing expected network console errors. /tmp/jj-hmr-native-final-qualified.log.
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
JJ_STORE_UI_TEST_FULL_RESTART=1 CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target JJ_STORE_DEV_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs pnpm test:dev:ui
```

**Notes**

Cold native acceptance plus actual pnpm dev:restart transition from active HMR. Uses existing custom JJ copied into ignored Appearance target/debug/jj; package Cargo build and UI build run only for this explicit full-stack qualification. Old Vite/backend PIDs end, served bytes equal built index, native project and installation/fixture identity survive. Disposable services cleaned up. /tmp/jj-hmr-native-full-restart.log.
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
node /tmp/jj-hmr-live-switch.mjs
```

**Notes**

Authorized interactive switch built-&gt;Vite with no UI/Rust/RustFS build or seed. Vite36218/browser7460; backend36217/API7462; storage36219. Appearance sources use persistent main .local/dev-demo. Before/after native snapshots, Thread directory, installation and fixture hashes equal; search ready. /tmp/jj-hmr-live-{before,ready}.json and switch.log. Interactive stack remains running.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --write (changed files, initial attempt)
```

**Notes**

Including .gitignore failed parser inference; formatter rerun omits that non-formatable file. Source/docs formatting subsequently passes. /tmp/jj-hmr-format.log.
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
pnpm check:progress (first ledger validation)
```

**Notes**

Ledger initially listed unrelated outstanding M16 qualification in this passing slice remaining array. Moved that context to scope and retained M16 Partial; no check suppression or milestone status change. Final focused rerun passes.
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
pnpm exec prettier --write (all changed supported files); pnpm exec prettier --check (same files); pnpm spec:first; pnpm spec:validate; pnpm check:progress
```

**Notes**

Canonical GOV-009, mapped chapters, verification row and enclosing-change ledger validated. Focused changed-file formatting passes. /tmp/jj-hmr-{format-complete,format-check,spec-first,spec-validate,progress}.log.
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
node --input-type=module (read-only final live readiness/state check and owned disposable cleanup)
```

**Notes**

Both ownership markers valid; Vite client and LocalTrusted bootstrap present. Installation/fixture hashes, native snapshots and Thread directory still equal the captured before state after final cold verification. /tmp/jj-hmr-live-final.json. Removed stopped failed initial OS-temporary fixture FH2UYC.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Shared development process lifecycle and Vite startup configuration can affect both source-serving and built-UI modes, warranting the complete direct development harness suite, web unit/check/build lane and cold native HMR plus full-stack transition. No production Rust source or public native contract changes. Root aggregate spec:check/check/test/build, Rust tests, unrelated catalog/story suites, publication, installed CLI changes and visual baseline approval are out of scope under targeted verification policy. The explicit retained full-stack smoke invokes its package Rust build, distinct from build-free dev:ui. M16 remains Partial: prior foundation release, registry-only consumer, inherited suite/composer-CSP and visual baseline gates remain separate. This passing development slice is not production CSP or release qualification.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; Node26.9.0; pnpm10.32.1; Vite8.3.2; Svelte5.57.1; JJ0.45.1; Rust1.98.1. Existing embedded RustFS binary and unchanged foundational package/lockfile versions.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Development runtime format2 adds mode=vite, apiOrigin, uiPid and uiRunner while retaining selected fixture paths. Separate ignored ui-server.toml removes web_root and uses public browser origin7460 with private backend7462. Vite preserves Host/Origin/Fetch Metadata/protocol headers without CORS/auth bypass and uses development-only HMR/CSS CSP plus private filesystem denials. Production asset manifest/CSP and native/storage/API/appearance formats are unchanged.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-UI-057 details 2](details-02.md)
