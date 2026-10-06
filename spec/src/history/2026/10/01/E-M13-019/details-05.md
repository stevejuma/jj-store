---
schema: 1
id: E-M13-019
---

# E-M13-019 details 5

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --check <changed Markdown/TypeScript/Svelte/scripts>
```

**Notes**

Only acceptance runner and current ledger needed formatting. Corrected runner immediately; ledger formatted after final evidence update. .scratch/local-browser-format-resumed.log.
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
pnpm exec prettier --write scripts/local-browser-acceptance.mjs
```

**Notes**

Acceptance runner formatted; no behavior change. .scratch/local-browser-format-runner.log.
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
pnpm check:boundaries
```

**Notes**

Final 15 packages pass. .scratch/local-browser-boundaries-final.log.
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
pnpm check:architecture
```

**Notes**

Final 471 modules pass. .scratch/local-browser-architecture-final.log.
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
cargo fmt --all -- --check
```

**Notes**

Final Rust formatting passes. .scratch/local-browser-rust-format-final.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/local-browser.test.ts -t local_reads_retry_busy_without_advancing_pins_or_replaying_writes
```

**Notes**

Exact before regression: one failure. Read transport did not retry a transient busy response. .scratch/local-browser-busy-read-before.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/local-browser.test.ts -t local_reads_retry_busy_without_advancing_pins_or_replaying_writes
```

**Notes**

Exact after regression: one pass. Two bounded read-only retries preserve exact operation/metadata pins and never replay writes. .scratch/local-browser-busy-read-after.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/local-browser.test.ts -t local_refresh_does_not_reuse_an_older_in_flight_capture
```

**Notes**

Exact before regression: one failure. Explicit refresh shared the previous in-flight capture. .scratch/local-browser-capture-before.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/local-browser.test.ts -t local_refresh_does_not_reuse_an_older_in_flight_capture
```

**Notes**

Exact after regression: one pass. Each explicit refresh has a separate capture key; delayed older responses cannot supply its selection. .scratch/local-browser-capture-after.log.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store-local-browser/.scratch/local-browser-runtime JJ_STORE_FILESYSTEM_PORT=7481 JJ_STORE_LOCAL_BROWSER_KEEP=1 node scripts/local-browser-acceptance.mjs
```

**Notes**

Cold strict-CSP complete filesystem lane: 2 passed, 1 failed; transient busy code-reference query prevented Agents loading. Added bounded read retries; writes retain receipt/unknown handling. .scratch/local-browser-filesystem-csp-complete.log.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store-local-browser/.scratch/local-browser-runtime JJ_STORE_TEST_FIXTURE_ROOT=/var/folders/_3/k1jv9rtn47s_0k5bm6qn_lk80000gn/T/jj-store-filesystem-browser-BZfoHF JJ_STORE_FILESYSTEM_PORT=7481 JJ_STORE_LOCAL_BROWSER_KEEP=1 node .scratch/reuse-filesystem-acceptance.mjs
```

**Notes**

Retained code/agent fixture, fresh installation: 2 passed, 1 failed. Keyboard Add ran while registration initialization still disabled the button. Test now waits for enabled before focus/Enter; keyboard and dialog assertions retained. .scratch/local-browser-filesystem-retry-complete.log.
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
pnpm web:test
```

**Notes**

Final read fixes: 66 tests passed. .scratch/local-browser-web-tests-busy-final.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite
```

**Notes**

Final shared runtime/transport consumers: 137 tests in 21 files passed. .scratch/local-browser-vite-read-final.log.
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
pnpm web:check
```

**Notes**

Final sources: zero errors/warnings. .scratch/local-browser-web-check-read-final.log.
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
pnpm catalog:check
```

**Notes**

Catalog consumer type/boundary check: zero errors/warnings. .scratch/local-browser-catalog-check-busy.log.
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
pnpm web:build
```

**Notes**

Final production bundle built; existing dependency CSS/chunk warnings retained. .scratch/local-browser-web-build-read-final.log.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store-local-browser/.scratch/local-browser-runtime JJ_STORE_DEV_PORT=7480 JJ_STORE_DEV_ROOT=/tmp/jlb-final-zTsELr JJ_STORE_DEV_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs node .scratch/dev-demo-prebuilt.mjs restart
```

**Notes**

Cold final mixed service restart with source-pinned binaries, final bundle and real embedded RustFS. Retained demo code and metadata; ignored runner omits only already verified builds. .scratch/local-browser-clean-mixed-qualified-restart.log.
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
JJ_STORE_REAL_REPOSITORY=f0622f8876c339fd655d620f47d26320 JJ_STORE_REAL_RUNTIME=/tmp/jlb-final-zTsELr JJ_STORE_REAL_ROOT=/Users/stevejuma/code/lapismd/jj-store-local-browser JJ_STORE_REAL_BASE_URL=http://127.0.0.1:7480/ JJ_STORE_DEV_ROOT=/tmp/jlb-final-zTsELr JJ_STORE_DEV_PORT=7480 CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store-local-browser/.scratch/local-browser-runtime pnpm exec playwright test --config web/playwright.real.config.ts local-mixed.spec.ts --output .scratch/test-results-mixed-qualified
```

**Notes**

Final read fixes: mixed Developer experience Git browsing, reload, persisted disclosure, Back/Forward and capability gating passed on real RustFS. Native WorkStore passed separately in the preceding focused two-test run, not as an aggregate native suite. .scratch/local-browser-mixed-qualified.log.
<!-- /run -->
<!-- /evidence-field -->

