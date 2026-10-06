---
schema: 1
id: E-M14-010
---

# E-M14-010

<!-- evidence-field:id markdown -->
## Id

E-M14-010
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M14
- M14.2
- M14.4
- M14.6
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-01
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

settings-page-tab-placement
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/StoreApplication.stories.svelte
- catalog/fixtures/SyncSettingsFixture.svelte
- spec/src/api.md
- spec/src/milestones.md
- spec/src/ui.md
- web/src/lib/ui/store-settings/StoreRepositorySettings.svelte
- web/src/lib/ui/store-settings/StoreSearchSettings.svelte
- web/src/lib/ui/store-settings/StoreSettings.css
- web/src/lib/ui/store-settings/StoreSettings.svelte
- web/src/lib/ui/store-settings/StoreSettingsPage.svelte
- web/src/lib/ui/store-settings/StoreSyncSettings.svelte
- web/tests/browser/local-search-settings.spec.ts
- web/tests/browser/local-sync.spec.ts
- web/tests/browser/local-workspaces.spec.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreApplication.stories.svelte -t '^Settings$'
```

**Notes**

Before implementation: tab list bottom 209.59 was below heading top 81; exact layout assertion failed.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreApplication.stories.svelte -t '^Settings$'
```

**Notes**

Exact regression passes after moving tabs, including final rerun with concise geometry assertion.
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

Zero errors or warnings; 107 catalog modules.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-application/StoreSyncSettings.stories.svelte catalog/components/store-application/StoreSearchSettings.stories.svelte
```

**Notes**

19 interaction/accessibility stories pass from a fresh runner. Local service, settings navigation, supported/unsupported Search and Sync, and errors retain behavior.
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
pnpm check:architecture
```

**Notes**

Initial test assertion made the shell story 402 counted lines. Reused its single heading/tablist locators to bring it below the existing 400-line budget.
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
pnpm check:architecture; pnpm check:boundaries
```

**Notes**

440 web modules; 15 package boundaries.
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
pnpm web:build
```

**Notes**

Fresh production bundle; existing large-chunk warning retained.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/local-search-settings.spec.ts web/tests/browser/local-workspaces.spec.ts web/tests/browser/local-sync.spec.ts
```

**Notes**

Repository and Sync pass including header geometry, mobile overflow and axe. Search stopped at a new assertion using the Storybook matcher toHaveFocus; corrected to Playwright toBeFocused.
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
pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/local-search-settings.spec.ts
```

**Notes**

Search passes with corrected keyboard assertion. Final exact rerun also proves sticky tabs stay at the same vertical position after scrolling 240px on mobile. All three cold browser scenarios pass across focused runs: header placement, draft retention, Save/Apply, controls, route preservation, mobile overflow and axe.
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

Owned main demo rebuilt and restarted at port 7460; repository, identity and RustFS volume retained; Search ready. Required helper builds development binaries; no Rust test or qualification lane run.
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
cua_repl: reload isolated review tab; profile Settings; Search; getAXState and screenshot
```

**Notes**

Live main demo shows Configuration, File scopes and Health as Projects-style pills in a full-width top strip above the Search heading. Left review tab open; no settings values changed.
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

Formatted the changed surface.
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
pnpm spec:first; pnpm spec:validate; pnpm check:progress
```

**Notes**

Protected-source mappings, 132 canonical requirements and 111 progress tasks pass; existing milestone status retained.
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

UI-only Settings composition and its direct catalog/browser consumers. Focused web/catalog checks and stories plus cold built-app browser acceptance. Rust tests, generated contracts, root aggregate lanes and full unrelated suites are out of scope under targeted verification policy.
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Repository, Search and Sync tab lists appeared inside the padded content below the page heading; Projects already placed its navigation above the heading.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No wire, persisted route, configuration, native or WAL format changes. Public Design Core Tabs/ScrollArea and app-owned Community-style navigation composition. Enclosing Jujutsu change rwkvwlxl is revision reference.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ 0.45.1; pnpm 10.32.1; locked Svelte, Storybook, Vitest and Playwright.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Review the top Settings tabs in the main demo; remaining M14 Agent, Storage and finite-frontier Sync implementation remains separately tracked.
<!-- /evidence-field -->

<!-- evidence-field:deferred list -->
## Deferred

- Visual baselines remain visual-pending; no milestone-wide qualification claim.
<!-- /evidence-field -->

<!-- evidence-field:artifacts markdown -->
## Artifacts

Ignored .scratch/settings-tabs-*.log and web/test-results. Isolated browser preview processes shut down; main owned demo remains available.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


