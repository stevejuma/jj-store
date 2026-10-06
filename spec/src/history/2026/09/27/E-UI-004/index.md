---
schema: 1
id: E-UI-004
---

# E-UI-004

<!-- evidence-field:id markdown -->
## Id

E-UI-004
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4.7
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-27
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

storybook-complete-application
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- web/src/lib/controller/contracts.ts
- catalog/controller/in-memory-store-controller.svelte.ts
- catalog/controller/store-controller.test.ts
- web/src/lib/ui/store-application/StoreRepositoryApplication.svelte
- catalog/components/store-application/StoreRepositoryApplication.stories.svelte
- catalog/components/store-application/RepositoryApplication.stories.svelte
- web/src/lib/ui/community-profile-menu/CommunityProfileMenu.svelte
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryFiles.svelte
- web/src/lib/ui/community-projects/CommunityProjectsRepositoryBranchSelector.svelte
- web/src/lib/ui/community-repository-branch-command/CommunityRepositoryBranchCommand.svelte
- docs/catalog.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- catalog/controller/store-controller.test.ts#controller_restores_snapshot_history
- catalog/controller/store-controller.test.ts#controller_isolates_instances_and_conflicts
<!-- /evidence-field -->

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
pnpm catalog:test catalog/controller/store-controller.test.ts
```

**Notes**

Before implementation: controller module absent; one failed suite, no tests executed.
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
pnpm catalog:test catalog/controller/store-controller.test.ts
```

**Notes**

2 tests pass: snapshot/history restoration, instance and conflict isolation.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:check
```

**Notes**

Local view named state shadowed the Svelte rune; renamed the derived view.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-application/StoreRepositoryApplication.stories.svelte
```

**Notes**

Two file selectors used basename instead of public full-path labels, menu close animation required pointer restoration, and empty breadcrumb had no accessible name. Same count reproduced on prebundle rerun.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-application/StoreRepositoryApplication.stories.svelte
```

**Notes**

Profile menu unmounted before global pointer restoration; wait for actual interactive state.
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
pnpm test:storybook catalog/components/store-application/StoreRepositoryApplication.stories.svelte
```

**Notes**

4 stories and blocking axe pass, including full change-ID search and divergent target selection.
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
pnpm test:storybook catalog/components/store-application/StoreRepositoryApplication.stories.svelte catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.stories.svelte
```

**Notes**

6 stories pass: repository files, conflicts, profile login/logout, mobile files, branch card filtering and empty cards.
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
pnpm test:storybook catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.stories.svelte
```

**Notes**

Bookmark cards regression reproduced joined author text (byMaya). Empty bookmarks passed.
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
pnpm test:storybook catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.stories.svelte
```

**Notes**

2 stories pass after preserving explicit whitespace between relative time and author.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check
```

**Notes**

Final 18 lanes pass: 19 catalog unit tests, 34 interaction/axe stories, cold static catalog, 8 existing browser cases, 369 Rust tests (6 external-service/credential tests ignored), rustfmt, contracts, governance, boundaries and mdBook.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test
```

**Notes**

Final aggregate passes: governance, storage harness self-tests, contracts, browser unit tests, 19 catalog unit tests, 34 stories, cold catalog and 369 Rust tests; 6 external-service/credential tests ignored.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build
```

**Notes**

Static Storybook, existing browser bundle/manifest and all locked Rust workspace binaries build successfully.
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
pnpm format:check
```

**Notes**

Rustfmt and Prettier pass.
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

Community d859f33a484a9238bb11f974f4ead8e9be729ec6; Storybook 10.5.7; Svelte CSF 5.1.2; Vitest 4.1.11; Design Core 0.2.0 via ^0.2.0; Tailwind and Tailwind Vite 4.3.3 via ^4.3.3.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Typed local presentation/controller contracts only. No API, server, credential or storage formats changed; enclosing Jujutsu change is the evidence reference.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


