---
schema: 1
id: E-CI-STORY-TURN-SELECTION-001
---

# E-CI-STORY-TURN-SELECTION-001

<!-- evidence-field:id markdown -->
## Id

E-CI-STORY-TURN-SELECTION-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M30
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-ARCH-005
- JST-UI-001
- JST-COLLAB-034
- JST-COLLAB-035
- JST-COLLAB-036
- JST-GOV-002
- JST-GOV-003
- JST-GOV-007
- JST-GOV-011
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

- catalog/components/store-application/AgentChatChanges.stories.svelte
- spec/src/current/contracts/architecture/31-ci-execution.md
- spec/src/current/contracts/ui/01-storybook-presentation-catalog.md
- spec/src/current/progress/M30/index.md
- spec/src/current/progress/M30/history/M30/index.md
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
gh run view 41554381624 --job 125649720378 --log
```

**Notes**

PR #41's source-Storybook shard 3/3 failed on head `30bec1b` because the story
selected the second completed-change region by DOM position. Hosted ordering put
the earlier-turn panel second, so the current turn's `+40` statistic was absent.
The cold-freshness test changed by that head was not in this failing lane.
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
npx --yes pnpm@10.32.1 test:storybook catalog/components/store-application/AgentChatChanges.stories.svelte
npx --yes pnpm@10.32.1 test:storybook catalog/components/store-application/AgentChatChanges.stories.svelte
```

**Notes**

Two fresh isolated runs each passed all three interactions. The repaired story
waits for both retained panels, then selects the current panel by its exact
accessible repository/turn name before asserting totals, files and controls.
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
npx --yes pnpm@10.32.1 test:storybook --shard=9/12
```

**Notes**

The exact hosted segment passed 18 files and 82 interactions without retries.
`SelectedLiveAvatarAndCompletedFilePanel` passed in 1.403 seconds. Validation
run `jj-store-validation-29e88f62-699e-4125-85d6-a9b14ce9375f` used UI image
`sha256:a30028e133bc2f1aecb6d66d1b3e67da3105851b83a88111e9bdf96b0f3bbfc9`.
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
npx --yes pnpm@10.32.1 exec prettier --check $(jj diff --name-only)
node node_modules/@lapismd/spec-validator/dist/cli.js validate --only governance,summary,fileLimits,verification
node node_modules/@lapismd/spec-validator/dist/cli.js first --base @- --head @
node scripts/check-progress.mjs
```

**Notes**

Changed-file formatting, selected specification validation, spec-first
enforcement and progress-ledger checks passed.
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

Node.js 22.22.0; pnpm 10.32.1; Vitest 4.1.11; JJ 0.45.1; Git 2.43.0.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No production source, API, event, snapshot or persisted format changed.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

The source-Storybook interaction for concurrent completed agent turns only.
Production presentation and the complete Storybook inventory are unchanged.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Require fresh hosted PR checks, then repair the independent project-preview and
reaction-geometry cold failures before complete integrated qualification.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: The story still proves the current turn's totals, file list, controls and
  concurrent earlier turn under JST-UI-001 and JST-COLLAB-034 through
  JST-COLLAB-036. It now resolves that current turn by accessible identity rather
  than incidental DOM ordering; no product behavior changed.
```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M30 remains In progress. Focused source-Storybook evidence passes, while
  fresh hosted checks, two independent cold failures and complete release
  qualification remain open.
```
<!-- /evidence-field -->

<!-- generated-navigation -->

## Pages
