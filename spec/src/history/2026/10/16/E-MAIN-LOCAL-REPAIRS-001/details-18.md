---
schema: 1
id: E-MAIN-LOCAL-REPAIRS-001
---

# E-MAIN-LOCAL-REPAIRS-001 details 18

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
JJ_STORE_DEV_RUSTFS_RUNTIME=$PWD/.local/main-qualification-2026-10-16/desktop-resources/jj-store-rustfs JJ_STORE_DEV_PORT=17480 JJ_STORE_DEV_BACKEND_PORT=17482 pnpm test:dev:ui
```

**Notes**

cold-hmr-30: exit 1, 42.37 seconds. 
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/code-selection-loading.test.ts
```

**Notes**

selection-before-64: exit 1, 4.3 seconds. 
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
pnpm exec prettier --write web/src/lib/controller/production-branch-actions.ts web/src/lib/controller/production-store-lifecycle.svelte.ts web/tests/vite/code-selection-loading.test.ts web/tests/real/repository-settings.spec.ts web/tests/real/work-seed.spec.ts web/tests/browser/local-capture-settings.spec.ts spec/src/current/contracts/ui/07-application-startup.md
```

**Notes**

selection-format-64: exit 0, 0.54 seconds. 
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/code-selection-loading.test.ts
```

**Notes**

selection-after-64: exit 0, 3.87 seconds. 
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/code-selection-loading.test.ts web/tests/vite/store-workspaces.test.ts web/tests/vite/production-reference-reader.test.ts web/tests/vite/projects-code-navigation.test.ts
```

**Notes**

selection-consumers-64: exit 0, 3.89 seconds. 
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
pnpm web:check
```

**Notes**

web-check-64: exit 0, 12.26 seconds. 
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
JJ_STORE_DEV_RUSTFS_RUNTIME=$PWD/.local/main-qualification-2026-10-16/desktop-resources/jj-store-rustfs JJ_STORE_DEV_PORT=17480 JJ_STORE_DEV_BACKEND_PORT=17482 node .local/main-qualification-2026-10-16/hmr-diagnostic64.mjs
```

**Notes**

hmr-diagnostic-64: exit 1, 39.87 seconds. 
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
JJ_STORE_DEV_RUSTFS_RUNTIME=$PWD/.local/main-qualification-2026-10-16/desktop-resources/jj-store-rustfs JJ_STORE_DEV_PORT=17480 JJ_STORE_DEV_BACKEND_PORT=17482 node .local/main-qualification-2026-10-16/hmr-diagnostic65.mjs
```

**Notes**

hmr-diagnostic-65: exit 0, 9.97 seconds. 
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
pnpm exec prettier --write scripts/dev-ui.acceptance.mjs
```

**Notes**

hmr-format-65: exit 0, 0.39 seconds. 
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
node --test scripts/dev-ui.test.mjs scripts/dev-agent-runtime.test.mjs
```

**Notes**

hmr-unit-65: exit 0, 0.66 seconds. 
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
JJ_STORE_DEV_RUSTFS_RUNTIME=$PWD/.local/main-qualification-2026-10-16/desktop-resources/jj-store-rustfs JJ_STORE_DEV_PORT=17480 JJ_STORE_DEV_BACKEND_PORT=17482 pnpm test:dev:ui
```

**Notes**

hmr-after-65: exit 0, 10.07 seconds. 
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
pnpm docs:review application "Reviewed explicit code selection ownership: accepted bookmark, working-copy and revision selections retire superseded navigation loading while graph/detail busy states stay visible. Rejected selections preserve exact owner and start no reads; late selection completions cannot clear newer navigation. Settings same-owner retention and existing routes, drafts, authorization and pending commands remain unchanged."
```

**Notes**

docs-application-review-66: exit 0, 0.48 seconds. 
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
JJ_STORE_DOC_REFERENCE=$PWD/target/debug/examples/docs_reference pnpm web:build
```

**Notes**

web-build-66: exit 0, 23.47 seconds. 
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
CI=true JJ_STORE_CI_PREBUILT=1 pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/local-capture-settings.spec.ts --grep "capture notifications retain dismissal and drafts until observed recovery"
```

**Notes**

capture-a11y-exact-63: exit 0, 5.5 seconds. 
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
CI=true JJ_STORE_CI_PREBUILT=1 pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/local-capture-settings.spec.ts --max-failures=0
```

**Notes**

capture-settings-all-63: exit 0, 18.3 seconds. 
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
pnpm exec prettier --write web/src/lib/controller/production-branch-actions.ts web/src/lib/controller/production-store-lifecycle.svelte.ts web/tests/vite/code-selection-loading.test.ts
```

**Notes**

selection-proxy-format-68: exit 0, 0.46 seconds. 
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/code-selection-loading.test.ts
```

**Notes**

selection-proxy-tests-68: exit 0, 4.7 seconds. 
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
CI=true JJ_STORE_CI_PREBUILT=1 JJ_STORE_CI_REPORTS=1 JJ_STORE_DEV_RUSTFS_RUNTIME=$PWD/.local/main-qualification-2026-10-16/desktop-resources/jj-store-rustfs JJ_STORE_DEV_PORT=17460 JJ_STORE_DEV_BACKEND_PORT=17462 JJ_STORE_REAL_TEST='native-repository.spec.ts|repository-settings.spec.ts|work-seed.spec.ts' pnpm test:local-release
```

**Notes**

cold-regressions-66: exit 1, 354.69 seconds. 
<!-- /run -->
<!-- /evidence-field -->

