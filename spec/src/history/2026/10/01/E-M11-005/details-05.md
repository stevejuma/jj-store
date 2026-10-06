---
schema: 1
id: E-M11-005
---

# E-M11-005 details 5

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

Final five steering builds/restarts passed, using independent ignored target and unchanged existing RustFS runtime. Latest cold startup2327ms; restart3762ms; preparation0ms; RSS154432KiB combined. Earlier fourth4272ms/152528KiB.
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
JJ_STORE_REAL_BASE_URL=http://127.0.0.1:7468 JJ_STORE_REAL_REPOSITORY=a97052bbbb54ccf91a085591e016376a JJ_STORE_REAL_RUNTIME=/tmp/jsh-cold-ijPU4c JJ_STORE_REAL_ROOT=/Users/stevejuma/code/lapismd/jj-store-projects-hub pnpm exec playwright test --config web/playwright.real.config.ts work-store.spec.ts
```

**Notes**

1/1 in47.3s (48.2s total). Cold real RustFS: two metadata homes and two code repos; four tabs; expanded linked repository children; unfiltered sidebar survives main filters; create/edit project/task; hierarchy/state; link/retirement; external SSE explicit refresh; threaded reply/edit/tombstone; one native committed lost-ack request recovered across navigation with exactly one comment; legacy route/reload/full IDs; linked-code Back; native review open/Back; axe and420px no overflow; runtimeErrors empty.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_REAL_BASE_URL=http://127.0.0.1:7468 JJ_STORE_REAL_REPOSITORY=a97052bbbb54ccf91a085591e016376a JJ_STORE_REAL_RUNTIME=/tmp/jsh-cold-ijPU4c JJ_STORE_REAL_ROOT=/Users/stevejuma/code/lapismd/jj-store-projects-hub pnpm exec playwright test --config web/playwright.real.config.ts native-repository.spec.ts
```

**Notes**

All5 original repository scenarios stopped in sole-repository bootstrap helper: previous WorkStore runs had added25 repositories and the app correctly opened the installation Projects hub. Reset only owned disposable state to run this exact original gate against its required single-repository fixture; no production contract weakened.
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
node .scratch/projects-qa.mjs
```

**Notes**

Inspected owned landing/form/mobile screenshots with0 runtime errors. Visual inspection led to final landing tabs before heading, create/refresh actions beside the icon heading and no redundant landing breadcrumb. Visual baselines remain pending.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/ProjectsHub.stories.svelte catalog/components/store-work/StoreWork.stories.svelte
```

**Notes**

Compact shared-repository story had11/12 passing; assertion counted two native select options outside the command picker along with its single deduplicated repository. Scoped assertion to the listbox.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/ProjectsHub.stories.svelte catalog/components/store-work/StoreWork.stories.svelte
```

**Notes**

Final12/12 production interaction/accessibility stories after Community heading/action placement and compact shared-repository deduplication. Previous heading-only11/11 also passed.
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
pnpm catalog:check
```

**Notes**

Final catalog check0 errors0 warnings,84 catalog modules after shared-repository fixture.
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
pnpm web:check
```

**Notes**

Final header layout0 errors0 warnings.
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
pnpm check:architecture
```

**Notes**

344 modules within unchanged budgets after final header/nav composition.
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
JJ_STORE_DEV_ROOT=/tmp/jsh-cold-ijPU4c JJ_STORE_DEV_PORT=7468 JJ_STORE_DEV_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs pnpm dev:restart
```

**Notes**

A restart requested before the owned reset finished was safely rejected as occupied/unowned port while the reset held the temporary seeding service. Let reset finish before retry; no user service stopped.
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
pnpm web:check
```

**Notes**

jsh-latest-web1.log: svelte-check found 0 errors and 0 warnings
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
pnpm web:check
```

**Notes**

jsh-latest-web2.log: svelte-check found 0 errors and 0 warnings
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

jsh-latest-web3.log: svelte-check found 0 errors and 0 warnings
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
pnpm web:check
```

**Notes**

jsh-latest-web4.log: svelte-check found 0 errors and 0 warnings
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
pnpm web:check
```

**Notes**

jsh-latest-web5.log: svelte-check found 0 errors and 0 warnings
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
pnpm web:check
```

**Notes**

jsh-latest-web6.log: svelte-check found 0 errors and 0 warnings
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
pnpm web:check
```

**Notes**

jsh-latest-web7.log: svelte-check found 0 errors and 0 warnings
<!-- /run -->

<!-- run -->
### Run 18

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

jsh-latest-catalog1.log: Two Testing Library getByRole overload errors; removed unsupported exact options.
<!-- /run -->
<!-- /evidence-field -->

