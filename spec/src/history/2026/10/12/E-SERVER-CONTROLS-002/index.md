---
schema: 1
id: E-SERVER-CONTROLS-002
---

# E-SERVER-CONTROLS-002

<!-- evidence-field:id markdown -->
## Id

E-SERVER-CONTROLS-002
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-12
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M27
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-022
- JST-UI-016
- JST-API-011
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

ui-verification
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

lssvruoo
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- web/src/lib/ui/store-connections/StoreServerSwitcher.svelte
- web/src/lib/ui/store-connections/StoreConnectionDialog.svelte
- catalog/components/store-application/ServerConnections.stories.svelte
- spec/src/current/contracts/ui/40-server-switcher.md
- spec/src/current/progress/M27/index.md
- spec/src/current/progress/M27/history/M27/index.md
- spec/src/history/2026/10/12/E-SERVER-CONTROLS-002/index.md
- spec/src/history/2026/10/12/E-SERVER-CONTROLS-002/SUMMARY.nav.md
- spec/src/history/2026/10/12/index.md
- spec/src/history/2026/10/12/SUMMARY.nav.md
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
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
jj --no-pager st; cua_repl read-only comparison of Add server and Agents button geometry in the live main app
```

**Notes**

Main started clean above the build-storage change. Before: Add server was 22px high with flex 1 1 0%; selected Agents was 36px with flex 0 0 auto. Both had 0px 10px CSS padding. The standalone column button was compressed by the horizontal row flex rule. Earlier acceptance compared Local with Agents and missed this standalone action.
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
cua_repl inspect updated Add server button and open the connection dialog
```

**Notes**

After: Add server is 36px high with the same 0px 10px padding. The connection/session-token introduction appears only in the Connection details card header; dialog title/close are retained and no absent dialog-description reference is emitted. HMR applies without restart. No settings, credentials, repositories or data were submitted or modified. The dialog was subsequently closed during user interaction; an attempted automated close found no element and inspection confirmed the current sidebar state.
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
node node_modules/vitest/vitest.mjs run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/ServerConnections.stories.svelte
```

**Notes**

12 stories pass including Axe, compact keyboard/overflow, sign-in focus, pending switching guard, busy/error and mobile. Expanded now compares Add server and More servers height with Local; Connection dialog checks that the introduction is in the section header and absent from the dialog heading. This is the affected leaf family, not a full-suite claim. Existing Vite default-config notices remain separate.
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
pnpm catalog:check
```

**Notes**

Both type checks have zero errors/warnings; 412-module catalog boundaries pass.
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
pnpm check:ui-ownership
pnpm check:architecture
node scripts/check-progress.mjs
node node_modules/@lapismd/spec-validator/dist/cli.js first
node node_modules/@lapismd/spec-validator/dist/cli.js validate
node node_modules/prettier/bin/prettier.cjs --check (changed authored UI, story and current spec pages)
```

**Notes**

Ownership and 1752-module architecture pass. Progress verifies 152 tasks/775 evidence records; spec-first verifies three protected files and the owning canonical chapter; seven validators pass. Formatting passes. Live app console has no error/warning entries; ignored sidebar review is .local/server-controls-review/sidebar-refined.jpg. Root aggregates and Rust lanes remain out of scope.
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

<!-- evidence-field:versions markdown -->
## Versions

Node 26.9.0; JJ 0.45.1; Vitest 4.1.11; released Design Core and Mira dependencies unchanged.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No route, API, configuration, credential or persisted-data changes.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Correct standalone server action sizing and consolidate dialog introduction. Only horizontal server-row buttons flex to fill remaining width; standalone Add/More keep the sidebar height. No controller, validation, availability, token or navigation changes. UI-only targeted policy excludes Rust, root aggregate checks/builds, unrelated UI suites and visual-baseline updates.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Review the main HMR app; M27 provider/platform qualification remains separate.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: JST-UI-022 server scoping/overflow and JST-API-011 connection behavior
  are unchanged. JST-UI-016 form hierarchy is clarified in the owning UI
  chapter. This verified presentation correction preserves all actions, focus,
  disabled states and session-token handling.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M27 remains In progress. Focused UI sizing/content acceptance does not
  promote installed-provider, Linux real-storage, desktop or visual-baseline
  gates.

```
<!-- /evidence-field -->

