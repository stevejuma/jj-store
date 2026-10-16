---
schema: 1
id: E-MAIN-PICKER-REPAIRS-001
---

# E-MAIN-PICKER-REPAIRS-001 details 2

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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreRepositoryPickerPending.stories.svelte
```

**Notes**

picker-pending-27: exit 0, 11.58 seconds. 
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
GitHub Actions PR #10 Formatting job 116864459173
```

**Notes**

hosted-format-79: exit 1. Rust formatting passed; Prettier rejected only spec/src/current/progress/M17/index.md and M28/index.md after shorter repair-evidence references changed wrapping. Exact hosted log retained; no product-source failure.
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
pnpm exec prettier --write spec/src/current/progress/M17/index.md spec/src/current/progress/M28/index.md
```

**Notes**

hosted-format-repair-80: exit 0. Formatting-only correction; full final formatting check remains required.
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
CI=true JJ_STORE_CI_PREBUILT=1 JJ_STORE_CI_REPORTS=1 JJ_STORE_DEV_RUSTFS_RUNTIME=$PWD/.local/main-qualification-2026-10-16/desktop-resources/jj-store-rustfs JJ_STORE_DEV_PORT=17460 JJ_STORE_DEV_BACKEND_PORT=17462 pnpm test:local-release
```

**Notes**

cold-real-browser-67: exit 1, 898.58 seconds. 
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-reference-reader.test.ts -t "recovers an interrupted workspace read"
```

**Notes**

picker-before-81: exit 1, 6.05 seconds. 
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
pnpm exec prettier --write web/src/lib/controller/production-branch-actions.ts web/tests/vite/code-selection-loading.test.ts web/tests/vite/production-reference-reader.test.ts spec/src/current/contracts/ui/07-application-startup.md
```

**Notes**

picker-format-81: exit 0, 0.51 seconds. 
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-reference-reader.test.ts -t "recovers an interrupted workspace read"
```

**Notes**

picker-after-81: exit 0, 3.5 seconds. 
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-reference-reader.test.ts web/tests/vite/code-selection-loading.test.ts web/tests/vite/production-store-workspaces.test.ts web/tests/vite/store-workspaces.test.ts web/tests/vite/projects-code-navigation.test.ts
```

**Notes**

picker-consumers-81: exit 1, 4.2 seconds. 
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
pnpm exec prettier --write web/src/lib/controller/production-branch-actions.ts web/tests/vite/code-selection-loading.test.ts web/tests/vite/production-reference-reader.test.ts spec/src/current/contracts/ui/07-application-startup.md web/tests/vite/store-workspaces.test.ts web/tests/real/native-repository.spec.ts
```

**Notes**

picker-format-81: exit 0, 0.51 seconds. 
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-reference-reader.test.ts -t "recovers an interrupted workspace read"
```

**Notes**

picker-after-81: exit 0, 3.5 seconds. 
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-reference-reader.test.ts web/tests/vite/code-selection-loading.test.ts web/tests/vite/production-store-workspaces.test.ts web/tests/vite/store-workspaces.test.ts web/tests/vite/projects-code-navigation.test.ts
```

**Notes**

picker-consumers-81: exit 1, 4.09 seconds. 
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
pnpm exec prettier --write web/src/lib/controller/production-branch-actions.ts web/tests/vite/code-selection-loading.test.ts web/tests/vite/production-reference-reader.test.ts spec/src/current/contracts/ui/07-application-startup.md web/tests/vite/store-workspaces.test.ts web/tests/real/native-repository.spec.ts
```

**Notes**

picker-format-81: exit 0, 0.48 seconds. 
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-reference-reader.test.ts -t "recovers an interrupted workspace read"
```

**Notes**

picker-after-81: exit 0, 3.34 seconds. 
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-reference-reader.test.ts web/tests/vite/code-selection-loading.test.ts web/tests/vite/production-store-workspaces.test.ts web/tests/vite/store-workspaces.test.ts web/tests/vite/projects-code-navigation.test.ts
```

**Notes**

picker-consumers-81: exit 0, 3.77 seconds. 
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
pnpm exec prettier --write web/tests/real/native-repository.spec.ts
```

**Notes**

preview-format-82: exit 0, 0.37 seconds. 
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
pnpm docs:review application "Reviewed branch selection recovery of interrupted operation-pinned reference reads. Current complete windows remain cached, obsolete results remain ignored, workspace loading recovers after accepted selections, and native command preview receipt readiness retains original identities and existing UI assertions."
```

**Notes**

docs-application-review-82: exit 0, 0.58 seconds. 
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
pnpm web:build
```

**Notes**

web-build-82: exit 0, 47.31 seconds. 
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
pnpm exec prettier --write docs/maintenance/coverage.json
```

**Notes**

coverage-format-83: exit 0, 1.08 seconds. 
<!-- /run -->
<!-- /evidence-field -->

