---
schema: 1
id: E-UI-100
---

# E-UI-100 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 6
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/RepositoryEntryControls.stories.svelte catalog/components/store-application/StoreLocalFolderDialog.stories.svelte catalog/components/store-application/StoreRepositoryConversion.stories.svelte catalog/components/store-work/ProjectsDirectory.stories.svelte (before)
```

**Notes**

before-stories.log: 6 failed / 7 passed; exact six requested UI regressions. Stories remain visual-pending.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/projects-repositories.test.ts (before)
```

**Notes**

before-units.log: 1 failed / 4 passed; retry clears the original summary error episode too early.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/projects-repositories.test.ts (after)
```

**Notes**

after-units.log: 5/5 pass; errors persist through queued and active reads, then clear after successful recovery.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/RepositoryEntryControls.stories.svelte catalog/components/store-application/StoreLocalFolderDialog.stories.svelte catalog/components/store-application/StoreRepositoryConversion.stories.svelte catalog/components/store-work/ProjectsDirectory.stories.svelte (initial after)
```

**Notes**

after-stories.log: 11 pass / 2 fail. Test selected both header and footer Close controls; the unclosed dialog blocked the next story. Repair uses an unambiguous footer control and keeps focus assertions.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:check (initial)
```

**Notes**

catalog-check.log: two TypeScript errors from Playwright-only exact option in Testing Library selectors. Removed the unsupported option; no production contract weakened.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/RepositoryEntryControls.stories.svelte (after selector repair)
```

**Notes**

entry-stories.log: 2/2 interaction/accessibility tests pass, including focus into dialogs, return to menu trigger and Escape.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/RepositoryEntryControls.stories.svelte catalog/components/store-application/StoreLocalFolderDialog.stories.svelte catalog/components/store-application/StoreRepositoryConversion.stories.svelte catalog/components/store-work/ProjectsDirectory.stories.svelte (requested full labels and GitBranchPlus icon)
```

**Notes**

final-focused-stories.log: 13/13 pass in a fresh Chromium/Vite run.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/projects-repositories.test.ts web/tests/vite/notification-service.test.ts
```

**Notes**

focused-units.log: 11/11 pass. Stable summary identities, successful independent metadata and notification dismissal/recovery retained.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/RepositoryEntryControls.stories.svelte -t OneRepositoryMenu (initial width assertion)
```

**Notes**

before-menu-width.log: test-authoring error selected the trailing Svelte comment and measured zero rectangles. Corrected it to measure the actual Repository label text node.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/RepositoryEntryControls.stories.svelte -t OneRepositoryMenu (corrected before width)
```

**Notes**

before-menu-width2.log: 1 failed / 1 skipped; full labels each wrap onto two lines at the default menu width.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/RepositoryEntryControls.stories.svelte (after width)
```

**Notes**

after-menu-width.log: 2/2 pass. Actual rendered labels occupy one text line and do not overflow; both repository add menus use 16rem bounded by viewport.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook <74 directly affected story files> (first run)
```

**Notes**

affected-stories.log: user refinement of labels/icon interrupted the in-progress run. Its exact process group was stopped; no completion or pass claimed. One incomplete validation process is counted; no product assertion failure count or pass inferred.
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
python3 (load .local/repository-ui-feedback/affected-stories.txt and invoke pnpm exec vitest run --config catalog/vitest.config.ts --project storybook for all 74 paths)
```

**Notes**

affected-stories-final.log: 53/74 files completed and 289 tests passed before one unhandled browser-connection closure while starting DesktopChrome. Complete run failed; no full coverage or success inferred. Retry that target separately, then every affected file in smaller fresh-browser batches.
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
pnpm web:check; pnpm catalog:check (corrected and final)
```

**Notes**

web-check2/web-check-final and catalog-check2/catalog-check3/catalog-check-final: zero errors/warnings; 190 catalog modules. Earlier web-check.log also passes.
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
pnpm check:ui-ownership; pnpm check:architecture
```

**Notes**

ownership.log and architecture.log pass; 950 production modules. Community and public Design Core reference composition inspected; existing owned CSS moved to the field import so dialogs receive form geometry without requiring a WorkStore page.
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
pnpm web:build (initial, requested labels and final width)
```

**Notes**

web-build.log, web-build2.log and web-build-final.log pass with the existing bundle-size/toolchain warnings. No Rust build/test or browser fixture regeneration.
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
pnpm exec playwright test --config .local/repository-ui-feedback/playwright.config.ts native-repository-creation.spec.ts repository-conversion.spec.ts sidebar-repository-actions.spec.ts notifications.spec.ts
```

**Notes**

cold-browser-final.log: 8/8 pass after the final web build using a fresh isolated preview process on 7458 and existing captured responses. Creation capability gates, inline name validation, exact intent/reload/retry, conversion preflight/reports, sidebar/collapsed actions and operational toast lifetimes pass.
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
python3 (pnpm exec prettier --write/--check all changed files except historical milestones.md)
```

**Notes**

format-final.log passes. Preserve historical ledger records rather than reformatting them.
<!-- /run -->
<!-- /evidence-field -->

