---
schema: 1
id: E-UI-050
---

# E-UI-050 details 2

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
pnpm exec tsx --test web/tests/navigation-storage.test.ts
```

**Notes**

Before implementation: missing navigation persistence module.
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
node --test scripts/install-cli.test.mjs
```

**Notes**

Before implementation: missing installer module.
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
node --test scripts/dev-demo-profiles.test.mjs
```

**Notes**

Before implementation: missing profile seeder.
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
node --test scripts/dev-demo-work.test.mjs
```

**Notes**

Before implementation: all three project links still target the same repository.
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
pnpm exec tsx --test web/tests/navigation-storage.test.ts web/tests/layout-storage.test.ts
```

**Notes**

6 focused storage tests.
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
node --test scripts/dev-demo-profiles.test.mjs scripts/dev-demo-work.test.mjs scripts/dev-demo-state.test.mjs scripts/dev-demo-repository.test.mjs scripts/install-cli.test.mjs
```

**Notes**

13 focused script tests before two further migration/folder regressions.
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
node --test scripts/dev-demo-folder.test.mjs scripts/dev-demo-work.test.mjs scripts/install-cli.test.mjs
```

**Notes**

9 focused tests, including interrupted migration, edited-task preservation, native-folder binding and installer target symlinks.
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

Zero errors or warnings.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:check
```

**Notes**

New story supplied unsupported Testing Library getByRole exact option; remove redundant option.
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
pnpm catalog:check
```

**Notes**

Zero errors or warnings; 114 catalog modules.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/NavigationPersistence.stories.svelte
```

**Notes**

New story used section tooltip as accessible name; use visible section name.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/NavigationPersistence.stories.svelte
```

**Notes**

Disclosure choices restore across fresh app mounts.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte catalog/components/store-work/ProjectsHub.stories.svelte catalog/components/store-work/ProjectsDirectory.stories.svelte catalog/components/store-work/ProjectsFilters.stories.svelte catalog/components/store-work/ProjectsActivity.stories.svelte catalog/components/store-work/ProjectsActivityControls.stories.svelte catalog/components/store-work/NavigationPersistence.stories.svelte
```

**Notes**

38 complete directly affected stories across 8 files, interaction and accessibility; cold Vitest process.
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
pnpm check:architecture
```

**Notes**

dev-demo entry point exceeded 300 counted lines; extract unchanged owned seed-work command guard.
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
pnpm check:architecture
```

**Notes**

456 modules pass after extraction.
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
pnpm check:boundaries
```

**Notes**

15 package boundaries.
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
pnpm spec:validate
```

**Notes**

4 validators, 21 chapters, 132 requirements.
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
pnpm cli:install --debug --no-build --bin-dir /tmp/jj-store-cli-install-review
```

**Notes**

Real built binaries installed into isolated prefix. No global installation.
<!-- /run -->
<!-- /evidence-field -->

