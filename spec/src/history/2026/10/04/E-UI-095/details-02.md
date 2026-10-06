---
schema: 1
id: E-UI-095
---

# E-UI-095 details 2

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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-threads/StoreThreadAlignment.stories.svelte catalog/components/store-application/StoreWorkspaceSettings.stories.svelte
```

**Notes**

7 passed; thread rename lacks Settings Field group and folder path group is capped too narrowly. Evidence: .local/readonly-before.log.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-threads/StoreThreadAlignment.stories.svelte
```

**Notes**

2 passed; Thread settings header has a 1px bottom border. Evidence: .local/thread-header-before.log.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-history-query/StoreHistoryQuery.stories.svelte
```

**Notes**

3 passed; Run query action still has text instead of an icon-only control. Evidence: .local/history-polish-before.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-threads/StoreThreadAlignment.stories.svelte catalog/components/store-application/StoreWorkspaceSettings.stories.svelte catalog/components/store-application/StoreSettingsIdentifier.stories.svelte
```

**Notes**

13/13 interactions and accessibility. Full row clipboard hit area, full values, tick feedback, keyboard, failure/retry, empty/zero and stale clipboard completion/disposal; rename form and folder control width. Evidence: .local/readonly-after.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/SettingsNavigation.stories.svelte catalog/components/store-application/StoreSearchSettings.stories.svelte catalog/components/store-application/StoreStorageSettings.stories.svelte catalog/components/store-application/StoreSyncSettings.stories.svelte catalog/components/store-application/StoreSyncReconciliation.stories.svelte catalog/components/store-application/StoreApplication.stories.svelte
```

**Notes**

29 passed; Sync expected stale Through 9 text after the value moved into a reusable copy row. SettingsNavigation, Search, Storage, SyncReconciliation and application consumers pass. Evidence: .local/readonly-consumers.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-threads/StoreThreadAlignment.stories.svelte catalog/components/store-application/StoreSyncSettings.stories.svelte
```

**Notes**

7 passed; initial fixture edit targeted canvas instead of c and missed the same Through 9 assertion; corrected to Copy full Push target through: 9. Evidence: .local/readonly-final-stories.log.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-history-query/StoreHistoryQuery.stories.svelte catalog/components/store-application/StoreSyncSettings.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte
```

**Notes**

17 passed; unselected workspace entry lacks its requested icon. Added FolderGit2 with CircleCheck for selection. Evidence: .local/history-polish-after.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-history-query/StoreHistoryQuery.stories.svelte catalog/components/store-application/StoreSyncSettings.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte catalog/components/store-application/StoreWorkspaces.stories.svelte catalog/components/store-threads/StoreThreadAlignment.stories.svelte
```

**Notes**

26/26 HistoryQuery, SyncSettings, RepositoryLog, Workspaces and ThreadAlignment interactions/accessibility. Includes corrected Sync accessible copy control. Evidence: .local/polish-final-stories.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/SettingsNavigation.stories.svelte catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-application/StoreWorkspaces.stories.svelte catalog/components/store-history-query/StoreHistoryQuery.stories.svelte catalog/components/store-application/StoreSettingsIdentifier.stories.svelte
```

**Notes**

28/28 navigation, application, workspaces, history and readonly-row direct consumers pass. Evidence: .local/polish-consumers-final.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreSearchSettings.stories.svelte catalog/components/store-application/StoreStorageSettings.stories.svelte
```

**Notes**

9/9 SearchSettings and StorageSettings interactions/accessibility pass after explanatory recovery/selection rows remain fully readable. Evidence: .local/polish-readonly-final.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte catalog/components/store-history-query/StoreHistoryQuery.stories.svelte
```

**Notes**

13/13 RepositoryLog and HistoryQuery pass after extracting header composition to preserve the existing module budget. Evidence: .local/polish-header-final.log.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-history-query/StoreHistoryQuery.stories.svelte
```

**Notes**

3 passed; info icons are above vertical center. Live Op Log confirms offsets of -31.5px and -21px. Evidence: .local/history-info-before.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-history-query/StoreHistoryQuery.stories.svelte
```

**Notes**

4/4 including geometric icon centering in the query guidance boxes. Scope remains the query-context composition. Evidence: .local/history-info-after.log.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check
```

**Notes**

Initial history context used unsupported Notice title props and nested $props.id() inside a template expression instead of a top-level initializer. Fixed before final checks. .local/history-polish-web.log.
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
pnpm web:check; pnpm catalog:check
```

**Notes**

Both checks report zero errors/warnings; catalog boundary checks 164 modules. .local/polish-web-final.log, .local/polish-catalog-final.log, .local/polish-web-last.log and .local/polish-catalog-last.log.
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
pnpm check:architecture
```

**Notes**

RepositoryLog reached 307 counted lines against its existing 300-line budget; extracted the owned header composition. .local/polish-architecture-last.log.
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
pnpm check:architecture; pnpm check:ui-ownership
```

**Notes**

851 architecture modules and UI ownership pass after extraction and final palette/icon styling. .local/polish-architecture-final.log, .local/polish-architecture-palette.log, .local/polish-ownership.log and .local/polish-ownership-final.log.
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
pnpm web:build
```

**Notes**

Production assets built repeatedly for cold verification, final .local/polish-build-palette.log; earlier .local/polish-build-last.log. Existing chunk-size/timing warnings remain informational. Reuses the prebuilt native browser_preview binary; no Rust build.
<!-- /run -->
<!-- /evidence-field -->

