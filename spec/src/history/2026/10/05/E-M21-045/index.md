---
schema: 1
id: E-M21-045
---

# E-M21-045

<!-- evidence-field:id markdown -->
## Id

E-M21-045
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M21
- M21.2
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-05
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

lqxwsswuwylruxrsqxkkznnwlpxnxsrx
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Ordinary local selected-file insertion and native squash propagation
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ/jj-lib 0.45.1; Node 26.9.0; pnpm 10.32.1; Vitest 4.1.11.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive LocalStacksCommand Commit and read-only preview endpoint; TOML v1 and native JJ storage unchanged.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-http/src/local_browser/stacks/tests.rs
- crates/jj-store-http/src/local_browser/stacks.rs
- crates/jj-store-jj/src/local_stacks/config.rs
- crates/jj-store-jj/src/local_stacks/mod.rs
- crates/jj-store-jj/src/local_stacks/mutations.rs
- crates/jj-store-jj/src/local_stacks/policy.rs
- crates/jj-store-jj/src/local_stacks/preview.rs
- crates/jj-store-jj/tests/local_stacks.rs
- docs/implementation-plan.md
- schemas/openapi.json
- spec/src/api.md
- spec/src/architecture.md
- spec/src/compatibility.md
- spec/src/jj-integration.md
- spec/src/local-jj.md
- spec/src/ui.md
- web/src/lib/api/generated.ts
- web/src/lib/controller/stack-workspace/controller-files.ts
- web/src/lib/controller/stack-workspace/controller.svelte.ts
- web/src/lib/controller/stack-workspace/local-session.ts
- web/src/lib/controller/stack-workspace/local-state.ts
- web/src/lib/controller/stack-workspace/local-transport.ts
- web/tests/vite/production-stack-adapter.test.ts
- web/tests/vite/stack-workspace-lazy-controller.test.ts
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-jj/tests/local_stacks.rs#selected_file_commit_uses_native_propagation_and_retains_wip
- crates/jj-store-jj/tests/local_stacks.rs#selected_file_commands_reject_invalid_stale_and_immutable_sources
- web/tests/vite/production-stack-adapter.test.ts#local selected-file previews
  bind WIP and do not execute before apply
- web/tests/vite/stack-workspace-lazy-controller.test.ts#loaded Unstaged files
  become draft selections without eager patches
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
cargo test -p jj-store-jj --test local_stacks --locked (baseline)
```

**Notes**

Nine existing core tests pass before mutation implementation.
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
cargo test -p jj-store-jj --test local_stacks selected_file_commit_uses_native_propagation_and_retains_wip --locked (before/fix)
```

**Notes**

First missing public APIs, then selected files remained in WIP because insertion used the pre-rebase WIP. Fixed by resolving the rewritten WIP and rebuilding selection against its current parent.
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
cargo test -p jj-store-jj --test local_stacks --locked
```

**Notes**

Eleven native tests pass, including invalid/stale selection and configured immutable-source rejection without publication.
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
cargo test -p jj-store-http --lib local_browser::stacks::tests --locked
```

**Notes**

Two focused admission tests pass, including same-origin and noninitializing preview.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-stack-adapter.test.ts web/tests/vite/stack-workspace-lazy-controller.test.ts
```

**Notes**

25 adapter/lazy-controller tests pass.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceCommands.stories.svelte
```

**Notes**

Six cold Storybook interactions pass. Earlier title filter skipped all six and was not counted as acceptance.
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
pnpm contracts:generate; pnpm test:contracts; pnpm web:check; pnpm catalog:check; pnpm check:ui-ownership
```

**Notes**

Generated public types; seven contract tests pass, type checks report zero errors/warnings, ownership passes.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:architecture; pnpm spec:first (initial)
```

**Notes**

Controller source budget exceeded by six lines; three mapped chapters missing. Extracted file reconciliation and added owning specification requirements.
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
pnpm spec:validate
```

**Notes**

Five validators, 23 chapters, 173 requirements pass.
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
pnpm web:check; pnpm check:architecture; pnpm spec:first (after)
```

**Notes**

Zero web diagnostics; 1105 modules meet architecture limits; 18 protected files/six canonical chapters pass.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --check changed supported files (initial)
```

**Notes**

Five changed files needed formatting; corrected with scoped prettier write.
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
pnpm exec prettier --check changed supported files; rustfmt --edition 2024 --check affected Rust; pnpm check:progress; pnpm spec:first; pnpm spec:validate
```

**Notes**

Formatting and all focused specification/progress checks pass. Wider aggregate lanes out of scope under targeted policy.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Continue approved branch/basic commit actions, transfers/splits, hunks, graph
  operations, deletion/conflict editing and native Undo/Redo; production
  capabilities stay disabled until accepted.
- Cold real-server integrated mutation acceptance remains required after backend
  build/restart. Package publication and screenshot baselines excluded.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


