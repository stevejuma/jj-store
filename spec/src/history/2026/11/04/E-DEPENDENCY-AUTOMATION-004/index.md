---
schema: 1
id: E-DEPENDENCY-AUTOMATION-004
---

# E-DEPENDENCY-AUTOMATION-004

<!-- evidence-field:id markdown -->
## Id

E-DEPENDENCY-AUTOMATION-004
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M21
- M30
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-001
- JST-UI-020
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-11-04
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

Enclosing Design Core 0.10.1 compatibility intake
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- package.json
- pnpm-lock.yaml
- web/src/lib/ui/community-repository-branch-command/CommunityRepositoryBranchCommand.svelte
- catalog/fixtures/stack-workspace/story-actions.ts
- catalog/components/store-appearance/AppearanceShell.stories.svelte
- catalog/components/store-application/AgentChatChanges.stories.svelte
- catalog/components/store-application/AgentMentionSettings.stories.svelte
- catalog/components/store-application/ServerAgentAdministration.stories.svelte
- catalog/components/store-application/StoreNumericSetting.stories.svelte
- catalog/components/store-application/StoreSearchSettings.stories.svelte
- catalog/components/store-application/StoreServerSettings.stories.svelte
- catalog/components/store-application/StoreSyncReconciliation.stories.svelte
- catalog/components/store-application/WorkspaceFilePresentation.stories.svelte
- catalog/components/store-application/WorkspaceFiles.stories.svelte
- catalog/components/store-application/WorkspaceFilesPerformance.stories.svelte
- catalog/components/store-application/capture-harness-play.ts
- catalog/components/store-application/capture-source-table-play.ts
- catalog/components/store-application/capture-status-command-play.ts
- catalog/components/store-workflows/WorkflowChoices.stories.svelte
- catalog/components/store-workflows/WorkflowTriggers.stories.svelte
- spec/src/current/contracts/ui/01-storybook-presentation-catalog.md
- spec/src/current/progress/M21/index.md
- spec/src/current/progress/M30/index.md
- spec/src/history/2026/11/04/E-DEPENDENCY-AUTOMATION-004/index.md
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
failures: 3
hasNote: true
noteEncoding: markdown
```

```shell
GitHub Actions run 41861909614: UI acceptance / Source Storybook / Storybook interactions (shards 1/3, 2/3 and 3/3)
```

**Notes**

The rebased candidate exposed duplicate Lezer runtime state through older direct
CodeMirror pins. After graph alignment, the released Select trigger semantics
also exposed stale button-role queries, and branch regrouping exposed a real
invalid `aria-activedescendant`. No failing assertion or axe rule was removed.
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
npx --yes pnpm@10.32.1 exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceRecovery.stories.svelte --reporter=verbose
```

**Notes**

All six recovery interactions pass, including branch regrouping accessibility,
unknown-outcome retry and stale-preview draft retention. The async helper keeps
the original disappearance assertion with an explicit five-second bound.
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
JJ_STORE_BROWSER_SHARD=1/3 npx --yes pnpm@10.32.1 exec node scripts/ci-browser.mjs storybook
JJ_STORE_BROWSER_SHARD=2/3 npx --yes pnpm@10.32.1 exec node scripts/ci-browser.mjs storybook
JJ_STORE_BROWSER_SHARD=3/3 npx --yes pnpm@10.32.1 exec node scripts/ci-browser.mjs storybook
```

**Notes**

Every repository-equivalent isolated Storybook segment passes. Shard 1's four
segments pass 105, 84, 93 and 90 interactions; shard 2's four segments all pass;
shard 3's four segments pass 87, 93, 82 and 80 interactions with its two
existing skips retained. Raw one-process diagnostics also passed shard 2 but
reached the known browser-lifetime ceiling on the larger shards; the controlling
CI-equivalent segmented lanes are green.
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
npx --yes pnpm@10.32.1 web:check
npx --yes pnpm@10.32.1 catalog:check
npx --yes pnpm@10.32.1 web:test
npx --yes pnpm@10.32.1 catalog:test
npx --yes pnpm@10.32.1 check:ui-ownership
```

**Notes**

Both Svelte/type-check surfaces report zero errors and warnings. Web unit tests
pass 180/180. Catalog unit tests pass 1,209 with one existing skip. The
production/catalog ownership boundary and Storybook parity validation pass.
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
npx --yes pnpm@10.32.1 evidence:new -- --id E-DEPENDENCY-AUTOMATION-004 --task M21 --task M30 --requirement JST-GOV-002 --requirement JST-ARCH-005 --requirement JST-UI-001 --requirement JST-UI-020
npx --yes pnpm@10.32.1 exec spec-validator validate --only governance,summary,fileLimits,verification --json
npx --yes pnpm@10.32.1 exec spec-validator first --base @- --head @ --json
npx --yes pnpm@10.32.1 check:progress
```

**Notes**

The initial generator attempt stopped on the pre-existing oversized
`E-UI-CATALOG-HELP-001/index.md` page after rewriting generated navigation.
Those broad generated edits were restored; this bounded record and its two
intended day-index links were authored directly, then all document checks
passed. No product validation failed.
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

Design Core 0.10.1; pnpm 10.32.1; one `@lezer/common` 1.5.5 graph.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No native storage, WAL, RPC, publication, route or persisted browser format
changes.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Compatible Design Core intake, direct CodeMirror graph alignment, released
combobox semantics in catalog interactions and bounded recovery accessibility.
Visual approval remains deferred and no visual status changes.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Require fresh hosted CI for the rewritten pull-request head. Visual approval
remains a separately deferred gate.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: JST-UI-001 and JST-UI-020 already require direct released semantics,
  retained interaction assertions and valid accessibility state. The owning
  chapter records how this intake restores those contracts without changing
  product authority or persisted formats.
```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M21 and M30 remain In progress. This compatibility intake does not
  qualify broader provider, platform, publication or visual gates.
```
<!-- /evidence-field -->
