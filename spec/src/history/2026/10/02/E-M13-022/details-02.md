---
schema: 1
id: E-M13-022
---

# E-M13-022 details 2

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
pnpm install --offline --frozen-lockfile
```

**Notes**

pnpmfile checksum mismatch; no dependency setup acceptance.
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
pnpm install --offline --frozen-lockfile --ignore-pnpmfile
```

**Notes**

Registry dependency setup; exposes existing six missing Design Core API errors.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 8
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test --locked -p jj-store-service --test local_review local_review_browser_pages_and_detail_pin_committed_metadata -- --exact
```

**Notes**

Before: new regression cannot compile; local page/detail methods absent.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test --locked -p jj-store-service --test local_review
```

**Notes**

6 provider tests including historical pins, foreign selection, absent metadata, closure audits, receipt replay and unknown restart.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test --locked -p jj-store-service --test review_threads
```

**Notes**

All 4 direct native review reconstruction/pin/budget regressions retained.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test --locked -p jj-store-cli --test local_browser filesystem_runtime_registers_and_reads_without_native_storage -- --exact
```

**Notes**

Cold filesystem HTTP/CLI same actor/receipt/detail, committed historical pins, page-size/foreign cursor rejection, stale workflow and unchanged code heads. Captures new responses for contract checking.
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
pnpm install --offline --no-frozen-lockfile; jj restore --from @- -- pnpm-lock.yaml
```

**Notes**

Existing checked-in hook enables local Design Core validation; package dependency files restored. No Community dependency/substitution.
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
pnpm web:check
```

**Notes**

0 errors/warnings after local source qualification and own composer callback fix. Earlier registry pass had six pre-existing API mismatches and one own callback mismatch, all resolved in final lane.
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

0 errors/warnings; 129 catalog modules boundary checked.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/local-browser.test.ts web/tests/vite/production-store-collaboration.test.ts
```

**Notes**

28 direct local transport and production collaboration controller regressions.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreCollaboration.stories.svelte catalog/components/store-application/NativeReviewDetail.stories.svelte
```

**Notes**

10 direct interaction/accessibility cases, cold Vite process. Visual approval remains separate.
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
pnpm check:ui-ownership
```

**Notes**

Production ownership/provenance boundary passes.
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
pnpm check:architecture
```

**Notes**

One counted-line boundary after adding repository creation; moved anchor construction into owning review action module.
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
pnpm check:architecture
```

**Notes**

725 modules after factoring action ownership.
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
pnpm web:build
```

**Notes**

Cold production bundle/manifest; inherited CSS and chunk-size warnings retained.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo build --locked -p jj-store-cli --bins
```

**Notes**

Explicit development binaries copied into ignored workspace target/debug for isolated cold fixture execution.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test --locked -p jj-store-http --lib contract::tests
```

**Notes**

Route-count inventory drift: parent generated OpenAPI has85 paths/18 local, while test expected84/66 native. New four routes bring89/22 local/67 unchanged native; exact inventory corrected.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test --locked -p jj-store-http --lib contract::tests
```

**Notes**

All10 owning contract tests including corrected exact inventory.
<!-- /run -->
<!-- /evidence-field -->

