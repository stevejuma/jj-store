---
schema: 1
id: E-M20-009
---

# E-M20-009

<!-- evidence-field:id markdown -->
## Id

E-M20-009
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M20
- M20.1
- M20.2
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-05
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change twwpnlvp
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

approved-backend-integration-foundation
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ 0.45.1; pnpm 10.32.1; current application parent ac401d2e; UI plan parent b5141eb0
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No storage, native, companion, checkout or wire format changes. Backend approval recorded without enabling unqualified capabilities.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- docs/implementation-plan.md
- spec/src/architecture.md
- spec/src/ui.md
- spec/src/api.md
- spec/src/spec-governance.md
- spec/src/milestones.md
- scripts/progress-mapping.test.mjs
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Both workspaces clean. UI plan and current main had diverged; four documentation conflicts, no source conflicts.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Merge the preserved stacks UI with current application code. Retain current main ledgers and eight UI-only historical entries byte-for-byte. Record explicit backend/UI execution approval and keep qualified capabilities as the runtime gate. Main owns M20 freshness; stacks now uses M21 with explicit E-M20-* historical mapping. Historical evidence payloads remain unchanged.
<!-- /evidence-field -->

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
pnpm web:check
```

**Notes**

zero errors/warnings; integration-web-check.log
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
pnpm catalog:check
```

**Notes**

zero errors/warnings, 220 modules; integration-catalog-check.log
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
pnpm catalog:test web/tests/vite/stack-workspace*.test.ts web/tests/vite/store-route-history.test.ts
```

**Notes**

8 files, 76 tests pass; integration-unit.log
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
pnpm test:storybook catalog/components/store-stack-workspace catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLogDiff.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLogFiles.stories.svelte catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.stories.svelte
```

**Notes**

22 files, 103 tests pass; integration-stories.log
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
pnpm exec prettier --write docs/implementation-plan.md spec/src/architecture.md spec/src/ui.md spec/src/api.md spec/src/spec-governance.md
```

**Notes**

changed prose only; historical ledger excluded
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
pnpm check:ui-ownership
```

**Notes**

one validator passes; integration-ownership.log
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
pnpm check:architecture
```

**Notes**

1052 web modules pass; integration-architecture.log
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
pnpm spec:validate
```

**Notes**

5 validators, 23 chapters, 173 requirements pass; integration-spec-validate.log
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
python3 .local/stacks-backend-evidence/integration-gates.py
```

**Notes**

472 main and eight UI-only raw objects preserved; actual diffs against both merge parents checked through spec:first --file; integration-mapped-first-ui.log and integration-mapped-first-main.log
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
pnpm spec:first
```

**Notes**

Tool assumes a single @- parent; merge has two. No specification violation diagnosed. integration-spec-first.log
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first --base b5141eb0 --head @; pnpm spec:first --base ac401d2e --head @
```

**Notes**

Explicit base mode in the installed validator uses Git and cannot resolve JJ workspace @. Exact actual changed paths from both parents are checked separately through its documented --file mode.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

First merged ledger run detected duplicate M20 task identity and missing result on new integration entry; corrected by allocating M21 and preserving historical family payloads through explicit mapping.
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
node --test scripts/progress-mapping.test.mjs
```

**Notes**

5 governance mapping tests pass; integration-mapping.log
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
pnpm check:progress
```

**Notes**

142 unique tasks; corrected M21 mapping and approval ledger pass; integration-progress.log
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Production stack adapter/contracts, bounded reads, durable local commands,
  named combinations, exact file/hunk commits, graph actions and linked native
  checkout remain to implement.
- Registry-only foundation qualification and screenshot baseline approval remain
  separate.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Define versioned owned production contracts and bounded local reads, then wire the real lifecycle/controller. Continue approved execution stages; integration is not feature completion.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M20-009 details 2](details-02.md)
