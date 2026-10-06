---
schema: 1
id: E-UI-047
---

# E-UI-047 details 2

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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/community-projects/RepositoryDocuments.stories.svelte
```

**Notes**

Before: shared Markdown preview absent. .scratch/project-polish-documents-before.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/projects-hub.test.ts web/tests/vite/store-route-history.test.ts -t 'repository Projects'
```

**Notes**

Before: repository browse lacks target scope and reload redirects to Files. .scratch/project-polish-repository-before.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/community-projects/RepositoryDocuments.stories.svelte
```

**Notes**

Five document tabs, inline code, emphasis, link and three-pixel indicator. 1 case. .scratch/project-polish-documents-after.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/native-activity-filter.test.ts web/tests/vite/projects-hub.test.ts web/tests/vite/store-route-history.test.ts
```

**Notes**

22 tests. .scratch/project-polish-unit.log.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/ProjectsActivityControls.stories.svelte
```

**Notes**

New story assumes visible right rail and a Files tab label. Actual controls require Toggle details and tab is Overview. Repaired fixture navigation, no product assertion removed. .scratch/project-polish-activity-stories.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/ProjectsActivityControls.stories.svelte
```

**Notes**

3 filter/scope cases. .scratch/project-polish-activity-stories-final.log.
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
pnpm exec tsx --test web/tests/store-route-codec.test.ts
```

**Notes**

10 route tests; invalid, repeated and empty activity values fail closed. .scratch/project-polish-route.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/projects-hub.test.ts web/tests/vite/projects-directories.test.ts web/tests/vite/projects-repositories.test.ts web/tests/vite/projects-code-navigation.test.ts web/tests/vite/native-activity.test.ts web/tests/vite/native-activity-filter.test.ts web/tests/vite/store-route-history.test.ts web/tests/vite/production-store-bootstrap.test.ts
```

**Notes**

54 direct boundary tests before the two extra scope regressions. .scratch/project-polish-direct-units.log.
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
```

**Notes**

Initial and final zero errors/warnings. .scratch/project-polish-web-{check,final}.log.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 6
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:check
```

**Notes**

Five new story ByRole exact options are unsupported and initial fixture prop captures without untrack warns once. Fixed types and explicit initial capture. .scratch/project-polish-catalog-check.log.
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
pnpm catalog:check
```

**Notes**

Zero errors/warnings; 98 catalog modules. .scratch/project-polish-catalog-final.log.
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
pnpm check:architecture
```

**Notes**

415 production modules, published exports and budgets. .scratch/project-polish-architecture{,-final}.log.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/community-projects/RepositoryDocuments.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte catalog/components/store-application/StoreFileEditor.stories.svelte catalog/components/store-work/ProjectsHub.stories.svelte catalog/components/store-work/ProjectsDirectory.stories.svelte catalog/components/store-work/ProjectsActivity.stories.svelte catalog/components/store-work/ProjectsActivityControls.stories.svelte
```

**Notes**

25 pass; existing preview assertion expects literal script markup from the old parser. Shared safe renderer omits executable HTML. Corrected expected policy while retaining source-byte and no-script checks. .scratch/project-polish-direct-stories.log.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreRepositoryApplication.stories.svelte
```

**Notes**

Repaired direct source/preview boundary: 6 pass, no script execution, original Source bytes retained. Together with the 20 other passing direct cases, all 26 covered. .scratch/project-polish-file-story-final.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/projects-code-navigation.test.ts
```

**Notes**

7 navigation tests, including separate authorized code connection for repository Projects. .scratch/project-polish-route-consumer-final.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/projects-hub.test.ts -t 'missing project never broadens Activity'
```

**Notes**

Existing fail-closed project scope also holds under the new filter. 1 pass / 9 skipped. .scratch/project-polish-missing-project-before.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-file-reader.test.ts web/tests/vite/production-store-collaboration.test.ts web/tests/vite/production-store-evolution.test.ts web/tests/vite/store-workspaces.test.ts
```

**Notes**

31 direct history consumers. .scratch/project-polish-history-consumers.log.
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
JJ_STORE_DEV_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs pnpm dev:restart
```

**Notes**

Owned demo only; builds web output and explicit development CLI binary (cargo build -p jj-store-cli --bins --locked). Existing native fixture retained; ready at 7460, 4763ms restart. .scratch/project-polish-restart.log.
<!-- /run -->
<!-- /evidence-field -->

