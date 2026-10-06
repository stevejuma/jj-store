---
schema: 1
id: E-M18-002
---

# E-M18-002 details 2

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
jj --no-pager st; jj describe -m "Align agent capture settings with settings navigation and rows"
```

**Notes**

Clean preview change; main and other services preserved; enclosing change zmyqspmo
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreCaptureSettings.stories.svelte -t "Shared settings rows and action columns"; repeat with -t shared-settings-rows
```

**Notes**

Both name filters skipped all stories, so they are not acceptance evidence; reran complete affected story file
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreCaptureSettings.stories.svelte (before implementation)
```

**Notes**

7 existing stories passed; added shared-row/Agents regression failed because Agents heading was absent
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreCaptureSettings.stories.svelte catalog/components/store-application/SettingsNavigation.stories.svelte (first after)
```

**Notes**

Navigation4 passed; Capture suite import failed transiently in Vite/Tailwind virtual CSS transformation; independently repeated affected Capture suite successfully
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreCaptureSettings.stories.svelte (retry, then final icons/notices)
```

**Notes**

8 stories passed; before/after functional regression1/0; all shared action-column, controls, receipts, unavailable and draft scenarios passed
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/SettingsNavigation.stories.svelte (final)
```

**Notes**

4 stories passed including dedicated installation Agents entry/no repository picker; existing navigation scenarios retained
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
pnpm web:check (initial and final); pnpm web:build (initial and final)
```

**Notes**

0 errors/warnings in type check; built bundle plus asset manifest passed; existing large-chunk warnings retained
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:check (first)
```

**Notes**

Testing Library role options do not support Playwright exact:true; corrected new fixture selectors to anchored names
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
pnpm catalog:check (final)
```

**Notes**

0 errors/warnings; catalog boundaries166 modules passed
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
pnpm exec playwright test --config .local/integration/playwright-settings.config.ts local-capture-settings.spec.ts (initial)
```

**Notes**

3 cold built-bundle tests passed using existing compiled disposable browser host and previously captured HTTP fixtures; no Rust compilation
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
pnpm exec playwright test --config .local/integration/playwright-settings.config.ts local-capture-settings.spec.ts (added geometry)
```

**Notes**

New exact-text source locator timed out because source label includes version/binding children; retained label match corrected;2 other tests passed
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
pnpm exec playwright test --config .local/integration/playwright-settings.config.ts local-capture-settings.spec.ts (geometry correction)
```

**Notes**

Axe observed enabled history button during public Button disabled-opacity transition; added enabled and settled-opacity waits before accessibility inspection;2 other tests passed
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
pnpm exec playwright test --config .local/integration/playwright-settings.config.ts local-capture-settings.spec.ts (settled)
```

**Notes**

3 passed; desktop/right and mobile/stacked column geometry; desktop/mobile Axe0 violations; no horizontal overflow; exact pending intent and request draft retained; polling stops on leaving Agents and closure
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
pnpm check:ui-ownership; pnpm check:architecture; pnpm check:boundaries
```

**Notes**

UI ownership passed; architecture867 modules;17 package boundaries
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
pnpm exec prettier --write changed Svelte, TypeScript and scope documents; pnpm exec prettier --check changed files (first)
```

**Notes**

New anchored navigation selectors needed formatting; formatted both affected tests
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
pnpm exec prettier --check changed files (after formatting)
```

**Notes**

All matched files formatted; final test-only opacity wait formatted subsequently
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
curl -fsSI --max-time 10 http://127.0.0.1:7470/; GET transformed StoreSettingsNavigation.svelte
```

**Notes**

Running preview responds200 and serves Agents navigation through existing HMR process
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
CUA preview AX/DOM/rebind/screenshot; native Codex fallback
```

**Notes**

In-app tab1 focus-emulation timed out and screenshot reset the tool; native Codex access was refused by tool safety. No user app actions applied. Built browser evidence remains distinct from live visual qualification.
<!-- /run -->
<!-- /evidence-field -->

