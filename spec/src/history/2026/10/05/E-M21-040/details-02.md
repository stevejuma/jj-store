---
schema: 1
id: E-M21-040
---

# E-M21-040 details 2

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
cargo test -p jj-store-jj --test local_stacks --locked
```

**Notes**

Initial linked/sparse expansion: six passed, one failed because the test used obsolete positional sparse CLI syntax. Corrected to --clear --add; no production or assertion weakening.
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

core-rust-tests-final.log: 7/7 real ordinary jj-lib tests, including separate linked physical TOML and sparse checkout. Initial core six-parent pass retained in core-rust-tests.log.
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
cargo test -p jj-store-cli --test local_browser --locked prepared_references
```

**Notes**

reference-regression.log: extended test supplied a change anchor to bookmark-only lookup; corrected to valid bookmark bounds.
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
cargo test -p jj-store-cli --test local_browser --locked prepared_references
```

**Notes**

reference-regression-final.log: test incorrectly sent bookmark-only queries to an intentionally unsupported prepared reader; corrected to ordinary metadata-only reads while retaining prepared All/Changes comparisons.
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
cargo test -p jj-store-cli --test local_browser --locked prepared_references
```

**Notes**

reference-regression-green.log: 1/1; ordinary search/exact/windows and prepared mixed windows exclude retention bookmarks; forward/backward pagination and target hydration retained.
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
cargo test -p jj-store-cli --test local_browser --locked bounded_reads
```

**Notes**

bounded-reads.log: 6 pass; one existing large-repository opt-in smoke remains ignored. Existing metadata/index/point-read consumers retain their behavior.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-stack-adapter.test.ts web/tests/vite/production-stacks-lifecycle.test.ts web/tests/vite/production-stacks-application.test.ts web/tests/vite/stack-workspace-setup.test.ts web/tests/vite/stack-workspace-lazy-controller.test.ts web/tests/vite/stack-workspace-reads.test.ts web/tests/vite/local-stack-history.test.ts
```

**Notes**

core-ui-tests-final.log: 49/49 across 7 direct files; original core-ui-tests.log records the earlier 48-parent pass.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-stack-adapter.test.ts
```

**Notes**

core-ui-followup.log: 11/11, including command settlement independent of new-operation graph preparation.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/stories/StoreStacksApplication.stories.svelte catalog/stories/StoreStacksProduction.stories.svelte catalog/stories/StoreStackWorkspaceRecovery.stories.svelte catalog/stories/StoreStackWorkspaceSetup.stories.svelte
```

**Notes**

storybook-final.log: no files matched incorrect paths; zero product stories ran. Corrected to the actual component-family paths.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceProduction.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceRecovery.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceSetup.stories.svelte catalog/components/store-stack-workspace/StoreStackWorkspaceFooter.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte
```

**Notes**

storybook-final-corrected.log: 28/28 in 6 families including ordinary repository Log; interaction/accessibility evidence, no screenshot comparisons. Earlier 17/17 recorded in storybook.log.
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
pnpm web:build
```

**Notes**

web-build-final.log: product bundle built, docs staging rejected a stale generated installation reference after the new endpoint. Regenerated docs and source review fingerprints; fixed final build qualifies the cold run.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
node scripts/stacks-browser-acceptance.mjs
```

**Notes**

cold-browser.log: first core test omitted explicit Prepare graph index. Corrected without removing real ancestor browsing assertion.
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
node scripts/stacks-browser-acceptance.mjs
```

**Notes**

cold-browser-rerun.log: JJ apply succeeded but eager new-operation history required another graph index, blocking UI settlement. Native operation changes now invalidate lazy ranges; a direct adapter regression proves completion remains independent.
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
node scripts/stacks-browser-acceptance.mjs
```

**Notes**

cold-browser-final.log: 1/1 cold built production browser; real setup/apply/unapply/create/switch/refresh/reload, separate WIP, TOML exclusion, unchanged companion ref, real adjacent Unstaged diff, disabled hunk transfer, axe and runtime checks. Disposable loopback service cleaned up; installed services unchanged.
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
cargo build -p jj-store-cli --bin jj --bin jj-store --locked
```

**Notes**

dev-build-final.log: explicit development executables built for cold acceptance. Existing duplicate macOS library linker warning remains; no installed executable replacement.
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
pnpm contracts:generate
```

**Notes**

contracts-generate.log: OpenAPI and public TypeScript generated for the local core state/commands.
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
pnpm docs:generate
```

**Notes**

docs-generate-final.log: parser references regenerated, including local core command/state schema.
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
pnpm docs:review local "Reviewed ordinary jj-lib snapshot/merge/rebase/checkout, local TOML namespaces, WIP retention and conflict-state projection; historical archive execution is outside this path."
```

**Notes**

Final local source fingerprint recorded; earlier same-topic review superseded after conflict projection.
<!-- /run -->
<!-- /evidence-field -->

