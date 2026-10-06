---
schema: 1
id: E-DESK-003
---

# E-DESK-003 details 3

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
pnpm test:storybook catalog/components/store-work/{ProjectsActivity,ProjectsActivityControls,ProjectsAgents,ProjectsDirectory,ProjectsFilters,ProjectsHub,ProjectsRepositoryActions,ProjectsReviewDetail,TaskScroll}.stories.svelte catalog/components/store-application/{DesktopChrome,StoreApplication,StoreWorkspaceSettings,StoreSettingsIdentifier}.stories.svelte catalog/components/store-appearance/StoreAppearance.stories.svelte
```

**Notes**

68/68 in14families on a fresh Vitest browser/Vite process, with normal accessibility assertions. All9 consumers of the changed shared Projects fixture plus application/native/settings/Appearance consumers; /tmp/jj-desktop-integration-all-consumers.log. Wider direct fixture lane required by the shared harness change.
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
pnpm catalog:test catalog/components/community-app-shell/shell-header-geometry.test.ts web/tests/vite/appearance.test.ts web/tests/vite/thread-navigation-integration.test.ts
```

**Notes**

16/16 exact geometry, Appearance and navigation units; /tmp/jj-desktop-integration-ui-units.log.
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
pnpm web:check; pnpm catalog:check (final after fixture repair)
```

**Notes**

0 errors/0 warnings in both scopes;156 catalog modules. Final fixture check repeated after repair; /tmp/jj-desktop-integration-{web-check-final,final-catalog-check}.log.
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
pnpm check:architecture; pnpm check:boundaries; pnpm check:ui-ownership
```

**Notes**

823 modules within existing budgets;16 Cargo package boundaries; UI source ownership validates22chapters/152requirements.
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
pnpm web:build (refreshed dependencies)
```

**Notes**

Fresh212-asset production bundle; inherited Design Core :global CSS minifier and large-chunk advisories retained; /tmp/jj-desktop-integration-web-build-final.log.
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
node scripts/desktop.mjs stage --server-dir /Users/stevejuma/code/lapismd/jj-store/.local/desktop-integration-target/debug --rustfs-runtime /Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs; pnpm desktop:bundle
```

**Notes**

Paired explicit executables/current built UI staged; official Tauri CLI builds debug ARM64 app, not a signed release or DMG; /tmp/jj-desktop-integration-app-build.log.
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
JJ_STORE_DESKTOP_TEST_RESOURCES=/Users/stevejuma/code/lapismd/jj-store-desktop/desktop/resources cargo test --manifest-path desktop/Cargo.toml --locked --no-default-features --test managed_runtime -- --ignored --exact real_managed_runtime_restarts_preserves_origin_and_rejects_occupied_port
```

**Notes**

1/1 cold real managed RustFS host-runtime proof: stable origin, occupied-port refusal, restart, unchanged installation identity/configuration and removed socket; /tmp/jj-desktop-integration-real-host.log.
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
CUA Cmd+Q old app; JJ_STORE_DESKTOP_DATA_DIR=/private/tmp/jj-desktop-integration-native.OUg8YL desktop/target/debug/bundle/macos/jj-store.app/Contents/MacOS/jj-store-desktop; CUA Projects/Settings/close/Quit; repeat launch at same test state
```

**Notes**

Old owned60321/60346/60348 process tree exited. Actual rebuilt WKWebView shows latest Agents/search UI and native controls with no separate titlebar/title. Local service Ready; Close preserves listening service and Settings, but CUA observation reactivates the window so visibility timing is unqualified. Cmd+Q returns clean exit, children/61046 listener absent; cold restart retains61046 and renders Projects again; second Quit clean. App-quit/noWindowsAvailable CUA results were corroborated by process/session checks. /tmp/jj-desktop-integration-native-{smoke,restart}.log.
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
pnpm exec prettier --check changed production/catalog/spec files
```

**Notes**

Resolved chapters, header styles, application props and fixture use canonical formatting.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress (first E-DESK-003 bookkeeping)
```

**Notes**

Named-test checker recognizes fn/def/test(), not generated Svelte story names. Kept the exact story command/count in runs and its source in files; named Rust regressions remain referenced. No checker change.
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
pnpm spec:first (first E-DESK-003 bookkeeping)
```

**Notes**

Shared catalog fixture maps to UI and API chapters; added the optional-review-reader explanation in API as well as UI. No public contract change.
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
pnpm spec:validate
```

**Notes**

5validators/22chapters/152requirements pass before final bookkeeping corrections.
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
pnpm check:progress; pnpm spec:first; pnpm spec:validate; pnpm exec prettier --check changed production/catalog/spec files (final)
```

**Notes**

125tasks; spec-first3protected files/3canonical chapters;5validators/22chapters/152requirements; canonical formatting passes. All298 existing main evidence entries retain identical parsed contents. Final web check after fixture repair and unused-type removal also0errors/0warnings: /tmp/jj-desktop-integration-final-web-check.log.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Native drag/double-click gestures, full native accessibility, macOS
  double-click preference, folder bridge, first-render handshake, dirty-edit
  protection and release/DMG gates remain incomplete; no visual baseline
  approval.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Focused CLI/service lifecycle, native host, cold real-runtime/package smoke and all directly affected UI consumers. Full regular foreground and governance suites qualify shared extracted startup/scanners; all14 direct UI families qualify the shared fixture repair. Root aggregates, full Rust workspace, unrelated storage/browser suites and release qualification remain out of scope.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Land verified rebased stack in clean main, preserve paired app/resources and persistent Application Support state, verify relocation, then forget desktop and move its checkout to Trash. Continue the pending M17 gates from main.
<!-- /evidence-field -->

