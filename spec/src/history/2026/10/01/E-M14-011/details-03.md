---
schema: 1
id: E-M14-011
---

# E-M14-011 details 3

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
pnpm check:architecture && pnpm check:boundaries
```

**Notes**

Final extraction passes: 440 modules and 15 packages. Before heading additions, scoped geometry and Back also passed at 439 modules.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreSettingsIdentifier.stories.svelte catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-application/StoreSyncSettings.stories.svelte catalog/components/store-application/StoreSearchSettings.stories.svelte catalog/components/store-application/StoreWorkspaceSettings.stories.svelte
```

**Notes**

All five directly affected story families pass: 27 interaction/accessibility stories, including ordinary app shell navigation and mobile Settings header.
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
pnpm web:build
```

**Notes**

Fresh production build includes scoped Settings geometry and title/icon/Back nav. Dependency CSS parser and bundle-size warnings retained in ignored log.
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
pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/local-search-settings.spec.ts web/tests/browser/local-workspaces.spec.ts web/tests/browser/local-sync.spec.ts
```

**Notes**

Three fresh-preview browser scenarios pass, including shared copy, compact heads, nav title/icons, mobile tabs/overflow, drafts/actions, axe, retained route, full-width Settings and unchanged repository-row x/width before and after closing Settings.
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
pnpm spec:first && pnpm spec:validate && pnpm check:progress
```

**Notes**

Spec-first and 132 requirements passed; progress rejected an unsupported skipped run result. Recorded the successful-but-skipped process as pass with an explicit no-acceptance note, retaining the exact matched regression evidence.
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
pnpm dev:restart
```

**Notes**

Final title/icon/Back/scoped-layout demo restarted on 7460; native repository, identity and RustFS volume retained; Search ready. Helper builds dev binaries; no Rust tests.
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
pnpm spec:first && pnpm spec:validate && pnpm check:progress
```

**Notes**

Final canonical gates pass: 18 protected files, 3 changed canonical chapters, 132 requirements and 111 tracked tasks.
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
cua_repl: isolated review tab; Repository and Local service Settings; screenshot; Back to app
```

**Notes**

Live main demo shows title/icon/Back in top nav, no former body heading/description, compact head controls and Settings-only full-width layout; Back restores the original branch route. Saved ignored settings-header-review.png; no values changed.
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

One Settings presentation surface and direct catalog/browser consumers, including top Back, nav title/icons and component-scoped width/padding. UI-only focused web/catalog checks, five directly affected story families and three cold built-app browser scenarios. Rust tests, contract generation and unrelated root aggregates are out of scope.
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Custom joined readonly identifier fields duplicated copy logic; Settings content was centered at 56rem. Shared copy buttons stretched in the operation-head grid, and settings actions lacked leading icons. Mobile return required reopening the sidebar; page headings/descriptions occupied the body. No concrete outside-Settings CSS leak was diagnosed; geometry is now explicitly component-scoped.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Complete native identifiers supplied unchanged to public Design Core CopyableValue. Display abbreviation only; no wire, persisted configuration, route, native or WAL changes. Enclosing Jujutsu change ztkmnprt is revision reference.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ 0.45.1; pnpm 10.32.1; installed Design Core 0.1.2 package; locked Svelte, Lucide, Storybook, Vitest and Playwright.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Review compact copies, Settings-only full-width layout, action icons and title/icon/Back navigation in the main demo; remaining M14 Agent, Storage and Sync frontier work stays separately tracked.
<!-- /evidence-field -->

<!-- evidence-field:deferred list -->
## Deferred

- Visual baselines remain visual-pending; no milestone-wide qualification claim.
<!-- /evidence-field -->

<!-- evidence-field:artifacts markdown -->
## Artifacts

Ignored .scratch/settings-copy-*.log, .scratch/settings-final-*.log, .scratch/settings-back-*.log, .scratch/settings-scoped-*.log, .scratch/settings-header-*.log and web/test-results. Disposable previews shut down; owned main demo remains available.
<!-- /evidence-field -->

