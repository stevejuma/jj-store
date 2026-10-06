---
schema: 1
id: E-UI-060
---

# E-UI-060

<!-- evidence-field:id markdown -->
## Id

E-UI-060
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M14
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-02
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

local-service-search-sync-settings-row-alignment
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change svtuyuxl
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Clean svtuyuxl above verified header/task/reviews slice zmzkuouq/c370073c. User supplies screenshots of ungrouped Git-backed folder metadata/actions, Search controls below rather than inside its table, descriptions right of action buttons, and raw unsupported Sync guidance. Existing Local service table and Appearance rows provide the intended composition. Community settings Field groups inspected first, then Design Core public Field/Empty/WorkspaceSettings references. Owning UI/API requirements and M14 scope read before implementation.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/StoreApplication.stories.svelte
- catalog/components/store-application/StoreSearchSettings.stories.svelte
- catalog/components/store-application/StoreSyncSettings.stories.svelte
- catalog/components/store-application/StoreWorkspaceSettings.stories.svelte
- catalog/fixtures/SearchSettingsFixture.svelte
- spec/src/api.md
- spec/src/ui.md
- spec/src/verification.md
- web/src/lib/ui/store-settings/StoreGraphIndexSettings.svelte
- web/src/lib/ui/store-settings/StoreLocalFolders.svelte
- web/src/lib/ui/store-settings/StoreSearchConfiguration.svelte
- web/src/lib/ui/store-settings/StoreSearchHealth.svelte
- web/src/lib/ui/store-settings/StoreSettings.css
- web/src/lib/ui/store-settings/StoreSettingsEmpty.svelte
- web/src/lib/ui/store-settings/StoreSyncSettings.svelte
- web/src/lib/ui/store-settings/StoreWorkspaceSettings.svelte
- web/tests/real/settings-layout.spec.ts
- spec/src/milestones.md
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
jj --no-pager st; jj describe -m "Align Local service, Search and Sync settings rows"; jj --no-pager diff --stat; scoped source/reference/spec inspection and final diffs
```

**Notes**

Same Appearance workspace and Vite backend retained; main unrelated work untouched. No new Community source copied or foundational dependency changed.
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
pnpm web:check (initial and final); pnpm catalog:check (initial and final)
```

**Notes**

Both final checks report0 errors/warnings;129 catalog modules. /tmp/jj-settings-rows-{web,catalog}-check{-final}.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreSearchSettings.stories.svelte catalog/components/store-application/StoreSyncSettings.stories.svelte catalog/components/store-application/StoreWorkspaceSettings.stories.svelte catalog/components/store-application/StoreApplication.stories.svelte -t "ConfigurationTableControls|SaveThenApply|FailureRetainsDraft|PauseResumeAndProgress|LocalOnly|Unsupported|FailureRequiresRefresh|SettingsDegradedSearch|SettingsUnavailableSearch|UnsupportedConnection" --reporter verbose (initial and final)
```

**Notes**

11 focused stories pass with runtime/accessibility guard: common bordered Search status/controls, description-left/actions-right, cancellation, save/apply and failed draft retention, sync pause/resume/progress/recovery, local-only and unsupported icon states.15 unrelated stories deliberately skipped. /tmp/jj-settings-rows-stories{-final}.log.
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
JJ_STORE_REAL_BASE_URL=http://127.0.0.1:7460 JJ_STORE_REAL_REPOSITORY=1bdb89c409c1b891ac7f180c01a265d5 pnpm exec playwright test --config web/playwright.real.config.ts web/tests/real/settings-layout.spec.ts (initial)
```

**Notes**

Desktop passes including axe and all layouts. Mobile waits for Open navigation sidebar because navigating to the identical hash retains mounted transient Settings. Corrected test to explicitly reload for a fresh shell; no production lifecycle change or assertion suppression. Failed trace in web/test-results-real.
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
JJ_STORE_REAL_BASE_URL=http://127.0.0.1:7460 JJ_STORE_REAL_REPOSITORY=1bdb89c409c1b891ac7f180c01a265d5 pnpm exec playwright test --config web/playwright.real.config.ts web/tests/real/settings-layout.spec.ts (corrected)
```

**Notes**

1 native read-only browser test passes in10.3s at1351x988 and390x844. Registration rows/actions, Add dialog cancellation, Search table enablement/budget/scopes and draft cancellation, Sync local-only table/icon, action placement, overflow and settled WCAG axe checks pass. No runtime errors, native mutation, registration removal, indexing or sync control action invoked. /tmp/jj-settings-rows-browser-final.log and6 desktop/mobile screenshots; native fixture unchanged.
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
node /tmp/jj-settings-rows-probe.mjs
```

**Notes**

Supplemental isolated live Local service source rendering and enabled Add/Refresh/Reconnect/Remove controls. Screenshot inspected along with Search/Sync desktop/mobile acceptance images. Not baseline approval.
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
pnpm check:ui-ownership
```

**Notes**

21 chapters/147 requirements validated. New Settings Empty wrapper composes released Design Core; no Community/catalog runtime or type dependency added.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreSearchSettings.stories.svelte -t "ConfigurationTableControls|SaveThenApply"; JJ_STORE_REAL_BASE_URL=http://127.0.0.1:7460 JJ_STORE_REAL_REPOSITORY=1bdb89c409c1b891ac7f180c01a265d5 pnpm exec playwright test --config web/playwright.real.config.ts web/tests/real/settings-layout.spec.ts (mobile toggle refinement)
```

**Notes**

Switches stay to the right of their descriptions on mobile while budget/scopes/actions stack.2 affected Search stories and final native desktop/mobile browser pass; settled axe checks have0 violations, with0 runtime errors. /tmp/jj-settings-rows-toggle-stories.log and browser-final2.log (11.9s).
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
pnpm exec prettier --write (changed supported files); pnpm exec prettier --check (same); pnpm spec:first; pnpm spec:validate; pnpm check:progress
```

**Notes**

Same-change canonical UI/API, verification mapping and ledger; focused formatting. /tmp/jj-settings-rows-{format*,spec-first,spec-validate,progress}.log.
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

UI-only Settings composition and direct consumers. Exact Search/Sync/unsupported stories, web/catalog type checks, ownership and read-only live native desktop/mobile browser regression qualify this presentation slice. Existing shared status rows retain their palette/grid recipe; Field controls and action rows now share borders and responsive layout. Native/API contracts, workers and process lifecycle unchanged. Rust builds/tests, root aggregates, unrelated stories, destructive/native writes, backend restarts, publication and production built-CSP/release qualification out of scope. M14 and M16 delivery states unchanged; other local-management, registry-only and visual gates remain separate.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Node26.9.0; pnpm10.32.1; Svelte5.57.1; Vite8.3.2; Playwright1.63.0; Storybook10.5.7; Vitest4.1.11; existing Design Core public Field, Empty, Switch, Input and Button. Dependencies unchanged.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No native/API/storage/preference format changes. Registration names, full wrapping folder paths, readiness and exact Add/Reconnect/Remove callbacks retained. Search desired/effective revisions, dirty drafts, Save/Apply separation and worker controls unchanged. Availability Empty icons are decorative with descriptive text; recoverable errors remain alerts. Responsive rows stack actions below guidance on mobile.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Continue UI iteration on the running Appearance Vite service; landing onto main and visual baseline approval remain separate user actions.
<!-- /evidence-field -->

<!-- evidence-field:artifacts markdown -->
## Artifacts

/tmp/jj-settings-rows-*.log/png and service probe; committed web/tests/real/settings-layout.spec.ts. Isolated browsers closed and persistent main fixture preserved.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


