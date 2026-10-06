---
schema: 1
id: E-M13-019
---

# E-M13-019 details 2

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
pnpm web:test
```

**Notes**

66 passed. Shared native route codec and API/graph consumers retained.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite
```

**Notes**

Initial 133 passed in 21 files; 135 passed after two new capture/history regressions. Native actor-capture assertion separately passes below.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-store-bootstrap.test.ts web/tests/vite/local-browser.test.ts
```

**Notes**

11 passed after restricting device-actor snapshot capture to the local profile.
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
pnpm web:check
```

**Notes**

Zero errors/warnings. Repeated after final UI changes.
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
pnpm catalog:check
```

**Notes**

Zero errors/warnings; catalog boundaries pass.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-http --lib --locked
```

**Notes**

13 HTTP contract tests pass.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-cli --test local_browser --locked -- filesystem_runtime_registers_and_reads_without_native_storage
```

**Notes**

1 passed after adding parallel snapshot-query/WorkStore startup regression. Local workers now wait within the existing request/deadline bound.
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
node --test scripts/dev-demo-repository.test.mjs
```

**Notes**

2 passed; symlink spelling is normalized before comparing the registered fixture binding.
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
pnpm web:build
```

**Notes**

Production build passed. Dependency CSS and chunk-size warnings remain; no native identity/schema format changes.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target node scripts/local-browser-acceptance.mjs
```

**Notes**

Initial cold filesystem-only run: 2 browser tests passed. Working README, bookmarks, operation log and companion ref compared before/after browsing; preserved. No RustFS or native RPC. Expanded 3-test run pending.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target JJ_STORE_DEV_PORT=7479 JJ_STORE_DEV_ROOT=/tmp/jlb-xw33phgv JJ_STORE_DEV_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs node scripts/dev-demo.mjs restart
```

**Notes**

Cold mixed startup and subsequent restarts pass with real embedded RustFS. First long macOS temp-root attempt exceeded SUN_LEN; short isolated /tmp root fixes the socket constraint. Restart initially rejected /tmp versus canonical /private/tmp binding; realpath normalization fixed it.
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
JJ_STORE_DEV_ROOT=/tmp/jlb-xw33phgv node scripts/dev-demo-profiles.acceptance.mjs
```

**Notes**

Pass: distinct native/local directory profiles, three project links/targets, 9 tasks, native references, server-only cleanup and companion identity. Exact three project records compared before/after restart; unchanged.
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
JJ_STORE_REAL_BASE_URL=http://127.0.0.1:7479/ JJ_STORE_REAL_RUNTIME=/tmp/jlb-xw33phgv CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target pnpm exec playwright test --config web/playwright.real.config.ts local-mixed.spec.ts
```

**Notes**

1 passed: Developer experience opens linked Git code, retains 40-character commits, navigation disclosure storage, reload and cross-profile Back/Forward.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreCollaboration.stories.svelte catalog/components/store-application/StoreWorkspaceSettings.stories.svelte
```

**Notes**

Initial 2 setup failures: shared node_modules symlinks escaped the Vite root. Parent-root, preserved-symlink and ignored /@fs diagnostic variations also failed. Replacing only this workspace dependency layout with unmodified owned copies resolved the gate; no dependency patches or tracked test configuration changes.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreCollaboration.stories.svelte catalog/components/store-application/StoreWorkspaceSettings.stories.svelte
```

**Notes**

14 interaction/accessibility tests pass in a cold Vitest process with workspace-owned dependency files.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreApplication.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte catalog/components/store-application/StoreConnection.stories.svelte catalog/components/store-application/StoreFileEditor.stories.svelte catalog/components/store-application/StoreEvolutionColumn.stories.svelte catalog/components/store-application/StoreWorkspaces.stories.svelte catalog/components/store-application/StoreRepositoryNavigation.stories.svelte
```

**Notes**

41 interaction/accessibility tests pass across 7 affected screen families.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/StoreWork.stories.svelte
```

**Notes**

5 project/task interaction tests pass.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target pnpm contracts:check
```

**Notes**

Generated contracts match; 115 actual HTTP/SSE responses conform. The final Node contract test process raced the dependency-layout replacement and failed its package import. Exact Node/TypeScript contract lane rerun successfully below.
<!-- /run -->
<!-- /evidence-field -->

