---
schema: 1
id: E-UI-033
---

# E-UI-033 details 2

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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreRepositoryNavigation.stories.svelte -t RightRailNavigation
```

**Notes**

Before integration: one failed and one skipped; existing tabs still used Activity, Files, Workspaces and Review.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreRepositoryNavigation.stories.svelte -t RightRailNavigation
```

**Notes**

After integration: one passed and one skipped. Left rail has no repository view links; right-rail icon rows select matching tabs.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreRepositoryNavigation.stories.svelte
```

**Notes**

First overflow acceptance: right-rail story passed, overflow story failed because the Tabs root retained column layout, leaving its tab list at intrinsic width.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreRepositoryNavigation.stories.svelte -t TrailingSectionsOverflowAndReturn
```

**Notes**

Diagnostic retry after constraining list width still failed. Measured 188px Tabs root in column direction with a 456px tab list; explicitly composing the navigation root as a row resolves the fitting boundary.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreRepositoryNavigation.stories.svelte
```

**Notes**

Both interactions and blocking accessibility checks pass after the row composition. At 220px Overview stays visible, trailing sections enter More, hidden Reviews retains current-page semantics and returns as the active tab when width is restored.
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
pnpm web:check
```

**Notes**

Zero errors and warnings in production web diagnostics.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 7
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:check
```

**Notes**

Seven type errors in the new story: Testing Library role options do not accept exact. Removed those unsupported options; role names remain exact by default.
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
pnpm catalog:check
```

**Notes**

Zero errors and warnings; catalog ownership boundary check passes across 72 modules.
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
pnpm check:architecture
```

**Notes**

Production, story and browser module budgets pass across 282 modules.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreRepositoryNavigation.stories.svelte catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte catalog/components/store-application/StoreFileEditor.stories.svelte catalog/components/store-application/RepositoryReview.stories.svelte catalog/components/store-application/StoreWorkspaces.stories.svelte catalog/components/store-application/RepositoryLoading.stories.svelte
```

**Notes**

All 35 directly affected interactions and blocking accessibility checks pass in seven files. Includes Settings substitution, mobile right-rail return, editor navigation, selection details, loading, workspace context and layout persistence. Fresh Vitest browser process.
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
pnpm web:build
```

**Notes**

Cold UI bundle succeeds; existing non-fatal Design Core :global minification and chunk-size warnings remain. No Rust build or fixture regeneration.
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
pnpm exec playwright test --config .scratch/workspace-browser.config.mts repository-navigation.spec.ts
```

**Notes**

Two new cold built-bundle browser tests pass. Verifies exact tab order, right-rail selection, empty left primary navigation, snapshot/workspace hashes, 390px keyboard overflow, focus restoration, cold reload, mobile stage return and resize to 1400px. Fresh isolated UI preview uses existing captured responses.
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
pnpm exec playwright test --config .scratch/workspace-browser.config.mts navigation.spec.ts workspaces.spec.ts reads.spec.ts mutations.spec.ts
```

**Notes**

All 22 selected cold browser tests pass. The navigation.spec.ts filter also selects repository-navigation.spec.ts; this count includes its two tests. Native reads, source rendering, mutation receipt recovery, reload/history, exact workspace context and stale/missing workspace behavior remain covered.
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
node .scratch/restart-workspace-ui.mjs
```

**Notes**

Rebuilt UI served by existing explicit development binaries at http://127.0.0.1:7460/. Owned server PID 91625. Existing repository 1bdb89c409c1b891ac7f180c01a265d5 and embedded RustFS data preserved; no reset, reseed, install or unrelated service restart.
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
CUA live browser acceptance at http://127.0.0.1:7460/
```

**Notes**

Reloaded the in-app preview and verified Overview first, six pill tabs, Workspace picker and Community-style Details icon rows. Selecting Branches preserves at_op and workspace=demo. Screenshot saved at ignored .scratch/repository-navigation.png; preview tab retained.
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
pnpm exec prettier --write changed navigation production, story, browser and canonical files
```

**Notes**

Changed source and canonical chapters formatted; exact paths are listed in files.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreRepositoryNavigation.stories.svelte -t RightRailNavigation
```

**Notes**

User requested a minimal rail with the page heading first. Added heading-order and absence-of-section/dl assertions; before cleanup this exact regression failed once.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreRepositoryNavigation.stories.svelte -t RightRailNavigation
```

**Notes**

After cleanup: heading and create action precede Details; no generic, selected-metadata or secondary-action panels remain. One passed, one skipped.
<!-- /run -->
<!-- /evidence-field -->

