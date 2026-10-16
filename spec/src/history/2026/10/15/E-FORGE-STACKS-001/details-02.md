---
schema: 1
id: E-FORGE-STACKS-001
---

# E-FORGE-STACKS-001 details 2

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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/forge.test.ts
```

**Notes**

Before correction: partial response discarded the other PR; refresh lost an enqueued merge. Both new regressions reproduced. Later focused runs pass.
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
pnpm dev:cargo -- test -p jj-store-http --test forge_admission --locked
```

**Notes**

Initial new fixture used HttpApi::new(None). Corrected to with_search(None,...); production admission was unchanged. Exact test subsequently passes.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check
```

**Notes**

Intermediate generated-contract/type errors in this feature: merges artifact drift and bigint object keys. Regenerated and retained exact IDs using string keys.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 8
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:check
```

**Notes**

Intermediate feature errors included the same four type errors, stale fixture schema and three unsupported exact role-query options. Corrected only new/changed files.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
node scripts/web-architecture.mjs
```

**Notes**

Four changed existing Stack files exceeded 300 counted lines after integration. Extracted the new controls/lifecycle and used direct adapter delegates; subsequent count check passes. An intermediate run still reported the lane at 305 before the final extraction.
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
pnpm dev:cargo -- test -p jj-forge --lib --tests --locked
```

**Notes**

16 tests: check-state normalization, 10 planning/journal/dependency guards, three provider fixtures and two submission recovery tests. No live credentials.
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
pnpm dev:cargo -- test -p jj-store-jj --lib local_stacks::forge:: --locked
```

**Notes**

Two tests: bounded/symlink-safe templates and real disposable Git/JJ two-bookmark push, squash landing and Restack. Empty final commit over meaningful content is accepted; change ID and physical draft survive. Original fixture needed explicit main@origin tracking; only the new fixture was fixed.
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
pnpm dev:cargo -- test -p jj-store --test forge_cli --locked
```

**Notes**

One acceptance test across all three forge configurations, including unchanged JJ operation and no persisted token. Cold CLI binary. Apple linker compact-unwind size warnings are retained; no link failure.
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
pnpm dev:cargo -- test -p jj-store-http --test forge_admission --locked
```

**Notes**

One exact LocalTrusted same-origin rejection test; secret absent from response.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/forge.test.ts web/tests/vite/production-stacks-lifecycle.test.ts
```

**Notes**

20 controller/lifecycle tests. Exact pending identity, unknown write retention, partial inventory, per-PR merge queue polling and existing ownership/teardown behavior.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-forge/StoreForge.stories.svelte
```

**Notes**

Two cold interaction stories with accessibility enabled: edit and CI logs; rejected save preserves draft and publishes the application toast. Visual-pending, no baselines updated.
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
pnpm web:check
```

**Notes**

Zero errors/warnings for the web consumer.
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
pnpm catalog:check
```

**Notes**

Zero errors/warnings and catalog ownership boundaries.
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
pnpm docs:generate
```

**Notes**

Generated command and API documentation. Initial generation exposed undocumented ready/remote fields; contextual help was added only to the new commands, then the exact generator passed.
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
node .local/verify-forge-contracts.mjs
```

**Notes**

Compiled Rust example exactly matches committed OpenAPI and generated TypeScript; no unrelated contract fixtures were launched.
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
node --input-type=module (scripts/docs/review.mjs review and coverage)
```

**Notes**

Refreshed only affected source topics and the new forge crate; guide/transport pages describe implementation and qualification limits.
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
pnpm contracts:generate
```

**Notes**

Validated OpenAPI and generated web TypeScript from Rust. Artifact comparison performed separately without launching unrelated contract fixtures.
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
node --test scripts/forge-boundaries.test.mjs
```

**Notes**

One dependency-boundary test; model/object/WAL cannot import forge runtime or upstream JJ.
<!-- /run -->
<!-- /evidence-field -->

