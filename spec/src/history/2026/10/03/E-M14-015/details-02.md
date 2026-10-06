---
schema: 1
id: E-M14-015
---

# E-M14-015 details 2

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
pnpm dev:ui
```

**Notes**

Restarted main UI on literal loopback 7460, backend 7462. Existing native binaries and fixture reused; no reset or Rust rebuild.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-application/SettingsNavigation.stories.svelte (before)
```

**Notes**

Missing picker, refresh placement and old Appearance introduction; .local/settings-navigation-before.log. First cold optimizer import/timing issues corrected before the recorded baseline.
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
pnpm test:storybook catalog/components/store-application/StoreWorkspaceSettings.stories.svelte (before)
```

**Notes**

5 pass/1 fail: folder form was outside bordered rows; .local/settings-folders-before.log.
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
pnpm check:architecture (during shell integration)
```

**Notes**

StoreApplication counted 322 then 321 lines against its existing 320 budget; redundant title derivation removed, budget unchanged.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-application/SettingsNavigation.stories.svelte catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-application/StoreSearchSettings.stories.svelte catalog/components/store-application/StoreStorageSettings.stories.svelte catalog/components/store-application/StoreSyncSettings.stories.svelte catalog/components/store-application/StoreWorkspaceSettings.stories.svelte catalog/components/store-application/StoreRepositoryPicker.stories.svelte catalog/components/store-appearance/StoreAppearance.stories.svelte catalog/components/store-application/NativeReviewDetail.stories.svelte (intermediate)
```

**Notes**

Initial run exposed four invalid dd role overrides via axe and one obsolete Appearance introduction assertion. Status/alert roles moved inside dd; intro test aligned with requested removal. Later run passed44/45 and exposed a1px ghost border; explicit borderless trigger fixed that user requirement. No acceptance threshold loosened.
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
pnpm test:storybook catalog/components/store-application/SettingsNavigation.stories.svelte catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-application/StoreSearchSettings.stories.svelte catalog/components/store-application/StoreStorageSettings.stories.svelte catalog/components/store-application/StoreSyncSettings.stories.svelte catalog/components/store-application/StoreWorkspaceSettings.stories.svelte catalog/components/store-application/StoreRepositoryPicker.stories.svelte catalog/components/store-appearance/StoreAppearance.stories.svelte catalog/components/store-application/NativeReviewDetail.stories.svelte
```

**Notes**

45/45 cold Chromium stories, interactions and configured accessibility acceptance. Equal-width vertical action checks and unchanged Appearance mode layout included. .local/settings-all-stories-final.log.
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
pnpm test:storybook catalog/components/store-application/StoreLocalFolderDialog.stories.svelte
```

**Notes**

1/1 cold story: visible1px path border, group fills control column, public status Alert and retained typing/inspection availability. .local/settings-dialog-story.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-search-settings.test.ts
```

**Notes**

6/6; original Search settings revision is retained with dirty drafts; cancellation adopts refreshed state. .local/settings-search-unit.log.
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
pnpm web:check; pnpm catalog:check; pnpm check:architecture; pnpm check:ui-ownership
```

**Notes**

Both Svelte checks0 errors/0 warnings;163 catalog and844 architecture modules; UI ownership passes. Scoped StoreWorkField bordered option leaves existing consumers at their prior default.
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
pnpm web:build
```

**Notes**

Fresh production build/asset manifest; existing large-chunk warning retained. .local/settings-build-final.log.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec playwright test --config .local/settings-playwright.config.ts local-search-settings.spec.ts local-storage.spec.ts local-sync.spec.ts local-workspaces.spec.ts (during iteration)
```

**Notes**

Earlier assertions expected silent Search draft revision adoption and Storage readiness while Maintenance remained selected. Confirmed production unit contract, retained strict conflict handling, asserted draft preservation and explicit Cancel/re-entry; retained active tab assertion before selecting Overview. Final original scenarios rerun.
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
pnpm exec playwright test --config .local/settings-playwright.config.ts local-search-settings.spec.ts local-storage.spec.ts local-sync.spec.ts local-workspaces.spec.ts
```

**Notes**

6/6 cold built-UI tests with prebuilt browser_preview, no Rust compilation: toolbar tooltip, repo picker and dirty guard, mobile, Search Save/Apply conflict flow, exact Storage pending target, explicit context from Projects, Sync/reconciliation and folder drafts/links plus axe. .local/settings-browser-final.log.
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
CUA live main UI preview at http://127.0.0.1:7460/
```

**Notes**

Verified centered compact picker, nav refresh, rounded rows, equal-width action columns and info notice. Folder border1px; final group/control both596.398px after correcting its fit-content default with explicit layout width. No live repository mutation. Screenshot .local/settings-final.jpg; visual baseline not approved.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: fail
failures: 8
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --check changed Settings files; pnpm check:progress (first ledger attempt)
```

**Notes**

Formatting passes. Ledger checker rejected seven test references missing named # anchors and a passing entry with unrelated milestone/visual gaps in remaining. Added named targets and moved out-of-scope gates to scope without changing milestone status.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress (named story references)
```

**Notes**

Ledger named-test scanner supports function/test definitions but not Svelte CSF story names. Story evidence remains in files and actual runs; tests lists executable browser test definitions. Checker unchanged.
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
pnpm exec prettier --check changed Settings files; pnpm exec prettier --write spec/src/milestones.md; pnpm check:progress; pnpm spec:first; pnpm spec:validate (final)
```

**Notes**

Formatting passes;125-task evidence bookkeeping;35 protected files and3 canonical chapters;5 validators,22 chapters and152 requirements. No milestone status changed.
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

Only Settings, direct form/picker consumers and Reviews action alignment; Appearance action layout excluded from column styling. Shared native chrome work excluded. Root aggregate checks, Rust suites, unrelated UI suites and schema generation are out of scope under the targeted UI policy. Visual baselines remain visual-pending, and M14 full integrated qualification and milestone status are unchanged.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Retain focused Settings regression coverage and continue separately governed remaining M14 work.
<!-- /evidence-field -->

