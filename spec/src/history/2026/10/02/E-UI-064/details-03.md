---
schema: 1
id: E-UI-064
---

# E-UI-064 details 3

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first
```

**Notes**

Mapped canonical compatibility, JJ/RPC, mutations, publication and provenance chapters missing; added precise additive target and unchanged authority requirements.
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
pnpm spec:first
```

**Notes**

34 protected files,11 canonical chapters before final ledger insertion.
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
pnpm spec:validate; pnpm check:ui-ownership
```

**Notes**

5 validators,21 chapters,147 requirements; local Community source/license provenance.
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
pnpm exec prettier --write (changed supported source, story and canonical files); cargo fmt --all
```

**Notes**

Focused format commands in /tmp/jj-task-*-format*.log; no baseline writes.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
Design Core: pnpm exec vitest run --project storybook src/shared/shadcn/dropdown-menu/DropdownMenuConsumer.stories.svelte -t SubmenuLeadingIconAlignment
```

**Notes**

Before selector fix: leading Copy offset54.84375px.
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
Design Core: pnpm exec vitest run --project storybook src/shared/shadcn/dropdown-menu/DropdownMenuConsumer.stories.svelte
```

**Notes**

After selector fix2/2, leading icons align and trailing chevron stays right.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
Design Core: pnpm spec:first; pnpm spec:validate
```

**Notes**

Source acceptance exceeded4 bullets and new bullet combined sentences; corrected without relaxing governance.
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
Design Core: pnpm checks
```

**Notes**

New story TypeScript used unsupported exact role option; fixed to /^Copy$/.
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
Design Core: pnpm checks
```

**Notes**

Types,unit715/715,Storybook642/642,static build and audit passed. Workspace pointer13/14 with one timeout under concurrent build load.
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
Design Core: pnpm test:workspace:pointer --grep bottom panel height resizes from its top rail
```

**Notes**

Exact retry1/1, no code/test changes. Full source checks resumed to finish remaining browser lanes.
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
pnpm contracts:check
```

**Notes**

Generated contracts match;138 actual HTTP/SSE captures conform;7 contract unit tests and TypeScript contracts pass.
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
cargo test -p jj-store-wal --test collab_commands --test collab_persistence --locked
```

**Notes**

21/21: commands8, persistence13.
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
cargo test -p jj-store-rpc --lib --locked collab
```

**Notes**

1/1 exact binary identity/metadata carrier test.
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
cargo clippy -p jj-collab -p jj-store-wal --all-targets --no-deps --locked -- -D warnings
```

**Notes**

Affected domain and WAL lint clean; wider service lint remains blocked by inherited issues recorded above.
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
pnpm web:check; pnpm catalog:check
```

**Notes**

Final checks after extracted leaf and feedback reload changes:0 errors/warnings, catalog129 modules.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/work-reactions.test.ts web/tests/vite/reaction-feedback.test.ts web/tests/vite/production-work.test.ts web/tests/vite/automatic-work-refresh.test.ts web/tests/vite/production-automatic-refresh.test.ts
```

**Notes**

Final direct lifecycle run18/18.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

New evidence linked Planned reducer task with -,E-UI-064; removed erroneous reducer task link. Reducer remains unchanged/unqualified.
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
pnpm check:progress
```

**Notes**

120 tasks; evidence bookkeeping only.
<!-- /run -->
<!-- /evidence-field -->

