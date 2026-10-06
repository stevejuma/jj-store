---
schema: 1
id: E-M21-053
---

# E-M21-053 details 2

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
cargo check -p jj-store-cli --tests --locked
```

**Notes**

cargo-check.log: archived index source omitted the new optional description field; supplied None for topology-only recovery.
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
cargo check -p jj-store-cli --tests --locked
```

**Notes**

cargo-check-final.log: intermediate String did not satisfy bounded Option&lt;Description&gt;; corrected to topology-only None.
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
cargo check -p jj-store-cli --tests --locked
```

**Notes**

cargo-check-verified.log: affected package and direct consumer test targets pass.
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
pnpm web:check
```

**Notes**

web-check.log: moved action type needed its own Model import; corrected the owning module.
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
pnpm catalog:check
```

**Notes**

catalog-check.log: the same missing Model import was the sole catalog diagnostic; corrected.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/read-query.test.ts web/tests/vite/thread-query-cache.test.ts web/tests/vite/production-stack-adapter.test.ts web/tests/vite/production-stacks-application.test.ts web/tests/vite/production-stacks-lifecycle.test.ts web/tests/vite/stack-workspace-segments.test.ts web/tests/vite/local-stack-history.test.ts web/tests/vite/local-stack-file-mutations.test.ts web/tests/vite/store-stacks-navigation.test.ts web/tests/vite/kind-preparation.test.ts web/tests/vite/production-patch-reader.test.ts
```

**Notes**

units-verified.log:75 tests /11 files pass.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/kind-preparation-http.test.ts web/tests/vite/local-browser.test.ts web/tests/vite/local-browser-merge.test.ts web/tests/vite/production-store-data-source.test.ts web/tests/vite/working-tree-http.test.ts web/tests/vite/production-stack-adapter.test.ts web/tests/vite/read-query.test.ts
```

**Notes**

adapter-units.log:67 tests /7 files pass; overlap with the preceding run is retained, not claimed as unique tests.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspacePresentation.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte catalog/components/community-projects/CommunityProjectsRevisionGraph.stories.svelte
```

**Notes**

stories.log:27 passed /1 failed. Op Log fixture did not supply main's binary unavailable reason. The reason and its precise explanation are now retained and asserted.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte -t OpLog
```

**Notes**

oplog-fixed.log: exact OpLog regression and comparison warning pass (2 tests); cold browser process.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte -t OpLog
```

**Notes**

Main workspace run. main-oplog-before.log: main's separate dependency setup could not import Storybook annotations; no product tests ran and no main source or dependencies were changed.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:architecture
```

**Notes**

architecture.log: combined local/native aggregate data sources and revision graph exceeded300 counted lines. Moved code observation, preparation forwarding and row measurement to their owning helpers; limits unchanged.
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
pnpm check:architecture
```

**Notes**

architecture-final.log:1124 production/catalog/script modules pass.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --write
```

**Notes**

Prettier could not infer the Rust parser. JS/Svelte/Markdown files formatted; affected Rust files subsequently pass native rustfmt. The initial selected argument vector is not retained in the output log; this records the command prefix only, not a reproducible path selection. Final full selection is recorded below.
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
rustfmt --edition 2024 --check crates/jj-store-cli/tests/local_browser.rs crates/jj-store-http/src/contract/tests.rs crates/jj-store-http/src/lib.rs crates/jj-store-jj/src/local_browser/mod.rs crates/jj-store-jj/src/local_execution/recovery/indexes/archive.rs crates/jj-store-jj/src/query/cache/resident.rs crates/jj-store-jj/src/query/cache.rs crates/jj-store-jj/src/query/index.rs crates/jj-store-jj/src/query/mod.rs
```

**Notes**

rust-format.log: native formatting passes.
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
cargo test -p jj-store-jj --locked --lib query::cache::resident::tests
```

**Notes**

resident-tests.log:1 native memory-accounting regression passes.
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
cargo test -p jj-store-jj --test local_stacks --locked
```

**Notes**

local-stacks.log:22 ordinary-local mutation regressions pass, including native megamerge/WIP preservation, selected files/hunks, graph actions and Undo/Redo.
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
pnpm check:ui-ownership
```

**Notes**

ownership.log: production ownership boundary passes.
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
pnpm check:boundaries
```

**Notes**

boundaries.log:17 package boundaries pass.
<!-- /run -->
<!-- /evidence-field -->

