---
schema: 1
id: E-DESK-001
---

# E-DESK-001 details 4

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
pnpm dlx @tauri-apps/cli@2.11.4 build --debug --bundles app
```

**Notes**

Official Tauri CLI builds one debug macOS app bundle; /tmp/jj-store-desktop-app-build.log. The same command is exposed as pnpm desktop:bundle. This is not a signed, notarized, release-identity or DMG qualification.
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
CUA getApp bundled jj-store.app; select com.lapismd.jj-store.desktop.dev; inspect Projects and open Profile > Settings
```

**Notes**

Actual WKWebView renders existing Projects route at127.0.0.1:57653. Existing Local service Settings reads Local trusted and authoritative storage Ready. No source-injected browser bridge or CSP relaxation.
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
CUA getScreenshot for running native app
```

**Notes**

Tool reports noWindowsAvailable despite AX window/content. Screenshot visual evidence unavailable; not counted as product acceptance.
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
CUA click native close button then read AX state
```

**Notes**

Host32690 and owned service32820/RustFS32821 remain alive; AX tool reactivates and shows same retained Projects screen. Visual hide timing remains unqualified.
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
CUA Cmd+Q; ps -p32690,32820,32821; loopback connect probes57653/57654
```

**Notes**

App quit reported; all3 owned processes gone and service/storage ports closed. Existing appearance demo untouched.
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
CUA getApp com.lapismd.jj-store.desktop.dev after Quit
```

**Notes**

Actual packaged restart reopens existing Projects screen at same57653 origin. Debug installation retained in Application Support.
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
cargo test --manifest-path desktop/Cargo.toml --no-default-features --locked (final after owned-process regressions)
```

**Notes**

5 passed:2 readiness/navigation units and3 actual subprocess owner/output-bound regressions. The separately qualified real managed-runtime test remains explicitly ignored in this regular lane.
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
pnpm test:governance
```

**Notes**

42 passed. Shared specification mappings and progress/boundary scanners affect multiple governance consumers, requiring their full relevant governance lane. No product aggregate escalation.
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
CUA Cmd+Q after packaged restart; remove isolated /private/tmp/jj-store-desktop-native.tc7xYp after shutdown
```

**Notes**

Final restarted app and its owned service/storage helper quit. Disposable native state removed; default development Application Support installation retained.
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
python3 process-path, socket and disposable-state cleanup assertions; ps -p 91193,91207 -o pid=,comm=
```

**Notes**

Zero processes from desktop/target; ports57653/57654 closed; disposable native state absent. Existing appearance service91193 and RustFS91207 remain running unchanged.
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
pnpm exec prettier --write desktop/README.md desktop/tauri.conf.json desktop/ui/index.html docs/cli/server-run.md package.json scripts/check-boundaries.mjs scripts/check-progress.mjs scripts/desktop.mjs scripts/governance.test.mjs spec/src/SUMMARY.md spec/src/architecture.md spec/src/cli.md spec/src/desktop.md spec/src/milestones.md spec/src/spec-governance.md spec/src/verification.md spec-validator.config.mjs
```

**Notes**

All17 changed supported files formatted; TOML is consumed by Cargo rather than Prettier.
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
cargo fmt -p jj-store-cli -- --check
```

**Notes**

Affected CLI Rust formatting passes.
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
cargo fmt --manifest-path desktop/Cargo.toml -- --check
```

**Notes**

Independent desktop Rust formatting passes.
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
pnpm spec:validate (final)
```

**Notes**

5 validators,22 chapters,151 requirements pass after final evidence update.
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
pnpm spec:first (final)
```

**Notes**

30 protected files and5 owning canonical chapters pass, including owned-process regression evidence.
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
pnpm check:progress (with E-DESK-001)
```

**Notes**

125 tasks; same-change bookkeeping only.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Native privileged controls, accessibility audit, first-render handshake and
  visual hide timing remain pending; actual cold debug packaged WKWebView
  Projects/Settings and restart have smoke evidence.
- Native folder bridge, chrome, zoom and dirty-edit Quit guard remain M17.3.
- Local DMG, release identity, relocated app and complete cold native repository
  flows remain M17.4.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Root aggregate pnpm spec:check/check/test/build, full Rust workspace and unrelated UI suites not run under the targeted policy. Full regular server_executable target is the direct consumer of extracted shared foreground startup, signal handling and readiness. Shared governance scanner/mapping changes require the complete42-test governance lane. Native process, real storage, host-runtime, actual WebView and package evidence remain distinct; no visual baseline approval.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Qualify M17.3 controls, dirty-edit guard and M17.4 local release identity/DMG/relocation with cold repository flows in the desktop workspace.
<!-- /evidence-field -->

