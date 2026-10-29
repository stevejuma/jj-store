---
schema: 1
id: E-CONFLICT-SHEET-WAIT-001
---

# E-CONFLICT-SHEET-WAIT-001

<!-- evidence-field:id markdown -->
## Id

E-CONFLICT-SHEET-WAIT-001
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-29
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4.5
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-002
- JST-API-005
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-mutations/StoreConflictEditor.stories.svelte
- spec/src/history/2026/10/29/E-CONFLICT-SHEET-WAIT-001/index.md
- spec/src/history/2026/10/29/E-CONFLICT-SHEET-WAIT-001/SUMMARY.nav.md
- spec/src/history/2026/10/29/index.md
- spec/src/history/2026/10/29/SUMMARY.nav.md
- spec/src/history/2026/10/SUMMARY.nav.md
- spec/src/history/2026/10/index.md
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
gh run view 40792237540 --repo stevejuma/jj-store --log-failed
```

**Notes**

Hosted Storybook interactions shard 2/3 failed on NativeEditorSharesMergeHeaderSettingsAndRetainsFilteredDrafts with Unable to find role region name Resolve native conflicts after the Application menu Resolve conflicts path. Local shard 2/3 and the focused story were green on the same tip.
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
pnpm test:storybook -- catalog/components/store-mutations/StoreConflictEditor.stories.svelte -t NativeEditorSharesMergeHeader
```

**Notes**

Hardened sheetFor to await Application menu and Resolve conflicts with findByRole, raise the Resolve native conflicts region timeout to 10s, and waitFor visibility before returning the dialog. Focused story: 1 passed, 6 skipped.
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

pnpm 10.32.1; Storybook vitest catalog project; Design Core 0.7.0 and Mira 0.1.1 already pinned on the enclosing branch tip.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No stored format, DTO, or public API schema change. Story helper wait contract only.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Stabilize StoreConflictEditor Storybook opening of the Resolve native conflicts sheet under hosted CI timing so shard 2/3 required checks can pass without changing product conflict-editor behavior.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Push the hardened helper, confirm PR 20 required Storybook shard 2/3 is green, then merge.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: JST-UI-002 and JST-API-005 conflict-sheet presentation are unchanged.
  Only the Storybook sheetFor helper waits longer and uses findByRole for the
  menu path that opens Resolve native conflicts.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M4.5 browser presentation gates are unchanged. This repair addresses a
  hosted Storybook flake on an existing conflict-editor story without promoting
  new product gates.

```
<!-- /evidence-field -->
