---
schema: 1
id: E-M18-019
---

# E-M18-019

<!-- evidence-field:id markdown -->
## Id

E-M18-019
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

enclosing Jujutsu change smtutknl
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Unified rounded harness configuration rows
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Svelte 5, installed Design Core ghost Button and Collapsible; Chromium Storybook and main Vite HMR on macOS ARM64.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

UI-only composition and geometry; one labelled rounded Settings group containing Codex, Cursor and OpenCode expandable rows. No configuration or capture contract change.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- web/src/lib/ui/store-settings/StoreCaptureConfiguration.svelte
- web/src/lib/ui/store-settings/StoreCaptureHarnessConfiguration.svelte
- web/src/lib/ui/store-settings/StoreSettings.css
- spec/src/ui.md
- spec/src/milestones.md
- spec/src/api.md
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
jj --no-pager st; jj --no-pager diff --stat; jj --no-pager diff spec/src/ui.md
```

**Notes**

Main began clean on cbd0b428. Narrow layout change; historical evidence and milestone statuses retained.
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
pnpm exec prettier --write web/src/lib/ui/store-settings/StoreCaptureConfiguration.svelte web/src/lib/ui/store-settings/StoreCaptureHarnessConfiguration.svelte web/src/lib/ui/store-settings/StoreSettings.css spec/src/ui.md
```

**Notes**

Only changed production and canonical UI files formatted.
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
pnpm test:storybook catalog/components/store-application/StoreMultiHarnessSettings.stories.svelte catalog/components/store-application/StoreCaptureSettings.stories.svelte
```

**Notes**

2 files, 23 stories passed with blocking accessibility checks. Existing save/reload, independent profiles, disclosure and draft-retention interactions preserved. Visual baselines remain separately pending.
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
pnpm web:check
```

**Notes**

0 errors and 0 warnings.
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
pnpm catalog:check
```

**Notes**

0 errors and 0 warnings; catalog boundaries passed for 183 modules.
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
pnpm check:ui-ownership
```

**Notes**

1 validator, 22 chapters, 167 requirements; no new sibling presentation imports.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
CUA observation and reconnect of retained main review tab 12
```

**Notes**

Old tab timed out on focus-emulation command twice. A fresh review tab in the same browser restored inspection; no service restart needed.
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
Fresh main UI review tab at http://127.0.0.1:7460/; Settings > Agents > Configuration; collapse all harnesses, inspect DOM styles, verify hover and keyboard Enter expansion/collapse, save screenshot
```

**Notes**

Shared group has 1px border and 10px radius. Row wrappers have no side borders, radius or padding; following rows have 1px top separators. Cursor pointer hover uses accent background; Codex keyboard focus is visible with accent background and ring-colored border. Keyboard expansion exposes unchanged nested fields/defaults. /tmp/jj-harness-rows-review.png. Capture and drafts were not changed or saved.
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
pnpm spec:first (initial)
```

**Notes**

Settings presentation maps to api.md as well as ui.md; added the independent-profile draft and disclosure-without-save contract to the API chapter.
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
pnpm spec:validate; pnpm check:progress; pnpm exec prettier --check web/src/lib/ui/store-settings/StoreCaptureConfiguration.svelte web/src/lib/ui/store-settings/StoreCaptureHarnessConfiguration.svelte web/src/lib/ui/store-settings/StoreSettings.css spec/src/ui.md
```

**Notes**

5 validators, 22 chapters, 167 requirements; progress bookkeeping passed for 136 tasks; all changed production/UI chapter formatting passed.
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
pnpm exec prettier --write spec/src/api.md; format only E-M18-019 JSON entry; compare previous ledger entries to jj file show -r @-
```

**Notes**

Canonical API clarification formatted. All pre-existing evidence objects preserved exactly; no whole-ledger formatter churn.
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
pnpm spec:first; pnpm spec:validate; pnpm check:progress; pnpm exec prettier --check web/src/lib/ui/store-settings/StoreCaptureConfiguration.svelte web/src/lib/ui/store-settings/StoreCaptureHarnessConfiguration.svelte web/src/lib/ui/store-settings/StoreSettings.css spec/src/ui.md spec/src/api.md (final)
```

**Notes**

Spec-first passed for 3 protected files and 3 canonical chapters; 5 validators, 22 chapters, 167 requirements; 136 tasks passed bookkeeping; all affected source and canonical formatting passed.
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

Requested harness layout in main only. UI checks, directly affected settings stories and live HMR review; Rust, public contracts, root aggregate lanes, service restart and visual-baseline approval out of scope. Existing main fixture Agent-session 503 and Cursor Desktop live qualification remain separate prior gaps.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Review the grouped harness rows at http://127.0.0.1:7460/ under Settings &gt; Agents &gt; Configuration.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


