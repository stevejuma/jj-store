---
schema: 1
id: E-M18-015
---

# E-M18-015

<!-- evidence-field:id markdown -->
## Id

E-M18-015
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M18.8
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-04
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change rxzwlukn
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Searchable multiple-repository source filter composed with status and harness
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Svelte5.57.1; installed released Design Core0.1.2 CommandView/Popover/Button; Vitest4.1.11; Playwright1.63.0; existing isolated browser_preview executable.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

UI-local repository selection arrays and shared owned command-filter composition only. Exact last-bound repository IDs, explicit unknown selection, unique options from all loaded rows. Native roots, source selectors/cursors, wire formats, imports, cached coverage counts and server policy unchanged.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/StoreCaptureSources.stories.svelte
- catalog/components/store-application/capture-repository-command-play.ts
- catalog/fixtures/CaptureSettingsFixture.svelte
- web/src/lib/ui/store-settings/StoreCaptureCommandFilter.svelte
- web/src/lib/ui/store-settings/StoreCaptureSourceFilters.svelte
- web/src/lib/ui/store-settings/StoreCaptureSources.svelte
- web/src/lib/ui/store-settings/StoreCaptureStatusFilter.svelte
- web/src/lib/ui/store-settings/capture-source-presentation.ts
- web/tests/browser/local-capture-loading.spec.ts
- spec/src/ui.md
- spec/src/api.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/browser/local-capture-loading.spec.ts#cold repository source
  filtering composes with status and harness filters
- web/tests/browser/local-capture-loading.spec.ts#cold grouped Codex
  configuration and harness-filtered source table retain identities
<!-- /evidence-field -->

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
jj --no-pager st; read current Sources UI/API requirements and inspect CommunityProjectFilesRepositoryCommand followed by the released/owned command composition
```

**Notes**

Main clean after a7b9879b. Reused existing owned status control as a common CommandView composition, retaining the status component entrypoint. Repository names use the existing catalog resolver; options retain full cached IDs and display compact ID context. No Community runtime/source dependency.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreCaptureSettings.stories.svelte -t SourceRepositoryCommand (before)
```

**Notes**

before.log: exact new repository regression fails because the repository filter is absent;22 unrelated existing scenarios skipped. New test later moved to its own Sources story file for the catalog line budget.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreCaptureSettings.stories.svelte (initial after)
```

**Notes**

stories.log:23 pass. New play exercises keyboard search, multiple repositories, unknown binding, toggling, status intersection, preserved counts/labels, empty combined results and All repositories reset.
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
pnpm check:architecture (initial)
```

**Notes**

Capture Settings story module reached406 counted lines against400 maximum. Restored its prior unchanged content and placed the new repository scenario in StoreCaptureSources.stories.svelte; no budget or checker weakened.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreCaptureSettings.stories.svelte catalog/components/store-application/StoreCaptureSources.stories.svelte (final)
```

**Notes**

stories-final.log:23 pass across both direct story files with interaction/accessibility; existing status search/toggles/focus and source import/navigation guards still pass.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec playwright test --config .local/capture-source-table/playwright.config.ts local-capture-loading.spec.ts -g "cold (repository source filtering|grouped Codex configuration)" (initial)
```

**Notes**

browser.log:1 pass/1 fail. New repository case interacted before first source coverage was accepted, during initial component handoff. Added explicit accepted3-row/current-coverage readiness assertions before querying and choosing repository options; original selection assertion retained.
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
pnpm web:build; pnpm exec playwright test --config .local/capture-source-table/playwright.config.ts local-capture-loading.spec.ts -g "cold (repository source filtering|grouped Codex configuration)" (final)
```

**Notes**

build.log passes with existing chunk-size warning. browser-final.log:2 cold passes in newly launched isolated fixture. Repository selections use exact cached identities and intersect with harness/status; options remain available independently of other filters, search resets without retargeting, unknown binding/empty state/clear work. Desktop and390px open popup Axe and containment pass. Existing original-selector import and status keyboard behavior pass. Screenshots copied to .local/capture-repository-filter/after.png; fixture exited.
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
pnpm web:check; pnpm catalog:check (initial/final); pnpm check:architecture (after story extraction); pnpm check:ui-ownership; focused changed-file Prettier
```

**Notes**

Zero web/catalog errors/warnings;182-module catalog boundary;922-module web architecture; UI ownership and formatting pass. New shared control stays repository-owned and uses released Design Core. Root aggregates, Rust, full app/browser/catalog inventories and visual baselines are outside this UI-only slice.
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
pnpm spec:first; pnpm spec:validate
```

**Notes**

Nine protected files/two owning chapters;22 chapters/167 requirements. UI/API chapters specify loaded-only repository filtering, full identity matching, unique options, unknown observation semantics and no access/query/paging authority change.
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
jj rebase -r uqwlnutu -d rxzwlukn; jj workspace update-stale; screenshot inspection
```

**Notes**

Clean preview workspace follows main UI change through existing Vite HMR. No backend restart, real capture/import/config mutation, fixture reset, installation or publication.
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
pnpm check:progress; pnpm spec:first; pnpm spec:validate; focused canonical formatting (final with ledger)
```

**Notes**

136 tasks;9 protected files/3 canonical chapters;22 chapters/167 requirements validated. Existing412 evidence objects and raw JSON prefix preserved, with no milestone promotion.
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

Narrow Sources repository filter and direct shared status-command presentation only. All412 prior evidence objects/raw JSON prefix preserved; milestone statuses unchanged, prior E-M18-013 broad diagnostics/deferred work retained. No claim of a full repository, Rust or multi-harness release pass.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Use repository/harness/status filters in main and preview Sources settings; filters describe loaded coverage and do not enroll or authorize sources.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


