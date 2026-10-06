---
schema: 1
id: E-UI-051
---

# E-UI-051 details 3

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
pnpm check:architecture
```

**Notes**

491 production/catalog modules pass; rerun after final selector change.
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
pnpm web:build
```

**Notes**

Final project-only picker bundle and asset manifest pass; inherited dependency CSS/chunk warnings retained. Prior section-icon build also passed. .scratch/sidebar-project-picker-build.log; .scratch/sidebar-section-icons-build.log.
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
JJ_STORE_DEV_PORT=7460 JJ_STORE_DEV_ROOT=/Users/stevejuma/code/lapismd/jj-store/.local/dev-demo JJ_STORE_DEV_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs node .scratch/local-browser-merge-prebuilt.mjs restart
```

**Notes**

Latest production UI served on 7460 using unchanged qualified Rust binaries. All three demo repository identities and data retained. .scratch/sidebar-project-picker-ui-restart.log.
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
CUA projectIconPreview.reload(); getAXState(); read rendered section SVG classes; projectIconPreview.close()
```

**Notes**

Fresh built UI shows Folder Kanban for Projects, Folder Git 2 for Repositories, Messages Square for Threads. Your projects popup contains only Developer experience, Native repository browser and Recovery and reliability, without linked repository options. Hash-only goto retained earlier JS until real reload; fresh reload verified current behavior. Temporary hidden verification tab closed; user tabs preserved.
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
pnpm exec prettier --write web/src/lib/ui/community-navigation/icons.ts web/src/lib/ui/community-navigation/CommunityNavigation.svelte catalog/components/store-work/ProjectsHub.stories.svelte spec/src/ui.md spec/src/api.md spec/src/milestones.md
```

**Notes**

Final six changed files formatted.
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
pnpm check:progress
```

**Notes**

Final evidence bookkeeping passed for 119 tasks.
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
pnpm spec:first
```

**Notes**

3 protected files and 3 canonical chapters passed the same-change gate.
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
pnpm spec:validate
```

**Notes**

4 validators, 21 chapters and 141 requirements passed.
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
pnpm exec prettier --check web/src/lib/ui/community-navigation/icons.ts web/src/lib/ui/community-navigation/CommunityNavigation.svelte catalog/components/store-work/ProjectsHub.stories.svelte spec/src/ui.md spec/src/api.md spec/src/milestones.md
```

**Notes**

Final source, story and canonical files passed formatting.
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

Existing Lucide Svelte registry dependency; Vitest 4.1.11, Chromium Storybook and macOS in-app browser.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Project destinations, rows and groups use Folder Kanban; compact Threads uses Messages Square; explicit channels retain Hash and repositories retain Folder Git. Mapping recognizes actual threads section ID. Collapsed Projects returns original project rows without flattening linked repositories; other sections retain deduplication and repository access. Exact home/project identities, expansion persistence, retry/paging and routes remain unchanged. Enclosing JJ change vkutkopr.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

One UI sidebar navigation surface, including shared right-rail project icons. Inspected Community navigation and existing Design Core sidebar example; reused installed Lucide components and Command View. Functional before/after regression plus four direct Storybook interaction/accessibility families, web/catalog type checks, architecture, built assets and live served verification qualify the final slice. No new icon-mirroring tests. Rust, root aggregates, unrelated suites and release qualification are out of scope under targeted policy. Stories remain visual-pending; visual baselines separately unapproved.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Retain project-only compact navigation, separate repository access and section-icon regressions. M11.4 and M15.5 retain their existing In progress status; deferred capabilities and visual baseline approval are unchanged.
<!-- /evidence-field -->

