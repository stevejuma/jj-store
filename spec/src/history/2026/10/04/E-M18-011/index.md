---
schema: 1
id: E-M18-011
---

# E-M18-011

<!-- evidence-field:id markdown -->
## Id

E-M18-011
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

enclosing Jujutsu change vsusztpy
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Source-table avatars, combined filters, icon history actions and observation freshness footer
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Svelte5.57.1; Storybook10.5.7; Vitest4.1.11; released Design Core Table/Select/Button/Badge; owned CommunityMemberAvatar and CommunityRelativeTime; existing browser_preview executable.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Presentation and internal installation-controller metadata only. sourcesObservedAt is browser-clock milliseconds on validated accepted coverage/paging; null until acceptance. No wire/schema/server/persisted transcript changes. Source filters preserve selectors, unfiltered coverage and cursors. No per-source activity/modification timestamps are inferred.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/StoreCaptureSettings.stories.svelte
- catalog/components/store-application/capture-harness-play.ts
- catalog/components/store-application/capture-source-table-play.ts
- spec/src/api.md
- spec/src/collaboration.md
- spec/src/ui.md
- spec/src/milestones.md
- web/src/lib/controller/capture-management.svelte.ts
- web/src/lib/controller/capture-observations.ts
- web/src/lib/controller/capture-types.ts
- web/src/lib/ui/store-settings/StoreCaptureSourceFilters.svelte
- web/src/lib/ui/store-settings/StoreCaptureSourceFooter.svelte
- web/src/lib/ui/store-settings/StoreCaptureSources.svelte
- web/src/lib/ui/store-settings/capture-source-presentation.ts
- web/tests/browser/local-capture-loading.spec.ts
- web/tests/vite/capture-freshness.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/vite/capture-freshness.test.ts#source observation time advances only
  after accepted coverage
- web/tests/vite/capture-freshness.test.ts#source paging advances observation
  time only for a validated page
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
jj --no-pager st; jj describe -m "Refine capture sources with harness avatars, status filters and freshness footer"; read owning UI/API/collaboration chapters and ledger; inspect Community runtime relative-time presentation and released Design Core table/button exports
```

**Notes**

Clean main started at vsusztpy after96ffe099. Current source contract has no per-source update timestamp. Quick memory search provides no task-specific implementation facts. New UI remains owned; no sibling source imports.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreCaptureSettings.stories.svelte (before)
```

**Notes**

stories-before.log:18 existing stories pass; new source avatar/external-status-filter/icon/footer regression fails before implementation.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/capture-freshness.test.ts (before)
```

**Notes**

units-before.log:8 existing tests pass; new accepted observation-time regression fails before implementation.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreCaptureSettings.stories.svelte (after)
```

**Notes**

stories-after.log:18 pass, one old row-count assertion includes a newly added spanning footer. Corrected assertions to count body rows and independently verify footer; no product assertions removed.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreCaptureSettings.stories.svelte (final)
```

**Notes**

stories-final.log:19 stories pass including interaction/accessibility. First cells identify Codex/Cursor with owned avatars; filters outside border combine over stable source labels; import is icon-only with accessible name; footer spans4 columns and uses shared relative time; exact import selector and prior draft/notification/loading guards retained. Visual-pending remains.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/capture-freshness.test.ts web/tests/vite/capture-management.test.ts web/tests/vite/capture-settings.test.ts web/tests/vite/capture-observation-http.test.ts web/tests/vite/production-capture-management.test.ts (after/final)
```

**Notes**

units-after.log:32 pass; units-final.log:33 across five files pass with paging freshness added. First successful source observation and accepted continuation update browser time; Busy, real unavailable and rejected duplicate paging retain previous timestamp and selected coverage.
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
pnpm web:check; pnpm catalog:check (initial/final)
```

**Notes**

web-check-final.log and catalog-check-final.log: zero errors/warnings; catalog179-module boundary passes. No generated contract change or Rust lane required.
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
pnpm web:build
```

**Notes**

build.log: fresh production bundle and asset manifest pass; existing chunk-size warning retained. Cold UI uses explicit existing target/debug/examples/browser_preview without Rust rebuild.
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
pnpm exec playwright test --config .local/capture-source-table/playwright.config.ts local-capture-settings.spec.ts local-capture-loading.spec.ts
```

**Notes**

browser.log: all7 cold tests pass. New table regression measures both filters above/outside border and trailing alignment, avatar first-cell image, combined Codex/degraded filtering, icon-only accessible history button,4-column footer and datetime; desktop/mobile Axe, narrow-screen containment, exact selector, drafts, notifications, loading/retry and polling lifetime remain qualified. Isolated7458 service stopped by Playwright.
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
pnpm check:architecture; pnpm check:boundaries; pnpm check:ui-ownership; focused pnpm exec prettier --write/--check
```

**Notes**

architecture-final.log:916 modules; boundaries.log:17 packages; ownership.log and format-check.log pass. Production table/filter/footer helpers stay under governed source budgets and independent of catalog. Ledger never whole-file formatted.
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
pnpm spec:first (initial)
```

**Notes**

spec-first.log: changing capture-types internal controller metadata requires mapped API chapter. Added explicit browser-only successful observation-time semantics to canonical API chapter; no schema or public response change.
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
pnpm spec:first (final); pnpm spec:validate
```

**Notes**

spec-first-final.log and spec-validate.log pass with UI/API/collaboration owning chapters. Canonical requirements distinguish accepted coverage freshness from transcript/file/server update time.
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
jj rebase -r uqwlnutu -d vsusztpy; jj workspace update-stale (clean preview); Cua live table DOM/screenshot
```

**Notes**

Preview7470 shows Codex avatars, two trailing filters above rounded table, icon imports and full-width footer with current coverage,4 shown/loaded/observed and Updated just now. .local/capture-source-table/after.png saved after ready observation. Temporary proof tab closed; user tab retained. No real capture/import/config command submitted; existing main/desktop/backend processes unchanged.
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
pnpm check:progress; prior evidence object/raw-prefix preservation assertions
```

**Notes**

progress.log:136 tasks pass. All408 prior evidence objects and raw JSON prefix preserved; ledger not whole-file formatted.
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
<!-- generated-navigation -->
## Pages

- [E-M18-011 details 2](details-02.md)
