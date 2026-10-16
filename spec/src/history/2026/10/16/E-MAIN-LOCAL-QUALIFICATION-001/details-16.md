---
schema: 1
id: E-MAIN-LOCAL-QUALIFICATION-001
---

# E-MAIN-LOCAL-QUALIFICATION-001 details 16

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
Validate and select separately staged qualified RustFS for cold browser and HMR acceptance
```

**Notes**

cold-runtime-resource-review-50: exit 0. Current-workspace embedded build copy, SHA-256 c3641db219705a7956dcfc4935ff2043f714f159b36dd315f7a234e1a3fac590. Uses supported JJ_STORE_DEV_RUSTFS_RUNTIME; no source or production launcher change, no live fixture or installed binary reused. Embedded target artifact cleanup preserved this separately qualified staged copy.
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
pnpm dev:cargo -- test --manifest-path runtime/local-agent/Cargo.toml --locked -p buzz-agent --lib -- --exact llm::tests::metadata_summary_never_retries_an_unknown_inference llm::chatgpt_plan::tests::pre_stream_401_renews_once_tool_rounds_replay_and_uncertain_calls_never_retry
```

**Notes**

local-agent-regressions-49: exit 0, 38.73 seconds. 
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
pnpm dev:cargo -- test --manifest-path runtime/local-agent/Cargo.toml --workspace --locked --no-fail-fast
```

**Notes**

local-agent-tests: exit 0, 62.63 seconds. 
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
pnpm dev:cargo -- clippy --manifest-path runtime/local-agent/Cargo.toml --workspace --all-targets --locked -- -D warnings
```

**Notes**

local-agent-clippy: exit 0, 12.64 seconds. 
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
pnpm dev:cargo -- test --manifest-path runtime/local-agent/Cargo.toml --locked -p buzz-agent --lib -- --exact llm::tests::metadata_summary_never_retries_an_unknown_inference llm::chatgpt_plan::tests::pre_stream_401_renews_once_tool_rounds_replay_and_uncertain_calls_never_retry
```

**Notes**

local-agent-regressions-49: exit 0, 7.3 seconds. 
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
pnpm dev:cargo -- test --manifest-path runtime/local-agent/Cargo.toml --workspace --locked --no-fail-fast
```

**Notes**

local-agent-tests: exit 0, 70.01 seconds. 
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
python3 .local/main-qualification-2026-10-16/check-rust-corpus.py
```

**Notes**

rust-corpus-coverage-51: exit 0, 1.74 seconds. 
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
python3 .local/main-qualification-2026-10-16/check-storybook-union.py
```

**Notes**

storybook-coverage-51: exit 0, 0.46 seconds. 
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
node --test scripts/catalog-browser.test.mjs
```

**Notes**

catalog-cold-44: exit 0, 33.12 seconds. 
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
node scripts/browser-fixtures.mjs
```

**Notes**

native-browser-fixtures-29: exit 0, 45.54 seconds. 
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
CI=true JJ_STORE_CI_PREBUILT=1 pnpm exec playwright test --config web/playwright.config.ts
```

**Notes**

served-browser-29: exit 1, 181.45 seconds. 
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
pnpm docs:test
```

**Notes**

docs-tests-29: exit 0, 35.25 seconds. 
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
CI=true JJ_STORE_CI_PREBUILT=1 pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/local-capture-loading.spec.ts --grep "cold capture loading keeps guarded controls and recovers from busy reads"
```

**Notes**

capture-browser-exact-52: exit 1, 6.35 seconds. 
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
CI=true JJ_STORE_CI_PREBUILT=1 pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/local-capture-loading.spec.ts
```

**Notes**

capture-browser-complete-52: exit 1, 5.35 seconds. 
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
CI=true JJ_STORE_CI_PREBUILT=1 pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/local-capture-loading.spec.ts --grep "cold capture loading keeps guarded controls and recovers from busy reads"
```

**Notes**

capture-browser-exact-52: exit 0, 6.11 seconds. 
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
CI=true JJ_STORE_CI_PREBUILT=1 pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/local-capture-loading.spec.ts
```

**Notes**

capture-browser-complete-52: exit 1, 12.58 seconds. 
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
pnpm exec prettier --write web/src/lib/ui/store-settings/StoreCaptureSourceFilters.svelte web/tests/browser/local-capture-loading.spec.ts
```

**Notes**

capture-format-53: exit 0, 0.48 seconds. 
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
pnpm web:check
```

**Notes**

web-check-53: exit 0, 12.78 seconds. 
<!-- /run -->
<!-- /evidence-field -->

