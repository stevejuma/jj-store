---
schema: 1
id: E-UI-MENU-001
---

# E-UI-MENU-001 details 2

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
cua_repl: connection Connect story and live agent session; read Help geometry and Agent menu computed styles before editing
```

**Notes**

Before: Help stretched to the full 480px card width. Footer Agent label used 600 weight while items used 400; both inherited the same Inter sans family. Community typography/overlay sources were inspected first, then public released Design Core Card, Popover and DropdownMenu parts. The requested app typography is an app-owned design choice, not a foundation defect.
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
pnpm exec storybook dev --host 127.0.0.1 --port 6048 --no-open --disable-telemetry
```

**Notes**

Fresh isolated UI process; existing Vite 7460, backend 7462 and main fixture preserved. No Rust build, restart or data reset.
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
pnpm web:check
```

**Notes**

0 errors and 0 warnings.
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
pnpm catalog:check
```

**Notes**

0 errors and 0 warnings; 352 catalog modules pass boundary checks.
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
pnpm test:storybook catalog/components/store-application/StoreConnection.stories.svelte catalog/components/store-application/StoreComparisonParentPicker.stories.svelte catalog/components/store-application/WorkspaceFiles.stories.svelte catalog/components/store-mutations/StoreMutationCommitPicker.stories.svelte catalog/components/store-mutations/StoreRepositoryActions.stories.svelte catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.stories.svelte catalog/components/store-work/ProjectsFilters.stories.svelte
```

**Notes**

31 tests across 7 files pass. Existing ResizeObserver loop completed with undelivered notifications warning reproduces in responsive stories; not suppressed or claimed resolved. Log: .local/menu-stories.log.
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
pnpm test:storybook catalog/components/store-application/StoreWorkspaces.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte -t 'Workspace|ProfileLoginFooter|Branch'
```

**Notes**

2 tests pass, 9 filtered; profile footer and retained workspace file editing. Log: .local/menu-pickers.log.
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
pnpm test:storybook catalog/components/store-application/StoreWorkspaces.stories.svelte -t 'PickerAndTabHistory|KeyboardSearchAndRepositoryOverview|CollapsedSidebar|MobileSectionHeader'
```

**Notes**

4 tests pass, 1 filtered. Workspace picker search, keyboard, navigation and mobile/collapsed layouts pass. Log: .local/menu-workspaces.log.
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
pnpm check:ui-ownership
```

**Notes**

Existing owned Community compositions and released Design Core imports remain valid.
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
cua_repl: live Agent, Workspace and Profile menus; connection Help at normal and 380px viewport; keyboard Enter/Escape
```

**Notes**

Agent, Workspace and Profile titles and adjacent menu items have matching Inter sans family, 14px size and 400 weight. Help is 80px wide and 24px from the footer edge at both widths. At 380px, document scrollWidth is 380px. Enter opens Help, Escape returns focus to Help. The catalog does not serve the bundled documentation readiness protocol, so its existing timeout notice appeared; this check qualifies trigger geometry and focus, not manual loading. A later optional navigation to the commit-picker story and state retry timed out in the browser bridge; the same picker passed its automated interaction stories. Temporary viewport override was reset.
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
pnpm exec prettier --check web/src/lib/ui/community-typography.css web/src/lib/ui/community-profile-menu/CommunityProfileMenu.svelte web/src/lib/ui/community-projects/CommunityProjectsRepositoryBranchSelector.svelte web/src/lib/ui/store-application/StoreConnection.svelte web/src/lib/ui/store-application/StoreWorkspacePicker.svelte web/src/lib/ui/store-mutations/StoreMutationCommitPicker.svelte spec/src/current/contracts/ui/08-device-appearance.md spec/src/current/contracts/ui/31-bundled-application-documentation.md
```

**Notes**

Initial check required formatting 08-device-appearance.md; corrected and rerun below.
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
pnpm exec prettier --write web/src/lib/ui/community-typography.css web/src/lib/ui/community-profile-menu/CommunityProfileMenu.svelte web/src/lib/ui/community-projects/CommunityProjectsRepositoryBranchSelector.svelte web/src/lib/ui/store-application/StoreConnection.svelte web/src/lib/ui/store-application/StoreWorkspacePicker.svelte web/src/lib/ui/store-mutations/StoreMutationCommitPicker.svelte spec/src/current/contracts/ui/08-device-appearance.md spec/src/current/contracts/ui/31-bundled-application-documentation.md
```

**Notes**

Only owning appearance chapter needed reflow.
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
pnpm exec prettier --check web/src/lib/ui/community-typography.css web/src/lib/ui/community-profile-menu/CommunityProfileMenu.svelte web/src/lib/ui/community-projects/CommunityProjectsRepositoryBranchSelector.svelte web/src/lib/ui/store-application/StoreConnection.svelte web/src/lib/ui/store-application/StoreWorkspacePicker.svelte web/src/lib/ui/store-mutations/StoreMutationCommitPicker.svelte spec/src/current/contracts/ui/08-device-appearance.md spec/src/current/contracts/ui/31-bundled-application-documentation.md
```

**Notes**

Final code and canonical chapter formatting passes.
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
pnpm check:progress
```

**Notes**

146 tasks, 708 evidence records; statuses unchanged.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:links
```

**Notes**

Generated book has no HTML pages in this workspace. This generated-site audit is unavailable; no book build was requested for the presentation slice. Source book and governance checks follow.
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
pnpm exec spec-validator validate --only governance,summary,fileLimits,specFirst,book
```

**Notes**

Reported 4 validators passing, 3380 chapters, 181 requirements.
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
pnpm spec:first
```

**Notes**

The broad web/ mapping also requires an API topic confirmation. Added explicit unchanged JST-API-001 confirmation in the owning API chapter.
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
pnpm exec spec-validator validate --only file-limits
```

**Notes**

Unknown validator spelling; corrected to fileLimits.
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
pnpm exec spec-validator validate --only fileLimits
```

**Notes**

Source physical-line and UTF-8 byte limits pass.
<!-- /run -->
<!-- /evidence-field -->

