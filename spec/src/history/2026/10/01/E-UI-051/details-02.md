---
schema: 1
id: E-UI-051
---

# E-UI-051 details 2

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
pnpm exec prettier --write web/src/lib/ui/community-navigation/icons.ts spec/src/ui.md spec/src/api.md
```

**Notes**

Changed presentation mapping and canonical chapters formatted.
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
pnpm web:check
```

**Notes**

0 errors and warnings. .scratch/projects-icon-web-check.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/NavigationPersistence.stories.svelte catalog/components/store-work/ProjectsHub.stories.svelte catalog/components/store-application/StoreRepositoryNavigation.stories.svelte -t 'Disclosure choices survive remount|Landing tabs sidebar hierarchy reviews and icon empty state|Compact projects sharing a repository|Collapsed repository section alignment|Right rail navigation'
```

**Notes**

Command exited zero but all 16 cases were skipped because the human story-name filter did not match generated test names. This run provides no acceptance; the complete three-file lane was rerun without filtering. .scratch/projects-icon-stories.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/NavigationPersistence.stories.svelte catalog/components/store-work/ProjectsHub.stories.svelte catalog/components/store-application/StoreRepositoryNavigation.stories.svelte
```

**Notes**

All 16 interaction/accessibility cases in 3 files pass: Projects hub, persisted project disclosure, compact/expanded sidebar and shared right-navigation consumer. Fresh process. .scratch/projects-icon-stories-final.log.
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
pnpm web:build
```

**Notes**

UI bundle and asset manifest pass; inherited dependency CSS and chunk warnings retained. .scratch/projects-icon-build.log.
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
JJ_STORE_DEV_PORT=7460 JJ_STORE_DEV_ROOT=/Users/stevejuma/code/lapismd/jj-store/.local/dev-demo JJ_STORE_DEV_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs node .scratch/local-browser-merge-prebuilt.mjs restart
```

**Notes**

Owned dev server restarted with rebuilt UI and qualified unchanged Rust binaries. Existing three demo identities/data retained; no Rust builds. .scratch/projects-icon-ui-restart.log.
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
CUA createBrowserTab(iab, current local repository Op Log URL, visible=false); reload; read rendered navigation SVG classes
```

**Notes**

Fresh served UI confirms Projects and Your projects controls have lucide-folder-kanban; Repositories retains lucide-folder-git-2. Temporary verification tab only; no changes to user tabs.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

One bookkeeping diagnostic: skipped-case run had result fail with zero failures. Recorded its actual zero exit as pass while explicitly retaining that no test acceptance came from it. Product acceptance remains the subsequent 16 executed passing cases.
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
pnpm spec:first
```

**Notes**

One protected mapping file and three canonical chapters pass.
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
pnpm spec:validate
```

**Notes**

21 chapters and 141 requirements pass.
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
pnpm exec prettier --check web/src/lib/ui/community-navigation/icons.ts spec/src/api.md spec/src/ui.md spec/src/milestones.md
```

**Notes**

Changed source and canonical files pass formatting.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-threads/StoreThreads.stories.svelte
```

**Notes**

3 Threads interaction/accessibility cases passed in a fresh process. .scratch/sidebar-section-icons-threads.log.
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
pnpm web:check
```

**Notes**

Final section icons and project-only picker: 0 errors and warnings. Earlier icon-only check also passed. .scratch/sidebar-section-icons-web-check.log; .scratch/sidebar-project-picker-web-check.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/ProjectsHub.stories.svelte
```

**Notes**

Before changing compactItems: strengthened CompactProjectsSharingARepository regression fails because linked Parser repository is still a Projects option; 1 failed and 8 passed. .scratch/projects-picker-before.log.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/ProjectsHub.stories.svelte catalog/components/store-work/NavigationPersistence.stories.svelte catalog/components/store-application/StoreRepositoryNavigation.stories.svelte catalog/components/store-threads/StoreThreads.stories.svelte
```

**Notes**

First combined rerun: 1 failed and 18 passed. New Repositories assertion matched two buttons; scoped it to Repository navigation without weakening project-only, search or repository-access assertions. .scratch/sidebar-project-picker-stories-final.log.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/ProjectsHub.stories.svelte
```

**Notes**

Exact before target rerun: all 9 ProjectsHub cases pass, including project-only choices, shared-repository projects, project search, repository-name empty state, focus restoration and separate Repositories access. .scratch/projects-picker-after.log.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/ProjectsHub.stories.svelte catalog/components/store-work/NavigationPersistence.stories.svelte catalog/components/store-application/StoreRepositoryNavigation.stories.svelte catalog/components/store-threads/StoreThreads.stories.svelte
```

**Notes**

Final 19 interaction/accessibility cases in 4 files pass. Covers Projects, persisted disclosure, expanded/compact navigation, shared right rail and Threads in a fresh process. .scratch/sidebar-project-picker-stories-qualified.log.
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
pnpm catalog:check
```

**Notes**

0 errors and warnings; 119 catalog boundary modules pass. Both initial and final checks pass. .scratch/sidebar-project-picker-catalog-check.log; .scratch/sidebar-project-picker-catalog-qualified.log.
<!-- /run -->
<!-- /evidence-field -->

