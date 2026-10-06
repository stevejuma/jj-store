---
schema: 1
id: E-M13-019
---

# E-M13-019 details 3

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
pnpm test:contracts
```

**Notes**

6 tests and contract TypeScript pass after dependency-layout restoration.
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
pnpm check:architecture
```

**Notes**

Initial frontend helper limits fixed by extracting repository source/folder/profile-route helpers. Directory Add introduced one 301-line screen failure; moving its composition into the existing header-controls component restores the 300-line boundary.
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

Final check: zero errors and warnings.
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
pnpm web:build
```

**Notes**

Final production bundle built for cold tests; dependency CSS/chunk warnings remain.
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

471 modules pass after extracting directory Add composition; prior 301-line failure resolved.
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
pnpm check:boundaries
```

**Notes**

15 package boundaries pass.
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
cargo fmt --all -- --check
```

**Notes**

Rust formatting passes.
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
pnpm spec:first
```

**Notes**

77 protected files and 8 canonical chapters pass after retaining the CLI concurrent-startup contract.
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
pnpm exec prettier --write web/tests/real/local-browser.spec.ts web/tests/real/work-store.spec.ts spec/src/cli.md
```

**Notes**

Final edited regression files formatted.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target node scripts/local-browser-acceptance.mjs
```

**Notes**

Expanded cold run: 1 passed, 2 failed. One fixture operation included graph characters; use --no-graph and validate the full operation ID. Reconnect left pointer events locked; explicit lifecycle assertion added for diagnosis. Parallel Playwright output paths collided, now isolated. Log .scratch/local-browser-filesystem-final.log.
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
JJ_STORE_REAL_REPOSITORY=b2ead948cbebd2512901b2f6813ae3a7 JJ_STORE_REAL_RUNTIME=/tmp/jlb-xw33phgv JJ_STORE_REAL_ROOT=/Users/stevejuma/code/lapismd/jj-store-local-browser JJ_STORE_REAL_BASE_URL=http://127.0.0.1:7479/ JJ_STORE_DEV_ROOT=/tmp/jlb-xw33phgv JJ_STORE_DEV_PORT=7479 pnpm exec playwright test --config web/playwright.real.config.ts native-repository.spec.ts work-store.spec.ts --grep "external publication|browser mutation|native projects tasks"
```

**Notes**

2 passed: timestamped external publication and unique native mutation descriptions. WorkStore directory timestamp failed with busy from the bounded native graph reader. Reused runtime accumulated many disposable test repos. Log .scratch/local-browser-native-repeatable.log.
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
JJ_STORE_REAL_REPOSITORY=b2ead948cbebd2512901b2f6813ae3a7 JJ_STORE_REAL_RUNTIME=/tmp/jlb-xw33phgv JJ_STORE_REAL_ROOT=/Users/stevejuma/code/lapismd/jj-store-local-browser JJ_STORE_REAL_BASE_URL=http://127.0.0.1:7479/ JJ_STORE_DEV_ROOT=/tmp/jlb-xw33phgv JJ_STORE_DEV_PORT=7479 pnpm exec playwright test --config web/playwright.real.config.ts work-store.spec.ts --output .scratch/test-results-native-work
```

**Notes**

Exact WorkStore retry: 1 failed. Fresh trace shows directory graph 503 busy rather than missing date data. Timestamp assertion retained.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target JJ_STORE_DEV_PORT=7479 JJ_STORE_DEV_ROOT=/tmp/jlb-xw33phgv JJ_STORE_DEV_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs node scripts/dev-demo.mjs restart
```

**Notes**

Cold restart preserves the original three project identities/links; read-cursor state restarts.
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
JJ_STORE_REAL_REPOSITORY=b2ead948cbebd2512901b2f6813ae3a7 JJ_STORE_REAL_RUNTIME=/tmp/jlb-xw33phgv JJ_STORE_REAL_ROOT=/Users/stevejuma/code/lapismd/jj-store-local-browser JJ_STORE_REAL_BASE_URL=http://127.0.0.1:7479/ JJ_STORE_DEV_ROOT=/tmp/jlb-xw33phgv JJ_STORE_DEV_PORT=7479 pnpm exec playwright test --config web/playwright.real.config.ts work-store.spec.ts --output .scratch/test-results-native-work-cold
```

**Notes**

Cold retry also hit the finite native cursor budget during repeated directory/Activity refresh over accumulated disposable repos. A clean mixed fixture will qualify the exact native regression.
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
node .scratch/modal-probe.mjs
```

**Notes**

Direct same-path reconnect restores body pointer events; narrowed the failure to the moved-folder flow. Diagnostic uses isolated 7479 only.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target JJ_STORE_FILESYSTEM_PORT=7481 JJ_STORE_LOCAL_BROWSER_KEEP=1 node .scratch/reconnect-acceptance.mjs
```

**Notes**

Setup did not reach browser tests: concurrent default-workspace build replaced the shared development binary; it rejected --storage filesystem. Switched to a private APFS-cloned Cargo target directory.
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
JJ_STORE_DEV_PORT=7480 JJ_STORE_DEV_ROOT=$(cat .scratch/local-browser-clean-mixed-root) CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target JJ_STORE_DEV_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs node scripts/dev-demo.mjs start
```

**Notes**

Runner accepts restart/reset/stop/seed-work; corrected clean-fixture setup to restart. No runtime was created by this attempt.
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

First evidence update incorrectly expanded dollar replacement patterns inside preserved ledger strings. Restored from valid JJ operation 9f670ff56bfd and used a replacement callback; no historical evidence removed.
<!-- /run -->
<!-- /evidence-field -->

