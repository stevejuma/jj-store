---
schema: 1
id: E-M15-025
---

# E-M15-025 details 3

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
pnpm exec playwright test --config /tmp/jj-thread-fix-playwright.config.mjs threads-alignment.spec.ts (rerun 5)
```

**Notes**

Cold production browser plus axe passed before latest settings/sidebar steering; /tmp/jj-thread-fix-browser5.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-threads/StoreThreadAlignment.stories.svelte (settings correction before)
```

**Notes**

2 / 3 pass; requested settings lifecycle controls absent. /tmp/jj-thread-fix-settings-correction-before.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-threads
```

**Notes**

8 / 8 pass after returning icon-led lifecycle controls to Community management/danger groups. /tmp/jj-thread-fix-settings-correction-stories.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-threads.test.ts (post recovery before, two runs)
```

**Notes**

21 / 23 pass in each run. First proves repository switch blockage and corrects ApiError display-text expectation; second proves rejected body unmounts the draft. /tmp/jj-thread-fix-post-before.log, post-before2.log.
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
pnpm web:check; pnpm catalog:check (settings and single-panel iterations)
```

**Notes**

0 errors/warnings and 121-module boundary. /tmp/jj-thread-fix-web-check5.log, web-check6.log, catalog-check3.log, catalog-check4.log.
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
pnpm check:architecture; pnpm check:ui-ownership (settings)
```

**Notes**

626 modules within source budgets; complete copy/provenance closure passes. /tmp/jj-thread-fix-architecture5.log, ownership2.log.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/store-shell-layout.test.ts (before)
```

**Notes**

Independent outer details rail panel absent. /tmp/jj-thread-fix-shell-before.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/store-shell-layout.test.ts
```

**Notes**

Independent standard pane and outer rail widths/open states survive remount. /tmp/jj-thread-fix-shell-after.log.
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
pnpm check:architecture (sidebar first iteration)
```

**Notes**

StoreApplication exceeds 320-line budget at 335. Extracted owned RightPanels composition; no budget relaxation. /tmp/jj-thread-fix-architecture6.log.
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
pnpm check:architecture (sidebar extraction)
```

**Notes**

627 modules pass unchanged budgets. /tmp/jj-thread-fix-architecture7.log.
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
pnpm web:build (settings/sidebar builds 5-8)
```

**Notes**

Four fresh asset builds pass, existing chunk-size advisory only. /tmp/jj-thread-fix-build5.log through build8.log. No Rust build or main restart.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 8
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-threads/StoreThreadAlignment.stories.svelte (independent sidebar iterations 1-3)
```

**Notes**

First: 3 failed interactions plus 3 diagnostics from missed onSelectPage extraction; second: 1 failed test trying to resize a constrained pane; third: 1 failed assertion for right-sidebar keyboard direction. Preserve callback guard; test simultaneous constrained visibility and resize before opening the rail. /tmp/jj-thread-fix-independent-panels-stories.log through independent-panels-stories3.log.
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
pnpm web:check; pnpm catalog:check (sidebar extraction initial)
```

**Notes**

1 Svelte initial-prop-capture warning; explicit untrack matches stable shell/controller lifetime. /tmp/jj-thread-fix-web-check7.log, catalog-check5.log.
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
pnpm exec tsx --test web/tests/layout-storage.test.ts
```

**Notes**

4 / 4 versioned layout/storage fallback tests. /tmp/jj-thread-fix-layout-storage.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-threads.test.ts web/tests/vite/local-browser-merge.test.ts web/tests/vite/store-shell-layout.test.ts
```

**Notes**

28 / 28 direct controller/profile/layout units. /tmp/jj-thread-fix-direct-units2.log.
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
pnpm exec prettier --write (changed source/spec/provenance increments 10-16)
```

**Notes**

Scoped formatting; /tmp/jj-thread-fix-format10.log through format16.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-threads/StoreThreadAlignment.stories.svelte (independent panes final)
```

**Notes**

3 / 3 pass, simultaneous constrained-pane visibility, independent global toggle and retained Thread width. /tmp/jj-thread-fix-independent-panels-stories4.log.
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
pnpm web:check; pnpm catalog:check (independent panes)
```

**Notes**

0 errors/warnings; 121-module catalog boundary. /tmp/jj-thread-fix-web-check8.log, catalog-check6.log.
<!-- /run -->
<!-- /evidence-field -->

