---
schema: 1
id: E-COMPUTE-004
---

# E-COMPUTE-004 details 3

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
pnpm docs:review application "Reviewed searchable Compute size/RAM-fit metadata and exact draft identities; Files default/divider and repository agent icon actions; API schema unchanged"; pnpm docs:check
```

**Notes**

Final manual and application-source review/complete bundled documentation checks.
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
pnpm exec prettier --write <explicit changed authored paths>; pnpm exec prettier --check <all changed authored paths>; pnpm spec:validate; pnpm check:progress
```

**Notes**

Changed authored source/canonical prose formatting and applicable document/progress lanes; generated evidence/task-history pages excluded from authored formatting. Root aggregate, Rust/build, installed inference, static production bundle and approved visual baselines out of scope under UI-only targeted policy.
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
curl -sS -o /dev/null -w "%{http_code}\n" http://127.0.0.1:7480/; lsof -nP -iTCP:7480 -iTCP:7482 -sTCP:LISTEN
```

**Notes**

Preview HTTP 200. Owned Vite PID 9640 and backend PID 97934 unchanged. HMR preview and local-compute workspace retained; no automatic browser refresh attempted.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --check <changed authored paths>
```

**Notes**

Two new documentation files needed formatting: generated coverage review and documentation 08. Applied targeted formatting; final check follows.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm docs:check
```

**Notes**

After valid source review, docs diagrams rendered but uv was absent from login-shell PATH. Existing /Users/stevejuma/.local/bin/uv found; no installation performed.
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
env PATH="/Users/stevejuma/.local/bin:$PATH" pnpm docs:check
```

**Notes**

Existing uv completes strict MkDocs build and documentation closure: 141 files, 17,742,333 bytes. Repeated after final Projects source review below.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 8
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:validate; pnpm check:progress
```

**Notes**

Spec validation passed. Progress rejected a pass record with remaining entries for out-of-scope gates, and seven new ledger/history paths lacked the initial evidence file coverage. Selected slice now has no unfinished UI work; independent gaps remain explicit in scope/runs/next. All current changed files included after ledger regeneration.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/ProjectsAgentActions.stories.svelte
```

**Notes**

Initial 2 passed / 1 failed due to test assuming an obsolete task-header class; assertion uses the actual task header.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/ProjectsAgentActions.stories.svelte catalog/components/store-work/ProjectsHub.stories.svelte catalog/components/store-work/ProjectsAgents.stories.svelte catalog/components/store-application/ManagedAgentIdentity.stories.svelte
```

**Notes**

21 passed / 1 failed. Task test found a transient page header before task data loaded; now waits for Task details. Projects explicitly forwards its controller manager to the task launcher instead of relying on snippet context inheritance. Existing exact task/repository context unchanged.
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
pnpm catalog:check
```

**Notes**

New optional execution prop was captured once in fixture controller literal. Changed to getter, retaining reactive prop ownership; final check has 0 warnings.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/ProjectsAgentActions.stories.svelte
```

**Notes**

3 passed: all six hub tabs have one icon-only New action with tooltips; Agents adds one title-row Manage action and no empty duplicate body strip; project and task launches retain context.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-work/ProjectsAgentActions.stories.svelte catalog/components/store-work/ProjectsHub.stories.svelte catalog/components/store-work/ProjectsAgents.stories.svelte catalog/components/store-application/ManagedAgentIdentity.stories.svelte
```

**Notes**

Final four direct Projects/header consumers: 22 passed, fresh Chromium run with configured Axe checks. All six tab headers have one icon-only New action; Agents places Manage beside it; selected project/task launch context and original recorded-agent navigation retained. Four managed-identity cases overlap the earlier 53-case lane.
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
pnpm docs:review application "Reviewed Compute command menu, Files layout and consistent icon agent actions across repository and Projects/task headers with retained context; API schema unchanged"; env PATH="/Users/stevejuma/.local/bin:$PATH" pnpm docs:check
```

**Notes**

Final application source/manual review with existing uv; no tool installation or backend replacement.
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
pnpm progress:history; pnpm check:progress; pnpm spec:validate; pnpm spec:first --base <explicit parent Git commit> --head <explicit working-copy Git commit>
```

**Notes**

Owned M4/M15.5/M24/M25 history refreshed and prior evidence navigation-only changes restored. Exact protected-change and progress qualification; no old evidence rewritten.
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
pnpm exec prettier --check <changed authored paths>
```

**Notes**

progress:history normalizes ledger tables after formatting. Reapplied Prettier to four owned milestone tables after the final history refresh, then checked all authored changes.
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
pnpm spec:first --base f5fc569613b677440fbf68e07cf964c4d6d2b730 --head 024991809d9e783f2892cd0a5402213677011b47
```

**Notes**

Concurrent formatting created/deleted a temporary root _tmp_ file during the JJ snapshot. Gate could not read a trustworthy working copy. Retried sequentially after all writes completed; no rule or source was bypassed.
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
pnpm spec:first --base f5fc569613b677440fbf68e07cf964c4d6d2b730 --head 8cf56f8bdb0eea330c21b69df98b28202ff8934e
```

**Notes**

Sequential settled snapshot: 33 protected files and 8 canonical chapters pass. Exact Git commit IDs avoid JJ @- ambiguity; no temporary writer race.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

[]
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; JJ 0.45.1; Node 26.9.0; pnpm 10.32.1; Svelte 5.57.1; Vitest 4.1.11; Storybook 10.5.7; bits-ui 2.19.4; pinned Buzz ef2aa1ae38fadcc0bc22b8bf6ed96b35933146be and mesh-llm 9f192c9d821991ba43d79bb15fd2f2f12e597571.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Existing layout/v1 saved open/width format, ComputeStatus/ComputeSettings API schema, exact GGUF identity strings, ACP and repository authority unchanged. New locally owned UI catalog with decimal GB weight estimates; advisory budget is 70% of observed RAM capped by draft memory_gb. Unknown sizes/hardware never recommend. Public Design Core Command View/Popover/Tooltip/Badge primitives.
<!-- /evidence-field -->

