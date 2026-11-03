---
schema: 1
id: E-CI-STORY-TURN-TYPECHECK-001
---

# E-CI-STORY-TURN-TYPECHECK-001

<!-- evidence-field:id markdown -->
## Id

E-CI-STORY-TURN-TYPECHECK-001
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
gh api --allow-escape-sequences repos/stevejuma/jj-store/actions/jobs/125654992001/logs
```

**Notes**

The fresh PR run compiled the new role query in the catalog lane and rejected
the unsupported `exact` member on `ByRoleOptions`. Testing Library role names
supplied as strings already use exact matching; removing that member retains the
literal current-turn identity while restoring the declared type contract.
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
npx --yes pnpm@10.32.1 catalog:check
npx --yes pnpm@10.32.1 test:storybook catalog/components/store-application/AgentChatChanges.stories.svelte
```

**Notes**

The exact hosted catalog command passed with zero Svelte diagnostics and all
489 catalog module boundaries valid. The affected Storybook file then passed
all three interactions; the current-turn interaction passed in 1.748 seconds.
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

Node.js 22.22.0 and hosted 22.23.4; pnpm 10.32.1; Svelte Check 4.7.6;
Vitest 4.1.11.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No production source, API, event, snapshot or persisted format changed.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

The type-safe exact-name query in one source-Storybook interaction only.
Production presentation and saved-change behavior are unchanged.
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
reason: Removing the unsupported ByRoleOptions member leaves the literal
  repository/turn accessible name and all current-turn assertions unchanged.
  Production code, presentation and saved-change contracts do not change.
```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M30 remains In progress. Catalog compilation and the affected interaction
  pass, while fresh hosted checks, two cold failures and complete release
  qualification remain open.
```
<!-- /evidence-field -->

<!-- generated-navigation -->

## Pages
