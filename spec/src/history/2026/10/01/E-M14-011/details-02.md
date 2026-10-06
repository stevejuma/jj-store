---
schema: 1
id: E-M14-011
---

# E-M14-011 details 2

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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreSettingsIdentifier.stories.svelte
```

**Notes**

Before replacement, the compact-control regression finds the custom readonly textbox.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreSettingsIdentifier.stories.svelte
```

**Notes**

Two focused stories pass: full value copied via keyboard, abbreviated compact display, denied clipboard feedback/retry and unavailable values.
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
pnpm web:check; pnpm catalog:check
```

**Notes**

Initial and final checks after icon additions pass with zero errors/warnings; 107 catalog modules.
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
pnpm check:architecture; pnpm check:boundaries
```

**Notes**

439 production web modules; 15 package boundaries. Final rerun after decorative icon additions passes.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreSettingsIdentifier.stories.svelte catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-application/StoreSyncSettings.stories.svelte
```

**Notes**

18 interaction/accessibility stories pass for copy replacement and full-width Settings.
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

Fresh production bundles pass after shared-control replacement, width correction and icon additions. Bundle-size and dependency stylesheet parser warnings remain visible in ignored logs.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/local-search-settings.spec.ts web/tests/browser/local-workspaces.spec.ts web/tests/browser/local-sync.spec.ts
```

**Notes**

New copy-control assertions exposed absent Search health and null Sync frontier fixture values; Repository passed. Added explicit typed-shaped endpoint/frontier data in the two owning tests; shared fixture infrastructure unchanged.
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
pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/local-search-settings.spec.ts web/tests/browser/local-sync.spec.ts
```

**Notes**

Both exact scenarios pass after supplying the identifiers required by the new assertions; mobile overflow and axe included.
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
pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/local-workspaces.spec.ts
```

**Notes**

Final natural-width Operation head button under 200px, full-width heading alignment, shared Repository ID button, folder draft/actions, mobile overflow and axe pass.
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
pnpm dev:restart
```

**Notes**

Owned main demo rebuilt/restarted after compact copy and full-width layout, and again after removing operation-head grid stretching. Repository, identity and RustFS data preserved; Search ready. The required helper builds dev binaries; no Rust test lane.
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
cua_repl: reload owned review tab; profile Settings; Repository; getAXState and screenshot
```

**Notes**

Live review confirms shared abbreviated copy controls and full-width content. Initial screenshot revealed grid-stretched operation heads; changed stack alignment and reloaded the final compact control. No live values changed.
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
pnpm exec prettier --write <changed Svelte/CSS/TypeScript/Markdown files>
```

**Notes**

Changed presentation and browser assertions formatted.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreSettingsIdentifier.stories.svelte catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-application/StoreSyncSettings.stories.svelte catalog/components/store-application/StoreSearchSettings.stories.svelte catalog/components/store-application/StoreWorkspaceSettings.stories.svelte
```

**Notes**

Earlier final run had one module-import failure while Vite optimized dependencies and reloaded the suite; 15 tests passed. Fresh unchanged rerun passed all 27 stories; ignored settings-final-stories logs retain both results.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreApplication.stories.svelte -t '^Settings mobile$'
```

**Notes**

Command exited successfully but all stories were skipped because the pattern did not match. This is not acceptance evidence.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreApplication.stories.svelte -t 'Settings.*[Mm]obile'
```

**Notes**

Before top Back implementation the mobile story could not find Back to app. After adding the header-title regression, it failed again because Local service heading remained in the body, outside page navigation.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreApplication.stories.svelte -t 'Settings.*[Mm]obile'
```

**Notes**

Exact mobile regression passes with top Back first, heading and decorative icon in nav, removed body description and retained app on close. Passed again after extracting the play function into its owning helper.
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
pnpm web:check; pnpm catalog:check
```

**Notes**

After scoped layout and top Back: zero errors/warnings. After nav title/icons: zero errors/warnings, 107 catalog modules. Extraction catalog rerun passes, 108 modules.
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
pnpm check:architecture && pnpm check:boundaries
```

**Notes**

Nav regression grew the shell story to 422 counted lines, exceeding its 400-line limit; boundaries did not run. Extracted the complete mobile play function, preserving the story and assertions; no budget relaxation.
<!-- /run -->
<!-- /evidence-field -->

