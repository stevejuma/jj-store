---
schema: 1
id: E-M11-005
---

# E-M11-005 details 7

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
JJ_STORE_DEV_ROOT=/tmp/jsh-cold-ijPU4c JJ_STORE_DEV_PORT=7468 JJ_STORE_DEV_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs pnpm dev:restart
```

**Notes**

jsh-latest-cold7.log: Startup: 1246 ms cold, 3889 ms restart; Resident memory: 173824 KiB server plus storage
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
JJ_STORE_REAL_BASE_URL=http://127.0.0.1:7468 JJ_STORE_REAL_REPOSITORY=5ae8240e6d77fe8d0f6dd0b13f3d3874 JJ_STORE_REAL_RUNTIME=/tmp/jsh-cold-ijPU4c JJ_STORE_REAL_ROOT=/Users/stevejuma/code/lapismd/jj-store-projects-hub pnpm exec playwright test --config web/playwright.real.config.ts work-store.spec.ts
```

**Notes**

jsh-latest-native1.log: Strict repository search placeholder collision; helper now addresses named combobox.
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
JJ_STORE_REAL_BASE_URL=http://127.0.0.1:7468 JJ_STORE_REAL_REPOSITORY=5ae8240e6d77fe8d0f6dd0b13f3d3874 JJ_STORE_REAL_RUNTIME=/tmp/jsh-cold-ijPU4c JJ_STORE_REAL_ROOT=/Users/stevejuma/code/lapismd/jj-store-projects-hub pnpm exec playwright test --config web/playwright.real.config.ts work-store.spec.ts
```

**Notes**

jsh-latest-native2.log: Toolbar click intercepted before Community hover delay; helper now hovers the actual message and waits for data-actions-visible for all three actions.
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
JJ_STORE_REAL_BASE_URL=http://127.0.0.1:7468 JJ_STORE_REAL_REPOSITORY=5ae8240e6d77fe8d0f6dd0b13f3d3874 JJ_STORE_REAL_RUNTIME=/tmp/jsh-cold-ijPU4c JJ_STORE_REAL_ROOT=/Users/stevejuma/code/lapismd/jj-store-projects-hub pnpm exec playwright test --config web/playwright.real.config.ts work-store.spec.ts
```

**Notes**

jsh-latest-native3.log: Generated CLI artifact non-executable (EACCES); isolated explicit binary rebuild restores execution.
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
JJ_STORE_REAL_BASE_URL=http://127.0.0.1:7468 JJ_STORE_REAL_REPOSITORY=5ae8240e6d77fe8d0f6dd0b13f3d3874 JJ_STORE_REAL_RUNTIME=/tmp/jsh-cold-ijPU4c JJ_STORE_REAL_ROOT=/Users/stevejuma/code/lapismd/jj-store-projects-hub pnpm exec playwright test --config web/playwright.real.config.ts work-store.spec.ts
```

**Notes**

jsh-latest-native4.log: All task/discussion assertions reached final runtime check; one null scrollWidth error in consumer document-tab fitting after navigation. Added original-node identity and connectivity checks across ticks.
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
node .scratch/projects-final-qa.mjs
```

**Notes**

jsh-latest-visual1.log: QA helper expected manager play actions in a direct iframe; direct iframe does not run play. Helper now navigates production controls explicitly.
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
node .scratch/projects-final-qa.mjs
```

**Notes**

jsh-latest-visual2.log: Partial screenshots captured, helper missed collapsed right rail before Task state. Reopen the actual rail before capture.
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
node .scratch/projects-final-qa.mjs
```

**Notes**

jsh-latest-visual3.log: Six partial captures followed by Task state selector failure. No completed QA claim.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreRepositoryPicker.stories.svelte catalog/components/store-application/StoreConnection.stories.svelte catalog/components/store-work/ProjectsHub.stories.svelte catalog/components/store-work/StoreWork.stories.svelte
```

**Notes**

jsh-latest-stories1.log: 22 pass, one query assertion failure; corrected selector.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreRepositoryPicker.stories.svelte catalog/components/store-application/StoreConnection.stories.svelte catalog/components/store-work/ProjectsHub.stories.svelte catalog/components/store-work/StoreWork.stories.svelte
```

**Notes**

jsh-latest-stories2.log: 22 pass, one sidebar collapse timing failure; waits follow Collapsible completion.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreRepositoryPicker.stories.svelte catalog/components/store-application/StoreConnection.stories.svelte catalog/components/store-work/ProjectsHub.stories.svelte catalog/components/store-work/StoreWork.stories.svelte
```

**Notes**

jsh-latest-stories3.log: Test Files  4 passed (4); Tests  23 passed (23)
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/ProjectsHub.stories.svelte catalog/components/store-work/StoreWork.stories.svelte
```

**Notes**

jsh-latest-stories4.log: Test Files  2 passed (2); Tests  14 passed (14)
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/ProjectsHub.stories.svelte catalog/components/store-work/StoreWork.stories.svelte
```

**Notes**

jsh-latest-stories5.log: Legacy 5/5 pass; hub import fails after Vite discovers list-filter during the run. Added list-filter/list-todo to startup optimization; cold targeted rerun follows.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/ProjectsHub.stories.svelte catalog/components/store-work/StoreWork.stories.svelte catalog/components/store-application/StoreRepositoryNavigation.stories.svelte catalog/components/store-application/StoreApplication.stories.svelte
```

**Notes**

jsh-latest-stories6.log: 28 pass, 3 fail: two assertions still required the removed section margin, and imported navigation styles retained menu space. Updated spacing expectations to the new requirement and scoped no-menu trailing control geometry.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/ProjectsHub.stories.svelte catalog/components/store-work/StoreWork.stories.svelte catalog/components/store-application/StoreRepositoryNavigation.stories.svelte catalog/components/store-application/StoreApplication.stories.svelte
```

**Notes**

jsh-latest-stories7.log: Test Files  4 passed (4); Tests  31 passed (31)
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

jsh-latest-stories8.log: Test Files  1 passed (1); Tests  9 passed (9)
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreRepositoryPicker.stories.svelte catalog/components/store-application/StoreConnection.stories.svelte catalog/components/store-work/ProjectsHub.stories.svelte catalog/components/store-work/StoreWork.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte
```

**Notes**

jsh-latest-stories9.log: 28 pass, 1 fail: both accessible title and Command label contain Select repository; checks now cover both hidden labels.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreRepositoryPicker.stories.svelte catalog/components/store-application/StoreConnection.stories.svelte catalog/components/store-work/ProjectsHub.stories.svelte catalog/components/store-work/StoreWork.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte
```

**Notes**

jsh-latest-stories10.log: Test Files  5 passed (5); Tests  29 passed (29)
<!-- /run -->
<!-- /evidence-field -->

