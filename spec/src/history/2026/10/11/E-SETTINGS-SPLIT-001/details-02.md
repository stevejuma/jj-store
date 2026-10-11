---
schema: 1
id: E-SETTINGS-SPLIT-001
---

# E-SETTINGS-SPLIT-001 validation

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
jj --no-pager st; pnpm spec:search -- JST-UI-012; pnpm spec:search -- JST-UI-014
```

**Notes**

Initial working copy clean: change ortvunut, parent 18ac447e. Read tracked/parent guides and owning Settings, navigation, API, configuration and milestone topics. Inspected maintained Community presentation before released Design Core controls. No subagents.
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
pnpm exec tsx --test web/tests/store-route-codec.test.ts
```

**Notes**

Existing baseline: 12 pass. Added category/subtab route regression: 12 pass / 1 fail before implementation; exact post-change target: 13 pass / 0 fail.
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
pnpm exec tsx --test web/tests/store-route-codec.test.ts web/tests/repository-settings-navigation.test.ts
```

**Notes**

Final: 20 pass. Round-trips/defaults, duplicate/malformed/incompatible fields, exact session Back and cross-repository return isolation. /tmp/jj-settings-final-routes.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/store-route-history.test.ts web/tests/vite/repository-settings-restoration.test.ts web/tests/vite/settings-freshness.test.ts web/tests/vite/production-search-settings.test.ts web/tests/vite/production-storage-settings.test.ts web/tests/vite/production-sync-settings.test.ts web/tests/vite/production-automatic-refresh.test.ts catalog/controller/store-controller.test.ts
```

**Notes**

Final combined run: 48 pass. Draft/revision retention, stale-response rejection, stable pending request IDs, automatic observations, route history, guarded external owner changes and no ordinary reads while viewing Settings. Earlier focused configuration/Stacks run: 46 pass; history/restoration run: 18 pass. /tmp/jj-settings-final-controllers.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/repository-settings-restoration.test.ts web/tests/vite/stacks-navigation.test.ts
```

**Notes**

Two exact restoration regressions pass, including connection preflight before teardown. The incorrectly named optional stacks-navigation path selects no tests; the correct store-stacks-navigation target is run separately. /tmp/jj-settings-final-restoration.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/store-stacks-navigation.test.ts web/tests/vite/server-settings.test.ts
```

**Notes**

24 pass for directly affected Stacks history and retained installation configuration owners. /tmp/jj-settings-final-direct-consumers.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/SettingsNavigation.stories.svelte catalog/components/store-application/StoreRepositorySettings.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceSettings.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceConflictVersions.stories.svelte
```

**Notes**

Final: 22 pass with interaction/Axe. Initial run had seven cold lazy-readiness failures; retained deliberate bounded 10-second view readiness waits, then SettingsNavigation eight pass. Four families pass after owner/guard/header extraction. /tmp/jj-settings-final-stories.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreRepositorySettings.stories.svelte catalog/components/store-application/StoreSearchSettings.stories.svelte catalog/components/store-application/StoreStorageSettings.stories.svelte catalog/components/store-application/StoreSyncSettings.stories.svelte catalog/components/store-application/StoreWorkspaceSettings.stories.svelte catalog/components/store-application/StoreServerSettings.stories.svelte
```

**Notes**

Direct consumers: 34 pass / 1 fail. LocalOnly expected a disabled Progress tab and an incorrect generic WorkStore warning. Updated that obsolete scenario to assert omission of inapplicable Progress and the warning. /tmp/jj-settings-final-settings-consumers.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreSyncSettings.stories.svelte -t LocalOnly
```

**Notes**

Exact regression: one pass, four skipped. Same intended local-only state and action restrictions retained. /tmp/jj-settings-sync-story-after.log.
<!-- /run -->
<!-- /evidence-field -->
