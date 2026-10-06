---
schema: 1
id: E-M11-005
---

# E-M11-005 details 8

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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/ProjectsHub.stories.svelte catalog/components/store-work/StoreWork.stories.svelte
```

**Notes**

jsh-latest-stories11.log: Test Files  2 passed (2); Tests  14 passed (14)
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/ProjectsHub.stories.svelte catalog/components/store-work/StoreWork.stories.svelte catalog/components/store-application/StoreRepositoryNavigation.stories.svelte catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-application/StoreRepositoryPicker.stories.svelte catalog/components/store-application/StoreConnection.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte
```

**Notes**

jsh-latest-stories12.log: Test Files  7 passed (7); Tests  46 passed (46)
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/ProjectsHub.stories.svelte
```

**Notes**

jsh-latest-stories13.log: Test Files  1 passed (1); Tests  9 passed (9)
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
cargo clippy -p jj-store-http --all-targets --locked -- -D warnings
```

**Notes**

jsh-latest-clippy1.log: Focused command completed.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/projects-code-navigation.test.ts web/tests/vite/projects-hub.test.ts web/tests/vite/projects-directories.test.ts web/tests/vite/production-work.test.ts web/tests/vite/production-store-bootstrap.test.ts web/tests/vite/store-route-history.test.ts web/tests/vite/production-store-data-source.test.ts
```

**Notes**

jsh-latest-controllers1.log: Test Files  7 passed (7); Tests  42 passed (42)
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
cargo build -p jj-store-cli --bin jj --locked
```

**Notes**

jsh-latest-cli-rebuild1.log: Focused command completed.
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
cargo clean -p jj-store-cli
```

**Notes**

jsh-latest-cli-clean1.log: Removed 4131 files, 7.9GiB total
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/ProjectsHub.stories.svelte -t 'Landing tabs sidebar hierarchy reviews and icon empty state'
```

**Notes**

jsh-latest-review-before.log: Zero tests matched the human story label (9 skipped); this is not acceptance. Full owning story file rerun below.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/ProjectsHub.stories.svelte 
```

**Notes**

jsh-latest-review-before2.log: 8/9 pass; native review count stops 151px before row edge because imported Community styles reserve two absent fields. Use the native five-field layout.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check
```

**Notes**

lapis-community jsh-preview-source-check1.log: UI policy passes 16 patterns; unchanged controller budgets 4267-&gt;4271 and transitions 332-&gt;339 stop architecture.
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
pnpm check:svelte
```

**Notes**

lapis-community jsh-preview-source-svelte1.log: Unchanged controller line 978 exactOptionalPropertyTypes timeline-position error.
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
pnpm check:types
```

**Notes**

lapis-community jsh-preview-source-types1.log: Same parent controller type error; source bytes verified equal to parent.
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
pnpm spec:validate
```

**Notes**

lapis-community jsh-preview-source-spec1.log: 4 validators, 10 chapters, 129 requirements.
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
pnpm spec:validate
```

**Notes**

lapis-community jsh-preview-source-spec2.log: Final evidence row passes validation.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:check
```

**Notes**

lapis-community jsh-preview-source-speccheck1.log: Stops at unchanged controller architecture budgets.
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
pnpm test
```

**Notes**

lapis-community jsh-preview-source-unit1.log: 1287 pass, 1 architecture failure, 2 skip across 282 files.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: fail
failures: 53
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook
```

**Notes**

lapis-community jsh-preview-source-storybook-all1.log: 435 pass, 53 fail; browser connection closed after 90 of 182 files. No full-suite acceptance claim.
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
pnpm test:storybook src/components/community-application/CommunityApplication.stories.svelte -t InboxShellPaintsBeforeConversationWork
```

**Notes**

lapis-community jsh-preview-source-inbox1.log: Focused wider failure reproduced: restoring instead of connected; controller is unchanged parent state.
<!-- /run -->
<!-- /evidence-field -->

