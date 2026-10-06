---
schema: 1
id: E-M11-005
---

# E-M11-005 details 10

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
pnpm catalog:check
```

**Notes**

jsh-directory-catalog3.log: svelte-check found 0 errors and 0 warnings; catalog boundaries ok (87 modules)
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
pnpm check:architecture
```

**Notes**

jsh-directory-arch1.log: web architecture ok (376 modules)
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
pnpm check:architecture
```

**Notes**

jsh-directory-arch2.log: web architecture ok (377 modules)
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
pnpm check:architecture
```

**Notes**

jsh-directory-arch3.log: web architecture ok (377 modules)
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
JJ_STORE_DEV_ROOT=/tmp/jsh-cold-ijPU4c JJ_STORE_DEV_PORT=7468 JJ_STORE_DEV_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs pnpm dev:restart
```

**Notes**

jsh-directory-cold1.log: Startup: 1246 ms cold, 4161 ms restart; Resident memory: 153984 KiB server plus storage
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
JJ_STORE_DEV_ROOT=/tmp/jsh-cold-ijPU4c JJ_STORE_DEV_PORT=7468 JJ_STORE_DEV_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs pnpm dev:restart
```

**Notes**

jsh-directory-cold2.log: Startup: 1246 ms cold, 3747 ms restart; Resident memory: 142320 KiB server plus storage
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
JJ_STORE_DEV_ROOT=/tmp/jsh-cold-ijPU4c JJ_STORE_DEV_PORT=7468 JJ_STORE_DEV_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs pnpm dev:restart
```

**Notes**

jsh-directory-cold3.log: Startup: 1246 ms cold, 4928 ms restart; Resident memory: 154432 KiB server plus storage
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/projects-repositories.test.ts web/tests/vite/projects-directories.test.ts web/tests/vite/projects-hub.test.ts web/tests/vite/projects-code-navigation.test.ts web/tests/vite/production-work.test.ts web/tests/vite/production-store-collaboration.test.ts web/tests/vite/production-store-data-source.test.ts web/tests/vite/production-store-bootstrap.test.ts web/tests/vite/store-route-history.test.ts
```

**Notes**

jsh-directory-units1.log: 58/59: mock response lacked JSON MIME required by Client; fixed fixture header.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/projects-repositories.test.ts web/tests/vite/projects-directories.test.ts web/tests/vite/projects-hub.test.ts web/tests/vite/projects-code-navigation.test.ts web/tests/vite/production-work.test.ts web/tests/vite/production-store-collaboration.test.ts web/tests/vite/production-store-data-source.test.ts web/tests/vite/production-store-bootstrap.test.ts web/tests/vite/store-route-history.test.ts
```

**Notes**

jsh-directory-units2.log: 59/59: cache, permission-loss clearing, late response cancellation, exact operation, partial pages and ambiguous heads.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/ProjectsDirectory.stories.svelte catalog/components/store-work/ProjectsHub.stories.svelte catalog/components/store-work/StoreWork.stories.svelte
```

**Notes**

jsh-directory-stories1.log: 17/17 across directory/hub/task stories; interaction and accessibility evidence, visual-pending.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/ProjectsDirectory.stories.svelte catalog/components/store-work/ProjectsHub.stories.svelte catalog/components/store-work/StoreWork.stories.svelte catalog/components/store-application/StoreRepositoryPicker.stories.svelte catalog/components/store-application/StoreConnection.stories.svelte catalog/components/store-application/StoreRepositoryNavigation.stories.svelte catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte catalog/components/store-application/StoreCollaboration.stories.svelte
```

**Notes**

jsh-directory-stories2.log: 56/57: strengthened review geometry regression reproduces 58px trailing gap.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/ProjectsDirectory.stories.svelte catalog/components/store-work/ProjectsHub.stories.svelte catalog/components/store-work/StoreWork.stories.svelte catalog/components/store-application/StoreRepositoryPicker.stories.svelte catalog/components/store-application/StoreConnection.stories.svelte catalog/components/store-application/StoreRepositoryNavigation.stories.svelte catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte catalog/components/store-application/StoreCollaboration.stories.svelte
```

**Notes**

jsh-directory-stories3.log: 57/57 across nine direct directory/hub/task/navigation/shell/connection/review story families.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/ProjectsDirectory.stories.svelte catalog/components/store-work/ProjectsHub.stories.svelte catalog/components/store-work/StoreWork.stories.svelte catalog/components/store-application/StoreRepositoryPicker.stories.svelte catalog/components/store-application/StoreConnection.stories.svelte catalog/components/store-application/StoreRepositoryNavigation.stories.svelte catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte catalog/components/store-application/StoreCollaboration.stories.svelte
```

**Notes**

jsh-directory-stories4.log: Test Files  9 passed (9); Tests  57 passed (57)
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/ProjectsDirectory.stories.svelte
```

**Notes**

jsh-directory-width-before.log: 2/3: review date ends 58px before row edge; exact metadata-row regression.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/ProjectsDirectory.stories.svelte
```

**Notes**

jsh-directory-width-after.log: 2/3: scoped relative-time start alignment wins; use native grid selector specificity.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/ProjectsDirectory.stories.svelte
```

**Notes**

jsh-directory-width-after2.log: 3/3: final date aligns to native row edge; list/cards retain identities and metadata.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/ProjectsDirectory.stories.svelte
```

**Notes**

jsh-directory-card-before.log: 2/3: native card comment count needs 24px in a 16px column and clips; allocate its essential count width.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-service --test review_threads --locked review_threads_are_root_pinned_and_details_resolve_selected_objects -- --exact
```

**Notes**

jsh-directory-review-before.log: 0/1: summary lacks creation actor; exact regression retained.
<!-- /run -->
<!-- /evidence-field -->

