---
schema: 1
id: E-M13-027
---

# E-M13-027 details 2

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
cargo check -p jj-store-service --locked; cargo test -p jj-store-service --test conversion --locked
```

**Notes**

import-check1/import-tests1: typed destination imports and five history/graph tests pass.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
cargo check -p jj-store-service -p jj-store-server -p jj-store-http --locked (orchestration first)
```

**Notes**

orchestration-check1: corrected metadata commit type and u64 journal byte limit.
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
cargo check -p jj-store-service -p jj-store-server -p jj-store-http --locked (iterations two/three)
```

**Notes**

orchestration-check2/3: owning Rust boundaries compile.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-server --test repository_conversion --locked (first fixture)
```

**Notes**

orchestration-tests1: corrected CreateProject fields without changing assertions.
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
cargo test -p jj-store-server --test repository_conversion --locked (initial round trip)
```

**Notes**

orchestration-tests2: three pass; export registration reused generation one. Advance registry revision before assigning the destination binding generation.
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
cargo test -p jj-store-server --test repository_conversion --locked conversion_round_trip_preserves_source_and_publishes_new_colocated_folder -- --exact
```

**Notes**

orchestration-generation-regression: exact source-safe round trip passes.
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
cargo test -p jj-store-server --test repository_conversion --locked (iterations three/four)
```

**Notes**

orchestration-tests3/4: all five pass; added non-allocating status polling and explicit archived-source retry proof.
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
pnpm contracts:generate
```

**Notes**

conversion-contracts-generate: actual owning OpenAPI inventory and generated TS client refreshed.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec tsx --test web/tests/repository-conversion-manager.test.ts (first)
```

**Notes**

conversion-manager-unit1: code-spelling regex assertions did not match human Error messages. Assert typed ApiError.code.
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
pnpm exec tsx --test web/tests/repository-conversion-manager.test.ts (corrected)
```

**Notes**

conversion-manager-unit2: four exact intent, reload/retry, foreign receipt, stale/publication conflict and blocked preview cases pass.
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
pnpm web:check (initial)
```

**Notes**

conversion-web-check1: zero diagnostics.
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
pnpm web:check (presentation split)
```

**Notes**

conversion-web-check2: dialog operationHeads must accept readonly controller arrays.
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
pnpm web:check (corrected)
```

**Notes**

conversion-web-check3: zero diagnostics.
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
pnpm exec vitest run --config .local/repository-conversion/vitest.config.ts --project storybook catalog/components/store-application/StoreRepositoryConversion.stories.svelte
```

**Notes**

conversion-stories1: four interaction/accessibility stories pass; visual-pending retained.
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
pnpm catalog:check; pnpm check:architecture; pnpm check:ui-ownership
```

**Notes**

conversion-catalog-check1/architecture1/ui-ownership1: zero type diagnostics, 183 catalog modules, 928 production modules, ownership validator passes.
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
pnpm web:build; node scripts/browser-fixtures.mjs; pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/repository-conversion.spec.ts web/tests/browser/native-repository-creation.spec.ts
```

**Notes**

conversion-web-build/browser-fixtures/browser1: build and actual native HTTP/SSE fixtures pass; browser host startup hits existing 30s deadline while waiting for Cargo lock. No tests ran; prebuild host before cold launch.
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
cargo build -p jj-store-http --example browser_preview --locked; pnpm exec playwright test --config .local/repository-conversion/playwright.config.ts web/tests/browser/repository-conversion.spec.ts web/tests/browser/native-repository-creation.spec.ts
```

**Notes**

conversion-browser2: creation three and blocked conversion pass. Reactive preflight proxy could not be structuredClone-d before request persistence.
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
pnpm exec tsx --test web/tests/repository-conversion-manager.test.ts (proxy counterexample)
```

**Notes**

conversion-manager-proxy-before: original reload/retry case now uses a proxy and fails DataCloneError before dispatch.
<!-- /run -->
<!-- /evidence-field -->

