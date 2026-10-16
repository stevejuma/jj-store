---
schema: 1
id: E-FORGE-WORKSPACE-001
---

# E-FORGE-WORKSPACE-001

<!-- evidence-field:id markdown -->
## Id

E-FORGE-WORKSPACE-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M31
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-FORGE-004
- JST-FORGE-005
- JST-FORGE-007
- JST-UI-020
- JST-GOV-008
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-15
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

Enclosing Jujutsu change rsxmvtus
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-forge/StoreForgeWorkspace.stories.svelte
- catalog/fixtures/ForgeWorkspaceFixture.svelte
- catalog/fixtures/forge/workspace-transport.ts
- spec/src/current/progress/M31/index.md
- spec/src/history/2026/10/15/E-FORGE-WORKSPACE-001/SUMMARY.nav.md
- spec/src/history/2026/10/15/E-FORGE-WORKSPACE-001/index.md
- spec/src/history/2026/10/15/SUMMARY.nav.md
- spec/src/history/2026/10/15/index.md
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
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:check
```

**Notes**

New fixture type errors only: assigning through a readonly optional adapter property and echoing an input plan whose connection defaults are optional. Fixed by constructing the adapter with its owned forge and normalizing the fixture receipt to the response contract. No shared tests or production contracts changed.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-forge/StoreForgeWorkspace.stories.svelte
```

**Notes**

Two cold interaction/accessibility stories. PR detail column is beside the API stack, both applied stacks and CI/PR badges remain present. Second story selects another PR through its branch badge, closes details and opens the ancestor-chain submission preview.
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
pnpm catalog:check
```

**Notes**

Zero errors/warnings; catalog ownership check passes.
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
node scripts/web-architecture.mjs
```

**Notes**

New fixture and story modules remain within maintained size boundaries.
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
pnpm check:ui-ownership
```

**Notes**

Production ownership is unchanged; fixtures import owned production components only.
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
pnpm exec prettier --check catalog/components/store-forge/StoreForgeWorkspace.stories.svelte catalog/fixtures/ForgeWorkspaceFixture.svelte catalog/fixtures/forge/workspace-transport.ts spec/src/current/progress/M31/index.md
```

**Notes**

Only changed authored source and ledger formatting is checked; evidence retains its canonical generator format.
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
pnpm check:progress
```

**Notes**

Fresh evidence and explicit M31 review cover this catalog-only change.
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
pnpm spec:first
```

**Notes**

Qualified unchanged-contract confirmation maps the new catalog surface to JST-UI-020 and JST-FORGE-004/005/007.
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
pnpm exec spec-validator validate --only fileLimits,governance,summary
```

**Notes**

Evidence/navigation references and authored page budgets pass.
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

Node v26.9.0; pnpm 10.32.1; Svelte 5.57.1; Storybook 10.5.7; Vitest 4.1.11; released Design Core 0.3.2. No Rust build or tests.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No public API, generated contract, production persistence or runtime format change. Forge responses and the local Workspace adapter are in-memory catalog fixtures only; no credentials or remote effects.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Before: Forge stories rendered the isolated PR panel. This slice adds a full StoreRepositoryApplication shell, two applied stacks, lower/top PR badges and passed/running/failed CI summaries, using production Stack and Forge controllers/components. The selected API stack directly precedes its PR column; the canvas is scrolled to this context for review. A second story opens a different PR and submission preview. No production or shared fixture/test infrastructure changed. Visual-pending; no baselines changed. Wider test, build and Rust lanes are out of scope per the user.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

User review of the contextual stories. Credentialed provider, native/platform and packaged application gates remain with M31 and E-FORGE-STACKS-001.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: JST-UI-020 and JST-FORGE-004/005/007 already specify PR presentation
  beside stacks. This is catalog composition and focused
  interaction/accessibility coverage of the existing owned production UI, with
  no changed behavior, protocol or dependency boundary. JST-GOV-008 ownership is
  retained.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M31 remains In progress with the new contextual catalog evidence linked.
  M21 and all live provider, packaged application and visual approval gates
  remain unchanged; this passing result qualifies only the new fixture/story
  surface.

```
<!-- /evidence-field -->

