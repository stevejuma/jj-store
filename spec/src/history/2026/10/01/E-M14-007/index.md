---
schema: 1
id: E-M14-007
---

# E-M14-007

<!-- evidence-field:id markdown -->
## Id

E-M14-007
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M14
- M14.2
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-01
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

presentation
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/StoreApplication.stories.svelte
- catalog/components/store-application/StoreSettingsIdentifier.stories.svelte
- spec/src/api.md
- spec/src/ui.md
- web/src/lib/ui/store-settings/StoreRepositorySettings.svelte
- web/src/lib/ui/store-settings/StoreSearchSettings.svelte
- web/src/lib/ui/store-settings/StoreSettings.css
- web/src/lib/ui/store-settings/StoreSettingsIdentifier.svelte
- web/src/lib/ui/store-settings/StoreWorkspaceSettings.svelte
- web/tests/browser/local-workspaces.spec.ts
- spec/src/milestones.md
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreWorkspaceSettings.stories.svelte catalog/components/store-application/StoreApplication.stories.svelte
```

**Notes**

17 passed; existing Settings story expected capabilities visible in General. Updated it to explicitly open Advanced before the same assertion.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreWorkspaceSettings.stories.svelte catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-application/StoreSettingsIdentifier.stories.svelte
```

**Notes**

19 interaction/accessibility stories pass; full ID copied, clipboard failure retains selectable readonly input, joined geometry and tab navigation verified.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check; pnpm catalog:check
```

**Notes**

Web check passed. Catalog check found Playwright-only exact options in two Storybook ByRole queries. Removed these options; names already match exactly as strings.
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
pnpm web:build; pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/local-workspaces.spec.ts
```

**Notes**

Fresh bundle and one cold browser acceptance pass: folder selection/save and draft retention across global sections and nested tabs, mobile overflow and axe. Existing bundle-size warning remains.
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

<!-- evidence-field:before markdown -->
## Before

Separate Folder path/Browse controls and a long single Repository settings page; long IDs rendered as wrapping code.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Settings UI and its direct shell/story/browser consumers only; no Rust changes or root aggregate lanes.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No wire or persistence format change; full identifiers preserved in readonly fields and clipboard. Enclosing Jujutsu change is the revision reference.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

pnpm 10.32.1; Svelte 5; Vitest 4.1.11; Storybook 10.5.7; Playwright Chromium; existing Community/Design Core resolution.
<!-- /evidence-field -->

<!-- evidence-field:validation markdown -->
## Validation

Final pnpm catalog:check passes with zero errors/warnings and 101 catalog modules. pnpm spec:first, pnpm spec:validate and pnpm check:progress pass: 132 requirements, 111 tasks.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Add configured Sync controls, then editable Search Configuration/Health tabs.
<!-- /evidence-field -->

<!-- evidence-field:deferred list -->
## Deferred

- Visual baselines remain visual-pending.
- Search and Sync management are separate following slices.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


