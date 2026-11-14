---
schema: 1
id: E-CI-STORYBOOK-ASYNC-WAIT-001
---

# E-CI-STORYBOOK-ASYNC-WAIT-001

<!-- evidence-field:id markdown -->
## Id

E-CI-STORYBOOK-ASYNC-WAIT-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M30
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-GOV-002
- JST-UI-001
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-11-14
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-stack-workspace/StoreStackWorkspaceHeaderDrops.stories.svelte
- catalog/components/store-stack-workspace/StoreStackWorkspacePinning.stories.svelte
- catalog/components/store-stack-workspace/StoreStackWorkspaceSelection.stories.svelte
- spec/src/current/contracts/ui/13-shared-application.md
- spec/src/current/progress/M30/index.md
- spec/src/history/2026/11/14/E-CI-STORYBOOK-ASYNC-WAIT-001/index.md
- spec/src/history/2026/11/14/E-CI-STORYBOOK-ASYNC-WAIT-001/SUMMARY.nav.md
- spec/src/history/2026/11/14/index.md
- spec/src/history/2026/11/14/SUMMARY.nav.md
- spec/src/history/2026/11/index.md
- spec/src/history/2026/11/SUMMARY.nav.md
<!-- /evidence-field -->

<!-- evidence-field:removed_files list -->
## Removed files

[]
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
GitHub Actions run 43595135517 attempt 1 job 132448055274
```

**Notes**

Source Storybook shard 1 passed 104 interactions and failed the initial
`Stack feature/api` region lookup in
`SelectedCommitsAndHeadersToggleClosedWithoutLosingAdjacentDiffs`. The exact
story passed unchanged locally, so no assertion was removed.
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
GitHub Actions run 43595135517 attempt 2 job 132457473113
```

**Notes**

The rerun passed the first failed story, then passed 104 interactions and
failed the different initial `Branches` region lookup in
`BranchesRemainFixedBesideAStackAndDiff`. This isolated a contended initial
fixture-render bound rather than the main-image workflow change.
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
pnpm test:storybook -- catalog/components/store-stack-workspace/StoreStackWorkspacePinning.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceSelection.stories.svelte
```

**Notes**

Both directly affected files passed all four interactions after their initial
production-region queries received a five-second asynchronous render window.
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
pnpm test:storybook --shard=1/12
```

**Notes**

The cold portable shard passed all 18 files and 105 interactions. Query roles,
accessible names, geometry, interactions and zero-retry behavior are unchanged.
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
pnpm spec:first
```

**Notes**

The spec-first gate passed both protected story files against the updated shared
application contract and fresh evidence record.
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
pnpm spec:check
```

**Notes**

Validation stopped because an explanatory paragraph pushed the already bounded
M30 ledger from 400 to 404 physical lines. The redundant paragraph was removed;
the evidence ID remains in M30's evidence column.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown
```

```shell
GitHub Actions run 43606973478 attempt 1 job 132483700929
```

**Notes**

Source Storybook shard 1 passed 92 interactions and failed the different
initial `Create commit in feature/api` button lookup in
`StagedControlsAlignToTheHeaderEdge`. The failure exhausted Testing Library's
default one-second bound before any geometry assertion ran.
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
pnpm test:storybook -- catalog/components/store-stack-workspace/StoreStackWorkspaceHeaderDrops.stories.svelte -t "StagedControlsAlignToTheHeaderEdge"
```

**Notes**

Before editing, the exact cold local story passed 1/1 after 2.9 seconds, with
five sibling interactions excluded. Together with the hosted one-second
failure, this confirms a contended initial fixture-render bound rather than a
missing control or changed geometry contract.
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
pnpm test:storybook -- catalog/components/store-stack-workspace/StoreStackWorkspaceHeaderDrops.stories.svelte -t "StagedControlsAlignToTheHeaderEdge"
```

**Notes**

After the initial button query received the same five-second readiness window,
the exact cold story passed 1/1 in 1.7 seconds. The original button role and
name, click, staged-group lookup and geometry assertions are unchanged.
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
pnpm test:storybook -- catalog/components/store-stack-workspace/StoreStackWorkspaceHeaderDrops.stories.svelte
```

**Notes**

The complete directly affected story file passed all six interactions. The
initial control, later drag-and-drop behavior and geometry assertions all ran.
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
pnpm test:storybook --shard=7/12
```

**Notes**

The cold hosted-equivalent partition passed all 18 files and 93 interactions,
including the formerly failing control lookup. This is the exact sub-shard
selected inside hosted Source Storybook shard 1/3.
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
pnpm catalog:check
pnpm format:check
pnpm spec:first
pnpm check:progress
```

**Notes**

Catalog Svelte diagnostics and boundaries passed, formatting passed, Spec-first
mapped all three protected story files to the shared application contract, and
the 186-task/1043-record progress ledger passed.
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

Storybook 10.5.7; Vitest 4.1.11; Svelte 5.57.1; Playwright 1.63.0.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No production, persistence, transport, package or visual-baseline format
changes. The story queries retain their original accessible roles and names.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Three complex Stacks stories wait up to five seconds for their first production
control or region under contended hosted shards. Later interaction, geometry
and state assertions retain their original bounds, and failed assertions are
not retried.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Qualify the repaired stories in a fresh complete hosted required check, then
publish the repaired main image from merged main.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: Shared application acceptance now records the bounded initial
  Stacks-fixture control or region render wait while retaining every role,
  label, geometry and interaction assertion and prohibiting assertion retries.
```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M30 remains In progress. This repairs a required interaction lane for
  the main-image workflow PR; fresh hosted qualification and GHCR publication
  remain required.
```
<!-- /evidence-field -->
