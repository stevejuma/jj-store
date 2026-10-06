---
schema: 1
id: E-UI-094
---

# E-UI-094

<!-- evidence-field:id markdown -->
## Id

E-UI-094
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M11.4
- M15.5
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-04
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Sidebar creation actions and task, thread and commit activity presentation
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change zulyxxpy
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Main workspace starts clean at rsouwqrt/c8b7ea72. Parent task button includes Parent prefix and ghost resting paint; selected work choice uses a 24px bare check inside a shortcut keycap; commit anchor only covers message content. Sidebar lacks add actions. Thread metadata divs inherit typography instead of sharing the Name button label tokens. Exact baseline stories: 2 failures / 21 passes; parent prefix and selected check size fail. Existing typography comparison passes in catalog, so live computed styles are additionally checked.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Node 24.10.0; pnpm 10.32.1; JJ 0.45.1; locked Svelte 5, Storybook 10.5.7, Vitest 4.1.11, Playwright 1.63.0 and Design Core public components. No dependency changes.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Presentation and existing route/dialog composition only. No HTTP, WAL, manifest, receipt, storage, companion metadata or native identity format changes.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-threads/StoreThreadAlignment.stories.svelte
- catalog/components/store-work/NativeActivity.stories.svelte
- catalog/components/store-work/ProjectsHub.stories.svelte
- catalog/components/store-work/StoreWork.stories.svelte
- catalog/fixtures/NativeActivityFixture.svelte
- catalog/fixtures/projects-hub-layout.ts
- spec/src/api.md
- spec/src/ui.md
- web/src/lib/ui/community-channel-settings/CommunityChannelSettings.css
- web/src/lib/ui/community-navigation/CommunityNavigation.svelte
- web/src/lib/ui/community-navigation/CommunityNavigationAction.svelte
- web/src/lib/ui/community-navigation/CommunityNavigationSectionPicker.svelte
- web/src/lib/ui/community-navigation/icons.ts
- web/src/lib/ui/community-navigation/model.ts
- web/src/lib/ui/store-application/StoreApplication.svelte
- web/src/lib/ui/store-application/StoreApplicationNavigation.svelte
- web/src/lib/ui/store-collaboration/StoreRepositoryActivity.svelte
- web/src/lib/ui/store-collaboration/community-native-activity.css
- web/src/lib/ui/store-work/StoreWorkPicker.svelte
- web/src/lib/ui/store-work/StoreWorkTaskDetail.svelte
- web/tests/browser/sidebar-repository-actions.spec.ts
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/browser/sidebar-repository-actions.spec.ts#sidebar add actions open
  the matching existing dialogs without writes
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-threads/StoreThreadAlignment.stories.svelte catalog/components/store-work/StoreWork.stories.svelte catalog/components/store-work/ProjectsHub.stories.svelte (before)
```

**Notes**

21 pass, 2 fail: Parent prefix retained; selected check is 24px instead of at most 18px. .local/sidebar-polish-before.log.
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
pnpm test:storybook catalog/components/store-threads/StoreThreadAlignment.stories.svelte catalog/components/store-work/StoreWork.stories.svelte catalog/components/store-work/ProjectsHub.stories.svelte catalog/components/store-work/NativeActivity.stories.svelte (first after)
```

**Notes**

22 pass, 2 fixture failures: readonly task helper still expects old Parent prefix; private activity fixture uses a non-route repository ID. Updated the expected accessible status/title and gave this isolated fixture a valid repository ID. .local/sidebar-polish-after.log.
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
pnpm test:storybook catalog/components/store-threads/StoreThreadAlignment.stories.svelte catalog/components/store-work/StoreWork.stories.svelte catalog/components/store-work/ProjectsHub.stories.svelte catalog/components/store-work/NativeActivity.stories.svelte
```

**Notes**

24/24 interactions and accessibility pass. Covers typography, status/title/secondary parent action, circular 16px selection check, whole-byline anchor hit area, pinned route IDs, independent copy controls and agent disclosure/virtualization. .local/sidebar-polish-after2.log.
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
pnpm test:storybook catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-threads/StoreThreadNavigation.stories.svelte catalog/components/store-work/NavigationPersistence.stories.svelte catalog/components/store-work/ProjectsActivity.stories.svelte catalog/components/store-work/ProjectsRepositoryActions.stories.svelte
```

**Notes**

26/26 directly affected shell, expanded/compact navigation, persistence and Projects activity/repository dialog interactions and accessibility. .local/sidebar-polish-consumers.log.
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
pnpm test:storybook catalog/components/store-work/ProjectsHub.stories.svelte (final selected-indicator assertion)
```

**Notes**

11/11 pass after the final assertion uses the foundation actual shortcut data-ui-part to prohibit a keycap around the indicator. .local/sidebar-polish-picker-final.log.
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
pnpm web:check; pnpm catalog:check; pnpm check:architecture; pnpm check:ui-ownership
```

**Notes**

Web and catalog 0 errors/0 warnings; 845-module architecture and 163-module catalog ownership boundaries pass. Navigation composition extracted to keep StoreApplication within its unchanged budget. .local/sidebar-polish-{web-final,catalog-final,architecture-final,ownership}.log.
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
pnpm web:build
```

**Notes**

Fresh production bundle in 7.67s; existing large-chunk warning retained. .local/sidebar-polish-build.log.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec playwright test --config .local/settings-playwright.config.ts web/tests/browser/sidebar-repository-actions.spec.ts (initial attempts)
```

**Notes**

Two starts found the shared 7458 preview port occupied; no assertions ran on those starts. One run reached both expanded actions, then timed out because its compact selector assumed Search Your projects rather than the Command View root accessible name. No services stopped. .local/sidebar-polish-browser{,2,3}.log.
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
pnpm exec playwright test --config .local/sidebar-polish.playwright.config.ts web/tests/browser/sidebar-repository-actions.spec.ts (first isolated run)
```

**Notes**

Wrong textbox role assumption times out. Snapshot confirms actual combobox named Your projects navigation. Corrected selector without changing product semantics. .local/sidebar-polish-browser4.log.
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
pnpm exec playwright test --config .local/sidebar-polish.playwright.config.ts web/tests/browser/sidebar-repository-actions.spec.ts
```

**Notes**

1/1 on fresh isolated Vite preview 7464 serving built assets; both correct dialogs from expanded headings and compact keyboard-filtered menus, focus inside dialogs, no route/group changes or registration/WorkStore writes, axe 0 violations. Isolated preview is not production-CSP qualification. .local/sidebar-polish-browser5.log.
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
CUA background-tab live verification at http://127.0.0.1:7460
```

**Notes**

Clicking bylines opens exact 40-hex Git-backed and 128-hex native commits with their operation/change pins and 2 changed files each. Add project opens Create project with writable-home selector. All 7 thread labels are DM Sans Variable,14px,400. Parent has in_progress icon and secondary resting background. Selected work check is circular16x16. Screenshots .local/task-parent-polish.jpg and .local/task-picker-polish.jpg saved and reviewed. Dialogs dismissed without saving; temporary tab closed; user tab and dev fixture untouched.
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
pnpm exec prettier --write changed files; pnpm exec prettier --check changed files; pnpm spec:first; pnpm spec:validate; pnpm check:progress
```

**Notes**

Changed-file format clean. Spec-first covers19protected files/3canonical chapters;5validators/22chapters/152requirements pass.125-task evidence bookkeeping passes, without milestone promotion. .local/sidebar-polish-{format,spec-first,spec-validate,progress}.log.
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

Five user refinements in the main workspace, including the follow-up that Your projects creates a project with Lucide briefcase-plus; Repositories retains Git-plus Add local repository and Browse threads. Reuses owned Community composition and released foundation APIs. UI-only targeted policy: 50 distinct affected stories and one cold browser flow, without root aggregates, Rust lanes, fixture resets, unrelated services or publication. Visual baselines remain pending and no milestone status is promoted.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Commit verified slice in main and retain existing Vite HMR server for review.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


