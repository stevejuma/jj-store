---
schema: 1
id: E-M21-046
---

# E-M21-046

<!-- evidence-field:id markdown -->
## Id

E-M21-046
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Ordinary local bookmark/basic commit actions and bounded policy inspection
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ/jj-lib 0.45.1; Node 26.9.0; pnpm 10.32.1; Vitest 4.1.11; package ranges unchanged.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive LocalStacks CreateStack/CreateBranch/RenameBranch/Reword/Empty commands and inspection endpoint. TOML v1/native JJ storage unchanged.
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-jj/tests/local_stacks.rs#local_bookmark_and_basic_commit_actions_use_native_rewrites
- crates/jj-store-jj/tests/local_stacks.rs#first_commit_on_empty_stack_forks_target_without_rewriting_other_branches
- web/tests/vite/production-stack-adapter.test.ts#local basic mutations use
  server previews and inspect only displayed commits
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
cargo check -p jj-store-jj --locked (initial)
```

**Notes**

Compile rejected nonexistent bookmark removal helper and Arc coercion after shared resolver extension. Replaced removal with native absent target and explicit repo references.
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
cargo check -p jj-store-jj --locked
```

**Notes**

Owning native package check passes.
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
cargo test -p jj-store-jj --test local_stacks local_bookmark_and_basic_commit_actions_use_native_rewrites --locked (initial)
```

**Notes**

Test expected a trailing newline from a description set without one. Corrected expectation to the exact message supplied; native data unchanged.
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
cargo test -p jj-store-jj --test local_stacks local_bookmark_and_basic_commit_actions_use_native_rewrites --locked; cargo test -p jj-store-jj --test local_stacks --locked
```

**Notes**

Exact regression and all 13 local core/mutation tests pass; new empty-stack fork case preserves unrelated branch/target commits and merge/WIP change IDs, with no refresh rebuild.
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
cargo test -p jj-store-http --lib local_browser::stacks::tests --locked
```

**Notes**

Two admission tests pass; inspection shares registered-folder/same-origin admission.
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
pnpm contracts:generate; pnpm test:contracts
```

**Notes**

Source-derived contracts regenerated; seven public contract tests pass.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check; pnpm catalog:check (initial)
```

**Notes**

Early check ran before contract generation settled. Final two diagnostics identified request bodies are inline operation types, not response Model components. Used generated operation body Exact type and preserved bigint generation.
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
pnpm web:check; pnpm catalog:check
```

**Notes**

Zero errors/warnings in both type checks; 232 catalog modules meet ownership boundary.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-stack-adapter.test.ts web/tests/vite/stack-workspace-lazy-controller.test.ts
```

**Notes**

26 direct adapter/controller tests pass, including native command mappings, preview-only behavior, displayed-point inspection and no eager history.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceCommands.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceMenus.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceForms.stories.svelte
```

**Notes**

13 cold interaction/accessibility stories pass across affected action forms and menus.
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
pnpm check:ui-ownership; pnpm check:architecture
```

**Notes**

Source ownership passes; 1107 modules meet architecture limits.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceProduction.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceLazy.stories.svelte
```

**Notes**

Five cold production-read/lazy-file stories pass; operation pins, preparation and read-only fixtures retained.
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
pnpm spec:first; pnpm spec:validate; pnpm check:progress
```

**Notes**

20 protected files/six canonical chapters, five validators/173 requirements and 142 ledger tasks pass. Historical ledger raw prefix retained.
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
pnpm exec prettier --check affected TypeScript/spec files; rustfmt --edition 2024 --check affected Rust files
```

**Notes**

Scoped formatting checks pass; no generated type or visual baseline hand edits.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Continue approved transfers/splits/inline edit, hunk selections, graph
  operations, deletion/conflict editing and native Undo/Redo.
- Cold real-server integrated acceptance and local backend refresh remain
  required; wider aggregate lanes and screenshots/publication excluded.
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-05
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

xlyqslwylzvynstsnoktkpwumwrywwqs
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M21
- M21.2
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M21-046 details 2](details-02.md)
