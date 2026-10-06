---
schema: 1
id: E-M21-047
---

# E-M21-047

<!-- evidence-field:id markdown -->
## Id

E-M21-047
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Native whole-file transfers, extraction splits and inline commit edits
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ/jj-lib 0.45.1, Node 26.9.0, pnpm 10.32.1, Vitest 4.1.11; released package ranges unchanged.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive local Transfer/Split/Edit command variants; source-derived API types, TOML v1/native JJ formats unchanged.
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-jj/tests/local_stacks.rs#selected_file_transfer_split_and_inline_edit_propagate_natively
- web/tests/vite/local-stack-file-mutations.test.ts#local file mutations map
  displayed sources to exact native commit selections
- web/tests/vite/stack-workspace-lazy-controller.test.ts#loaded commit files and
  patches become reusable draft selections
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
cargo test -p jj-store-jj --test local_stacks selected_file_transfer_split_and_inline_edit_propagate_natively --locked (before)
```

**Notes**

New exact acceptance fails compilation because Transfer/Split/Edit are absent.
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
cargo test -p jj-store-jj --test local_stacks selected_file_transfer_split_and_inline_edit_propagate_natively --locked; pnpm contracts:generate (initial)
```

**Notes**

Pinned JJ resolve_change_id returns optional ResolvedChangeTargets, not a vector. Corrected native visible-target extraction and rejected missing/divergent remaps.
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
cargo test -p jj-store-jj --test local_stacks selected_file_transfer_split_and_inline_edit_propagate_natively --locked; cargo test -p jj-store-jj --test local_stacks --locked
```

**Notes**

Exact regression and all 14 local tests pass. Cross-branch transfer, child extraction, inline edit and WIP preservation use native squash/descendant rebasing; refresh does not rebuild the merge.
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
pnpm contracts:generate; pnpm test:contracts
```

**Notes**

Contracts regenerated from Rust; seven contract tests and contract TypeScript check pass.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/local-stack-file-mutations.test.ts web/tests/vite/production-stack-adapter.test.ts web/tests/vite/stack-workspace-lazy-controller.test.ts
```

**Notes**

28 exact selection mapping/adapter/lazy controller tests pass. Net branch unions and display hunk IDs do not authorize native file mutations.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceDrag.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceInline.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceCommands.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceLazy.stories.svelte
```

**Notes**

14 cold interactions/accessibility scenarios pass across direct consumers.
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
pnpm web:check; pnpm catalog:check; pnpm check:architecture; pnpm check:ui-ownership
```

**Notes**

Zero type diagnostics, 232 catalog modules and 1108 architecture modules; UI ownership passes. Wider aggregate lanes excluded by targeted policy.
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
pnpm spec:first; pnpm spec:validate; pnpm check:progress; pnpm exec prettier --check affected TypeScript/spec files; rustfmt --edition 2024 --check affected Rust
```

**Notes**

14 protected files/five canonical chapters, five validators/173 requirements, ledger and scoped formatting pass. Historical raw evidence prefix remains unchanged.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Continue native hunks, move/reorder/squash/uncommit/rebase, bookmark/commit
  deletion, native conflict editing, repository Undo/Redo and external-reference
  recovery.
- Cold real-server integrated acceptance and backend refresh remain required;
  screenshot baselines and publication excluded.
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-05
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

zmmwvxkovuukmxywqruorkspxoskuryl
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M21
- M21.2
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-jj/src/local_stacks/basic.rs
- crates/jj-store-jj/src/local_stacks/content.rs
- crates/jj-store-jj/src/local_stacks/mod.rs
- crates/jj-store-jj/src/local_stacks/mutations.rs
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
- web/src/lib/controller/stack-workspace/controller-files.ts
- web/src/lib/controller/stack-workspace/local-mutations.ts
- web/src/lib/controller/stack-workspace/local-selections.ts
- web/src/lib/controller/stack-workspace/local-state.ts
- web/tests/vite/local-stack-file-mutations.test.ts
- web/tests/vite/stack-workspace-lazy-controller.test.ts
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


