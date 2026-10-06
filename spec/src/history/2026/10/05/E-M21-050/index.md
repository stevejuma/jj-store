---
schema: 1
id: E-M21-050
---

# E-M21-050

<!-- evidence-field:id markdown -->
## Id

E-M21-050
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Native bookmark deletion, conflicts, discard, repository Undo/Redo and explicit reference reconciliation
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

jj-lib/JJ 0.45.1; existing Rust/TypeScript and released foundational dependencies
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive local command/state/conflict contracts; local TOML v1 unchanged
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-jj/tests/local_stacks.rs#native_undo_redo_includes_cli_and_never_reapplies_toml_membership
- crates/jj-store-jj/tests/local_stacks.rs#native_conflict_resolution_and_selected_discard_preserve_other_content
- crates/jj-store-jj/tests/local_stacks.rs#selected_hunk_discard_retains_unselected_hunks
- web/tests/vite/stack-workspace-lazy-controller.test.ts#accepted lazy native
  file rows can drag whole files before patches are loaded
- web/tests/vite/production-stack-adapter.test.ts#lazy local diff reads replace
  display hunks with native byte-bound selections
- crates/jj-store-jj/tests/local_stacks.rs#cli_branch_advancement_requires_explicit_reconciliation_and_retains_wip
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
cargo test -p jj-store-jj --test local_stacks native_undo_redo_includes_cli_and_never_reapplies_toml_membership --locked (before/initial)
```

**Notes**

Missing commands before implementation; then missing conflict trait import and observational CLI log produced a new import/export operation. Fixed trait and used --ignore-working-copy for observational assertions.
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
cargo test -p jj-store-jj --test local_stacks --locked
```

**Notes**

22 native tests pass. CLI advance now requires explicit reconciliation; WIP content retained.
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
cargo test -p jj-store-http local_browser::stacks::tests --locked
```

**Notes**

Two transport/admission tests pass including guarded conflict requests.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/local-stack-file-mutations.test.ts web/tests/vite/production-stack-adapter.test.ts web/tests/vite/stack-workspace-lazy-controller.test.ts (initial)
```

**Notes**

Drag event stub lacked stopPropagation; runtime FileListing import caused timeout. Fixed stub and type-only import.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/local-stack-file-mutations.test.ts web/tests/vite/production-stack-adapter.test.ts web/tests/vite/stack-workspace-lazy-controller.test.ts web/tests/vite/stack-workspace.test.ts
```

**Notes**

Focused controller, production adapter, lazy drag and recovery regressions pass.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceRecovery.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceMenus.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceCommands.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceDrag.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceLazy.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceProduction.stories.svelte (initial)
```

**Notes**

One suite import failed during Vite dependency optimization reload; two read-only production diff stories requested mutation tokens. Guarded token reads by accepted mutable capability/source.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceRecovery.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceMenus.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceCommands.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceDrag.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceLazy.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceProduction.stories.svelte
```

**Notes**

23 interaction/accessibility scenarios pass in a fresh test process.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:architecture (initial and after public type reuse)
```

**Notes**

Adapter exceeded source budget by four then one counted line; used existing public function type for delegation, without changing allowance.
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
pnpm web:check; pnpm catalog:check; pnpm test:contracts; pnpm check:architecture; pnpm check:boundaries; pnpm check:ui-ownership
```

**Notes**

Zero type diagnostics; seven contract tests and ownership/architecture/boundary checks pass.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/local-stack-file-mutations.test.ts web/tests/vite/production-stack-adapter.test.ts web/tests/vite/stack-workspace-lazy-controller.test.ts web/tests/vite/stack-workspace.test.ts (expanded initial)
```

**Notes**

Native token fixture lacked qualified inspect/preview capabilities. Added those interfaces so the test exercises the actual mutable provider profile.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/local-stack-file-mutations.test.ts web/tests/vite/production-stack-adapter.test.ts web/tests/vite/stack-workspace-lazy-controller.test.ts web/tests/vite/stack-workspace.test.ts (final)
```

**Notes**

69 units pass, including lower-bookmark retention and capability gating.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec spec-validator first; pnpm check:progress (initial)
```

**Notes**

Spec-first passed. Progress identified two file-only test references and renamed historical automatic-reconciliation test. Corrected exact references and recorded replaced regression in removed_tests; historical objects retained.
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
cargo fmt --all; pnpm exec prettier --write affected files; pnpm exec spec-validator first; pnpm check:progress (final)
```

**Notes**

Formatting, spec-first and evidence mapping pass; prior evidence objects/raw prefix preserved.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Dependent-stack projection and cold real-server mutation acceptance follow.
  Historical prepared-command M21 profile remains separate.
  Screenshots/baselines, release publication and aggregate lanes excluded.
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-05
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

wpyuuwwvsxtknwvvkutyltyulxyvvqun
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M21
- M21.2
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M21-050 details 2](details-02.md)
