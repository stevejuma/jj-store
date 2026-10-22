---
schema: 1
id: E-SHELL-BOUNDARIES-001
---

# E-SHELL-BOUNDARIES-001

<!-- evidence-field:schema yaml -->
## Schema

```yaml
1

```
<!-- /evidence-field -->

<!-- evidence-field:id markdown -->
## Id

E-SHELL-BOUNDARIES-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4.5
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-API-005
- JST-GOV-002
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-22
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

contract-confirmation
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

Enclosing Jujutsu change xnqonvqt
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/community-app-shell/shell-header-geometry.test.ts
- catalog/components/store-application/ApplicationMenu.stories.svelte
- catalog/components/store-application/DesktopChrome.stories.svelte
- catalog/components/store-application/DiscussionSheets.stories.svelte
- catalog/components/store-application/ShellOverlay.stories.svelte
- catalog/components/store-application/StoreApplication.stories.svelte
- catalog/components/store-application/WorkspaceFiles.stories.svelte
- catalog/components/store-application/WorkspaceInspector.stories.svelte
- catalog/components/store-mutations/StoreConflictEditor.stories.svelte
- catalog/components/store-stack-workspace/StoreStackWorkspaceConflictLayout.stories.svelte
- catalog/fixtures/DiscussionSheetFixture.svelte
- catalog/fixtures/ShellOverlayFixture.svelte
- docs/catalog.md
- docs/ui-provenance.json
- spec/src/current/contracts/ui/11-shell-composition.md
- spec/src/current/progress/M4/index.md
- web/src/lib/ui/community-app-shell/CommunityAppShell.svelte
- web/src/lib/ui/community-app-shell/right-sidebar.css
- web/src/lib/ui/community-app-shell/shell-header-geometry.ts
- web/src/lib/ui/store-agent-execution/StoreAgentDetailsSidebar.svelte
- web/src/lib/ui/store-agent-workspace/StoreWorkspaceFiles.svelte
- web/src/lib/ui/store-application/StoreApplication.svelte
- web/src/lib/ui/store-application/StoreApplicationHeaderActions.svelte
- web/src/lib/ui/store-application/StoreApplicationInspectionPanels.svelte
- web/src/lib/ui/store-application/StoreApplicationMain.svelte
- web/src/lib/ui/store-application/StoreApplicationRightPanels.svelte
- web/src/lib/ui/store-application/StoreRepositoryDetails.svelte
- web/src/lib/ui/store-application/StoreRepositoryNavigation.svelte
- web/src/lib/ui/store-collaboration/StoreAgentSidebar.svelte
- web/src/lib/ui/store-conflicts/StoreConflictSurface.svelte
- web/src/lib/ui/store-conflicts/store-conflicts.css
- web/src/lib/ui/store-discussion/StoreDiscussionSheet.svelte
- web/src/lib/ui/store-threads/StoreThreadFocus.svelte
<!-- /evidence-field -->

<!-- evidence-field:removed_files list -->
## Removed files

- catalog/components/community-app-shell/shell-header-geometry.test.ts
- web/src/lib/ui/community-app-shell/shell-header-geometry.ts
<!-- /evidence-field -->

<!-- evidence-field:removed_tests list -->
## Removed tests

- catalog/components/community-app-shell/shell-header-geometry.test.ts#coalesces_and_cancels_header_measurement
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
pnpm check:ui-ownership
```

**Notes**

Production remains independent of catalog and Community checkouts. Removing retired geometry provenance retains attribution for the owned composition.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/ApplicationMenu.stories.svelte catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-application/DesktopChrome.stories.svelte catalog/components/store-application/DiscussionSheets.stories.svelte catalog/components/store-application/ShellOverlay.stories.svelte --reporter verbose
```

**Notes**

20/20 cold shell/menu/overlay scenarios pass, preserving native actions, history, controller identity and pending drafts.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/WorkspaceFiles.stories.svelte catalog/components/store-application/WorkspaceInspector.stories.svelte catalog/components/store-mutations/StoreConflictEditor.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceConflictLayout.stories.svelte --reporter verbose
```

**Notes**

14/14 complete affected file/conflict stories pass without changing native mutation requests or application capabilities.
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

macOS ARM64; Node 26.9.0; pnpm 10.32.1; jj 0.45.1; Vitest 4.1.11; Chromium via repository Playwright. Registry manifest remains @lapismd/design-core ^0.5.1. Local prepared archive from Design Core change kulovuqu/e6320c0d and verification ab1570ca uses a development 0.6.0 manifest; registry 0.6.0 does not contain these unpublished additions. SHA-256 272678190a387b017f73b58c46417093340d9ccc069d31240e4e857fa629817a.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No API, route, saved-layout, pending-command or storage format changes. Existing panel IDs and widths remain authoritative. Tracked package.json and pnpm-lock.yaml retain registry resolutions. Ignored archive and pnpm readPackage hook are documented in docs/catalog.md.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Confirm only unchanged API presentation/command ownership and provenance governance contracts for the shell primitive adoption. This record does not qualify registry release, global architecture, documentation generation or visual approval; those gaps stay in E-SHELL-LAYOUT-001.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Retain application-owned API/controller state and source provenance while completing separate release gates.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: "JST-API-005 repository/snapshot, graph, file, metadata,
  mutation-preview and search capabilities stay on their existing controllers
  and public commands. JST-GOV-002 ownership/provenance policy is unchanged:
  retired geometry is replaced by upstream public exports, owned Community
  adaptation retains attribution, and no runtime/source checkout import, backend
  policy, schema or permission is changed. Fresh UI behavioral evidence is
  E-SHELL-LAYOUT-001."

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M4.5 retains its established shell acceptance status. This narrow
  API/ownership confirmation does not promote the deferred registry-release gate
  or other incomplete milestones.

```
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


