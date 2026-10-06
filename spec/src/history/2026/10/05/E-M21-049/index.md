---
schema: 1
id: E-M21-049
---

# E-M21-049

<!-- evidence-field:id markdown -->
## Id

E-M21-049
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Native graph move/reorder, branch rebase/squash and uncommit
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ/jj-lib 0.45.1; existing toolchain and released dependencies
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive local graph command variants; TOML v1 unchanged
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-jj/tests/local_stacks.rs#graph_mutations_move_reorder_squash_uncommit_without_rebuilding_merge
- crates/jj-store-jj/tests/local_stacks.rs#branch_graph_actions_stop_at_lower_bookmark_and_preserve_it
- crates/jj-store-jj/tests/local_stacks.rs#moving_a_tip_retains_applied_edges_in_inactive_workspaces
- web/tests/vite/local-stack-file-mutations.test.ts#local graph actions use
  exact displayed commits and branch boundaries
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
cargo test -p jj-store-jj --test local_stacks graph_mutations_move_reorder_squash_uncommit_without_rebuilding_merge --locked (before)
```

**Notes**

Command variants absent.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-jj --test local_stacks graph_mutations_move_reorder_squash_uncommit_without_rebuilding_merge --locked (initial)
```

**Notes**

Added required native parent_tree field. Corrected decorated bookmark-label assumption to compare exact commit relationships.
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
cargo test -p jj-store-jj --test local_stacks branch_graph_actions_stop_at_lower_bookmark_and_preserve_it --locked (initial and diagnostic --nocapture)
```

**Notes**

Single-revision rebase deliberately detached retained descendants. Corrected branch rebase to move its root with descendants; tested native applied-edge preservation for individual moves.
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
cargo test -p jj-store-jj --test local_stacks --locked
```

**Notes**

18 real native tests pass, including all exact regressions.
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
pnpm contracts:generate; pnpm test:contracts
```

**Notes**

Generated source-owned types; seven contract tests pass.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 7
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check; pnpm catalog:check (before generation completed)
```

**Notes**

Checks captured old generated union while contract generation held Cargo lock. Reran after generation.
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
pnpm web:check; pnpm catalog:check; pnpm check:architecture; pnpm check:ui-ownership; pnpm check:boundaries
```

**Notes**

Zero diagnostics; ownership, architecture and boundary checks pass.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/local-stack-file-mutations.test.ts web/tests/vite/production-stack-adapter.test.ts web/tests/vite/stack-workspace-lazy-controller.test.ts; pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/local-stack-file-mutations.test.ts
```

**Notes**

30 units and two final exact mapping units pass; corrected oldest-to-newest directional mapping.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceHeaderDrops.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceMenus.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceDrag.stories.svelte
```

**Notes**

13 fresh-run interactions/accessibility scenarios pass.
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
cargo fmt --all; pnpm exec prettier --write affected files; pnpm exec spec-validator first; pnpm check:progress
```

**Notes**

Formatting, canonical mapping and evidence bookkeeping pass. Historical objects/raw prefix preserved.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Continue bookmark/commit deletion, native conflict editing, Undo/Redo,
  reference repair and selected discard; cold real-server acceptance remains.
  Aggregate lanes, screenshots/baselines and publication excluded.
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-05
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

rururlvptoqpzolpsxqpytzytuzxymrs
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M21
- M21.2
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-jj/src/local_stacks/graph.rs
- crates/jj-store-jj/src/local_stacks/mod.rs
- crates/jj-store-jj/src/local_stacks/preview.rs
- crates/jj-store-jj/tests/local_stacks.rs
- schemas/openapi.json
- spec/src/api.md
- spec/src/compatibility.md
- spec/src/jj-integration.md
- spec/src/local-jj.md
- spec/src/milestones.md
- spec/src/ui.md
- web/src/lib/api/generated.ts
- web/src/lib/controller/stack-workspace/local-mutations.ts
- web/tests/vite/local-stack-file-mutations.test.ts
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


