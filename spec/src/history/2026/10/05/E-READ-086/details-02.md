---
schema: 1
id: E-READ-086
---

# E-READ-086 details 2

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
cargo test -p jj-store-cli --test native_lifecycle path_metadata_crosses_merges_and_retains_the_ancestry_bound --locked
```

**Notes**

Initial native-before.log had two test-harness receiver errors; wrapped the CommittedRead in Arc before establishing the product baseline.
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
cargo test -p jj-store-cli --test native_lifecycle path_metadata_crosses_merges_and_retains_the_ancestry_bound --locked
```

**Notes**

native-before-corrected.log: exact regression returned Unavailable/Merge for a.txt.
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
cargo test -p jj-store-cli --test local_browser local_path_metadata_crosses_merges_using_first_parent_history --locked
```

**Notes**

local-before.log: real JJ merge fixture returned Unavailable/Merge for a.txt.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreDirectoryNavigation.stories.svelte
```

**Notes**

skeleton-before.log: folder navigation with a retained snapshot had no Loading repository files skeleton.
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
cargo test -p jj-store-cli --test native_lifecycle path_metadata_crosses_merges_and_retains_the_ancestry_bound --locked
```

**Notes**

native-after.log: merge integration and unchanged-path attribution passed; 70 unchanged descendants retained Budget beyond the existing 64-visit cap.
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
cargo test -p jj-store-cli --test local_browser local_path_metadata_crosses_merges_using_first_parent_history --locked
```

**Notes**

local-after.log: one real local JJ merge regression passed.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreDirectoryNavigation.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryDirectory.stories.svelte
```

**Notes**

stories-after.log: three cold browser stories passed, including retained-snapshot navigation, pending optional metadata and existing document interactions.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/production-file-reader.test.ts
```

**Notes**

file-reader.log: four lifecycle tests passed; rows stay independent of metadata/documents, and cancellation/failure preserve selection.
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
pnpm web:check
```

**Notes**

web-check.log: zero errors or warnings.
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
pnpm catalog:check
```

**Notes**

catalog-check.log: new story fixture getter inferred string instead of StoreLoadStatus; fixed its explicit return type.
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
pnpm catalog:check
```

**Notes**

catalog-check-final.log: zero errors/warnings and 196 owned catalog modules.
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
pnpm check:ui-ownership; pnpm check:architecture; pnpm exec spec-validator validate --only governance,summary,verification,book
```

**Notes**

Ownership and 1000-module architecture passed; four canonical validators passed across 23 chapters and 172 requirements.
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
cargo fmt -p jj-store-jj -p jj-store-cli -- --check
```

**Notes**

rust-format.log: affected Rust formatting passed.
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
pnpm exec prettier --write catalog/components/store-application/StoreDirectoryNavigation.stories.svelte catalog/fixtures/RepositoryDirectoryNavigationFixture.svelte web/src/lib/ui/community-projects/CommunityProjectsRepositoryFiles.svelte spec/src/api.md spec/src/jj-integration.md spec/src/local-jj.md spec/src/cli.md spec/src/ui.md
```

**Notes**

format.log: affected presentation and canonical chapters formatted.
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
cargo check -p jj-store-jj -p jj-store-cli --locked
```

**Notes**

rust-check.log: affected packages and their required dependency checks passed.
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
cargo test -p jj-store-cli --test native_lifecycle browsing_acceptance --locked
```

**Notes**

native-browsing.log: nine direct browsing/HTTP consumer tests passed. verification-scope.md records the escalation for both path-reader profiles and their direct consumers.
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
cargo test -p jj-store-cli --test local_browser bounded_reads --locked
```

**Notes**

local-bounded.log: seven passed, one existing optional Linux fixture ignored. Actual original Linux acceptance was run separately.
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
JJ_STORE_DEV_ROOT=/Users/stevejuma/code/lapismd/jj-store-codex-preview/.local/codex-preview JJ_STORE_DEV_PORT=7470 JJ_STORE_DEV_BACKEND_PORT=7472 pnpm dev:backend:build
```

**Notes**

backend-build.log: built the explicit development backend; no installed CLI replacement. Existing linker unwind warning retained.
<!-- /run -->
<!-- /evidence-field -->

