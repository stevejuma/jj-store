---
schema: 1
id: E-M11-003
---

# E-M11-003 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 10
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check
```

**Notes**

Before implementation completed: ten diagnostics from naming a derived value state alongside $state; renamed to view.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check
```

**Notes**

Two accidental task.view replacements; restored task.state.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:architecture
```

**Notes**

Two oversized production modules; extracted existing controller/lifecycle/view helpers without changing baselines.
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
pnpm catalog:check
```

**Notes**

Two invalid Testing Library exact role options; removed.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check
```

**Notes**

Three toReversed diagnostics against the configured TypeScript lib; replaced with slice().reverse().
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
pnpm web:test
```

**Notes**

57 directly affected route, transport and model tests pass. Shared production route/controller contracts justify the complete web unit lane.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-work.test.ts
```

**Notes**

Four tests pass: retained unknown intent, late-read discard, explicit admission rejection and loaded hierarchy fragments.
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

Final production type check: zero errors and warnings.
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

Final catalog type check: zero errors and warnings; 74-module catalog boundary audit passes.
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
pnpm catalog:test
```

**Notes**

80 tests in 18 files pass. Escalation: shared production controller and route/lifecycle extraction affect existing code, review and agent consumers.
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
pnpm check:architecture
```

**Notes**

296 production modules pass the unchanged counted-line and dependency boundaries.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/StoreWork.stories.svelte catalog/components/store-shell/StoreShell.stories.svelte
```

**Notes**

Three Work stories pass; the supplied Shell path matched no file, so this run supplies Work evidence only.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/StoreWork.stories.svelte catalog/components/store-application/StoreApplication.stories.svelte
```

**Notes**

14 Work/Shell interactions pass with accessibility addon set to error. These are direct consumers of the new Projects navigation.
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
node --test scripts/dev-demo-collaboration.test.mjs scripts/dev-demo-repository.test.mjs scripts/dev-demo-state.test.mjs scripts/governance.test.mjs scripts/web-architecture.test.mjs
```

**Notes**

34 governance, architecture and development fixture/state source tests pass. Shared startup harness changes justify all directly consuming development helper tests.
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
cargo clippy -p jj-store-cli --test work_rustfs --locked -- -D warnings
```

**Notes**

Exact new real-storage acceptance target compiles with warnings denied.
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
cargo fmt -p jj-store-cli -- --check
```

**Notes**

Affected Rust package formatting passes.
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
JJ_STORE_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs cargo test -p jj-store-cli --test work_rustfs --locked -- --ignored --exact workstore_real_rustfs_restart_checkpoint_copy_and_historical_recovery
```

**Notes**

One real RustFS acceptance passes in 5.6s: isolated source/destination runtimes, exact manifest, historical body, tombstone/target, checkpoint/copy, both cold recoveries, frozen origin ledger, local diagnostic readiness and original durable receipt.
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
JJ_STORE_DEV_PORT=7477 JJ_STORE_DEV_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs JJ_STORE_REAL_TEST=work-store.spec.ts JJ_STORE_REAL_KEEP=1 pnpm web:test:real
```

**Notes**

Runner rejected before Playwright: 1037ms first startup, 30006ms server/search restart, 332986ms separate fixture preparation, 136160KiB RSS. Strict 30000ms and 524288KiB gates retained. Full representative agent/review fixture used.
<!-- /run -->
<!-- /evidence-field -->

