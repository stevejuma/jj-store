---
schema: 1
id: E-READ-078
---

# E-READ-078 details 2

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
Python targeted controller/UI edit script
```

**Notes**

ValueError: substring not found; controller edits were written, UI edits were not. Replaced the failed UI edit with a targeted patch. Tooling failure, not a product regression.
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
jj restore -- pnpm-lock.yaml
```

**Notes**

Restored the original tracked registry lockfile after refreshing only the ignored local dependency installation. No committed dependency or sibling-source change.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/kind-preparation.test.ts
```

**Notes**

5.08s; 1 tests passed. .local/bounded-reads/performance/kind-browser-after-078.log
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
pnpm check:architecture
```

**Notes**

1.47s; 0 tests passed. .local/bounded-reads/performance/kind-browser-architecture-078.log
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
pnpm check:architecture
```

**Notes**

1.73s; 0 tests passed. .local/bounded-reads/performance/kind-browser-architecture-final-078.log
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/kind-preparation.test.ts
```

**Notes**

5.39s; 0 tests passed. .local/bounded-reads/performance/kind-browser-before-078.log
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
pnpm check:boundaries
```

**Notes**

1.87s; 0 tests passed. .local/bounded-reads/performance/kind-browser-boundaries-078.log
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
pnpm catalog:check
```

**Notes**

15.64s; 0 tests passed. .local/bounded-reads/performance/kind-browser-catalog-check-078.log
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
pnpm catalog:check
```

**Notes**

17.65s; 0 tests passed. .local/bounded-reads/performance/kind-browser-catalog-check-final-078.log
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
pnpm install --force --frozen-lockfile
```

**Notes**

1.18s; 0 tests passed. .local/bounded-reads/performance/kind-browser-dependency-refresh-078.log
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
pnpm install --force --no-frozen-lockfile --offline
```

**Notes**

20.59s; 0 tests passed. .local/bounded-reads/performance/kind-browser-dependency-refresh-offline-078.log
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-store-settings.test.ts web/tests/vite/index-freshness.test.ts web/tests/vite/local-browser.test.ts web/tests/vite/production-store-data-source.test.ts web/tests/vite/production-automatic-refresh.test.ts web/tests/vite/kind-preparation.test.ts web/tests/vite/kind-preparation-http.test.ts
```

**Notes**

7.94s; 72 tests passed. .local/bounded-reads/performance/kind-browser-direct-consumers-078.log
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/kind-preparation.test.ts web/tests/vite/kind-preparation-http.test.ts
```

**Notes**

4.19s; 17 tests passed. .local/bounded-reads/performance/kind-browser-focused-078.log
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
pnpm exec prettier --write catalog/components/store-application/StoreKindPreparation.stories.svelte catalog/fixtures/KindPreparationFixture.svelte docs/implementation-plan.md pnpm-lock.yaml spec/src/api.md spec/src/architecture.md spec/src/spec-governance.md spec/src/ui.md web/src/lib/controller/contracts.ts web/src/lib/controller/http-store-data-source.ts web/src/lib/controller/http-store-write-data-source.ts web/src/lib/controller/index-preparation-labels.ts web/src/lib/controller/native-data-source.ts web/src/lib/controller/production-index-preparation.ts web/src/lib/controller/production-store-actions.ts web/src/lib/controller/production-store-index.ts web/src/lib/controller/production-store-state.svelte.ts web/src/lib/controller/production-store-view.ts web/src/lib/ui/store-application/StoreRepositoryNotices.svelte web/src/lib/ui/store-settings/StoreGraphIndexSettings.svelte web/tests/vite/kind-preparation-http.test.ts web/tests/vite/kind-preparation.test.ts
```

**Notes**

2.32s; 0 tests passed. .local/bounded-reads/performance/kind-browser-format-078.log
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
pnpm exec prettier --check catalog/components/store-application/StoreKindPreparation.stories.svelte catalog/fixtures/KindPreparationFixture.svelte docs/implementation-plan.md spec/src/api.md spec/src/architecture.md spec/src/milestones.md spec/src/spec-governance.md spec/src/ui.md web/src/lib/controller/contracts.ts web/src/lib/controller/http-store-data-source.ts web/src/lib/controller/http-store-write-data-source.ts web/src/lib/controller/index-preparation-labels.ts web/src/lib/controller/native-data-source.ts web/src/lib/controller/production-index-preparation.ts web/src/lib/controller/production-store-actions.ts web/src/lib/controller/production-store-index.ts web/src/lib/controller/production-store-state.svelte.ts web/src/lib/controller/production-store-view.ts web/src/lib/controller/store-settings-types.ts web/src/lib/ui/store-application/StoreRepositoryNotices.svelte web/src/lib/ui/store-settings/StoreGraphIndexSettings.svelte web/tests/vite/kind-preparation-http.test.ts web/tests/vite/kind-preparation.test.ts
```

**Notes**

3.76s; 0 tests passed. .local/bounded-reads/performance/kind-browser-format-check-078.log
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
pnpm exec prettier --check spec/src/milestones.md
```

**Notes**

3.44s; 0 tests passed. .local/bounded-reads/performance/kind-browser-ledger-format-final-078.log
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
pnpm check:ui-ownership
```

**Notes**

2.58s; 0 tests passed. .local/bounded-reads/performance/kind-browser-ownership-078.log
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
pnpm check:progress
```

**Notes**

4.26s; 0 tests passed. .local/bounded-reads/performance/kind-browser-progress-078.log
<!-- /run -->
<!-- /evidence-field -->

