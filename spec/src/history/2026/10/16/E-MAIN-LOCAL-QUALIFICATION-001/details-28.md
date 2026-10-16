---
schema: 1
id: E-MAIN-LOCAL-QUALIFICATION-001
---

# E-MAIN-LOCAL-QUALIFICATION-001 details 28

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
pnpm check:progress
```

**Notes**

sync-progress-88: exit 0, 5.28 seconds. 
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
pnpm spec:first --base @- --head @
```

**Notes**

sync-spec-first-parent-88: exit 0, 5.57 seconds. 
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
pnpm spec:first --base aad2ad7f45f0bfdabfbb5617c77e00117a303d3d --head @
```

**Notes**

sync-spec-first-pr-88: exit 0, 6.11 seconds. 
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
python3 .local/main-qualification-2026-10-16/check-storybook-union.py
```

**Notes**

storybook-coverage-67: exit 0, 0.05 seconds. 
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
CI=true JJ_STORE_CI_PREBUILT=1 pnpm exec playwright test --config web/playwright.config.ts --max-failures=0
```

**Notes**

served-browser-63: exit 0, 313.34 seconds. 
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
CI=true JJ_STORE_CI_PREBUILT=1 pnpm exec playwright test --config web/playwright.docs.config.ts
```

**Notes**

help-browser-67: exit 0, 45.83 seconds. 
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
pnpm build-storybook
```

**Notes**

storybook-build-67: exit 0, 87.14 seconds. 
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
node --test scripts/catalog-browser.test.mjs
```

**Notes**

catalog-cold-67: exit 0, 16.89 seconds. 
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
CI=true JJ_STORE_CI_PREBUILT=1 JJ_STORE_CI_REPORTS=1 JJ_STORE_DEV_RUSTFS_RUNTIME=$PWD/.local/main-qualification-2026-10-16/desktop-resources/jj-store-rustfs JJ_STORE_DEV_PORT=17460 JJ_STORE_DEV_BACKEND_PORT=17462 pnpm test:local-release
```

**Notes**

cold-real-browser-67: exit 1, 946.58 seconds. 
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
CI=true JJ_STORE_CI_PREBUILT=1 JJ_STORE_CI_REPORTS=1 JJ_STORE_DEV_RUSTFS_RUNTIME=$PWD/.local/main-qualification-2026-10-16/desktop-resources/jj-store-rustfs JJ_STORE_REAL_TEST=profiles-emoji.spec.ts pnpm test:local-release
```

**Notes**

profile-restored-90: exit 0, 34.87 seconds. 
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
CI=true JJ_STORE_CI_PREBUILT=1 JJ_STORE_CI_REPORTS=1 JJ_STORE_DEV_RUSTFS_RUNTIME=$PWD/.local/main-qualification-2026-10-16/desktop-resources/jj-store-rustfs JJ_STORE_DEV_PORT=17460 JJ_STORE_DEV_BACKEND_PORT=17462 pnpm test:local-release
```

**Notes**

cold-real-browser-67: exit 0, 823.47 seconds. 
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
node .local/main-qualification-2026-10-16/evidence.mjs
```

**Notes**

final-evidence-generate-91: exit 0, 1.28 seconds. 
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
pnpm exec prettier --write spec/src/current/progress/M16/index.md spec/src/current/progress/M24/main-local-qualification.md spec/src/current/progress/M30/index.md spec/src/current/progress/M31/index.md spec/src/history/2026/10/16/E-MAIN-LOCAL-QUALIFICATION-001/*.md spec/src/history/2026/10/16/index.md spec/src/history/2026/10/16/SUMMARY.nav.md
```

**Notes**

final-evidence-format-91: exit 0, 0.47 seconds. 
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
JJ_STORE_DOC_REFERENCE=$PWD/target/debug/examples/docs_reference pnpm docs:check
```

**Notes**

docs-check-62: exit 0, 17.85 seconds. 
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
pnpm spec:validate
```

**Notes**

spec-validate-62: exit 0, 4.38 seconds. 
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
pnpm check:progress
```

**Notes**

progress-62: exit 0, 5.62 seconds. 
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
pnpm format:check
```

**Notes**

format-62: exit 0, 35.14 seconds. 
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
pnpm check:boundaries
```

**Notes**

boundaries-62: exit 0, 0.33 seconds. 
<!-- /run -->
<!-- /evidence-field -->

