---
schema: 1
id: E-UI-006
---

# E-UI-006 details 5

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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test:storybook catalog/components/store-application/RepositoryReview.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte
```

**Notes**

Landing-page rerun after moving the snapshot selector into the directory card failed one review assertion: the new summary row height no longer matched the file-tree action row exactly; the removed toolbar-controls CSS also emitted an unused-selector warning.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test:storybook catalog/components/store-application/RepositoryReview.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte
```

**Notes**

Second landing-page rerun failed only the README tab styling assertion because Design Core tab triggers still computed centered content after adding the document icon and table-header background split.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test:storybook catalog/components/store-application/RepositoryReview.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte
```

**Notes**

Seven focused stories pass with the branch picker inside the directory commit/action row, bordered/radiused file table, distinct table-header grey, GitHub-style README preview tabs, safe markdown preview and markdown source toggle.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test:storybook catalog/components/store-application/RepositoryReview.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte catalog/components/community-projects/CommunityProjectsRevisionGraphIds.stories.svelte catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.stories.svelte
```

**Notes**

Nineteen focused stories pass after the repository landing updates, including review refinements, repository log interactions, connected application files, split native-ID controls and branch cards.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm format:check
```

**Notes**

Rustfmt and Prettier pass after the repository landing table, tabs, markdown preview and ledger/spec updates.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check
```

**Notes**

Cold catalog browser lane failed because its README flow still expected clicking README.md to render a CodeBlock immediately; markdown files now open safe preview mode first.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm catalog:test:browser
```

**Notes**

Cold catalog browser contract passes after checking README markdown preview, verifying script text is inert, switching to Source mode and then asserting the CodeBlock and font contract.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check
```

**Notes**

All 18 specification lanes pass after the repository landing markdown/table updates: cold catalog browser, catalog check, 20 catalog unit tests, 37 Storybook interaction/axe stories, static catalog build, Rust tests, mdBook and mapped spec-first coverage.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check
```

**Notes**

Normal check passes after repository landing updates: spec validation, boundaries, progress bookkeeping, rustfmt, contracts, web/catalog Svelte checks and workspace Clippy.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test
```

**Notes**

Aggregate test passes after repository landing updates: governance, RustFS harness self-test, contract tests, 8 web tests, 20 catalog unit tests, 37 Storybook interaction/axe stories, cold catalog browser and the locked Rust workspace tests. Six external-service/credential cases remain intentionally ignored.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build
```

**Notes**

Static Storybook, existing web bundle/manifest and locked Rust workspace build pass after repository landing markdown/table updates.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

[]
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Community d859f33a484a9238bb11f974f4ead8e9be729ec6; released Design Core 0.2.0 via ^0.2.0; Storybook 10.5.7; Svelte CSF 5.1.2; Vitest 4.1.11.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Storybook-only presentation layout change. Selected-file Rust highlighting uses CodeBlock custom-tokenizer presentation data only; markdown previews render deterministic fixture text without executing HTML. No API, storage, native object, credential or backend format changes. Enclosing Jujutsu change is the version-control evidence reference.
<!-- /evidence-field -->

