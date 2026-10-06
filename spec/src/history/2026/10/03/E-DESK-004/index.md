---
schema: 1
id: E-DESK-004
---

# E-DESK-004

<!-- evidence-field:id markdown -->
## Id

E-DESK-004
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M17
- M17.2
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-03
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

desktop-workspace-retirement
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

wukmpoml (enclosing change)
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; JJ0.45.1; Rust1.98.1; pnpm10.32.1. The independently locked debug Tauri app and real RustFS pair from E-DESK-003 are retained in main.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No product or persistence change. Rebased foundation91a46ab0, chrome96d84e82 and verified integrationf00002ae descend from main67040334. Main tree retains all prior work. Application Support installation/configuration/device port files compare byte-identical by SHA256; origin remains127.0.0.1:57653.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- spec/src/milestones.md
- spec/src/desktop.md
- desktop/README.md
- desktop/tests/managed_runtime.rs
- desktop/tauri.conf.json
- scripts/desktop.mjs
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- desktop/tests/managed_runtime.rs#real_managed_runtime_restarts_preserves_origin_and_rejects_occupied_port
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
jj --no-pager st; jj rebase -r wukmpoml -o yuzrmrwx; jj --no-pager diff --from default@ --to desktop@- --summary
```

**Notes**

Main clean before landing; empty main change advanced to verified stack with an identical tree and no conflicting ancestors.
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
cp -cR /Users/stevejuma/code/lapismd/jj-store-desktop/desktop/target desktop/target; cp -cR /Users/stevejuma/code/lapismd/jj-store-desktop/desktop/resources desktop/resources
```

**Notes**

APFS clones retain usable app, host cache, paired jj-store/custom jj/RustFS and current212-asset built UI under main before retirement. Isolated service build target remains under ignored .local/desktop-integration-target; active main development binary untouched.
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
pnpm check:progress; pnpm spec:validate; pnpm check:ui-ownership; pnpm check:architecture (main)
```

**Notes**

125tasks;5validators/22chapters/152requirements; ownership passes;823modules within current budgets. Main tree consumers validated after landing.
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
JJ_STORE_DESKTOP_TEST_RESOURCES=/Users/stevejuma/code/lapismd/jj-store/desktop/resources cargo test --manifest-path desktop/Cargo.toml --locked --no-default-features --test managed_runtime -- --ignored --exact real_managed_runtime_restarts_preserves_origin_and_rejects_occupied_port
```

**Notes**

1/1 from main paths with real paired RustFS, restart, stable origin, occupied-port refusal and retained configuration/identity. /tmp/jj-desktop-integration-main-runtime.log.
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
desktop/target/debug/bundle/macos/jj-store.app/Contents/MacOS/jj-store-desktop (main, default persistent state); CUA getApp exact bundle path; Cmd+Q; process/session/port checks
```

**Notes**

Actual relocated WKWebView renders updated Projects/Agents at original57653. Host/service/RustFS executable paths all resolve under main; device settings, installation identity and configuration hashes unchanged. Quit returns0; owned45335/45699/45740 and listener absent. One initial CUA timeout and ambiguous bundle ID resolved by exact bundle path; stale input state refreshed before Quit. /tmp/jj-desktop-integration-main-relocation.log.
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
jj --no-pager st; jj --no-pager diff --from desktop@ --to default@ --summary; jj --no-pager log -r "(::desktop@ ~ ::default@) & ~empty()" --no-graph; pgrep owned old-checkout processes
```

**Notes**

Before retirement both tracked trees clean/identical; no unique nonempty changes and no owned processes in old checkout. Only its empty working leaf is outside main ancestry. Native app artifacts retained first; unrelated main34614/34630 development service/RustFS and95352 Vite remain running.
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
jj workspace forget desktop; trash -v /Users/stevejuma/code/lapismd/jj-store-desktop; trash -v /private/tmp/jj-desktop-integration-native.OUg8YL
```

**Notes**

Only desktop registration removed. Exact checkout and disposable native smoke state moved recoverably to ~/.Trash/jj-store-desktop and ~/.Trash/jj-desktop-integration-native.OUg8YL. Other workspaces retained; old checkout path absent.
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
CUA getApp /Users/stevejuma/code/lapismd/jj-store/desktop/target/debug/bundle/macos/jj-store.app after retirement; verify process paths and SHA256 state equality
```

**Notes**

Clean native launch left running from main, still57653; host is detached from shell with parent1 and owns its current service/RustFS. No process executable references removed checkout. Persistent Application Support state and independent main UI fixture remain retained.
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
pnpm exec prettier --write spec/src/milestones.md; pnpm check:progress; pnpm spec:validate; pnpm exec prettier --check spec/src/milestones.md
```

**Notes**

Retirement evidence validates125tasks and5validators/22chapters/152requirements; canonical ledger formatting passes. Documentation-only cleanup evidence requires no further product-suite escalation.
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

<!-- evidence-field:scope markdown -->
## Scope

Only verified workspace landing, usable debug-app relocation and retirement are complete. M17 remains In progress: native drag/double-click, accessibility, render handshake, folder bridge, dirty-edit protection and signed/release/DMG flows retain E-DESK-003 pending gates. Root aggregates/full Rust and unrelated UI/storage suites out of scope. No installation reset, user repository migration, public release or visual baseline approval.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Continue M17 from main; use retained desktop/resources and debug app for local iteration. Keep remaining native and distribution gates explicit.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


