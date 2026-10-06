---
schema: 1
id: E-UI-006
---

# E-UI-006 details 4

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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test:storybook catalog/components/store-application/RepositoryReview.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte
```

**Notes**

Second equal-width card rerun failed only the old collapse assertion. The operation card correctly stayed visible while the metadata row collapsed.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test:storybook catalog/components/store-application/RepositoryReview.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte
```

**Notes**

15 focused stories pass with Op Log operation and file diff rendered as equal-width stacked cards in one Column Canvas column; collapse hides metadata only, parent navigation remains available, and no standalone Operation details column renders.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm format:check
```

**Notes**

Rustfmt and Prettier pass after the equal-width Op Log stacked-card adjustment.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test:storybook catalog/components/store-application/RepositoryReview.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte
```

**Notes**

First empty-state polish rerun loaded 7 tests before Vite reoptimized the new file-search/file-warning icon imports, causing one dynamic-import suite failure; the card close-button hover assertion also stayed unchanged because the test runner did not expose CSS :hover through userEvent.hover.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test:storybook catalog/components/store-application/RepositoryReview.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte
```

**Notes**

After prebundling the new Lucide icons and retargeting global data-attribute styles, all 15 stories loaded; both Op Log stories still failed only the computed-style hover comparison, so the hover CSS stayed in product code and the regression assertion changed to visible themed close-action styling.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test:storybook catalog/components/store-application/RepositoryReview.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte
```

**Notes**

15 focused stories pass with shadcn Empty icon states for no selected Op Log file and binary/unavailable diffs, single-border operation metadata, themed copy/parent pills, styled close action and stable full-app drill-down.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm format:check
```

**Notes**

Rustfmt passed; Prettier reported four touched Svelte files after the Empty and operation metadata styling edits. Ran targeted Prettier over touched source/spec/config files.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm format:check
```

**Notes**

Rustfmt and Prettier pass after formatting the touched Storybook and repository-log Svelte files.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check
```

**Notes**

Run was intentionally interrupted after the cold spec lane started because new review comments changed the branch-card divider, operation-card radius and split native-ID copy contract.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test:storybook catalog/components/store-application/RepositoryReview.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte catalog/components/community-projects/CommunityProjectsRevisionGraphIds.stories.svelte catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.stories.svelte
```

**Notes**

19 focused stories pass with final bookmark-card dividers and square lower corners, Column Canvas radius on stacked Op Log cards, split commit/change copy controls for revisions, operation-only ID buttons unchanged, Empty icon states and connected full-app drill-down intact.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm format:check
```

**Notes**

Rustfmt and Prettier pass after the branch-card, stacked-card radius and split-ID copy control updates.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check
```

**Notes**

All 18 specification lanes pass after the latest Storybook shell polish: cold Storybook/browser contract, catalog check, 20 catalog unit tests, 37 Storybook interaction/axe cases, static build, browser contracts, rustfmt, Rust tests, mdBook and mapped spec-first coverage.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test:storybook catalog/components/community-projects/CommunityProjectsRevisionGraphIds.stories.svelte
```

**Notes**

Two native-ID stories pass after wrapping each split copy target in shadcn Tooltip; hovering each half exposes the full commit or change value before copy.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test:storybook catalog/components/store-application/RepositoryReview.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte catalog/components/community-projects/CommunityProjectsRevisionGraphIds.stories.svelte catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.stories.svelte
```

**Notes**

19 focused stories still pass with the split ID shadcn tooltip contract plus full app, repository log and branch-card refinements.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm format:check
```

**Notes**

Rustfmt and Prettier pass after adding shadcn tooltips to split ID copy controls.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test:storybook catalog/components/store-application/RepositoryReview.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte
```

**Notes**

First selected-file source preview rerun failed because forcing CodeBlock span mode for all files exposed duplicate plain-space keys in Design Core CodeBlock, and the source preview story could not complete while the Svelte error was unhandled.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test:storybook catalog/components/store-application/RepositoryReview.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte
```

**Notes**

Second selected-file source preview rerun passed rendering but kept a stale exact three-line stats assertion for the merged Rust fixture, which correctly has four non-empty lines.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test:storybook catalog/components/store-application/RepositoryReview.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte
```

**Notes**

Seven focused stories pass with selected-file metadata, deterministic file statistics, filename source headers, theme-background source body, Rust source tokens via the public CodeBlock custom tokenizer, and existing Files navigation preserved.
<!-- /run -->
<!-- /evidence-field -->

