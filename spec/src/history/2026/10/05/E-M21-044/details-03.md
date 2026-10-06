---
schema: 1
id: E-M21-044
---

# E-M21-044 details 3

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
pnpm exec prettier --check changed supported files (excluding the historical progress ledger); pnpm exec prettier --write docs/maintenance/coverage.json
```

**Notes**

An initial format command used numbered search output as paths (format.log) and failed; format-corrected.log formats supported paths. A later changed-files check found only coverage.json needed formatting after review refresh. Corrected it without reformatting historical ledger entries.
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
pnpm exec prettier --check changed supported files (excluding spec/src/milestones.md)
```

**Notes**

format-recovered.log: all 45 supported changed files pass formatting.
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
node scripts/stacks-browser-acceptance.mjs
```

**Notes**

cold-combined-recovered.log: focused history navigation and the post-apply physical diff pass. The core test later times out waiting for a switch preview immediately after active workspace deletion. Trace shows an open picker option click during a background read, silently rejected by the locked controller.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreRepositoryPickerPending.stories.svelte
```

**Notes**

picker-pending-before.log: precise source regression proves open option aria-disabled remains false after the picker becomes disabled.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreRepositoryPickerPending.stories.svelte catalog/components/store-application/StoreRepositoryPicker.stories.svelte
```

**Notes**

picker-pending-after.log: four stories pass. All open choices/empty choice/continuation are disabled with the trigger, callbacks are guarded, the popover remains open and selection resumes when the read settles.
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
pnpm web:check; pnpm catalog:check
```

**Notes**

web-check-picker.log and catalog-check-picker.log: zero errors/warnings; 232 catalog modules. Rechecked after the shared picker correction.
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
pnpm docs:check
```

**Notes**

docs-check-recovered.log: nine Mermaid diagrams and closure of 139 files/13,953,873 bytes; links/anchors valid before the subsequent picker documentation update.
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
pnpm spec:first
```

**Notes**

spec-first-recovered.log: 41 protected files, five canonical chapters before subsequent picker correction.
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
pnpm exec prettier --write catalog/fixtures/RepositoryPickerPendingFixture.svelte catalog/components/store-application/StoreRepositoryPickerPending.stories.svelte web/src/lib/ui/store-application/StoreRepositoryPicker.svelte spec/src/ui.md docs/site/guides/browsing.md docs/maintenance/coverage.json; pnpm docs:review application "Browsing documents focused Stacks navigation, contextual workspace and selection details, retained canvas and history lifetimes, and open picker choices pausing during pending reads."
```

**Notes**

Focused picker fixture/styles and Browsing guidance formatted, application documentation fingerprint refreshed with the exact presentation/lifetime explanation. No historical ledger formatting.
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

web-build-picker.log: fresh bundle with the picker correction, nine Mermaid diagrams, documentation closure 139 files/13,954,043 bytes and valid links/anchors. Existing Vite large-chunk warning retained.
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
node scripts/stacks-browser-acceptance.mjs
```

**Notes**

cold-combined-picker.log: both real-folder cold scenarios pass together, navigation 6.6 seconds and local core 15.4 seconds (51.9-second runner including setup/teardown). Explicit new-operation index preparation and restored picker choices preserve real adjacent diff/hunks, create/switch/apply/unapply, Settings/delete, named WIP/TOML locality and accessibility assertions.
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
pnpm check:architecture; pnpm check:ui-ownership
```

**Notes**

architecture-picker.log: 1105 modules. ownership-picker.log: dedicated UI source-ownership validator passes.
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
pnpm spec:first; pnpm spec:validate
```

**Notes**

spec-first-picker.log: 44 protected files/five canonical chapters; spec-validate-picker.log: five validators, 23 chapters, 173 requirements pass.
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
pnpm exec prettier --check changed supported files (excluding spec/src/milestones.md)
```

**Notes**

format-final-picker.log: all 48 supported changed files pass formatting.
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
Python comparison of current milestones JSON/raw prefix with jj file show -r @- spec/src/milestones.md
```

**Notes**

All 525 historical evidence objects and their raw JSON prefix remain identical; only the new entry and milestone links were added.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/*.stories.svelte catalog/components/store-application/StoreRepositoryPicker*.stories.svelte catalog/components/store-application/StoreConnection.stories.svelte catalog/components/store-application/StoreWorkspaceSettings.stories.svelte catalog/components/store-application/StoreWorkspaces.stories.svelte catalog/components/store-work/*.stories.svelte
```

**Notes**

picker-consumers.log: 181 pass, one fails across 42 files/182 tests. InsetSkeletonsAcrossAllMainTabs expects the repository directory loading skeleton after tab navigation; the predicate only considered hub selection loading, which settles before viewport-owned metadata reads.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/ProjectsActivity.stories.svelte -t InsetSkeletonsAcrossAllMainTabs
```

**Notes**

projects-loading-exact.log reproduces the original assertion. Diagnostic/labeled runs also fail and show Projects, Tasks and Reviews skeletons while Repositories shows unloaded rows. Temporary diagnostic edits removed; assertion unchanged.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/ProjectsActivity.stories.svelte -t InsetSkeletonsAcrossAllMainTabs
```

**Notes**

projects-loading-after.log: exact regression passes (one pass/six unselected) after including visible-row loading in the existing skeleton predicate. No weakened assertions or read-controller changes.
<!-- /run -->
<!-- /evidence-field -->

