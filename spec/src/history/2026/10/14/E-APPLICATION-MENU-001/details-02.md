---
schema: 1
id: E-APPLICATION-MENU-001
---

# E-APPLICATION-MENU-001 details 2

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
jj --no-pager st; inspect current UI/M27/M28 chapters, Projects mobile toolbar, Community shell source and released Design Core dropdown guidance
```

**Notes**

Working copy contained only the requested in-flight menu slice. The preceding cards/Description slice is committed separately as 76b5dc67. No unrelated work, native authority, credentials or dev fixture changed.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/ApplicationMenu.stories.svelte
```

**Notes**

Before target: desktop showed separate Repository actions; mobile still exposed top Application navigation because headerMenu forced its visibility. Log /tmp/jj-store-application-menu-before.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/ApplicationMenu.stories.svelte
```

**Notes**

Initial implementation: 2 passed. Log /tmp/jj-store-application-menu-after.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/ApplicationMenu.stories.svelte
```

**Notes**

Retention harness queried Description as a textbox; the existing composer exposes Markdown editor. Corrected the locator and asserted retained editor text. Native dialog now restores focus to the connected relocated menu trigger. Log /tmp/jj-store-application-menu-retention.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/ApplicationMenu.stories.svelte
```

**Notes**

4 passed, including reload dialog and native change draft retention across desktop/mobile. Log /tmp/jj-store-application-menu-retention-final.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/ApplicationMenu.stories.svelte catalog/components/store-application/RuntimeReload.stories.svelte catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-application/StoreRepositoryNavigation.stories.svelte catalog/components/store-application/WorkflowHostSettings.stories.svelte catalog/components/store-mutations/StoreRepositoryActions.stories.svelte
```

**Notes**

37 passed; the earlier Details-at-far-right assertion expected no following menu. Updated this requested shared-header layout assertion to verify Details and the single trailing menu together, preserving inset and adjacency. Log /tmp/jj-store-application-menu-consumers.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/ApplicationMenu.stories.svelte catalog/components/store-application/RuntimeReload.stories.svelte catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-application/StoreRepositoryNavigation.stories.svelte catalog/components/store-application/WorkflowHostSettings.stories.svelte catalog/components/store-mutations/StoreRepositoryActions.stories.svelte
```

**Notes**

40 passed across 6 affected story files, including blocking accessibility. Additional denied-reload and unknown original-request recovery scenarios pass; unknown error asserts a toast. Log /tmp/jj-store-application-menu-consumers-final.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentLayout.stories.svelte catalog/components/store-application/SettingsShell.stories.svelte catalog/components/store-threads/StoreThreadNavigation.stories.svelte catalog/components/store-work/ProjectsHub.stories.svelte catalog/components/store-workflows/WorkflowHistory.stories.svelte
```

**Notes**

40 passed across 5 direct routed consumers, including blocking accessibility. Combined affected coverage: 80 passed. Log /tmp/jj-store-application-menu-hubs.log.
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
pnpm web:check
pnpm catalog:check
pnpm check:ui-ownership
pnpm check:boundaries
```

**Notes**

Web/catalog closure: zero errors/warnings, catalog 435 modules, UI ownership and all 18 package boundaries. An initial catalog check warned that initialPage was captured locally; fixture initialization now explicitly uses untrack. Logs /tmp/jj-store-application-menu-web-closure.log, catalog-final.log, ownership.log, boundaries.log.
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
Targeted countedLines/fileKind/limits inspection via scripts/web-architecture.mjs exports for every changed source/story file
```

**Notes**

Shared Root composition moved its existing focused views into persistent Overlays, unchanged behavior, to keep every changed production file within 300 counted lines. No architecture baseline changed.
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
Live CUA: fresh temporary Workflows document; 390px viewport; menu/read-only reload dialog; desktop resize and focus; Projects comparison; History toolbar and cold refresh
```

**Notes**

Overview loaded native demo/Persistence cards. At width390: document scrollWidth390, one Application menu in Main toolbar and zero top Application navigation. Mobile Projects shares this exact placement. Reload dialog survives mobile-to-desktop resize; closing focuses the new Application menu. History retains its toolbar during cold loading; Mine/Other native rows later loaded. No new execution or completed-provider claim. No reload, workflow save, host admission, restart or native mutation performed. Screenshot /tmp/jj-store-workflow-mobile-menu.png; temporary tab and viewport override cleaned up.
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
pnpm docs:generate
node scripts/docs.mjs review application "Reviewed one shared horizontal Application menu in desktop and mobile body headers, Workflows using the Projects toolbar, persistent mutation and reload dialogs, retained drafts, exact unknown requests and focus across resizing. Existing native commands, authorization and configuration formats are unchanged."
PATH="/Users/stevejuma/.local/bin:$PATH" pnpm docs:check
```

**Notes**

Nine Mermaid diagrams in both themes; 143 files/26,005,409 bytes, valid links/anchors and current source fingerprints. Logs /tmp/jj-store-application-menu-docs-generate.log and docs.log. Formatting restored the review tool JSON layout.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first
pnpm check:progress
pnpm spec:validate
```

**Notes**

Evidence initially used descriptive test references rather than existing file paths, pass with outstanding gates, and lacked navigation for generated continuation. Corrected paths, retained broader pending status and generated only the new evidence directory navigation. Logs /tmp/jj-store-application-menu-spec-first.log, progress.log and spec.log.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first
pnpm check:progress
pnpm spec:validate
```

**Notes**

The existing named-test reference validator only accepts fn/def/test definitions, not Svelte Story declarations. Retained these interaction/Axe scenarios as file and actual run references, leaving the optional named-unit-test list empty. Navigation now includes the evidence continuation. No artificial unit test added for a reversible UI composition.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/ApplicationMenu.stories.svelte catalog/components/store-application/SettingsShell.stories.svelte
pnpm web:check
pnpm catalog:check
```

**Notes**

Final toolbar ordering above retained Repository Settings body: 13 passed, including one new regression (81 distinct affected stories total). Final web/catalog: zero errors/warnings and 435 modules. Logs settings-order.log, web-complete.log and catalog-complete.log under /tmp/jj-store-application-menu-*.
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
pnpm exec prettier --check catalog/components/store-application/ApplicationMenu.stories.svelte catalog/components/store-application/StoreApplication.stories.svelte catalog/fixtures/RuntimeReloadFixture.svelte docs/maintenance/coverage.json docs/site/guides/settings.md docs/ui-provenance.json web/src/lib/ui/community-app-shell/CommunityAppShell.svelte web/src/lib/ui/store-application/StoreApplication.svelte web/src/lib/ui/store-application/StoreApplicationMain.svelte web/src/lib/ui/store-application/StoreApplicationMenu.svelte web/src/lib/ui/store-application/StoreApplicationHeaderActions.svelte web/src/lib/ui/store-application/StoreApplicationOverlays.svelte web/src/lib/ui/store-mutations/StoreMutationDialog.svelte web/src/lib/ui/store-mutations/StoreRepositoryActions.svelte web/src/lib/ui/store-settings/StoreRuntimeReload.svelte spec/src/current/contracts/ui/11-shell-composition.md spec/src/current/contracts/ui/28-complete-supported-configuration-settings.md spec/src/current/contracts/ui/41-workflows.md spec/src/current/contracts/api/16-browsing-ui-execution.md spec/src/current/contracts/documentation/09-remote-agent-connections.md spec/src/current/progress/M27/index.md spec/src/current/progress/M27/runtime-reload-review.md spec/src/current/progress/M28/index.md spec/src/current/governance/spec-governance.md spec/src/current/verification/ui.md
Targeted countedLines/fileKind/limits inspection
git diff --check
```

**Notes**

Authored formatting and changed-file size limits pass. Read-only whitespace inspection identified an extra generator newline at the new evidence continuation EOF; normalized only the new evidence directory. Logs /tmp/jj-store-application-menu-format.log. No source architecture baseline modified.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first
pnpm check:progress
pnpm spec:validate
node scripts/docs.mjs review application "Reviewed one shared horizontal Application menu in desktop and mobile body headers, Workflows using the Projects toolbar, persistent dialogs, retained exact requests and focus across resizing. The mobile toolbar also precedes the retained Repository Settings body. Existing APIs and reload authority are unchanged."
PATH="/Users/stevejuma/.local/bin:$PATH" pnpm docs:check
pnpm spec:first
```

**Notes**

Progress (153 tasks/790 records), 7 validators/3770 chapters/196 requirements and documentation closure passed. Spec-first first required mapped API/documentation chapters, then their fresh evidence file coverage. Added explicit unchanged JST-API-001 / documentation confirmations and the final file inventory, without changing APIs. Logs spec-closure.log, progress-closure.log, spec-validate-closure.log, docs-final.log and spec-qualified.log under /tmp/jj-store-application-menu-*.
<!-- /run -->
<!-- /evidence-field -->
