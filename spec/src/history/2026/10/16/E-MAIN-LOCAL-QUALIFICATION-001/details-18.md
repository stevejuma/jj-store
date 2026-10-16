---
schema: 1
id: E-MAIN-LOCAL-QUALIFICATION-001
---

# E-MAIN-LOCAL-QUALIFICATION-001 details 18

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
CI=true JJ_STORE_CI_PREBUILT=1 pnpm exec playwright test --config web/playwright.config.ts --max-failures=0
```

**Notes**

served-browser-29: exit 1, 268.22 seconds. 
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
pnpm exec prettier --write web/tests/browser/repository-settings-navigation.ts
```

**Notes**

settings-helper-format-59: exit 0, 0.51 seconds. 
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
CI=true JJ_STORE_CI_PREBUILT=1 pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/local-sync.spec.ts --grep "Sync reconciliation uses fresh heads and keeps the historical route after unknown recovery"
```

**Notes**

sync-owner-exact-59: exit 0, 6.64 seconds. 
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
CI=true JJ_STORE_CI_PREBUILT=1 pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/repository-conversion.spec.ts --grep "cold export preflight, pending reload, exact retry and verified report"
```

**Notes**

conversion-owner-exact-59: exit 0, 8.55 seconds. 
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
CI=true JJ_STORE_CI_PREBUILT=1 pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/local-search-settings.spec.ts web/tests/browser/local-server-settings.spec.ts web/tests/browser/local-storage.spec.ts web/tests/browser/local-sync.spec.ts web/tests/browser/local-workspaces.spec.ts web/tests/browser/notifications.spec.ts web/tests/browser/repository-conversion.spec.ts --max-failures=0
```

**Notes**

settings-helper-consumers-59: exit 1, 84.87 seconds. 
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
pnpm exec prettier --write web/tests/browser/repository-settings-navigation.ts
```

**Notes**

settings-helper-format-59: exit 0, 0.41 seconds. 
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
CI=true JJ_STORE_CI_PREBUILT=1 pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/local-sync.spec.ts --grep "Sync reconciliation uses fresh heads and keeps the historical route after unknown recovery"
```

**Notes**

sync-owner-exact-59: exit 1, 13.61 seconds. 
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
CI=true JJ_STORE_CI_PREBUILT=1 pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/repository-conversion.spec.ts --grep "cold export preflight, pending reload, exact retry and verified report"
```

**Notes**

conversion-owner-exact-59: exit 0, 7.88 seconds. 
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
CI=true JJ_STORE_CI_PREBUILT=1 pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/local-search-settings.spec.ts web/tests/browser/local-server-settings.spec.ts web/tests/browser/local-storage.spec.ts web/tests/browser/local-sync.spec.ts web/tests/browser/local-workspaces.spec.ts web/tests/browser/notifications.spec.ts web/tests/browser/repository-conversion.spec.ts --max-failures=0
```

**Notes**

settings-helper-consumers-59: exit 1, 73.68 seconds. 
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
pnpm exec prettier --write web/src/lib/ui/store-settings/repository-settings-guards.ts web/src/lib/ui/store-settings/repository-settings-guard.svelte.ts web/tests/vite/repository-settings-guards.test.ts web/tests/browser/repository-settings-navigation.ts spec/src/current/contracts/ui/10-local-management-delivery.md docs/site/guides/settings.md
```

**Notes**

guard-format-61: exit 0, 2.8 seconds. 
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/repository-settings-guards.test.ts
```

**Notes**

guard-policy-tests-61: exit 0, 6.39 seconds. 
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

web-check-61: exit 0, 23.48 seconds. 
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
pnpm catalog:check
```

**Notes**

catalog-check-61: exit 0, 32.85 seconds. 
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
pnpm docs:review application "Reviewed the narrowly allowed same-owner repository Settings category/subtab transitions during pending actions. Every other route field remains exact; repository, operation, workspace, file, page and global/hub changes keep their guards. Settings guide now states retention and no replay. Existing capture listbox naming, token colours and safe link rendering remain unchanged."
```

**Notes**

docs-application-review-61: exit 0, 1.31 seconds. 
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
JJ_STORE_DOC_REFERENCE=$PWD/target/debug/examples/docs_reference pnpm web:build
```

**Notes**

web-build-61: exit 0, 48.97 seconds. 
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
CI=true JJ_STORE_CI_PREBUILT=1 pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/local-sync.spec.ts --grep "Sync reconciliation uses fresh heads and keeps the historical route after unknown recovery"
```

**Notes**

sync-owner-exact-61: exit 0, 7.29 seconds. 
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
CI=true JJ_STORE_CI_PREBUILT=1 pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/local-search-settings.spec.ts web/tests/browser/local-server-settings.spec.ts web/tests/browser/local-storage.spec.ts web/tests/browser/local-sync.spec.ts web/tests/browser/local-workspaces.spec.ts web/tests/browser/notifications.spec.ts web/tests/browser/repository-conversion.spec.ts --max-failures=0
```

**Notes**

settings-helper-consumers-61: exit 1, 49.26 seconds. 
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
CI=true JJ_STORE_CI_PREBUILT=1 pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/local-workspaces.spec.ts
```

**Notes**

workspaces-exact-62: exit 0, 7.48 seconds. 
<!-- /run -->
<!-- /evidence-field -->

