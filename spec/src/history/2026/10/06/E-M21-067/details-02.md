---
schema: 1
id: E-M21-067
---

# E-M21-067 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: pass
details: Selection did not match generated test names; 12 skipped, not
  acceptance evidence.
failures: 0
passed: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStacksNavigation.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceConflicts.stories.svelte -t 'Workspace rail uses repository Details navigation|Unavailable resolution has no executable notification action'
```
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: pass
details: Identified generated CamelCase names. Saved .local/stacks-rail-tests.json.
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec vitest list --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStacksNavigation.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceConflicts.stories.svelte --json
```
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
details: "Before: no focused Details navigation and unavailable Resolve
  conflicts was still rendered. .local/stacks-rail-before.log."
failures: 2
passed: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStacksNavigation.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceConflicts.stories.svelte -t 'WorkspaceRailUsesRepositoryDetailsNavigation|UnavailableResolutionHasNoExecutableNotificationAction'
```
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: pass
details: "After: both original regressions pass; 10 unrelated stories skipped.
  .local/stacks-rail-after.log."
failures: 0
passed: 2
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStacksNavigation.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceConflicts.stories.svelte -t 'WorkspaceRailUsesRepositoryDetailsNavigation|UnavailableResolutionHasNoExecutableNotificationAction'
```
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
details: 21 directly affected interaction/accessibility stories. Recovery
  queried a detached Actions section removed during refresh; reconciliation
  fixture attempted preview before clearing its conflicts. Corrected the story
  queries/setup without weakening the production eligibility guards.
  .local/stacks-rail-stories.log.
failures: 2
passed: 19
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStacksNavigation.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceConflicts.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceSetup.stories.svelte catalog/components/store-application/StoreRepositoryNavigation.stories.svelte
```
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
details: Two isolated probes reproduced the fixture conflict-policy rejection,
  including one temporary diagnostic removed before delivery.
  .local/stacks-rail-pending.log and .local/stacks-rail-pending-debug.log.
failures: 1
passed: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceConflicts.stories.svelte -t 'PendingReconciliationHidesRailActionsUntilCancellation'
```
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: pass
details: Requery the live rail after refresh; recover fixture conflicts before
  preview. Pending preview hides all action controls; cancellation restores
  eligible actions. Combined with the other 19 unaffected passing stories, all
  21 targets pass. .local/stacks-rail-corrections.log.
failures: 0
passed: 2
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceConflicts.stories.svelte -t 'PendingReconciliationHidesRailActionsUntilCancellation|RailActionsAndNotificationOpenTheRetainedConflictEditor'
```
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: pass
details: 0 errors, 0 warnings. .local/stacks-rail-web-check.log.
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm web:check
```
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: pass
details: 0 errors, 0 warnings; 251 catalog modules pass boundaries.
  .local/stacks-rail-catalog-check.log.
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm catalog:check
```
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: pass
details: Repository-owned presentation verified; no reference-checkout
  dependency or new copied source.
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm check:ui-ownership
```
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: pass
details: Confirmed the already-built current executable supports Stacks before
  restart. The previous backend process had been running since October 4;
  current binary dated October 6.
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
target/debug/jj-store stack --help
```
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: pass
details: User requested investigation of failed live loading. Restarted only
  main backend to PID 74283, retained Vite PID 53716 on 7460 and fixture state.
  No build/reset/migration. .local/stacks-main-backend-restart.log.
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
pnpm dev:backend:restart
```
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: pass
details: Cold page reload after backend restart. Stacks, snapshot and query
  endpoints returned 200. The transient access error from the old session
  cleared on reload. Unconfigured local fixture correctly shows target/setup
  controls; compact focused navigation works on all four routes, with only
  Refresh workspace in Actions. No actual repository mutations or repair
  performed.
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
Native in-app browser: reload main 7460; inspect Network.responseReceived; select Branches, Commits, Op Log and Workspace through Details
```
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: pass
failures: 0
details: Regenerated task backlinks; formatting normalized unrelated generated
  whitespace back to the committed baseline.
hasNote: false
noteEncoding: markdown

```

```shell
pnpm progress:history
```
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 1
details: New evidence used generated Storybook names where the checker supports
  named Rust/Python/Node tests, omitted failures counters/generated-path
  coverage and put out-of-scope lanes in remaining gaps. Corrected evidence
  metadata; story coverage stays explicit in exact run commands.
hasNote: false
noteEncoding: markdown

```

```shell
pnpm check:progress
```
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: fail
failures: 1
details: Rejected the same incomplete evidence metadata; no product-source failure.
hasNote: false
noteEncoding: markdown

```

```shell
pnpm spec:first
```
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: pass
failures: 0
details: 7 validators; 3,014 chapters; 175 requirements.
hasNote: false
noteEncoding: markdown

```

```shell
pnpm spec:validate
```
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: pass
failures: 0
details: All changed authored code/stories/spec/evidence files are formatted.
hasNote: false
noteEncoding: markdown

```

```shell
pnpm exec prettier --check <all changed files from jj diff --name-only>
```
<!-- /run -->

<!-- run -->
### Run 19

```yaml
result: pass
failures: 0
details: "Corrected evidence validates: 143 tasks and 618 records.
  .local/stacks-rail-progress-check-final.log."
hasNote: false
noteEncoding: markdown

```

```shell
pnpm check:progress
```
<!-- /run -->
<!-- /evidence-field -->

