---
schema: 1
id: E-M14-012
---

# E-M14-012 details 2

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
pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/local-workspaces.spec.ts
```

**Notes**

Before: first section starts 60px below the Settings body; regression requires less than 24px.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreApplication.stories.svelte -t 'Settings'
```

**Notes**

Before: desktop title precedes tabs; mobile Back contains text. Four other selected Settings stories pass; six unrelated stories not selected.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreApplication.stories.svelte -t 'Settings'
```

**Notes**

First implementation: five selected stories pass; icon-sm with rounded-full computes an 8px radius on a 32px button. Replaced with public icon size and --ui-button-radius customization.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreApplication.stories.svelte -t 'Settings'
```

**Notes**

Exact regression: six selected Settings stories pass, including title after tabs, no-tab title at right, circular icon-only Back, retained mobile route. Six unrelated stories not selected.
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
pnpm web:check
```

**Notes**

No errors or warnings.
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
pnpm catalog:check
```

**Notes**

No errors or warnings; catalog boundaries pass.
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
pnpm check:architecture
```

**Notes**

443 modules pass.
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
pnpm check:boundaries
```

**Notes**

15 packages pass.
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
pnpm web:build
```

**Notes**

Two builds pass, the second after final public button shape token. Existing dependency CSS parser and bundle-size warnings retained in ignored logs.
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
pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/local-workspaces.spec.ts
```

**Notes**

Exact fresh built-app regression passes: compact top/inline inset, circular Back, title at right, folder actions and unchanged app row geometry after closing Settings.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreSettingsIdentifier.stories.svelte catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-application/StoreSyncSettings.stories.svelte catalog/components/store-application/StoreSearchSettings.stories.svelte catalog/components/store-application/StoreWorkspaceSettings.stories.svelte
```

**Notes**

All five directly affected story families pass: 27 interaction/accessibility stories.
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
pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/local-workspaces.spec.ts web/tests/browser/local-search-settings.spec.ts web/tests/browser/local-sync.spec.ts
```

**Notes**

Three cold preview scenarios pass; draft/actions, mobile navigation, overflow and axe retained.
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
pnpm exec prettier --write spec/src/ui.md spec/src/api.md web/src/lib/ui/store-settings/StoreSettingsPage.svelte web/tests/browser/local-workspaces.spec.ts web/tests/browser/local-search-settings.spec.ts web/tests/browser/local-sync.spec.ts catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-application/settings-shell-play.ts
```

**Notes**

Changed prose and regression assertions formatted.
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
pnpm spec:first
```

**Notes**

6 protected files, 3 canonical chapters.
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
pnpm spec:validate
```

**Notes**

21 chapters, 132 requirements validated.
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
pnpm check:progress
```

**Notes**

111 tasks; evidence bookkeeping only.
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
pnpm exec prettier --write spec/src/milestones.md; pnpm exec prettier --check catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-application/settings-shell-play.ts spec/src/api.md spec/src/ui.md spec/src/milestones.md web/src/lib/ui/store-settings/StoreSettingsPage.svelte web/tests/browser/local-search-settings.spec.ts web/tests/browser/local-sync.spec.ts web/tests/browser/local-workspaces.spec.ts
```

**Notes**

Ledger and all changed files conform to formatter.
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

Settings presentation only and its direct catalog/browser consumers. Projects headers unchanged. Rust, contract generation, root aggregates and unrelated suites are out of scope under targeted verification policy.
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Settings title/icon appeared before subsection tabs, Back included visible text, and the first section added a large top margin over the body padding.
<!-- /evidence-field -->

