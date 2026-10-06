---
schema: 1
id: E-DOC-002
---

# E-DOC-002

<!-- evidence-field:id markdown -->
## Id

E-DOC-002
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M19
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-04
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change ptltynkp
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Help sidebar alignment and workspace preview
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; Svelte 5.57.1; Playwright 1.63.0 Chromium and WebKit; existing local Design Core validation hook.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No repository, API, browser or documentation manifest format change.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- web/src/lib/ui/store-help/StoreHelp.svelte
- web/src/lib/ui/store-application/StoreApplication.svelte
- docs/maintenance/coverage.json
- spec/src/ui.md
- spec/src/api.md
- spec/src/documentation.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/browser/help.spec.ts#Help is available from the installation sidebar
  before a repository exists
<!-- /evidence-field -->

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
jj --no-pager st; inspect Community navigation, Design Core Button/AppShell and official shadcn-svelte Button documentation; live browser geometry comparison
```

**Notes**

Clean working copy before edits. Help used a 48x36 button at x=0 with icon center x=19; Projects used 32x32 at x=8 with center x=24. No product mutation during measurement.
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
JJ_STORE_DEV_PORT=7484 JJ_STORE_DEV_BACKEND_PORT=7486 JJ_STORE_DEV_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs pnpm dev:restart (CommandLineTools DEVELOPER_DIR/CXXFLAGS)
```

**Notes**

Fresh isolated documentation-workspace initialization built web/docs/native binaries and seeded code plus agent activity. Sample review seeding failed: development review seed could not read src/lib.rs. Shutdown logged its existing five-second limit. Incomplete fixture preserved without reset; 7460/7462 and other deployments untouched. No complete fixture marker fabricated.
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
pnpm web:check; pnpm catalog:check; pnpm check:ui-ownership; pnpm check:architecture
```

**Notes**

Zero web/catalog errors or warnings; 182 catalog and 931 architecture modules; ownership valid.
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
pnpm exec vite build --config web/vite.config.ts; node scripts/web-manifest.mjs; node scripts/docs.mjs stage; pnpm exec playwright test --config web/playwright.docs.config.ts -g "Help is available from the installation sidebar"
```

**Notes**

Fresh production build and docs stage; 2/2 exact Chromium/WebKit sidebar Help tests pass, including focus return and keyboard boundary. 117 docs assets, 9 rendered diagrams, valid links/anchors. Existing foundational CSS :global and bundle-size warnings remain; no visual baselines updated.
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
pnpm docs:review application <specific spacing review>; pnpm docs:review; changed-file Prettier --write
```

**Notes**

Reviewed browsing/editing/settings prose; workflow unchanged. Application fingerprint updated and all source reviews current.
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
JJ_STORE_DEVELOPMENT_STATE_ROOT=<documentation-workspace>/.local/dev-demo ./target/debug/jj-store run --config <documentation-workspace>/.local/dev-demo/local/server.toml --format=json; curl app/help; cold browser reload and collapsed/expanded geometry comparison
```

**Notes**

Direct native built preview recovered at loopback 7484 using the preserved partial fixture; both app and Help HTTP 200. Help/Projects both 32x32 at x=8 with icon center x=24 collapsed; both 264x36 at x=12 with icon center x=31 expanded in 288px sidebar. Startup fixture remains partial; this is a built preview, not managed HMR mode.
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
pnpm spec:first; pnpm spec:validate; pnpm check:progress; changed-file pnpm exec prettier --check
```

**Notes**

Spec-first covers 3 protected files and 4 canonical chapters; 5 validators, 23 chapters, 171 requirements; 137 tasks; changed-file formatting passes. Existing 414 evidence entries preserved. Live corrected sidebar screenshot saved in ignored workspace fixture; no baseline update.
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

Narrow application-owned Help geometry. All 414 prior evidence entries preserved. M19 remains Partial; registry/native-host release gates remain in E-DOC-001. Existing cold exact browser test and live before/after geometry cover the change without adding an implementation-mirroring test. Rust, root aggregates, full browser/Storybook/storage suites and visual baselines are out of scope. Development fixture seeding failure remains explicit and is not a Help acceptance failure.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Use the built workspace preview on 7484 to review Help. Repair or resume the incomplete demo seeding separately before using managed dev:ui for this fixture.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


