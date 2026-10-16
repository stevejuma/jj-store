---
schema: 1
id: E-MAIN-LOCAL-QUALIFICATION-001
---

# E-MAIN-LOCAL-QUALIFICATION-001 details 26

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
pnpm check:ui-ownership
```

**Notes**

ownership-picker-84: exit 0, 2.95 seconds. 
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
pnpm check:architecture
```

**Notes**

architecture-picker-84: exit 0, 0.35 seconds. 
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
node .local/main-qualification-2026-10-16/picker-evidence.mjs
```

**Notes**

picker-evidence-84: exit 0, 0.33 seconds. 
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
pnpm exec prettier --write spec/src/current/progress/M16/index.md spec/src/current/progress/M30/index.md
```

**Notes**

picker-evidence-format-84: exit 0, 0.38 seconds. 
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
pnpm spec:validate
```

**Notes**

picker-spec-84: exit 0, 2.22 seconds. 
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
pnpm check:progress
```

**Notes**

picker-progress-84: exit 0, 5.45 seconds. 
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
pnpm spec:first --base @- --head @
```

**Notes**

picker-spec-first-84: exit 0, 5.61 seconds. 
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
JJ_STORE_DEV_RUSTFS_RUNTIME=$PWD/.local/main-qualification-2026-10-16/desktop-resources/jj-store-rustfs JJ_STORE_DEV_PORT=17480 JJ_STORE_DEV_BACKEND_PORT=17482 pnpm test:dev:ui
```

**Notes**

hmr-picker-83: exit 0, 14.11 seconds. 
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
jj commit -m "Recover interrupted workspace reads during branch selection"; jj bookmark set codex/main-local-qualification -r @-; jj git push --remote origin --bookmark codex/main-local-qualification
```

**Notes**

picker-push-85: exit 0. Verified follow-up szznutyo, signed commit 94259ebe41b8, pushed to existing draft PR #10; no main push or merge.
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
gh pr edit 10 --repo stevejuma/jj-store --body-file .local/main-qualification-2026-10-16/pr-body.md
```

**Notes**

picker-pr-update-85: exit 0. PR now distinguishes completed exact repair acceptance from pending fresh broad qualification.
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

