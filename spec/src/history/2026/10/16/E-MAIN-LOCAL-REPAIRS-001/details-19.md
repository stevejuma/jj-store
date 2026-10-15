---
schema: 1
id: E-MAIN-LOCAL-REPAIRS-001
---

# E-MAIN-LOCAL-REPAIRS-001 details 19

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
python3 .local/main-qualification-2026-10-16/check-rust-corpus.py
```

**Notes**

rust-corpus-coverage-66: exit 0, 0.4 seconds. 
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
pnpm docs:review application "Reviewed browser proxy-wrapped route acceptance: accepted selections change the previous stored route; rejected/no-op selections preserve it. Explicit bookmark, working-copy and revision selection retires superseded restoration while graph/detail retain busy phases; no late completion clears newer navigation. Existing documentation, exact routes, guards, drafts and pending request identities remain accurate."
```

**Notes**

docs-application-review-68: exit 0, 0.65 seconds. 
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
JJ_STORE_DOC_REFERENCE=$PWD/target/debug/examples/docs_reference pnpm web:build
```

**Notes**

web-build-68: exit 0, 24.03 seconds. 
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
CI=true JJ_STORE_CI_PREBUILT=1 JJ_STORE_CI_REPORTS=1 JJ_STORE_DEV_RUSTFS_RUNTIME=$PWD/.local/main-qualification-2026-10-16/desktop-resources/jj-store-rustfs JJ_STORE_DEV_PORT=17460 JJ_STORE_DEV_BACKEND_PORT=17462 JJ_STORE_REAL_TEST='native-repository.spec.ts|repository-settings.spec.ts|work-seed.spec.ts' pnpm test:local-release
```

**Notes**

cold-regressions-68: exit 1, 332.42 seconds. 
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
JJ_STORE_DEV_RUSTFS_RUNTIME=$PWD/.local/main-qualification-2026-10-16/desktop-resources/jj-store-rustfs JJ_STORE_DEV_PORT=17480 JJ_STORE_DEV_BACKEND_PORT=17482 pnpm test:dev:ui
```

**Notes**

hmr-after-68: exit 1, 39.91 seconds. 
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
python3 .local/main-qualification-2026-10-16/check-rust-corpus.py
```

**Notes**

rust-corpus-coverage-68: exit 0, 0.42 seconds. 
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/code-selection-loading.test.ts
```

**Notes**

context-before-70: exit 1, 5.43 seconds. 
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
pnpm exec prettier --write web/src/lib/controller/production-store-lifecycle.svelte.ts web/tests/vite/code-selection-loading.test.ts spec/src/current/contracts/ui/07-application-startup.md .changeset/main-local-qualification.md
```

**Notes**

context-format-70: exit 0, 0.79 seconds. 
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/code-selection-loading.test.ts
```

**Notes**

context-after-70: exit 0, 5.05 seconds. 
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/code-selection-loading.test.ts web/tests/vite/store-workspaces.test.ts web/tests/vite/production-reference-reader.test.ts web/tests/vite/projects-code-navigation.test.ts
```

**Notes**

selection-consumers-70: exit 0, 4.14 seconds. 
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
pnpm docs:review application "Reviewed accepted code-selection ownership and independent revision detail/context reads. Browser proxy-wrapped routes preserve rejected/no-op selections; valid selections retire superseded navigation while graph/detail retain busy phases. Review/agent associations no longer wait for file inspection; pinned operations, request ownership, guards, existing user flows and documentation remain unchanged."
```

**Notes**

docs-application-review-70: exit 0, 0.58 seconds. 
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
JJ_STORE_DOC_REFERENCE=$PWD/target/debug/examples/docs_reference pnpm web:build
```

**Notes**

web-build-70: exit 0, 23.58 seconds. 
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
CI=true JJ_STORE_CI_PREBUILT=1 JJ_STORE_CI_REPORTS=1 JJ_STORE_DEV_RUSTFS_RUNTIME=$PWD/.local/main-qualification-2026-10-16/desktop-resources/jj-store-rustfs JJ_STORE_DEV_PORT=17460 JJ_STORE_DEV_BACKEND_PORT=17462 JJ_STORE_REAL_TEST='native-repository.spec.ts|repository-settings.spec.ts|work-seed.spec.ts' pnpm test:local-release
```

**Notes**

cold-regressions-70: exit 1, 460.16 seconds. 
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
JJ_STORE_DEV_RUSTFS_RUNTIME=$PWD/.local/main-qualification-2026-10-16/desktop-resources/jj-store-rustfs JJ_STORE_DEV_PORT=17480 JJ_STORE_DEV_BACKEND_PORT=17482 pnpm test:dev:ui
```

**Notes**

hmr-after-70: exit 0, 16.36 seconds. 
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
python3 .local/main-qualification-2026-10-16/check-rust-corpus.py
```

**Notes**

rust-corpus-coverage-70: exit 0, 0.49 seconds. 
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
JJ_STORE_DEV_RUSTFS_RUNTIME=$PWD/.local/main-qualification-2026-10-16/desktop-resources/jj-store-rustfs JJ_STORE_DEV_PORT=17480 JJ_STORE_DEV_BACKEND_PORT=17482 pnpm test:dev:ui
```

**Notes**

hmr-confirm-71: exit 0, 12.67 seconds. 
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
JJ_STORE_DEV_RUSTFS_RUNTIME=$PWD/.local/main-qualification-2026-10-16/desktop-resources/jj-store-rustfs JJ_STORE_DEV_PORT=17480 JJ_STORE_DEV_BACKEND_PORT=17482 pnpm test:dev:ui
```

**Notes**

hmr-confirm-72: exit 0, 10.72 seconds. 
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
pnpm exec prettier --write web/tests/real/native-repository.spec.ts
```

**Notes**

mutation-receipt-format-73: exit 0, 0.41 seconds. 
<!-- /run -->
<!-- /evidence-field -->

