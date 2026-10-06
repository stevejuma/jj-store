---
schema: 1
id: E-READ-087
---

# E-READ-087 details 3

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
pnpm contracts:check
```

**Notes**

contracts-final.log: registered captures were interrupted near the end by concurrent scratch implementation with a temporary compile error. This run is not passing contract evidence.
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
pnpm exec vitest run --config web/vitest.config.ts web/tests/vite/production-file-reader.test.ts
```

**Notes**

ui-busy-final.log: nonexistent config; corrected to catalog/vitest.config.ts.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project unit web/tests/vite/production-file-reader.test.ts
```

**Notes**

ui-busy-corrected.log: invalid project selection; corrected to catalog-unit.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-file-reader.test.ts
```

**Notes**

ui-frozen-final.log: eight tests pass, including Pending/Busy continuation, failure preservation, cancellation and ready-row retention.
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
pnpm web:check
```

**Notes**

web-frozen-final.log: zero errors and warnings.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreDirectoryNavigation.stories.svelte
```

**Notes**

cold-directory.log: one fresh browser/component story passes; retained-snapshot folder skeleton preserved.
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
pnpm check:architecture
```

**Notes**

architecture-frozen-final.log: 1000 production modules pass.
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
pnpm check:boundaries
```

**Notes**

boundaries-frozen-final.log: 17 package boundaries pass.
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
pnpm check:ui-ownership
```

**Notes**

ownership-frozen-final.log: UI source ownership passes.
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
pnpm spec:first
```

**Notes**

spec-first-frozen-final.log: 33 protected files with seven owning canonical chapters.
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
pnpm spec:validate
```

**Notes**

canonical-final.log: five validators, 23 chapters and 172 requirements before final scratch contract prose; final canonical lane remains required.
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
cargo fmt --all
```

**Notes**

merge-format.log: Rust formatting applied.
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
pnpm exec prettier --write (seven changed canonical chapters and four frontend files)
```

**Notes**

format-final.log: changed canonical and UI files formatted.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm dev:backend:build
```

**Notes**

Earlier linker output recorded disk exhaustion and truncated generated executables. Removed only regenerable artifacts owned by this slice and rebuilt; installed CLI/source/fixture preserved. backend-build-errors.log now contains the subsequent successful rebuild.
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
pnpm dev:backend:build
```

**Notes**

backend-build-runtime.log: recovered explicit development binaries; later native/local scratch tests rebuilt the same bins successfully. Existing linker unwind warning remains.
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
JJ_STORE_DEV_ROOT=/Users/stevejuma/code/lapismd/jj-store-codex-preview/.local/codex-preview JJ_STORE_DEV_SERVER=/Users/stevejuma/code/lapismd/jj-store-bounded-reads/target/debug/jj-store JJ_STORE_DEV_PORT=7470 JJ_STORE_DEV_BACKEND_PORT=7472 pnpm dev:backend:restart
```

**Notes**

preview-restart-final.log: old scoped File polling stalled graceful shutdown. Verified owned process/executable/config before stopping that preview backend; no other service changed.
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
JJ_STORE_DEV_ROOT=/Users/stevejuma/code/lapismd/jj-store-codex-preview/.local/codex-preview JJ_STORE_DEV_SERVER=/Users/stevejuma/code/lapismd/jj-store-bounded-reads/target/debug/jj-store JJ_STORE_DEV_PORT=7470 JJ_STORE_DEV_BACKEND_PORT=7472 pnpm dev:backend:restart
```

**Notes**

preview-restart-merge.log: scratch-enabled backend started against preserved fixture; Vite remains running.
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
cargo test -p jj-store-cli --test native_lifecycle --locked (full binary; concurrency flags not retained)
```

**Notes**

native-suite-frozen-final.log: 239 passed/0 failed after request-local scratch/allocation fixes; before the final private candidate/path admission and union filtering.
<!-- /run -->
<!-- /evidence-field -->

