---
schema: 1
id: E-MAIN-SYNC-STORIES-001
---

# E-MAIN-SYNC-STORIES-001 details 4

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
pnpm web:test
```

**Notes**

web-tests-67: exit 0, 3.45 seconds. 
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
JJ_STORE_WORKING_OBSERVATION_CAPTURE=$PWD/.scratch/qualification-working-tree.json pnpm catalog:test
```

**Notes**

catalog-unit-67: exit 0, 18.56 seconds. 
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
JJ_STORE_QUALIFICATION_REPORT=./storybook-shard-1-67.jsonl pnpm exec vitest run --config catalog/vitest.config.ts --project storybook --shard=1/4 --reporter=default --reporter=./.local/main-qualification-2026-10-16/live-reporter.mjs
```

**Notes**

storybook-shard-1-67: exit 0, 227.6 seconds. 
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
JJ_STORE_QUALIFICATION_REPORT=./storybook-shard-2-67.jsonl pnpm exec vitest run --config catalog/vitest.config.ts --project storybook --shard=2/4 --reporter=default --reporter=./.local/main-qualification-2026-10-16/live-reporter.mjs
```

**Notes**

storybook-shard-2-67: exit 0, 255.4 seconds. 
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
JJ_STORE_QUALIFICATION_REPORT=./storybook-shard-3-67.jsonl pnpm exec vitest run --config catalog/vitest.config.ts --project storybook --shard=3/4 --reporter=default --reporter=./.local/main-qualification-2026-10-16/live-reporter.mjs
```

**Notes**

storybook-shard-3-67: exit 0, 243.69 seconds. 
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
JJ_STORE_QUALIFICATION_REPORT=./storybook-shard-4-67.jsonl pnpm exec vitest run --config catalog/vitest.config.ts --project storybook --shard=4/4 --reporter=default --reporter=./.local/main-qualification-2026-10-16/live-reporter.mjs
```

**Notes**

storybook-shard-4-67: exit 1, 258.67 seconds. 
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

catalog-check-67: exit 0, 20.36 seconds. 
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
JJ_STORE_QUALIFICATION_REPORT=./storybook-shard-4-67.jsonl pnpm exec vitest run --config catalog/vitest.config.ts --project storybook --shard=4/4 --reporter=default --reporter=./.local/main-qualification-2026-10-16/live-reporter.mjs
```

**Notes**

storybook-shard-4-67: exit 0, 281.92 seconds. 
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

catalog-check-67: exit 0, 19.29 seconds. 
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
JJ_STORE_DEV_RUSTFS_RUNTIME=$PWD/.local/main-qualification-2026-10-16/desktop-resources/jj-store-rustfs JJ_STORE_DEV_PORT=17480 JJ_STORE_DEV_BACKEND_PORT=17482 pnpm test:dev:ui
```

**Notes**

hmr-picker-83: exit 0, 14.11 seconds. 
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
JJ_STORE_RUSTFS_RUNTIME=$PWD/.local/main-qualification-2026-10-16/desktop-resources/jj-store-rustfs pnpm dev:cargo -- test --locked -p jj-store --test server_executable -- --ignored --exact git_bridge_acceptance::real_rustfs_native_git_external_merge_cycle_survives_cold_restart
```

**Notes**

git-cycle-picker-83: exit 0, 63.89 seconds. 
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

web-tests-67: exit 0, 1.93 seconds. 
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
JJ_STORE_WORKING_OBSERVATION_CAPTURE=$PWD/.scratch/qualification-working-tree.json pnpm catalog:test
```

**Notes**

catalog-unit-67: exit 0, 18.02 seconds. 
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
JJ_STORE_QUALIFICATION_REPORT=./storybook-shard-1-67.jsonl pnpm exec vitest run --config catalog/vitest.config.ts --project storybook --shard=1/4 --reporter=default --reporter=./.local/main-qualification-2026-10-16/live-reporter.mjs
```

**Notes**

storybook-shard-1-67: exit 0, 307.47 seconds. 
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
jj git fetch --remote origin --branch main; jj new @- main@origin; jj commit; jj bookmark set codex/main-local-qualification -r @-; jj git push --remote origin --bookmark codex/main-local-qualification
```

**Notes**

pr-main-sync-86: exit 0. User requested PR synchronization. Merged current remote main aad2ad7f into review branch, signed merge 20a3495188c0. Tree equals verified repair head 94259ebe: zero changed files. GitHub compare confirms behind_by=0; no main push or PR merge.
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
JJ_STORE_QUALIFICATION_REPORT=./storybook-shard-2-67.jsonl pnpm exec vitest run --config catalog/vitest.config.ts --project storybook --shard=2/4 --reporter=default --reporter=./.local/main-qualification-2026-10-16/live-reporter.mjs
```

**Notes**

storybook-shard-2-67: exit 1, 464.73 seconds. 
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
JJ_STORE_QUALIFICATION_REPORT=./storybook-shard-3-67.jsonl pnpm exec vitest run --config catalog/vitest.config.ts --project storybook --shard=3/4 --reporter=default --reporter=./.local/main-qualification-2026-10-16/live-reporter.mjs
```

**Notes**

storybook-shard-3-67: exit 0, 345.92 seconds. 
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
GitHub Actions PR #10 Specification and release policy 116876024409
```

**Notes**

hosted-merge-evidence-86: exit 1. Post-sync merge requires fresh combined-parent evidence by existing direct-parent policy; no new entry in the empty merge caused failure. Existing regression explicitly requires this, so no checker/policy change. Fresh integration evidence will include the complete PR range, preserving immutable earlier records.
<!-- /run -->
<!-- /evidence-field -->

