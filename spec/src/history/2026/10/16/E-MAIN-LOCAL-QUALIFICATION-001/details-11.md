---
schema: 1
id: E-MAIN-LOCAL-QUALIFICATION-001
---

# E-MAIN-LOCAL-QUALIFICATION-001 details 11

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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/WorkflowHostSettings.stories.svelte
```

**Notes**

workflow-reload-32: exit 0, 18.68 seconds. 
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
pnpm spec:links
```

**Notes**

spec-links-29: exit 0. Book links and anchors valid: 2,909 HTML files and 109,884 local references.
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
pnpm build-storybook
```

**Notes**

storybook-build-28: exit 0, 76.78 seconds. 
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
node --test scripts/catalog-browser.test.mjs
```

**Notes**

catalog-cold-28: exit 0, 16.76 seconds. 
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
JJ_STORE_DOC_REFERENCE=$PWD/target/debug/examples/docs_reference pnpm web:build
```

**Notes**

web-build-28: exit 0, 28.69 seconds. 
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
pnpm release:check
```

**Notes**

release-check-32: exit 0. 53 release/CI tests passed; all 21 public Cargo packages have contained inputs and coordinated metadata.
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
pnpm exec prettier --check .changeset/main-local-qualification.md catalog/components/store-application/StoreSyncReconciliation.stories.svelte catalog/components/store-application/WorkflowHostSettings.stories.svelte
```

**Notes**

format-changed-32: exit 0. All selected files conform to Prettier.
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
CI=true JJ_STORE_CI_PREBUILT=1 pnpm exec playwright test --config web/playwright.docs.config.ts
```

**Notes**

help-browser-29: exit 0, 49.47 seconds. 
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
JJ_STORE_QUALIFICATION_REPORT=./storybook-shard-3-32.jsonl pnpm exec vitest run --config catalog/vitest.config.ts --project storybook --shard=3/4 --reporter=default --reporter=./.local/main-qualification-2026-10-16/live-reporter.mjs
```

**Notes**

storybook-shard-3-32: exit 0, 263.41 seconds. 
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
python3 .local/main-qualification-2026-10-16/check-storybook-union.py
```

**Notes**

storybook-coverage-33: exit 0. Complete 1,021-case inventory across 211 files: 1,021 passed; no missing, unexpected or duplicate cases. Reports: shard1-27, shard2-24, shard3-32, shard4-24. Earlier failed diagnostics remain recorded.
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
CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0 RUST_TEST_THREADS=1 pnpm dev:cargo -- test --workspace --locked --no-fail-fast
```

**Notes**

source-registry-hang-34: exit 1. Diagnostic: source_registry::killed_reader_releases_both_leases_and_keeps_durable_source_identity stalled over 2 minutes. Parent blocked reading child stdout while child held its lease awaiting stdin. Serial libtest prefixes the source-ID line; test expected an ID-only line. Terminate only the owned held probe to let no-fail-fast inventory continue. Original lease exclusion and durable identity assertions remain required.
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
pnpm build-storybook
```

**Notes**

storybook-build-36: exit 0, 70.47 seconds. 
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
node --test scripts/catalog-browser.test.mjs
```

**Notes**

catalog-cold-36: exit 0, 16.82 seconds. 
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
CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0 RUST_TEST_THREADS=1 pnpm dev:cargo -- test --workspace --locked --no-fail-fast
```

**Notes**

cursor-supervisor-before-37: exit 1. cursor_supervisor: 1 passed, 1 failed. Raw JSON substring count saw 2 occurrences of first backfill text: display discovery_text and actual AgentActivity::Message content. Changed deduplication assertion to count published message records, retaining one-session/empty-spool assertions and additionally requiring all three fixture messages exactly once across pause and restart.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm dev:cargo -- test --workspace --locked --no-fail-fast
```

**Notes**

workspace-tests-serial-12: exit 1, 4255.95 seconds. 
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
pnpm format:rust:check
```

**Notes**

rust-format-37: exit 0. Root and embedded Rust formatting pass after both private test repairs.
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
pnpm dev:cargo -- test --locked -p jj-store-capture --test source_registry killed_reader_releases_both_leases_and_keeps_durable_source_identity -- --exact --nocapture
```

**Notes**

source-registry-exact-34: exit 0, 237.76 seconds. 
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
pnpm dev:cargo -- test --locked -p jj-store-capture --test source_registry
```

**Notes**

source-registry-all-34: exit 0, 6.36 seconds. 
<!-- /run -->
<!-- /evidence-field -->

