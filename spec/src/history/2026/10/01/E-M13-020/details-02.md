---
schema: 1
id: E-M13-020
---

# E-M13-020 details 2

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
jj new @- local-browser@- -m "Merge local JJ browser support into default workspace"
```

**Notes**

Main 4dee949b and local-browser e88073b6 merged at common ancestor 2994c626. Resolved 14 conflicted paths; retained the 237 unique evidence entries from both parents without rewriting prior signatures.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/local-browser-merge.test.ts
```

**Notes**

Before: mixed Thread home selection, local capability visibility/state discard and Thread restoration without code selection failed. .scratch/local-browser-merge-before.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/local-browser-merge.test.ts
```

**Notes**

Exact after: all 3 regression tests pass. .scratch/local-browser-merge-after.log.
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
pnpm web:test
```

**Notes**

66 web unit tests pass. .scratch/local-browser-merge-web-units.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite
```

**Notes**

154 tests in 23 files pass. .scratch/local-browser-merge-web-consumers.log.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 8
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check
```

**Notes**

Automatic merge duplicated four generated schema aliases: eight TypeScript diagnostics in generated.ts. Regenerated public contracts from merged Rust authority; no hand-edited schema suppression. .scratch/local-browser-merge-web-check.log.
<!-- /run -->

<!-- run -->
### Run 7

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

Automatic merge duplicated four generated schema aliases: eight TypeScript diagnostics in generated.ts. Regenerated public contracts from merged Rust authority; no hand-edited schema suppression. .scratch/local-browser-merge-catalog-check.log.
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
pnpm check:progress
```

**Notes**

119 tasks pass, auditing both direct merge parents and new integration evidence.
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
pnpm check:boundaries
```

**Notes**

16 package boundaries pass.
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
pnpm check:architecture
```

**Notes**

Combined controller exceeded 300-line budget (303 counted lines). Moved display projection into its existing owning view module; no budget waiver.
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
pnpm check:architecture
```

**Notes**

First narrow extraction still had 302 counted lines. Extracted the controller surface together with view state, preserving live getters and source ownership.
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

491 production/story/script modules now satisfy unchanged budgets.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-threads/StoreThreads.stories.svelte catalog/components/store-threads/StoreCapturedReactions.stories.svelte
```

**Notes**

Initial cold import suite failed during dependency optimizer reload/virtual CSS import; two executed cases passed. No assertion or dependency changes. .scratch/local-browser-merge-threads-stories.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-threads/StoreThreads.stories.svelte catalog/components/store-threads/StoreCapturedReactions.stories.svelte
```

**Notes**

Exact fresh-process retry: both files, all 5 interaction/accessibility cases pass. .scratch/local-browser-merge-threads-stories-retry.log.
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
CARGO_BUILD_JOBS=1 CARGO_INCREMENTAL=0 cargo test -p jj-store-http --lib --locked
```

**Notes**

All 13 HTTP library/contract tests pass, including combined 84-route inventory and native/local identity contracts. .scratch/local-browser-merge-http-lib.log.
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
CARGO_BUILD_JOBS=1 CARGO_INCREMENTAL=0 pnpm contracts:generate
```

**Notes**

Generated both public artifacts from merged Rust contracts, preserving native Thread/Reactions and local browser API. .scratch/local-browser-merge-contract-generate.log.
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
pnpm web:check
```

**Notes**

zero errors/warnings. .scratch/local-browser-merge-web-check-final.log.
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
pnpm catalog:check
```

**Notes**

zero errors/warnings. .scratch/local-browser-merge-catalog-check-final.log.
<!-- /run -->
<!-- /evidence-field -->

