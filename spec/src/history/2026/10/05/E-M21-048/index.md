---
schema: 1
id: E-M21-048
---

# E-M21-048

<!-- evidence-field:id markdown -->
## Id

E-M21-048
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Native byte-bound hunk selection and lazy UI wiring
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

jj-lib/JJ 0.45.1; existing released foundational dependencies unchanged
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive Selection.hunks and POST stacks/hunks; TOML v1 unchanged
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-jj/tests/local_stacks.rs#native_hunk_tokens_move_only_selected_content_and_reject_stale_display_ids
- web/tests/vite/production-stack-adapter.test.ts#lazy local diff reads replace
  display hunks with native byte-bound selections
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
cargo test -p jj-store-jj --test local_stacks native_hunk_tokens_move_only_selected_content_and_reject_stale_display_ids --locked (before)
```

**Notes**

Missing hunk API/selection field before implementation.
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

15 tests pass; exact two-hunk move/stale-ID rejection included.
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
pnpm contracts:generate; pnpm test:contracts; cargo test -p jj-store-http local_browser::stacks::tests --locked
```

**Notes**

Generated contracts, seven contract tests and two HTTP admission tests pass.
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
pnpm exec prettier --write affected files; pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/local-stack-file-mutations.test.ts web/tests/vite/production-stack-adapter.test.ts web/tests/vite/stack-workspace-lazy-controller.test.ts (initial)
```

**Notes**

Test editing inserted assertion inside an object; restored parent test and reapplied the assertion at the test end.
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

29 tests pass.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check; pnpm catalog:check (initial)
```

**Notes**

Native unavailable reason is free text, while presentation reasons are an enum. Retain the existing whole-file content and return no executable partial hunks.
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

Zero diagnostics, ownership and architecture pass.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceDrag.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceCommands.stories.svelte
```

**Notes**

Nine fresh-run interaction/accessibility scenarios pass.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec spec-validator validate --only spec-first (initial)
```

**Notes**

Unknown validator alias; corrected to the documented first command.
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
pnpm exec spec-validator first; pnpm check:progress
```

**Notes**

Canonical change mapping and progress checks pass; historical entries preserved.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Continue graph mutations, deletion/conflict editing, Undo/Redo,
  external-reference recovery and cold real-server acceptance. Wider aggregate
  lanes, visual baselines and publication out of scope.
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-05
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

yxyxkuntluqksytuonwrornrpoqvqxpy
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M21
- M21.2
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-http/src/local_browser/stacks.rs
- crates/jj-store-http/src/local_browser/stacks/tests.rs
- crates/jj-store-jj/src/local_stacks/hunks.rs
- crates/jj-store-jj/src/local_stacks/mod.rs
- crates/jj-store-jj/src/local_stacks/mutations.rs
- crates/jj-store-jj/tests/local_stacks.rs
- schemas/openapi.json
- spec/src/api.md
- spec/src/compatibility.md
- spec/src/jj-integration.md
- spec/src/local-jj.md
- spec/src/milestones.md
- spec/src/ui.md
- web/src/lib/api/generated.ts
- web/src/lib/controller/stack-workspace/local-selections.ts
- web/src/lib/controller/stack-workspace/local-session.ts
- web/src/lib/controller/stack-workspace/local-transport.ts
- web/src/lib/controller/stack-workspace/production-adapter.ts
- web/tests/vite/local-stack-file-mutations.test.ts
- web/tests/vite/production-stack-adapter.test.ts
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


