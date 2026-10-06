---
schema: 1
id: E-DESK-010
---

# E-DESK-010 details 2

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
jj --no-pager st; jj --no-pager log -n 5; inspect desktop/icons/icon.png and the existing bundle Info.plist
```

**Notes**

Clean txuqtnrs above committed spec migration 40382d35. The prior default icon is a 32×32 opaque placeholder, 104 bytes, SHA-256 287050363f97f39b4b4fd26394ff78fc18fb74998fe9a9ba30b7cfd4600a03cc. Existing bundle used generated jj-store.icns. Narrow acceptance target: import the approved transparent branding, prove the actual bundled ICNS bytes, and cold-reopen the actual WKWebView. Before-state record: .local/desktop-icon/before.json.
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
pnpm spec:search -- JST-DESK-001
```

**Notes**

The repository guide example passes an extra literal -- which this installed spec-validator search rejects. No product failure. Retried using pnpm spec:search JST-DESK-001 and opened the canonical desktop requirement/delivery and M17 task pages.
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
pnpm spec:search JST-DESK-001; pnpm evidence:new -- --id E-DESK-010 --task M17 --task M17.2 --task M17.4 --requirement JST-DESK-001 --requirement JST-DESK-004 --requirement JST-DESK-005
```

**Notes**

Current owning topic read before implementation. Started a fresh schema-1 Markdown record; retained original qualification selections and historical evidence.
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
CUA: get the existing jj-store app; press Cmd-Q; ps -p 70966,71084,71085 -o pid=,command=
```

**Notes**

Actual app was displaying native history with no active edit form. Normal Quit removed the host, supervised server and RustFS helper before bundle replacement. CUA subsequently reported noWindowsAvailable, expected for the exited app. Existing unrelated dev-demo services were not restarted.
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
Copy approved Desktop logo-kit PNG, ICNS and SVG assets into desktop/icons; Node PNG/ICNS/SVG assertions and config inspection
```

**Notes**

Both 1024 px RGBA PNGs retain opaque marks and transparent corners. Both ICNS containers have valid declared sizes and a 1024 px ic10 representation. Each SVG has 184 paths and no image/script/foreignObject/filter. Default outline variant matches approved kit bytes. Default PNG SHA-256 88c173b3a4865fb0aa34a22222bbd74f49ca269d4f413d51188fbe25a3cd6e93; default ICNS b283d0f2a9030b0cc622a8b6494de78903d79f4c88283858c5c55cc4519ef396. Local proof: .local/desktop-icon/assets.json.
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
node scripts/desktop.mjs stage > .local/desktop-icon/stage.log 2>&1
```

**Notes**

Reused explicit current workspace executables and complete production browser/help build. Staged paired server/custom JJ/embedded RustFS resources and 212 browser assets. No unrelated Rust or UI rebuild, reset, fixture reseed or installed-tool replacement.
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
pnpm desktop:check > .local/desktop-icon/check.log 2>&1
```

**Notes**

Locked desktop host compile check passed in 17.71 seconds, including native generated context and default icon. No Rust implementation change and no unrelated Rust suites required.
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
pnpm desktop:bundle > .local/desktop-icon/bundle.log 2>&1; inspect Info.plist, packaged icon bytes, production and help asset manifests
```

**Notes**

Pinned Tauri CLI 2.11.4 debug app build passed; dev compilation 20.54 seconds. Actual app bundle CFBundleIconFile is icon.icns and the packaged resource is byte-identical to desktop/icons/icon.icns. All 212 production and 139 help manifest files exist in the bundle. Bundled debug app is under desktop/target/debug/bundle/macos/jj-store.app. This is app/icon packaging evidence, not signed public distribution, DMG, relocation or full JST-DESK-005 qualification. Local proof: .local/desktop-icon/bundle.json.
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
CUA: launch rebuilt jj-store.app, select Commits, inspect actual WKWebView accessibility state and screenshot; inspect Finder bundle icon; Node retained-device/process/listener assertions
```

**Notes**

Cold host launch rendered the actual production WKWebView on retained literal-loopback origin http://127.0.0.1:57653. Existing two projects and two repositories were visible. Branches → Commits rendered JJ Log 26; no error state, only the informational completed graph-index toast. Finder displayed the blue-jay icon next to the unchanged placeholder on the separate acceptance app. Retained desktop.json version/port matched before state; new host/server/RustFS PIDs 67857/67866/67867 use the rebuilt bundle resources. lsof proves literal-loopback listener. The user then navigated to Stacks; the app remains open. Existing private installation was reopened under the explicit user request, not reset or migrated; no disposable-state/release acceptance is claimed. Local proof: .local/desktop-icon/native.json.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
CUA: get Dock accessibility state; Finder Go to Folder via AX setValue; retry using paste
```

**Notes**

Direct Dock AX selection timed out. Finder setValue was rejected by a stale-state guard before changing its text; refreshed state and paste succeeded, then Return opened the exact bundle directory. These are UI automation limitations; Finder visual icon and byte-identical packaged ICNS independently verified the asset. No automated Dock screenshot qualification is claimed.
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
pnpm progress:history; format changed/generated Markdown and desktop config with pnpm exec prettier --write
```

**Notes**

Refreshed task backlinks and bounded navigation. The navigation writer temporarily removed formatting in unrelated indexes; formatting the generated changes restored their parent bytes. The final diff is confined to the icon assets/config, owning delivery topic, selected M17 histories/rows, DESK verification and new evidence/date navigation. No migration/governance implementation change. Format logs are under .local/desktop-icon.
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
pnpm check:progress > .local/desktop-icon/progress.log 2>&1
```

**Notes**

Progress checker passed 143 tasks and 617 evidence records; fresh coverage and immutable historical payloads preserved. This is evidence bookkeeping, not milestone completion.
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
pnpm spec:validate > .local/desktop-icon/spec-validate.log 2>&1
```

**Notes**

Document validation passed 7 validators, 3010 chapters and 175 requirements at this run, including authored-page limits and canonical references. No root aggregate spec:check or product test suite run.
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
pnpm spec:first > .local/desktop-icon/spec-first.log 2>&1
```

**Notes**

Spec-first passed: 8 protected files and 1 owning canonical chapter. Owning desktop behavior topic was updated with the branding selection; no unchanged-contract exception needed.
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
node <<'NODE'
const fs=require('node:fs');const {execFileSync}=require('node:child_process');
const files=execFileSync('jj',['--no-pager','diff','--name-only'],{encoding:'utf8'}).trim().split('\n').filter(file=>file.endsWith('.md')||file==='desktop/tauri.conf.json');
const result=execFileSync('pnpm',['exec','prettier','--check',...files],{encoding:'utf8',maxBuffer:1024*1024});fs.writeFileSync('.local/desktop-icon/format-check.log',result);console.log(result);
NODE
```

**Notes**

All changed Markdown and desktop config use the existing Prettier format. SVG path geometry is retained as supplied and was checked by rendering and asset parsing.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

[]
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; Rust 1.98.1; Node 24.10.0; pnpm 10.32.1; Tauri host 2.11.5/build 2.6.3; pinned CLI 2.11.4; pngjs 7.0.0; classic macOS ICNS with 1024 px representation. Upstream prepared assets were traced with VTracer 0.6.5 and rendered with Chrome 154/Playwright 1.63.0.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Presentation-only RGBA PNG, multi-resolution ICNS and pure-path SVG. Existing private desktop.json version 1 and repository/storage format 2 retained. No API, CLI launch, authorization, WAL, receipt, manifest or durable repository format changes.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

User-requested native app icon integration and app startup checks. Repository owns both prepared theme variants; default PNG/window/tray and explicit macOS bundle ICNS use the pale-outlined variant for contrast on both surfaces. Native Rust logic, browser production source and package dependencies are unchanged. Focused asset, host compile, bundle and actual native startup checks suffice for this packaging slice. Root aggregate pnpm check/test/build/spec:check, Rust service/storage suites, desktop logic tests, UI type/Storybook suites, fake-store substitution and visual baselines are out of scope. Actual retained-installation launch is distinct from separate-state cold lifecycle, real-RustFS faults, DMG relocation, signing/notarization and distribution qualification. No automatic icon/theme switching was added.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: Updated the owning desktop initial-delivery topic with explicit
  repository-owned transparent branding and static default icon selection under
  JST-DESK-001. JST-DESK-004 window/tray lifecycle, JST-DESK-005 broader
  qualification and all storage/native authorization semantics remain unchanged.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: updated
reason: Added E-DESK-010 to M17, M17.2 and M17.4 evidence and partial
  JST-DESK-001 verification. M17/M17.2 remain In progress; M17.4 remains
  Planned. Icon packaging and this requested retained-installation native launch
  do not complete the full original app/DMG, recovery, relocation, dirty-edit,
  signing or platform gates.

```
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Review the running app. Continue separate M17 native controls, dirty-edit and complete cold app/DMG distribution gates independently.
<!-- /evidence-field -->

