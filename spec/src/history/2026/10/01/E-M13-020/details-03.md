---
schema: 1
id: E-M13-020
---

# E-M13-020 details 3

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

7 Node contract tests and TypeScript pass. .scratch/local-browser-merge-node-contracts.log.
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
pnpm spec:validate
```

**Notes**

canonical specifications pass. .scratch/local-browser-merge-spec-validate.log.
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
pnpm web:build
```

**Notes**

final production bundle passes; inherited dependency CSS/chunk warnings retained. .scratch/local-browser-merge-bundle-final.log.
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
CARGO_BUILD_JOBS=1 CARGO_INCREMENTAL=0 cargo test -p jj-store-service --test agent_sessions --test local_agent_pages --locked
```

**Notes**

5 native AgentStore session tests and 2 local AgentStore page tests pass. .scratch/local-browser-merge-agent-readers.log.
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
CARGO_BUILD_JOBS=1 CARGO_INCREMENTAL=0 pnpm contracts:check
```

**Notes**

Merged contracts match: 137 actual HTTP/SSE responses conform, including native Thread/reaction and filesystem browser fixtures; 7 Node contract tests and TypeScript pass. .scratch/local-browser-merge-contract-check.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/local-browser-merge.test.ts -t local_profile_switch_preserves_unknown_native_thread_intent
```

**Notes**

Before with source-scoped assertion: unknown native intent was discarded on return from a local provider. Local pending visibility stays null; retaining another repository's intent does not expose it. .scratch/local-browser-merge-unknown-qualified-before.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/local-browser-merge.test.ts
```

**Notes**

After: all 4 integration regressions pass. Unknown native intent remains bound to original repository/principal and retries the exact request after reconnect. .scratch/local-browser-merge-unknown-qualified-after.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite
```

**Notes**

Final native/local runtime consumers: 155 tests, 23 files pass. .scratch/local-browser-merge-consumers-final.log.
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
CARGO_BUILD_JOBS=1 CARGO_INCREMENTAL=0 cargo rustc -p jj-store-cli --bin jj-store --locked -- -C strip=symbols
```

**Notes**

Merged source binary built explicitly for cold qualification. .scratch/local-browser-merge-server-build.log. Installed CLI unchanged.
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
CARGO_BUILD_JOBS=1 CARGO_INCREMENTAL=0 cargo rustc -p jj-store-cli --bin jj --locked -- -C strip=symbols
```

**Notes**

Merged source binary built explicitly for cold qualification. .scratch/local-browser-merge-jj-build.log. Installed CLI unchanged.
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
CARGO_BUILD_JOBS=1 CARGO_INCREMENTAL=0 cargo test -p jj-store-http --test assets --locked
```

**Notes**

All 6 strict-CSP/asset tests pass. .scratch/local-browser-merge-assets.log.
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
JJ_STORE_FILESYSTEM_PORT=7483 JJ_STORE_LOCAL_BROWSER_KEEP=1 node scripts/local-browser-acceptance.mjs
```

**Notes**

Fresh filesystem installation, merged binaries/bundle and strict CSP: all 3 browser tests passed (26.4s), including local history/Agents, UI/CLI collaboration recovery, folder reconnection/removal and keyboard/axe. No RustFS/native RPC; fixture retained and service stopped. .scratch/local-browser-merge-cold-filesystem.log.
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
JJ_STORE_DEV_PORT=7482 JJ_STORE_DEV_ROOT=/var/folders/_3/k1jv9rtn47s_0k5bm6qn_lk80000gn/T/jj-store-mixed-merge-ACWXwu JJ_STORE_DEV_BROWSER_ANNOTATIONS=0 JJ_STORE_DEV_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs node .scratch/local-browser-merge-prebuilt.mjs restart
```

**Notes**

Cold fresh mixed real-RustFS installation ready, all three fixture profiles admitted. Ignored prebuilt runner skips only separately verified build commands; no default deployment or data touched. .scratch/local-browser-merge-cold-mixed-start.log.
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
JJ_STORE_REAL_REPOSITORY=4fc5e9281061c1ec866a538cec36747d JJ_STORE_REAL_RUNTIME=/var/folders/_3/k1jv9rtn47s_0k5bm6qn_lk80000gn/T/jj-store-mixed-merge-ACWXwu JJ_STORE_REAL_ROOT=/Users/stevejuma/code/lapismd/jj-store JJ_STORE_REAL_BASE_URL=http://127.0.0.1:7482/ pnpm exec playwright test --config web/playwright.real.config.ts local-mixed.spec.ts --output .scratch/test-results-local-browser-merge-mixed
```

**Notes**

1 passed/1 failed: native message test reloaded before write acknowledgement. Trace shows message POST aborted (status -1). Test now requires a committed receipt before reload; persistence assertion retained. .scratch/local-browser-merge-cold-mixed.log.
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
JJ_STORE_DEV_PORT=7482 JJ_STORE_DEV_ROOT=/var/folders/_3/k1jv9rtn47s_0k5bm6qn_lk80000gn/T/jj-store-mixed-merge-ACWXwu JJ_STORE_DEV_BROWSER_ANNOTATIONS=0 JJ_STORE_DEV_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs node .scratch/local-browser-merge-prebuilt.mjs restart
```

**Notes**

Cold restart of owned mixed instance, fixture identities/data preserved. .scratch/local-browser-merge-mixed-restart-final.log.
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
JJ_STORE_REAL_REPOSITORY=4fc5e9281061c1ec866a538cec36747d JJ_STORE_REAL_RUNTIME=/var/folders/_3/k1jv9rtn47s_0k5bm6qn_lk80000gn/T/jj-store-mixed-merge-ACWXwu JJ_STORE_REAL_ROOT=/Users/stevejuma/code/lapismd/jj-store JJ_STORE_REAL_BASE_URL=http://127.0.0.1:7482/ pnpm exec playwright test --config web/playwright.real.config.ts local-mixed.spec.ts --output .scratch/test-results-local-browser-merge-mixed-final
```

**Notes**

1 passed/1 failed: after committed receipt, text assertion matched both the real message and a restored composer draft. Scope both persistence assertions to the accessible message log, excluding editor text. .scratch/local-browser-merge-cold-mixed-final.log.
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
JJ_STORE_REAL_REPOSITORY=4fc5e9281061c1ec866a538cec36747d JJ_STORE_REAL_RUNTIME=/var/folders/_3/k1jv9rtn47s_0k5bm6qn_lk80000gn/T/jj-store-mixed-merge-ACWXwu JJ_STORE_REAL_ROOT=/Users/stevejuma/code/lapismd/jj-store JJ_STORE_REAL_BASE_URL=http://127.0.0.1:7482/ pnpm exec playwright test --config web/playwright.real.config.ts local-mixed.spec.ts --output .scratch/test-results-local-browser-merge-mixed-qualified
```

**Notes**

Both mixed consumer tests pass: Developer experience Git code/navigation and native Thread create/send/committed receipt/reload persistence. Real embedded RustFS, final merged source and fresh browser process; same cold-restarted isolated server. .scratch/local-browser-merge-cold-mixed-qualified.log.
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
JJ_STORE_DEV_ROOT=/var/folders/_3/k1jv9rtn47s_0k5bm6qn_lk80000gn/T/jj-store-mixed-merge-ACWXwu node scripts/dev-demo-profiles.acceptance.mjs
```

**Notes**

Three sample projects, separate code links, native metadata home and nine tasks preserved. .scratch/local-browser-merge-profiles.log.
<!-- /run -->
<!-- /evidence-field -->

