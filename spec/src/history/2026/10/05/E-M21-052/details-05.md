---
schema: 1
id: E-M21-052
---

# E-M21-052 details 5

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
node scripts/stacks-browser-acceptance.mjs -g cold_native_selected_file_and_hunk_mutations_with_undo_redo (nineteenth)
```

**Notes**

browser-mutations-nineteenth.log: Card-surface drag dispatched transfer successfully. Trace proved retiring diff cleanup restored the previous batch loaded state after scope invalidation.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceProduction.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceBranches.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceLazy.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceMenus.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceCommands.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceHeaderDrops.stories.svelte (loaded file projection)
```

**Notes**

loaded-file-stories.log: SIGINT exit130 while awaiting output; incomplete run is not acceptance evidence. Final rerun follows.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceProduction.stories.svelte (loaded file projection)
```

**Notes**

loaded-file-production-story.log: four interaction/accessibility stories pass.
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
pnpm web:check; pnpm catalog:check; pnpm check:architecture (loaded file projection)
```

**Notes**

loaded-file-web-types/catalog-types/architecture.log: zero diagnostics,233 catalog modules and1110 production modules.
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
pnpm web:build (loaded file projection)
```

**Notes**

loaded-file-web-build.log: complete production build and staged admission/help assets.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceLazy.stories.svelte -t ChangedOperationReloadsRetainedAdjacentPatches (corrected operation fixture)
```

**Notes**

changed-scope-patches-before-corrected.log: one passes/four skipped; four actual patch reads asserted after adapter-owned operation changed.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceLazy.stories.svelte -t ChangedOperationReloadsRetainedAdjacentPatches (asynchronous opaque operation)
```

**Notes**

async-operation-patches-before.log: one passes/four skipped. Fixture now mirrors native asynchronous readKey changes with unchanged UI revision; this fixture alone did not reproduce the cold native cleanup bug.
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
pnpm docs:review application (patch-trace)
```

**Notes**

patch-trace-doc-review.log: reviewed temporary patch diagnostics; unchanged public contracts.
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
pnpm web:build (patch-trace)
```

**Notes**

patch-trace-web-build.log: complete traced production build; all temporary diagnostics removed after diagnosis.
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
pnpm docs:review application (patch-state)
```

**Notes**

patch-state-doc-review.log: reviewed temporary patch diagnostics; unchanged public contracts.
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
pnpm web:build (patch-state)
```

**Notes**

patch-state-web-build.log: complete traced production build; all temporary diagnostics removed after diagnosis.
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
pnpm exec prettier --write web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceDiff.svelte web/src/lib/controller/stack-workspace/lazy-details.svelte.ts web/tests/real/stacks-browser.spec.ts catalog/fixtures/StackWorkspaceLazyApplicationFixture.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceLazy.stories.svelte spec/src/ui.md
```

**Notes**

patch-lifecycle-format.log: final patch lifetime files formatted.
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
pnpm docs:review application (patch lifetime)
```

**Notes**

patch-lifecycle-doc-review.log: reviewed ordinary-local mutations, retained exact-scope file selection and patch ownership; diagnostics removed.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
node scripts/stacks-browser-acceptance.mjs -g cold_native_selected_file_and_hunk_mutations_with_undo_redo (twentieth)
```

**Notes**

browser-mutations-twentieth.log: original retained patch failure repaired; next assertion used a description revset that returned empty. Replaced it with exact selected native change identity and explicit description preservation.
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
node scripts/stacks-browser-acceptance.mjs -g cold_native_selected_file_and_hunk_mutations_with_undo_redo (twenty-first)
```

**Notes**

browser-mutations-twenty-first.log: one passes; real file/hunk drags, exact ancestor squash, unselected content, retained merge/WIP IDs, native Undo/Redo including CLI reword, TOML unchanged and cold reload/axe/page-error checks.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
node scripts/stacks-browser-acceptance.mjs (complete initial)
```

**Notes**

final-browser-full.log: navigation passes; setup locator falsely required mutually exclusive visible diff/preparation controls. Mutation startup tested a transient unconfigured snapshot before the initial read settled. 1 pass/2 fail. An overlapping isolated core run also replaced trace output; final complete rerun is sequential.
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
JJ_STORE_STACKS_BROWSER_PORT=7491 node scripts/stacks-browser-acceptance.mjs -g cold_local_megamerges_apply_unapply_and_named_wip_survive_reload
```

**Notes**

core-readiness-after.log: one passes; ordinary named workspaces, apply/unapply, settings update/deletion, independent unfinished work, TOML/reload and axe/page-error checks. Explicit preparation retains strict content assertions.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:build (patch lifetime)
```

**Notes**

patch-lifecycle-web-build.log: full built UI, admission manifest and documentation closure pass; existing chunk advisory.
<!-- /run -->
<!-- /evidence-field -->

