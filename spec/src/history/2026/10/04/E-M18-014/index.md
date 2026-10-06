---
schema: 1
id: E-M18-014
---

# E-M18-014

<!-- evidence-field:id markdown -->
## Id

E-M18-014
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

enclosing Jujutsu change ltmupmlx
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Searchable multiple-status CommandView for installation capture sources
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Svelte 5.57.1; installed released Design Core 0.1.2 CommandView/Popover/Button; Vitest 4.1.11; Playwright 1.63.0; existing isolated browser_preview binary.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

UI-local status arrays only: union of statuses intersects the selected harness over loaded coverage. Existing sources, selectors, import commands, cursor and observed counts unchanged. No server/configuration/public-wire format change.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/StoreCaptureSettings.stories.svelte
- catalog/components/store-application/capture-source-table-play.ts
- catalog/components/store-application/capture-status-command-play.ts
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
jj --no-pager st; read owning UI/API/governance and milestone chapters; inspect CommunityProjectsActivityFilter then released Design Core CommandView/Popover and official shadcn command/combobox documentation
```

**Notes**

Main workspace clean after 5e440835. New repository-owned composition uses existing released primitives and source status inventory; Community is a design reference without copied implementation or dependency.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreCaptureSettings.stories.svelte -t "Source status command searches" (before selection attempt)
```

**Notes**

All 22 scenarios skipped because generated test names use PascalCase. This is not accepted regression evidence; the complete exact story file was run next.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreCaptureSettings.stories.svelte (before)
```

**Notes**

before.log: 21 pass / 1 fail. New SourceStatusCommandSearchesTogglesAndCombinesMultipleSelections cannot find searchable command input in the former single-select control.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreCaptureSettings.stories.svelte (initial after); same file -t SourceStatusCommand
```

**Notes**

stories.log: 21 pass / 1 fail because Command input uses its owning Root label for the accessible name. Set that label explicitly. exact.log then reached the harness-switch/reopen step and failed during Select exit pointer suppression; added the existing bounded pointer-readiness assertion without weakening interaction checks.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 6
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:check (initial and intermediate)
```

**Notes**

catalog-check.log: six inappropriate testing-library exact properties. Removed them; string role queries remain exact by default. catalog-check-final.log exposed one remaining multiline instance, then corrected it. No production diagnostic suppression.
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
pnpm exec playwright test --config .local/capture-source-table/playwright.config.ts local-capture-loading.spec.ts -g "cold grouped Codex configuration" (initial)
```

**Notes**

browser.log: open command popup lacked landmark/dialog semantics and failed Axe region rule. Added labelled dialog semantics to the Popover content. Keyboard and multi-status selection assertions had already passed; Axe assertions retained.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreCaptureSettings.stories.svelte (accepted)
```

**Notes**

stories-accepted.log: all 22 direct capture scenarios pass, with interaction/accessibility. New regression covers search, Enter selection, toggling both directions, union/intersection, checked state and accessible descriptions, retained loaded/observed counts, empty results, Escape focus return, selection preservation, query reset and All statuses clearing.
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
pnpm web:build (initial and final); pnpm exec playwright test --config .local/capture-source-table/playwright.config.ts local-capture-loading.spec.ts -g "cold grouped Codex configuration" (final and accepted)
```

**Notes**

build-final.log passes with existing chunk-size/plugin timing diagnostics. browser-final.log and browser-accepted.log each pass the exact cold production Sources journey using a freshly launched isolated fixture. Accepted run includes multiple selection/search/reset, desktop and open mobile popup Axe, popup viewport containment, stable labels/counts and exact imported original selector. Desktop/mobile screenshots under .local/capture-source-table/browser-results; fixture exited.
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
pnpm web:check; pnpm catalog:check (accepted); pnpm check:architecture; pnpm check:ui-ownership; focused changed-file Prettier check
```

**Notes**

Zero web/catalog errors or warnings; catalog 180 modules; web architecture 919 modules; ownership and focused formatting pass. UI-only slice: no Rust, root aggregates, full browser or repository Storybook inventories. Visual baselines remain unapproved.
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
pnpm spec:first (initial)
```

**Notes**

Missing mapped api.md chapter for protected web changes. Added the existing API boundary: filters only present loaded observations and cannot change query/paging/selector authority.
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
pnpm spec:first; pnpm spec:validate (after mapped API clarification)
```

**Notes**

Eight protected files / two canonical chapters; 22 chapters / 167 requirements validated. UI chapter owns multiple-status search, any-status matching, clear behavior and focus. Existing milestone statuses retained.
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
jj rebase -r uqwlnutu -d ltmupmlx; jj workspace update-stale
```

**Notes**

Updated the clean preview workspace to this main UI slice; existing Vite HMR can serve it without backend restart or fixture reset. No real import, configuration write, installed service change or publication.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress (initial)
```

**Notes**

Ledger named-test validator accepts test declarations, not exported Storybook play helpers or bare file paths. Replaced those references with the exact checked-in Playwright test declaration; Storybook regression names remain documented in runs.
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
pnpm spec:first; pnpm spec:validate; focused Prettier (final with ledger)
```

**Notes**

Eight protected files / three canonical chapters (including milestone ledger),22 chapters/167 requirements; changed documentation/UI formatting passes. Existing 411 evidence objects and raw JSON prefix preserved.
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
pnpm check:progress (accepted); final historical evidence preservation audit
```

**Notes**

136 tasks validated; all411 prior evidence objects/raw JSON prefix unchanged. Milestone statuses and prior deferred/broad diagnostic evidence retained.
<!-- /run -->
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M18-014 details 2](details-02.md)
