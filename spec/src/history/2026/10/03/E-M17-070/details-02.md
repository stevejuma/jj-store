---
schema: 1
id: E-M17-070
---

# E-M17-070 details 2

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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/capture-management.test.ts
```

**Notes**

Before implementation: missing capture-http module confirms no browser management implementation.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/capture-management.test.ts
```

**Notes**

Initial six focused regressions pass.
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
pnpm web:check
```

**Notes**

Initial and integrated web type checks pass.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreCaptureSettings.stories.svelte
```

**Notes**

Shared dependency symlinks resolve outside this JJ workspace; Vite could not import the Storybook setup module. No story assertions ran.
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
pnpm exec vitest run --config .scratch/capture-vitest.config.ts --project storybook catalog/components/store-application/StoreCaptureSettings.stories.svelte
```

**Notes**

Local untracked config extends the ordinary configuration with fs.allow for this workspace and the existing jj-store/node_modules path. Six interaction/axe stories pass, repeated after final presentation changes.
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
pnpm catalog:check
```

**Notes**

Catalog ownership rejected direct api/client model/error imports. Replaced with controller-owned capture types and the standalone error class; no transport capability enters fixtures.
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

Final catalog check: zero errors/warnings; 132 catalog modules meet ownership boundary.
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
pnpm check:architecture
```

**Notes**

Production composition initially exceeded its 300 counted-line budget by two lines. Extracted the installation composition helper without weakening the budget.
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

Final 732-module production architecture check passes.
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
pnpm web:build
```

**Notes**

Fresh production assets built twice, before each cold browser run. Existing large-chunk diagnostic retained.
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
pnpm exec playwright test --config .scratch/capture-playwright.config.ts local-capture-settings.spec.ts
```

**Notes**

Two cold built-UI tests pass, repeated after final changes (11.5s final). Untracked config extends normal Playwright settings and starts an isolated Vite preview at port 7458 with reuseExistingServer=false; no Rust build or live installation was changed. Native browse responses reused from existing captured fixtures; capture responses controlled by test routes.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/capture-management.test.ts web/tests/vite/production-capture-management.test.ts web/tests/vite/production-store-bootstrap.test.ts
```

**Notes**

Initial direct-consumer set: 12 tests. Final set: 13 tests after the completed-boundary regression, including three existing bootstrap tests.
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
pnpm check:ui-ownership
```

**Notes**

No Community/catalog runtime dependencies; released Design Core presentation components.
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
pnpm check:boundaries
```

**Notes**

17 package boundaries pass.
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

Five specification validators, 21 chapters, 160 requirements pass.
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
pnpm exec prettier --write web/src/lib/controller/capture* web/tests/vite/capture-management.test.ts catalog/fixtures/capture-management.ts web/src/lib/controller/{contracts,production-store-controller.svelte,production-store-lifecycle.svelte,production-store-state.svelte,production-store-view}.ts spec/src/{ui,api,collaboration}.md
```

**Notes**

Formatting of new controller boundary and affected canonical chapters.
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
pnpm exec prettier --write web/src/lib/ui/store-settings/StoreCapture*.svelte web/src/lib/ui/store-settings/StoreServiceSettings.svelte
```

**Notes**

Owned settings presentation formatting.
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
pnpm exec prettier --write web/src/lib/controller/capture* catalog/fixtures/CaptureSettingsFixture.svelte catalog/components/store-application/StoreCaptureSettings.stories.svelte
```

**Notes**

Fixture/story and error module formatting.
<!-- /run -->
<!-- /evidence-field -->

