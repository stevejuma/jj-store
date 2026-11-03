---
schema: 1
id: E-CI-WORKFLOW-SIDEBAR-READINESS-001
---

# E-CI-WORKFLOW-SIDEBAR-READINESS-001

<!-- evidence-field:id markdown -->
## Id

E-CI-WORKFLOW-SIDEBAR-READINESS-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M30
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-GOV-003
- JST-GOV-007
- JST-UI-001
- JST-WF-008
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-11-03
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-workflows/WorkflowSidebarActions.stories.svelte
- spec/src/history/2026/11/03/E-CI-WORKFLOW-SIDEBAR-READINESS-001/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown
```

```shell
gh api --allow-escape-sequences repos/stevejuma/jj-store/actions/jobs/126001791199/logs
```

**Notes**

PR 52 source-Storybook shard 1/3 passed 92 of 93 interactions. The remaining
`WorkflowSidebarActions` case exhausted the default rendered lookup while
waiting for the `Workflows by repository` navigation in `openMenu`. The failure
preceded every workflow action assertion and was unrelated to the spacer story
originally changed by the PR.
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
npx --yes pnpm@10.32.1 test:storybook -- catalog/components/store-workflows/WorkflowSidebarActions.stories.svelte --reporter=default
```

**Notes**

All five focused Workflow sidebar interactions passed in 4.89 seconds after the
shared helper received the established ten-second rendered-story lookup bound.
The action, command, recovery and toast assertions are unchanged. Report:
`.local/validation/runs/jj-store-validation-d40f6b2b-1601-40ba-8026-79f8755558e0`.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown
```

```shell
npx --yes pnpm@10.32.1 catalog:check
npx --yes pnpm@10.32.1 exec prettier --check catalog/components/store-workflows/WorkflowSidebarActions.stories.svelte spec/src/history/2026/11/03/E-CI-WORKFLOW-SIDEBAR-READINESS-001/index.md spec/src/history/2026/11/03/index.md spec/src/history/2026/11/03/SUMMARY.nav.md
node node_modules/@lapismd/spec-validator/dist/cli.js validate --only governance,summary,fileLimits,verification --json
node node_modules/@lapismd/spec-validator/dist/cli.js first --base @- --head @ --json
node scripts/check-progress.mjs
```

**Notes**

Catalog type checking, ownership, story parity, formatting, document validation
and progress passed. Spec-first required one unchanged-contract confirmation to
cover both catalog paths in the combined slice; the primary qualification
record now names this Workflow story as well.
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
npx --yes pnpm@10.32.1 exec prettier --check catalog/components/store-workflows/WorkflowSidebarActions.stories.svelte spec/src/history/2026/11/03/E-CI-QUALIFICATION-READINESS-001/index.md spec/src/history/2026/11/03/E-CI-WORKFLOW-SIDEBAR-READINESS-001/index.md spec/src/history/2026/11/03/index.md spec/src/history/2026/11/03/SUMMARY.nav.md
node node_modules/@lapismd/spec-validator/dist/cli.js validate --only governance,summary,fileLimits,verification --json
node node_modules/@lapismd/spec-validator/dist/cli.js first --base @- --head @ --json
node scripts/check-progress.mjs
```

**Notes**

Formatting and four selected document validators passed. Spec-first accepted all
six protected paths through the qualified unchanged-contract confirmations, and
progress accepted 186 tasks and 1,026 evidence records.
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

Node.js 26.8.2; pnpm 10.32.1; Storybook 10.5.7; Vitest 4.1.11.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No production source, API, wire, generated asset, cache or persisted format
changed.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

The source-Storybook Workflow sidebar helper only. It waits up to ten seconds
for fixture navigation to render before running the existing interaction
assertions. Workflow commands, state, presentation and production code are
unchanged.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Require the updated PR source-Storybook gate, then continue the merged complete
qualification plan recorded by E-CI-QUALIFICATION-READINESS-001.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: JST-UI-001 and JST-WF-008 retain the same isolated catalog and Workflow
  interaction contracts. The helper only receives a bounded rendered-fixture
  wait; every protected action and recovery assertion remains unchanged.
```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M30 remains In progress. Focused Workflow Storybook acceptance passes,
  while the updated PR gate and complete merged qualification remain pending.
```
<!-- /evidence-field -->
